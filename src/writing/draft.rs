//! The unfinished writing draft, stored in `writing/draft.json`.
//!
//! A draft is saved every few seconds while the editor is open, when the app is
//! backgrounded, and when the editor is left. Empty drafts are removed instead
//! of written, including their backup, so an old draft can never be resurrected
//! by the corruption fallback.

use crate::storage::atomic::write_json_atomic;
use serde::{Deserialize, Serialize};
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Draft {
    #[serde(default)]
    pub text: String,
    /// Optional. Empty means "use the date".
    #[serde(default)]
    pub title: String,
    /// Unix timestamp (seconds) when this draft was started.
    #[serde(default)]
    pub created_at: u64,
    /// Unix timestamp (seconds) of the last autosave.
    #[serde(default)]
    pub updated_at: u64,
    /// Whole seconds spent in the editor while the app was in the foreground.
    #[serde(default)]
    pub elapsed_seconds: u64,
    /// Set when the editor was opened from an existing saved story. Sharing then
    /// updates that story instead of creating a second copy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub editing_id: Option<String>,
}

impl Draft {
    /// Load a usable draft. A missing, empty, or whitespace-only draft is `None`.
    pub fn load(path: &Path) -> Option<Draft> {
        let loaded = crate::storage::atomic::load_json::<Draft>(path);
        if let Some(warning) = &loaded.warning {
            eprintln!("[writing] {warning}");
        }
        if loaded.value.is_empty() {
            None
        } else {
            Some(loaded.value)
        }
    }

    /// Persist the draft. Empty drafts are cleared instead of written.
    pub fn save(&self, path: &Path) -> io::Result<()> {
        if self.is_empty() {
            Self::clear(path)
        } else {
            write_json_atomic(path, self)
        }
    }

    /// Remove the draft and its atomic-write siblings. Missing files are fine.
    pub fn clear(path: &Path) -> io::Result<()> {
        for candidate in [
            path.to_path_buf(),
            with_suffix(path, ".bak"),
            with_suffix(path, ".tmp"),
        ] {
            match fs::remove_file(&candidate) {
                Ok(()) => {}
                Err(error) if error.kind() == io::ErrorKind::NotFound => {}
                Err(error) => return Err(error),
            }
        }
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        super::is_empty_text(&self.text)
    }
}

fn with_suffix(path: &Path, suffix: &str) -> PathBuf {
    let mut name: OsString = path.as_os_str().to_os_string();
    name.push(suffix);
    PathBuf::from(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    struct TestDir(PathBuf);

    impl TestDir {
        fn new() -> Self {
            static COUNTER: AtomicU64 = AtomicU64::new(0);
            let n = COUNTER.fetch_add(1, Ordering::Relaxed);
            let dir =
                std::env::temp_dir().join(format!("dutchreader-draft-{}-{n}", std::process::id()));
            let _ = fs::remove_dir_all(&dir);
            fs::create_dir_all(&dir).unwrap();
            TestDir(dir)
        }

        fn file(&self) -> PathBuf {
            self.0.join("writing").join("draft.json")
        }
    }

    impl Drop for TestDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn draft(text: &str) -> Draft {
        Draft {
            text: text.into(),
            title: "Mijn verhaal".into(),
            created_at: 1_700_000_000,
            updated_at: 1_700_000_100,
            elapsed_seconds: 42,
            editing_id: None,
        }
    }

    #[test]
    fn saves_and_loads_a_draft() {
        let dir = TestDir::new();
        let path = dir.file();
        draft("Hallo wereld.").save(&path).unwrap();
        let loaded = Draft::load(&path).expect("draft exists");
        assert_eq!(loaded, draft("Hallo wereld."));
    }

    #[test]
    fn empty_drafts_are_never_saved() {
        let dir = TestDir::new();
        let path = dir.file();
        draft("   ").save(&path).unwrap();
        assert!(!path.exists());
        assert!(Draft::load(&path).is_none());
    }

    #[test]
    fn clearing_removes_the_draft_and_its_backup() {
        let dir = TestDir::new();
        let path = dir.file();
        draft("Eerste versie.").save(&path).unwrap();
        draft("Tweede versie.").save(&path).unwrap();
        assert!(with_suffix(&path, ".bak").exists());

        Draft::clear(&path).unwrap();
        assert!(!path.exists());
        assert!(!with_suffix(&path, ".bak").exists());
        assert!(!with_suffix(&path, ".tmp").exists());
        assert!(Draft::load(&path).is_none());
    }

    #[test]
    fn saving_empty_text_clears_an_existing_draft_and_backup() {
        let dir = TestDir::new();
        let path = dir.file();
        draft("Eerste versie.").save(&path).unwrap();
        draft("Tweede versie.").save(&path).unwrap();
        draft("").save(&path).unwrap();
        assert!(!path.exists());
        assert!(!with_suffix(&path, ".bak").exists());
        assert!(Draft::load(&path).is_none());
    }

    #[test]
    fn a_corrupt_draft_recovers_from_the_backup() {
        let dir = TestDir::new();
        let path = dir.file();
        draft("Eerste versie.").save(&path).unwrap();
        draft("Tweede versie.").save(&path).unwrap();
        fs::write(&path, b"{ this is not json").unwrap();

        let loaded = Draft::load(&path).expect("backup draft");
        assert_eq!(loaded.text, "Eerste versie.");
    }
}
