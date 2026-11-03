//! Text-to-speech bridge.
//!
//! A single worker thread owns the Android `TextToSpeech` object. The UI sends
//! commands through a channel and receives [`TtsEvent`]s from another channel
//! that the UI timer polls. The worker speaks one sentence at a time and waits
//! for `isSpeaking()` to become false, which gives the reader a sentence
//! highlight and lets a page turn replace the queue cleanly.
//!
//! On the desktop the engine reports a friendly "Android only" error instead of
//! touching JNI, so the preview and the tests build and run normally.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread;
use std::time::{Duration, Instant};

/// Messages sent from the TTS worker to the UI thread.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TtsEvent {
    /// The worker started sentence `index` of the current page. The generation
    /// lets the UI ignore events from a page that has already been replaced.
    Sentence { generation: u64, index: usize },
    /// All queued sentences finished normally.
    Finished { generation: u64 },
    /// Initialization or speaking failed. The message is shown to the learner.
    Error { generation: u64, message: String },
}

#[derive(Debug)]
enum Command {
    Speak {
        generation: u64,
        sentences: Vec<String>,
    },
    Stop,
    Speed(f32),
}

/// UI-side handle. Cheap to hold in the app state; the worker thread exits when
/// this handle (and therefore the command sender) is dropped.
pub struct TtsHandle {
    commands: Sender<Command>,
    generation: AtomicU64,
}

impl TtsHandle {
    /// Start the worker and return the handle plus the event receiver.
    pub fn start(initial_speed: f32) -> (TtsHandle, Receiver<TtsEvent>) {
        let (command_tx, command_rx) = mpsc::channel();
        let (event_tx, event_rx) = mpsc::channel();
        thread::Builder::new()
            .name("dutchreader-tts".into())
            .spawn(move || worker(command_rx, event_tx, initial_speed))
            .expect("could not start the TTS worker thread");
        (
            TtsHandle {
                commands: command_tx,
                generation: AtomicU64::new(0),
            },
            event_rx,
        )
    }

    /// Replace whatever is being spoken with `sentences`. Returns the new
    /// generation; events from older generations must be ignored by the UI.
    pub fn speak(&self, sentences: Vec<String>) -> u64 {
        let generation = self.generation.fetch_add(1, Ordering::SeqCst) + 1;
        let _ = self.commands.send(Command::Speak {
            generation,
            sentences,
        });
        generation
    }

    /// Stop immediately and invalidate in-flight events.
    pub fn stop(&self) {
        self.generation.fetch_add(1, Ordering::SeqCst);
        let _ = self.commands.send(Command::Stop);
    }

    /// Change the Android speech rate (0.5 - 2.0). Applies to the next
    /// utterance and, on most devices, to the current one as well.
    pub fn set_speed(&self, speed: f32) {
        let _ = self.commands.send(Command::Speed(speed));
    }
}

#[derive(Debug)]
struct Job {
    generation: u64,
    sentences: Vec<String>,
    index: usize,
}

#[derive(Debug, PartialEq, Eq)]
enum JobOutcome {
    Finished,
    Stopped,
    ChannelClosed,
}

fn worker(commands: Receiver<Command>, events: Sender<TtsEvent>, initial_speed: f32) {
    let mut speed = initial_speed;
    // Initialization is slow (the TTS service binds asynchronously), so do it
    // once at startup rather than on the first tap.
    let mut engine = engine::Engine::new(speed);

    loop {
        match commands.recv() {
            Ok(Command::Speak {
                generation,
                sentences,
            }) => {
                let job = Job {
                    generation,
                    sentences,
                    index: 0,
                };
                let job_generation = job.generation;
                match run_job(&mut engine, job, &commands, &events, &mut speed) {
                    JobOutcome::Finished => {
                        let _ = events.send(TtsEvent::Finished {
                            generation: job_generation,
                        });
                    }
                    JobOutcome::Stopped => {}
                    JobOutcome::ChannelClosed => break,
                }
            }
            Ok(Command::Stop) => {
                if let Ok(engine) = &mut engine {
                    engine.stop();
                }
            }
            Ok(Command::Speed(new_speed)) => {
                speed = new_speed;
                if let Ok(engine) = &mut engine {
                    let _ = engine.set_speed(speed);
                }
            }
            Err(_) => break,
        }
    }
}

