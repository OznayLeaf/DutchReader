//! DutchReader: a local-first graded Dutch reader and writing app.
//!
//! Business logic lives in plain Rust modules. `platform/` holds the JNI
//! bridges (TTS, lifecycle) and `app.rs` is the only place that touches both
//! Slint and the logic modules.

pub mod app;
pub mod dictionary;
pub mod lookup;
pub mod platform;
pub mod reader;
pub mod storage;
pub mod stories;
pub mod time;
pub mod writing;

slint::include_modules!();

/// Android entry point. `android-activity` calls this on its own thread.
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub fn android_main(android_app: slint::android::AndroidApp) {
    use slint::android::android_activity::{MainEvent, PollEvent};

    let lifecycle = platform::lifecycle::Lifecycle::new();
    let listener_lifecycle = lifecycle.clone();
    if let Err(error) =
        slint::android::init_with_event_listener(android_app.clone(), move |event| {
            if let PollEvent::Main(main) = event {
                match main {
                    MainEvent::Resume { .. } => {
                        listener_lifecycle.set_foreground(true);
                    }
                    MainEvent::Pause | MainEvent::Stop | MainEvent::Destroy => {
                        listener_lifecycle.set_foreground(false);
                    }
                    _ => {}
                }
            }
        })
    {
        eprintln!("DutchReader: could not initialize the Slint Android backend: {error}");
        return;
    }

    let data_dir = android_app.internal_data_path();
    if let Err(error) = app::run_with_lifecycle(data_dir, lifecycle) {
        eprintln!("DutchReader: fatal startup error: {error}");
    }
}
