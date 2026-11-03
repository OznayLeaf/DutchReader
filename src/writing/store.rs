//! Finished writings, stored in `writing/stories.json`.
//!
//! Sharing a story upserts it here. Editing a story keeps its original id and
//! creation time, so My Writing does not grow duplicates.

use super::draft::Draft;
use crate::storage::atomic::write_json_atomic;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoryEntry {
    pub id: String,
    /// Optional title; empty means the UI shows the creation date.
    #[serde(default)]
    pub title: String,
    pub text: String,
    /// Unix timestamp (seconds) when the story was first saved.
    #[serde(default)]
    pub created_at: u64,
    /// Unix timestamp (seconds) of the last edit/share.
    #[serde(default)]
    pub updated_at: u64,
    /// Stored so statistics do not have to re-tokenize every story on each open.
    #[serde(default)]
    pub word_count: usize,
    /// Whole seconds spent writing this story while the app was in the foreground.
    #[serde(default)]
    pub elapsed_seconds: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WritingStore {
    #[serde(default)]
    pub stories: Vec<StoryEntry>,
}

impl WritingStore {
    pub fn load(path: &Path) -> Self {
        let loaded = crate::storage::atomic::load_json::<WritingStore>(path);
        if let Some(warning) = &loaded.warning {
            eprintln!("[writing] {warning}");
        }
        loaded.value
    }

    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        write_json_atomic(path, self)
    }

    pub fn get(&self, id: &str) -> Option<&StoryEntry> {
        self.stories.iter().find(|story| story.id == id)
    }

    /// Newest first by creation date, then by last update and id so the order is
    /// deterministic when two stories were saved in the same second.
    pub fn newest_first(&self) -> Vec<&StoryEntry> {
        let mut stories: Vec<&StoryEntry> = self.stories.iter().collect();
        stories.sort_by(|a, b| {
            b.created_at
                .cmp(&a.created_at)
                .then_with(|| b.updated_at.cmp(&a.updated_at))
                .then_with(|| b.id.cmp(&a.id))
        });
        stories
    }

    /// Save a draft as a finished story. When the draft came from editing an
    /// existing story, that story is updated in place.
    pub fn upsert(&mut self, draft: &Draft, now: u64) -> StoryEntry {
        let text = draft.text.clone();
        let title = draft.title.trim().to_string();
        let word_count = super::word_count(&text);
        let elapsed_seconds = draft.elapsed_seconds;
        let created_at = if draft.created_at == 0 {
            now
        } else {
            draft.created_at
        };

        if let Some(id) = draft.editing_id.as_deref()
            && let Some(existing) = self.stories.iter_mut().find(|story| story.id == id)
        {
            existing.title = title;
            existing.text = text;
            existing.updated_at = now;
            existing.word_count = word_count;
            existing.elapsed_seconds = elapsed_seconds;
            return existing.clone();
        }

        let id = self.next_id(now);
        let story = StoryEntry {
            id,
            title,
            text,
            created_at,
            updated_at: now,
            word_count,
            elapsed_seconds,
        };
        self.stories.push(story.clone());
        story
    }

    /// Remove a story. Returns true when something was removed.
    pub fn remove(&mut self, id: &str) -> bool {
        let before = self.stories.len();
        self.stories.retain(|story| story.id != id);
        self.stories.len() != before
    }

    fn next_id(&self, now: u64) -> String {
        let base = format!("w-{now}");
        if self.get(&base).is_none() {
            return base;
        }
        let mut suffix = 2;
        loop {
            let candidate = format!("{base}-{suffix}");
            if self.get(&candidate).is_none() {
                return candidate;
            }
            suffix += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    struct TestDir(PathBuf);

    impl TestDir {
        fn new() -> Self {
            static COUNTER: AtomicU64 = AtomicU64::new(0);
            let n = COUNTER.fetch_add(1, Ordering::Relaxed);
            let dir =
                std::env::temp_dir().join(format!("dutchreader-store-{}-{n}", std::process::id()));
            let _ = fs::remove_dir_all(&dir);
            fs::create_dir_all(&dir).unwrap();
            TestDir(dir)
        }

        fn file(&self) -> PathBuf {
            self.0.join("writing").join("stories.json")
        }
    }

    impl Drop for TestDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn draft(text: &str, created_at: u64, elapsed: u64) -> Draft {
        Draft {
            text: text.into(),
            title: String::new(),
            created_at,
            updated_at: created_at,
            elapsed_seconds: elapsed,
            editing_id: None,
        }
    }

    #[test]
    fn sharing_a_draft_adds_a_story_with_a_word_count() {
        let mut store = WritingStore::default();
        let story = store.upsert(&draft("Een twee drie.", 100, 42), 200);
        assert_eq!(story.id, "w-200");
        assert_eq!(story.created_at, 100);
        assert_eq!(story.updated_at, 200);
        assert_eq!(story.word_count, 3);
        assert_eq!(story.elapsed_seconds, 42);
        assert!(store.get("w-200").is_some());
    }

    #[test]
    fn editing_updates_the_same_story() {
        let mut store = WritingStore::default();
        let original = store.upsert(&draft("Een twee.", 100, 10), 200);
        let mut edit = draft("Een twee drie vier.", 100, 30);
        edit.editing_id = Some(original.id.clone());
        let updated = store.upsert(&edit, 300);

        assert_eq!(updated.id, original.id);
        assert_eq!(updated.created_at, 100);
        assert_eq!(updated.updated_at, 300);
        assert_eq!(updated.word_count, 4);
        assert_eq!(store.stories.len(), 1);
    }

    #[test]
    fn newest_first_sorts_by_creation_then_update() {
        let mut store = WritingStore::default();
        store.upsert(&draft("oud", 100, 1), 100);
        store.upsert(&draft("nieuw", 300, 1), 300);
        store.upsert(&draft("midden", 200, 1), 200);

        let titles: Vec<&str> = store
            .newest_first()
            .iter()
            .map(|story| story.text.as_str())
            .collect();
        assert_eq!(titles, vec!["nieuw", "midden", "oud"]);
    }

    #[test]
    fn ids_do_not_collide_in_the_same_second() {
        let mut store = WritingStore::default();
        let first = store.upsert(&draft("een", 100, 1), 500);
        let second = store.upsert(&draft("twee", 100, 1), 500);
        assert_ne!(first.id, second.id);
        assert_eq!(second.id, "w-500-2");
    }

    #[test]
    fn remove_deletes_only_the_requested_story() {
        let mut store = WritingStore::default();
        let first = store.upsert(&draft("een", 100, 1), 100);
        store.upsert(&draft("twee", 200, 1), 200);
        assert!(store.remove(&first.id));
        assert!(store.get(&first.id).is_none());
        assert_eq!(store.stories.len(), 1);
        assert!(!store.remove("missing"));
    }

    #[test]
    fn saves_and_loads_through_the_atomic_layer() {
        let dir = TestDir::new();
        let path = dir.file();
        let mut store = WritingStore::default();
        store.upsert(&draft("Een verhaal.", 100, 5), 200);
        store.save(&path).unwrap();

        let loaded = WritingStore::load(&path);
        assert_eq!(loaded, store);
    }

    #[test]
    fn an_empty_title_is_kept_empty_for_the_date_fallback() {
        let mut store = WritingStore::default();
        let story = store.upsert(&draft("Een verhaal.", 100, 5), 200);
        assert_eq!(story.title, "");
    }
}
