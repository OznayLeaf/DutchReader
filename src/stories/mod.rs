//! Story loading, indexing, word tracking, and validation.
//!
//! Stories are data: `build.rs` embeds every `assets/stories/<level>/*.json`
//! file, so adding a story never requires a Rust or Slint change.

pub mod index;
pub mod loader;
pub mod model;
pub mod starter;
pub mod tokenize;
pub mod validate;
pub mod wordlist;

pub use index::StoryIndex;
pub use loader::{LoadError, LoadedStory, load_dir, load_embedded};
pub use model::{GlossaryEntry, Level, Story};
pub use starter::StarterWords;
pub use wordlist::{AssumedKnown, Wordlist};
