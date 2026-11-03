//! Personal dictionary and the 30-second page-reading rule.
//!
//! Entries always come from the bundled story glossary, never from online
//! lookups. A word is stored once, keyed by its lemma, together with the story
//! and level where it was first met.

use crate::stories::model::{GlossaryEntry, Level, Story, normalize_word};
use crate::stories::tokenize::tokenize;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// How the Personal Dictionary screen sorts its list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Sort {
    #[default]
    Alphabetical,
    Newest,
}

impl Sort {
    pub fn from_code(code: &str) -> Self {
        match code.to_ascii_lowercase().as_str() {
            "newest" | "recent" => Sort::Newest,
            _ => Sort::Alphabetical,
        }
    }

    pub fn code(self) -> &'static str {
        match self {
            Sort::Alphabetical => "alpha",
            Sort::Newest => "newest",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DictionaryEntry {
    /// Base form (lemma), e.g. `lopen`, `fiets`.
    pub word: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub article: Option<String>,
    pub meaning: String,
    #[serde(default)]
    pub pos: String,
    pub story_id: String,
    pub level: Level,
    /// Unix timestamp in seconds.
    #[serde(default)]
    pub added_at: u64,
}

impl DictionaryEntry {
    /// `de fiets`, or just `lopen` when there is no article.
    pub fn display_word(&self) -> String {
        match &self.article {
            Some(article) if !article.is_empty() => format!("{article} {}", self.word),
            _ => self.word.clone(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dictionary {
    #[serde(default)]
    pub entries: Vec<DictionaryEntry>,
}

impl Dictionary {
    pub fn load(path: &Path) -> Self {
        let loaded = crate::storage::atomic::load_json::<Dictionary>(path);
        if let Some(warning) = &loaded.warning {
            eprintln!("[dictionary] {warning}");
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

    pub fn contains_lemma(&self, lemma: &str) -> bool {
        let lemma = normalize_word(lemma);
        self.entries
            .iter()
            .any(|entry| normalize_word(&entry.word) == lemma)
    }

    /// Add every dictionary-worthy glossary word that occurs on `page`.
    ///
    /// Starter words are always included, even when their part of speech would
    /// normally be filtered out, because a beginner needs them. `starter_origins`
    /// maps a starter lemma to the first story (in reading order) that uses it,
    /// so `de` is attributed to A1-01 even when it is met again in A2.
    ///
    /// Returns the number of new entries. Re-reading a page adds nothing.
    pub fn add_page_words(
        &mut self,
        story: &Story,
        page: &str,
        now: u64,
        starter_origins: &HashMap<String, (String, Level)>,
    ) -> usize {
        let lexicon = story.lexicon();
        let mut seen_on_page = HashSet::new();
        let mut added = 0;

        for token in tokenize(page) {
            let Some(entry) = lexicon.resolve(token.text) else {
                continue;
            };
            if !is_dictionary_word(entry) {
                continue;
            }
            let key = normalize_word(&entry.lemma);
            if !seen_on_page.insert(key.clone()) || self.contains_lemma(&entry.lemma) {
                continue;
            }
            let (story_id, level) =
                if crate::stories::starter::StarterWords::embedded().is_lemma(&entry.lemma) {
                    starter_origins
                        .get(&key)
                        .cloned()
                        .unwrap_or_else(|| (story.id.clone(), story.level))
                } else {
                    (story.id.clone(), story.level)
                };
            // The dictionary is keyed by lemma, so it stores the lemma's own
            // meaning and article, not the meaning of whichever surface form
            // happened to appear first on the page. Otherwise `ben` would save
            // `zijn` = "am" and `heeft` would save `hebben` = "has".
            let lemma_entry = lexicon.lemma_entry(&entry.lemma).unwrap_or(entry);
            self.entries.push(DictionaryEntry {
                word: entry.lemma.clone(),
                article: lemma_entry.article.clone(),
                meaning: lemma_entry.meaning.clone(),
                pos: lemma_entry.pos.clone(),
                story_id,
                level,
                added_at: now,
            });
            added += 1;
        }
        added
    }

    /// Filtered and sorted entries for the screen. Matching is case-insensitive
    /// across the word, article and meaning.
    pub fn search(&self, query: &str, sort: Sort) -> Vec<DictionaryEntry> {
        let query = query.trim().to_lowercase();
        let mut result: Vec<DictionaryEntry> = self
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
                b.added_at
                    .cmp(&a.added_at)
                    .then(a.word.to_lowercase().cmp(&b.word.to_lowercase()))
            }),
        }
        result
    }
}

/// Function words are glossed for the reader but make a poor personal
/// dictionary. Starter words are the exception: a beginner needs `de`, `is`,
/// and `heb`, so they are always saved even when their part of speech would
/// otherwise be filtered out.
pub fn is_dictionary_word(entry: &GlossaryEntry) -> bool {
    if crate::stories::starter::StarterWords::embedded().is_lemma(&entry.lemma) {
        return true;
    }
    !matches!(
        entry.pos.to_ascii_lowercase().as_str(),
        "article"
            | "determiner"
            | "name"
            | "conjunction"
            | "preposition"
            | "numeral"
            | "interjection"
    )
}

/// Format a Unix timestamp as `12 May 2026` in the local timezone.
pub use crate::time::format_date;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stories::index::StoryIndex;

    fn story(id: &str) -> Story {
        StoryIndex::load_embedded()
            .expect("embedded stories")
            .get(id)
            .expect("story exists")
            .story
            .clone()
    }

    #[test]
    fn reading_a_page_adds_glossary_words_once() {
        let story = story("a1-01");
        let mut dictionary = Dictionary::default();
        let page = &story.sections[0];
        let added = dictionary.add_page_words(&story, page, 100, &HashMap::new());
        assert!(added > 0, "A1 page 1 should add words");
        assert_eq!(dictionary.len(), added);

        let again = dictionary.add_page_words(&story, page, 200, &HashMap::new());
        assert_eq!(again, 0);
        assert_eq!(dictionary.len(), added);
    }

    #[test]
    fn dictionary_stores_the_lemma_and_its_lemma_meaning() {
        let story = story("a1-01");
        let mut dictionary = Dictionary::default();
        // "Hallo! Ik ben Sam. Hier is Noor." — `ben` is a form of `zijn`.
        dictionary.add_page_words(&story, &story.sections[0], 0, &HashMap::new());
        let zijn = dictionary
            .entries
            .iter()
            .find(|entry| entry.word == "zijn")
            .expect("zijn is saved from ben");
        assert_eq!(zijn.meaning, "to be", "the lemma keeps the lemma meaning");
        assert_eq!(zijn.pos, "verb");
        assert!(zijn.article.is_none());

        // "Bo heeft een bal..." — `heeft` is a form of `hebben`.
        let mut dictionary = Dictionary::default();
        dictionary.add_page_words(&story, &story.sections[3], 0, &HashMap::new());
        let hebben = dictionary
            .entries
            .iter()
            .find(|entry| entry.word == "hebben")
            .expect("hebben is saved from heeft");
        assert_eq!(hebben.meaning, "to have");

        // "Bo loopt met de bal in de tuin." — `loopt` is a form of `lopen`.
        let mut dictionary = Dictionary::default();
        dictionary.add_page_words(&story, &story.sections[5], 0, &HashMap::new());
        let lopen = dictionary
            .entries
            .iter()
            .find(|entry| entry.word == "lopen")
            .expect("lopen is saved from loopt");
        assert_eq!(lopen.meaning, "to walk");
    }

    #[test]
    fn starter_words_are_added_but_other_function_words_are_not() {
        let story = story("a2-01");
        let mut dictionary = Dictionary::default();
        dictionary.add_page_words(&story, &story.text(), 0, &HashMap::new());
        // Starter words are saved even though they are articles/pronouns.
        assert!(dictionary.contains_lemma("de"));
        assert!(dictionary.contains_lemma("het"));
        assert!(dictionary.contains_lemma("zijn"));
        // Non-starter function words are still filtered out.
        assert!(!dictionary.contains_lemma("bij"));
        assert!(!dictionary.contains_lemma("achter"));
    }

    #[test]
    fn entries_remember_story_and_level() {
        let story = story("a2-01");
        let mut dictionary = Dictionary::default();
        dictionary.add_page_words(&story, &story.sections[0], 1234, &HashMap::new());
        let entry = dictionary
            .entries
            .iter()
            .find(|entry| entry.word == "sleutel")
            .expect("new word is saved");
        assert_eq!(entry.story_id, "a2-01");
        assert_eq!(entry.level, Level::A2);
        assert_eq!(entry.added_at, 1234);
    }

    #[test]
    fn search_matches_word_article_and_meaning() {
        let mut dictionary = Dictionary::default();
        dictionary.entries.push(DictionaryEntry {
            word: "fiets".into(),
            article: Some("de".into()),
            meaning: "bicycle".into(),
            pos: "noun".into(),
            story_id: "a1-01".into(),
            level: Level::A1,
            added_at: 10,
        });
        dictionary.entries.push(DictionaryEntry {
            word: "lopen".into(),
            article: None,
            meaning: "to walk".into(),
            pos: "verb".into(),
            story_id: "a1-01".into(),
            level: Level::A1,
            added_at: 20,
        });

        assert_eq!(dictionary.search("fiets", Sort::Alphabetical).len(), 1);
        assert_eq!(dictionary.search("bicycle", Sort::Alphabetical).len(), 1);
        assert_eq!(dictionary.search("de", Sort::Alphabetical).len(), 1);
        assert_eq!(dictionary.search("", Sort::Alphabetical).len(), 2);
    }

    #[test]
    fn sorts_alphabetically_or_newest_first() {
        let mut dictionary = Dictionary::default();
        for (word, added_at) in [("zebra", 10), ("appel", 30), ("muis", 20)] {
            dictionary.entries.push(DictionaryEntry {
                word: word.into(),
                article: None,
                meaning: "x".into(),
                pos: "noun".into(),
                story_id: "a1-01".into(),
                level: Level::A1,
                added_at,
            });
        }
        let alpha = dictionary.search("", Sort::Alphabetical);
        assert_eq!(
            alpha.iter().map(|e| e.word.as_str()).collect::<Vec<_>>(),
            vec!["appel", "muis", "zebra"]
        );
        let newest = dictionary.search("", Sort::Newest);
        assert_eq!(
            newest.iter().map(|e| e.word.as_str()).collect::<Vec<_>>(),
            vec!["appel", "muis", "zebra"]
        );
    }

    #[test]
    fn dates_format_with_the_local_offset() {
        assert_eq!(format_date(0, 0), "1 Jan 1970");
        // 2026-05-12 00:00:00 UTC
        assert_eq!(format_date(1_778_544_000, 0), "12 May 2026");
        // Leap day
        assert_eq!(format_date(1_709_164_800, 0), "29 Feb 2024");
        // 23:30 UTC is already the next day in a +01:00 timezone.
        assert_eq!(format_date(1_778_628_600, 3_600), "13 May 2026");
    }

    #[test]
    fn round_trips_through_json() {
        let story = story("a1-01");
        let mut dictionary = Dictionary::default();
        dictionary.add_page_words(&story, &story.sections[0], 42, &HashMap::new());
        let json = serde_json::to_string(&dictionary).unwrap();
        let parsed: Dictionary = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, dictionary);
    }
}