/// Speak one job to completion. New `Speak` commands replace the job; `Stop`
/// ends it. This is where page turns take over the narration.
fn run_job(
    engine: &mut Result<engine::Engine, String>,
    mut job: Job,
    commands: &Receiver<Command>,
    events: &Sender<TtsEvent>,
    speed: &mut f32,
) -> JobOutcome {
    'sentence: loop {
        // Apply every queued command before the next sentence starts. This is
        // what makes a fast on/off tap safe while the engine is initializing.
        loop {
            match commands.try_recv() {
                Ok(Command::Stop) => {
                    if let Ok(engine) = engine {
                        engine.stop();
                    }
                    return JobOutcome::Stopped;
                }
                Ok(Command::Speed(new_speed)) => {
                    *speed = new_speed;
                    if let Ok(engine) = engine {
                        let _ = engine.set_speed(*speed);
                    }
                }
                Ok(Command::Speak {
                    generation,
                    sentences,
                }) => {
                    if let Ok(engine) = engine {
                        engine.stop();
                    }
                    job = Job {
                        generation,
                        sentences,
                        index: 0,
                    };
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => return JobOutcome::ChannelClosed,
            }
        }

        if job.index >= job.sentences.len() {
            return JobOutcome::Finished;
        }

        let index = job.index;
        let sentence = job.sentences[index].clone();
        let engine = match engine {
            Ok(engine) => engine,
            Err(error) => {
                let _ = events.send(TtsEvent::Error {
                    generation: job.generation,
                    message: error.clone(),
                });
                return JobOutcome::Stopped;
            }
        };

        if let Err(error) = engine.speak(&sentence, job.generation, index) {
            let _ = events.send(TtsEvent::Error {
                generation: job.generation,
                message: error,
            });
            return JobOutcome::Stopped;
        }
        let _ = events.send(TtsEvent::Sentence {
            generation: job.generation,
            index,
        });
        job.index += 1;

        // Wait for this utterance to finish, while still reacting to commands.
        let started = Instant::now();
        let mut seen_speaking = false;
        let mut waited_ms = 0u64;
        loop {
            thread::sleep(Duration::from_millis(50));
            waited_ms += 50;

            let mut replacement = None;
            loop {
                match commands.try_recv() {
                    Ok(Command::Stop) => {
                        engine.stop();
                        return JobOutcome::Stopped;
                    }
                    Ok(Command::Speed(new_speed)) => {
                        *speed = new_speed;
                        let _ = engine.set_speed(*speed);
                    }
                    Ok(Command::Speak {
                        generation,
                        sentences,
                    }) => {
                        engine.stop();
                        replacement = Some(Job {
                            generation,
                            sentences,
                            index: 0,
                        });
                        break;
                    }
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => return JobOutcome::ChannelClosed,
                }
            }
            if let Some(next) = replacement {
                job = next;
                continue 'sentence;
            }

            let speaking = engine.is_speaking();
            if speaking {
                seen_speaking = true;
            }
            if seen_speaking && !speaking {
                break;
            }
            // Some very short utterances never report `true` because the poll
            // misses the window; treat them as done after a short grace period.
            if !seen_speaking && waited_ms >= 800 {
                break;
            }
            if started.elapsed() > Duration::from_secs(120) {
                engine.stop();
                let _ = events.send(TtsEvent::Error {
                    generation: job.generation,
                    message: "Narration stopped because the sentence took too long.".into(),
                });
                return JobOutcome::Stopped;
            }
        }
    }
}

/// Missing-voice message shown in the reader toast and in the popup note.
pub const MISSING_VOICE_MESSAGE: &str = "Dutch text-to-speech is not available. Install a Dutch voice in Android Settings → System → Languages & input → Text-to-speech output, then reopen the app.";

#[cfg(target_os = "android")]
mod engine {
    use super::MISSING_VOICE_MESSAGE;
    use jni::objects::{GlobalRef, JObject, JValue};
    use jni::{JNIEnv, JavaVM};
    use std::thread;
    use std::time::{Duration, Instant};

