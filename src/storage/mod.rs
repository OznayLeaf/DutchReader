//! Local persistence. All files live in the app-private directory and use the
//! atomic write layer in [`atomic`].

pub mod atomic;
pub mod progress;
pub mod settings;

use std::path::{Path, PathBuf};

/// Resolves the paths of every storage file the app uses.
#[derive(Debug, Clone)]
pub struct Storage {
    root: PathBuf,
}

impl Storage {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Storage { root: root.into() }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn settings_path(&self) -> PathBuf {
        self.root.join("settings.json")
    }

    pub fn progress_path(&self) -> PathBuf {
        self.root.join("progress.json")
    }

    pub fn dictionary_path(&self) -> PathBuf {
        self.root.join("dictionary.json")
    }

    pub fn tapped_words_path(&self) -> PathBuf {
        self.root.join("tapped_words.json")
    }

    pub fn lookup_cache_path(&self) -> PathBuf {
        self.root.join("lookup_cache.json")
    }

    pub fn writing_dir(&self) -> PathBuf {
        self.root.join("writing")
    }

    pub fn draft_path(&self) -> PathBuf {
        self.writing_dir().join("draft.json")
    }

    pub fn writings_path(&self) -> PathBuf {
        self.writing_dir().join("stories.json")
    }

    pub fn load_settings(&self) -> settings::Settings {
        settings::Settings::load(&self.settings_path())
    }

    pub fn save_settings(&self, settings: &settings::Settings) -> std::io::Result<()> {
        settings.save(&self.settings_path())
    }

    pub fn load_progress(&self) -> progress::Progress {
        progress::Progress::load(&self.progress_path())
    }

    pub fn save_progress(&self, progress: &progress::Progress) -> std::io::Result<()> {
        progress.save(&self.progress_path())
    }

    pub fn load_dictionary(&self) -> crate::dictionary::Dictionary {
        crate::dictionary::Dictionary::load(&self.dictionary_path())
    }

    pub fn save_dictionary(
        &self,
        dictionary: &crate::dictionary::Dictionary,
    ) -> std::io::Result<()> {
        dictionary.save(&self.dictionary_path())
    }

    pub fn load_tapped_words(&self) -> crate::dictionary::TappedWords {
        crate::dictionary::TappedWords::load(&self.tapped_words_path())
    }

    pub fn save_tapped_words(
        &self,
        tapped: &crate::dictionary::TappedWords,
    ) -> std::io::Result<()> {
        tapped.save(&self.tapped_words_path())
    }

    pub fn load_lookup_cache(&self) -> crate::lookup::LookupCache {
        crate::lookup::LookupCache::load(&self.lookup_cache_path())
    }

    pub fn save_lookup_cache(&self, cache: &crate::lookup::LookupCache) -> std::io::Result<()> {
        cache.save(&self.lookup_cache_path())
    }
}

/// Pick a writable data directory.
///
/// - Android passes the app's internal data directory from `android_main`.
/// - Desktop preview honours `DUTCHREADER_DATA_DIR`, then uses a local folder
///   next to the project so development data is easy to inspect.
pub fn resolve_data_dir(android_dir: Option<PathBuf>) -> PathBuf {
    if let Some(dir) = android_dir {
        return dir;
    }
    if let Ok(dir) = std::env::var("DUTCHREADER_DATA_DIR")
        && !dir.trim().is_empty()
    {
        return PathBuf::from(dir);
    }
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".dutchreader-data")
}
