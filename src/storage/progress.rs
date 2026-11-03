//! Reading progress, stored in `progress.json`.
//!
//! The reading position is a byte offset into the story's joined text, not a
//! page number. Pagination is computed for the actual screen, so an offset
//! survives font-size or screen-size changes and still resumes at the same
//! sentence.

use super::atomic::{load_json, write_json_atomic};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoryProgress {
    /// Byte offset into the story text where the reader was last open.
    #[serde(default)]
    pub last_offset: usize,
    #[serde(default)]
    pub completed: bool,
    /// Unix timestamp (seconds) of the last update, for future statistics.
    #[serde(default)]
    pub updated_at: Option<u64>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Progress {
    /// The story most recently opened, used to offer "continue reading".
    #[serde(default)]
    pub last_story: Option<String>,
    /// Byte offset into `last_story`.
    #[serde(default)]
    pub last_offset: usize,
    #[serde(default)]
    pub stories: BTreeMap<String, StoryProgress>,
}

impl Progress {
    pub fn load(path: &Path) -> Self {
        let loaded = load_json::<Progress>(path);
        if let Some(warning) = &loaded.warning {
            eprintln!("[progress] {warning}");
        }
        loaded.value
    }

    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        write_json_atomic(path, self)
    }

    pub fn record_offset(&mut self, story_id: &str, offset: usize) {
        self.record_offset_at(story_id, offset, now_unix());
    }

    pub fn record_offset_at(&mut self, story_id: &str, offset: usize, now: u64) {
        self.last_story = Some(story_id.to_string());
        self.last_offset = offset;
        let entry = self.stories.entry(story_id.to_string()).or_default();
        entry.last_offset = offset;
        entry.updated_at = Some(now);
    }

    pub fn mark_completed(&mut self, story_id: &str) {
        self.mark_completed_at(story_id, now_unix());
    }

    pub fn mark_completed_at(&mut self, story_id: &str, now: u64) {
        let entry = self.stories.entry(story_id.to_string()).or_default();
        entry.completed = true;
        entry.updated_at = Some(now);
    }

    pub fn is_completed(&self, story_id: &str) -> bool {
        self.stories
            .get(story_id)
            .map(|entry| entry.completed)
            .unwrap_or(false)
    }

    /// True once the story has been opened, even if it is still at offset 0.
    pub fn has_started(&self, story_id: &str) -> bool {
        self.stories.contains_key(story_id)
    }

    pub fn last_offset_of(&self, story_id: &str) -> usize {
        self.stories
            .get(story_id)
            .map(|entry| entry.last_offset)
            .unwrap_or(0)
    }

    /// Completed stories for a level, given the ids that belong to it.
    pub fn completed_count<'a>(&self, story_ids: impl Iterator<Item = &'a str>) -> usize {
        story_ids.filter(|id| self.is_completed(id)).count()
    }
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_offset_and_resumes() {
        let mut progress = Progress::default();
        progress.record_offset_at("a1-01", 1234, 100);
        assert_eq!(progress.last_story.as_deref(), Some("a1-01"));
        assert_eq!(progress.last_offset, 1234);
        assert_eq!(progress.last_offset_of("a1-01"), 1234);
        assert!(progress.has_started("a1-01"));
        assert!(!progress.is_completed("a1-01"));
    }

    #[test]
    fn unopened_story_has_not_started() {
        let progress = Progress::default();
        assert!(!progress.has_started("a1-01"));
        assert_eq!(progress.last_offset_of("a1-01"), 0);
    }

    #[test]
    fn completion_does_not_erase_last_offset() {
        let mut progress = Progress::default();
        progress.record_offset_at("a1-01", 4321, 100);
        progress.mark_completed_at("a1-01", 200);
        assert!(progress.is_completed("a1-01"));
        assert_eq!(progress.last_offset_of("a1-01"), 4321);
        assert_eq!(progress.stories.get("a1-01").unwrap().updated_at, Some(200));
    }

    #[test]
    fn completed_count_filters_ids() {
        let mut progress = Progress::default();
        progress.mark_completed("a1-01");
        progress.mark_completed("a1-03");
        let ids = ["a1-01", "a1-02", "a1-03"];
        assert_eq!(progress.completed_count(ids.iter().copied()), 2);
    }

    #[test]
    fn round_trips_through_json() {
        let mut progress = Progress::default();
        progress.record_offset_at("a2-01", 987, 42);
        progress.mark_completed_at("a2-01", 43);
        let json = serde_json::to_string(&progress).unwrap();
        let parsed: Progress = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, progress);
    }
}
