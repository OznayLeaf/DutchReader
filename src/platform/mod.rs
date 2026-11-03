//! Android platform bridges (JNI): TTS, share sheet, lifecycle, keyboard.
//!
//! Implemented in Phases 3 and 4. This is the only module allowed to touch JNI.

pub mod lifecycle;
pub mod share;
pub mod timezone;
pub mod tts;
