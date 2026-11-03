//! Reader logic: pagination/layout, word rectangles, page timer, TTS control.
//!
//! Pagination and the 30-second timer are pure Rust with an injectable clock so
//! they can be tested on the desktop. TTS lives in `platform::tts`.

pub mod layout;
pub mod page_timer;