    pub struct Engine {
        vm: JavaVM,
        tts: GlobalRef,
    }

    impl Engine {
        pub fn new(speed: f32) -> Result<Engine, String> {
            let context = ndk_context::android_context();
            let vm = unsafe { JavaVM::from_raw(context.vm().cast()) }
                .map_err(|error| format!("could not access the Android Java VM: {error}"))?;
            let mut env = vm
                .attach_current_thread()
                .map_err(|error| format!("could not attach the TTS thread: {error}"))?;

            // The activity owns this local reference; never delete it from here.
            let app_context = unsafe { JObject::from_raw(context.context().cast()) };
            let null_listener = JObject::null();
            let local = env
                .new_object(
                    "android/speech/tts/TextToSpeech",
                    "(Landroid/content/Context;Landroid/speech/tts/TextToSpeech$OnInitListener;)V",
                    &[JValue::Object(&app_context), JValue::Object(&null_listener)],
                )
                .map_err(|error| format!("could not create Android TextToSpeech: {error}"))?;
            std::mem::forget(app_context);

            let tts = env
                .new_global_ref(&local)
                .map_err(|error| format!("could not keep the TextToSpeech object: {error}"))?;
            let dutch_nl = make_locale(&mut env, "nl", "NL")?;
            let dutch = make_locale(&mut env, "nl", "")?;
            // The working locale has already been applied with setLanguage; the
            // Java TTS object keeps its own reference, so the global ref can go.
            let _ = wait_for_dutch(&mut env, &tts, &dutch_nl, &dutch)?;
            drop(env);

            let mut engine = Engine { vm, tts };
            engine.set_speed(speed)?;
            Ok(engine)
        }

        pub fn set_speed(&mut self, speed: f32) -> Result<(), String> {
            let mut env = self.attach()?;
            let _ = env
                .call_method(
                    self.tts.as_obj(),
                    "setSpeechRate",
                    "(F)I",
                    &[JValue::Float(speed.clamp(0.5, 2.0))],
                )
                .map_err(|error| format!("could not set the speech rate: {error}"))?;
            clear_exception(&mut env);
            Ok(())
        }

        pub fn speak(&mut self, text: &str, generation: u64, index: usize) -> Result<(), String> {
            let mut env = self.attach()?;
            let text = JObject::from(
                env.new_string(text)
                    .map_err(|error| format!("could not send text to TTS: {error}"))?,
            );
            let utterance = JObject::from(
                env.new_string(format!("dutchreader-{generation}-{index}"))
                    .map_err(|error| format!("could not create an utterance id: {error}"))?,
            );
            let null_bundle = JObject::null();
            let result = env
                .call_method(
                    self.tts.as_obj(),
                    "speak",
                    "(Ljava/lang/CharSequence;ILandroid/os/Bundle;Ljava/lang/String;)I",
                    &[
                        JValue::Object(&text),
                        JValue::Int(0), // TextToSpeech.QUEUE_FLUSH
                        JValue::Object(&null_bundle),
                        JValue::Object(&utterance),
                    ],
                )
                .map_err(|error| format!("could not call TextToSpeech.speak: {error}"))?
                .i()
                .map_err(|error| format!("could not read the TTS result: {error}"))?;
            clear_exception(&mut env);
            if result != 0 {
                return Err(format!(
                    "Android TextToSpeech could not start speaking (error {result})."
                ));
            }
            Ok(())
        }

        pub fn is_speaking(&mut self) -> bool {
            let Ok(mut env) = self.attach() else {
                return false;
            };
            let speaking = env
                .call_method(self.tts.as_obj(), "isSpeaking", "()Z", &[])
                .ok()
                .and_then(|value| value.z().ok())
                .unwrap_or(false);
            clear_exception(&mut env);
            speaking
        }

        pub fn stop(&mut self) {
            let Ok(mut env) = self.attach() else {
                return;
            };
            let _ = env.call_method(self.tts.as_obj(), "stop", "()I", &[]);
            clear_exception(&mut env);
        }

