//! Words the reader tapped to see a meaning.
//!
//! This is a separate list from the 30-second Personal Dictionary: it records
//! exactly what was looked up (the tapped surface form and the meaning shown in
//! the popup), so the learner can review those words later. Repeated taps of
//! the same word update the entry instead of creating duplicates.

use crate::dictionary::Sort;
use crate::lookup::LookupOutcome;
use crate::stories::model::{Level, normalize_word};
use serde::{Deserialize, Serialize};
use std::path::Path;

fn default_tap_count() -> u32 {
    1
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TappedWord {
    /// The tapped surface form, as shown in the popup (e.g. `loopt`).
    pub word: String,
    /// Article, only present when the tapped form is the lemma itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub article: Option<String>,
    pub meaning: String,
    /// Secondary popup line such as `form of: lopen, to walk`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    /// Where the meaning came from (`Story glossary`, `Machine translated`, ...).
    #[serde(default)]
    pub source: String,
    pub story_id: String,
    pub level: Level,
    /// Unix timestamp of the most recent tap, in seconds.
    pub tapped_at: u64,
    #[serde(default = "default_tap_count")]
    pub tap_count: u32,
}

impl TappedWord {
    /// `de fiets`, or just `loopt` when there is no article.
    pub fn display_word(&self) -> String {
        match &self.article {
            Some(article) if !article.is_empty() => format!("{article} {}", self.word),
            _ => self.word.clone(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TappedWords {
    #[serde(default)]
    pub entries: Vec<TappedWord>,
}

impl TappedWords {
    pub fn load(path: &Path) -> Self {
        let loaded = crate::storage::atomic::load_json::<TappedWords>(path);
        if let Some(warning) = &loaded.warning {
            eprintln!("[tapped] {warning}");
        }
        loaded.value
    }

    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        crate::storage::atomic::write_json_atomic(path, self)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn contains(&self, word: &str) -> bool {
        let word = normalize_word(word);
        self.entries
            .iter()
            .any(|entry| normalize_word(&entry.word) == word)
    }

    /// Save one successful lookup. Returns false when there is no usable
    /// meaning (for example a proper name with no glossary entry), so the tab
    /// does not fill with "no entry" messages.
    pub fn record(
        &mut self,
        outcome: &LookupOutcome,
        story_id: &str,
        level: Level,
        now: u64,
    ) -> bool {
        let meaning = outcome.meaning.trim();
        if meaning.is_empty()
            || meaning.starts_with("No glossary entry for ")
            || meaning.starts_with("No meaning found for ")
        {
            return false;
        }

        let key = normalize_word(&outcome.word);
        if key.is_empty() {
            return false;
        }

        if let Some(existing) = self
            .entries
            .iter_mut()
            .find(|entry| normalize_word(&entry.word) == key)
        {
            existing.meaning = outcome.meaning.clone();
            existing.base = outcome.base.clone();
            existing.source = outcome.source.label().to_string();
            existing.tapped_at = now;
            existing.tap_count = existing.tap_count.saturating_add(1);
            return true;
        }

        self.entries.push(TappedWord {
            word: outcome.word.clone(),
            article: outcome.article.clone(),
            meaning: outcome.meaning.clone(),
            base: outcome.base.clone(),
            source: outcome.source.label().to_string(),
            story_id: story_id.to_string(),
            level,
            tapped_at: now,
            tap_count: 1,
        });
        true
    }

    /// Filtered and sorted entries for the screen. Matching is case-insensitive
    /// across the word, article, meaning, base line, and source.
    pub fn search(&self, query: &str, sort: Sort) -> Vec<TappedWord> {
        let query = query.trim().to_lowercase();
        let mut result: Vec<TappedWord> = self
            .entries
            .iter()
            .filter(|entry| {
                if query.is_empty() {
                    return true;
                }
                entry.word.to_lowercase().contains(&query)
                    || entry.meaning.to_lowercase().contains(&query)
                    || entry
                        .article
                        .as_deref()
                        .is_some_and(|article| article.to_lowercase().contains(&query))
                    || entry
                        .base
                        .as_deref()
                        .is_some_and(|base| base.to_lowercase().contains(&query))
                    || entry.source.to_lowercase().contains(&query)
            })
            .cloned()
            .collect();

        match sort {
            Sort::Alphabetical => result.sort_by(|a, b| {
                a.word
                    .to_lowercase()
                    .cmp(&b.word.to_lowercase())
                    .then(a.meaning.cmp(&b.meaning))
            }),
            Sort::Newest => result.sort_by(|a, b| {
                b.tapped_at
                    .cmp(&a.tapped_at)
                    .then(a.word.to_lowercase().cmp(&b.word.to_lowercase()))
            }),
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lookup::MeaningSource;

    fn outcome(word: &str, meaning: &str) -> LookupOutcome {
        LookupOutcome {
            word: word.into(),
            article: None,
            meaning: meaning.into(),
            base: None,
            source: MeaningSource::Glossary,
            note: None,
        }
    }

    #[test]
    fn records_a_tap_once_and_counts_repeats() {
        let mut tapped = TappedWords::default();
        assert!(tapped.record(&outcome("loopt", "to walk"), "a1-01", Level::A1, 10));
        assert_eq!(tapped.len(), 1);
        assert!(tapped.record(&outcome("Loopt", "to walk"), "a1-01", Level::A1, 20));
        assert_eq!(tapped.len(), 1);
        let entry = &tapped.entries[0];
        assert_eq!(entry.tap_count, 2);
        assert_eq!(entry.tapped_at, 20);
        assert_eq!(entry.word, "loopt");
    }

    #[test]
    fn keeps_the_lemma_article_and_base_line() {
        let mut tapped = TappedWords::default();
        let mut result = outcome("hond", "dog");
        result.article = Some("de".into());
        assert!(tapped.record(&result, "a1-01", Level::A1, 0));
        assert_eq!(tapped.entries[0].display_word(), "de hond");

        let mut form = outcome("loopt", "to walk");
        form.base = Some("form of: lopen, to walk".into());
        assert!(tapped.record(&form, "a1-01", Level::A1, 0));
        assert_eq!(
            tapped.entries[1].base.as_deref(),
            Some("form of: lopen, to walk")
        );
    }

    #[test]
    fn skips_lookups_without_a_meaning() {
        let mut tapped = TappedWords::default();
        assert!(!tapped.record(
            &outcome("Sam", "No glossary entry for \"Sam\"."),
            "a1-01",
            Level::A1,
            0
        ));
        assert!(!tapped.record(&outcome("Sam", "   "), "a1-01", Level::A1, 0));
        assert!(tapped.is_empty());
    }

    #[test]
    fn search_matches_word_meaning_and_base() {
        let mut tapped = TappedWords::default();
        let mut form = outcome("loopt", "to walk");
        form.base = Some("form of: lopen, to walk".into());
        tapped.record(&form, "a1-01", Level::A1, 10);
        tapped.record(&outcome("hond", "dog"), "a1-01", Level::A1, 20);

        assert_eq!(tapped.search("loop", Sort::Alphabetical).len(), 1);
        assert_eq!(tapped.search("dog", Sort::Alphabetical).len(), 1);
        assert_eq!(tapped.search("form of", Sort::Alphabetical).len(), 1);
        assert_eq!(tapped.search("", Sort::Alphabetical).len(), 2);
    }

    #[test]
    fn sorts_alphabetically_or_newest_first() {
        let mut tapped = TappedWords::default();
        tapped.record(&outcome("zebra", "z"), "a1-01", Level::A1, 10);
        tapped.record(&outcome("appel", "a"), "a1-01", Level::A1, 30);
        tapped.record(&outcome("muis", "m"), "a1-01", Level::A1, 20);

        let alpha = tapped.search("", Sort::Alphabetical);
        assert_eq!(
            alpha.iter().map(|e| e.word.as_str()).collect::<Vec<_>>(),
            vec!["appel", "muis", "zebra"]
        );
        let newest = tapped.search("", Sort::Newest);
        assert_eq!(
            newest.iter().map(|e| e.word.as_str()).collect::<Vec<_>>(),
            vec!["appel", "muis", "zebra"]
        );
    }

    #[test]
    fn round_trips_through_json() {
        let mut tapped = TappedWords::default();
        tapped.record(&outcome("hond", "dog"), "a1-01", Level::A1, 42);
        let json = serde_json::to_string(&tapped).unwrap();
        let parsed: TappedWords = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, tapped);
    }
}