        fn attach(&self) -> Result<jni::AttachGuard<'_>, String> {
            self.vm
                .attach_current_thread()
                .map_err(|error| format!("could not attach the TTS thread: {error}"))
        }
    }

    impl Drop for Engine {
        fn drop(&mut self) {
            if let Ok(mut env) = self.attach() {
                let _ = env.call_method(self.tts.as_obj(), "shutdown", "()V", &[]);
                clear_exception(&mut env);
            }
        }
    }

    fn make_locale(env: &mut JNIEnv, language: &str, country: &str) -> Result<GlobalRef, String> {
        let language = JObject::from(
            env.new_string(language)
                .map_err(|error| format!("could not create a locale: {error}"))?,
        );
        let country = JObject::from(
            env.new_string(country)
                .map_err(|error| format!("could not create a locale: {error}"))?,
        );
        let locale = env
            .new_object(
                "java/util/Locale",
                "(Ljava/lang/String;Ljava/lang/String;)V",
                &[JValue::Object(&language), JValue::Object(&country)],
            )
            .map_err(|error| format!("could not create the Dutch locale: {error}"))?;
        env.new_global_ref(&locale)
            .map_err(|error| format!("could not keep the Dutch locale: {error}"))
    }

    /// The service binds asynchronously and `setLanguage` returns an error
    /// until it is ready. Retry nl-NL first, then plain `nl` for devices that
    /// only ship a language-level voice.
    fn wait_for_dutch(
        env: &mut JNIEnv,
        tts: &GlobalRef,
        dutch_nl: &GlobalRef,
        dutch: &GlobalRef,
    ) -> Result<GlobalRef, String> {
        let started = Instant::now();
        let mut candidate = dutch_nl;
        loop {
            let result = env
                .call_method(
                    tts.as_obj(),
                    "setLanguage",
                    "(Ljava/util/Locale;)I",
                    &[JValue::Object(candidate.as_obj())],
                )
                .map_err(|error| format!("could not set the TTS language: {error}"))?
                .i()
                .map_err(|error| format!("could not read the TTS language result: {error}"))?;
            clear_exception(env);
            if result >= 0 {
                return Ok(candidate.clone());
            }

            let elapsed = started.elapsed();
            if std::ptr::eq(candidate, dutch_nl) && elapsed > Duration::from_millis(2_500) {
                candidate = dutch;
            }
            if elapsed > Duration::from_secs(6) {
                return Err(MISSING_VOICE_MESSAGE.to_string());
            }
            thread::sleep(Duration::from_millis(100));
        }
    }

    fn clear_exception(env: &mut JNIEnv) {
        if env.exception_check().unwrap_or(false) {
            let _ = env.exception_clear();
        }
    }
}

#[cfg(not(target_os = "android"))]
mod engine {
    pub struct Engine;

    impl Engine {
        pub fn new(_speed: f32) -> Result<Engine, String> {
            Err("Narration is only available in the Android app.".into())
        }

        pub fn set_speed(&mut self, _speed: f32) -> Result<(), String> {
            Ok(())
        }

        pub fn speak(
            &mut self,
            _text: &str,
            _generation: u64,
            _index: usize,
        ) -> Result<(), String> {
            Err("Narration is only available in the Android app.".into())
        }

        pub fn is_speaking(&mut self) -> bool {
            false
        }

        pub fn stop(&mut self) {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn speak_increments_the_generation() {
        let (tts, _events) = TtsHandle::start(1.0);
        let first = tts.speak(vec!["Hallo.".into()]);
        let second = tts.speak(vec!["Tot ziens.".into()]);
        assert_eq!(first, 1);
        assert_eq!(second, 2);
        tts.stop();
    }

    #[test]
    fn desktop_worker_reports_a_clear_error() {
        let (tts, events) = TtsHandle::start(1.0);
        tts.speak(vec!["Hallo.".into()]);
        let event = events.recv_timeout(Duration::from_secs(2)).expect("event");
        match event {
            TtsEvent::Error {
                generation,
                message,
            } => {
                assert_eq!(generation, 1);
                assert!(message.contains("Android"));
            }
            other => panic!("unexpected event: {other:?}"),
        }
    }
}
