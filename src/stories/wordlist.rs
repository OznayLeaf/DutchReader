//! Cumulative word list: which lemma was introduced by which story.
//!
//! This is the file future story writers and the AI prompt template use to know
//! what the learner has already met. It is generated from the story files with
//! `validate_stories --write-wordlist`; never edit it by hand.

use super::loader::LoadedStory;
use super::model::{Level, normalize_word};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

pub const WORDLIST_VERSION: u32 = 1;

/// Compile-time copy of `assets/wordlist.json`.
pub const EMBEDDED_WORDLIST: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/wordlist.json"));

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WordlistStory {
    pub id: String,
    pub level: Level,
    pub order: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Wordlist {
    pub version: u32,
    /// Global story order, for documentation and for the AI prompt template.
    pub stories: Vec<WordlistStory>,
    /// lemma -> id of the story that introduced it.
    pub words: BTreeMap<String, String>,
}

impl Default for Wordlist {
    fn default() -> Self {
        Wordlist {
            version: WORDLIST_VERSION,
            stories: Vec::new(),
            words: BTreeMap::new(),
        }
    }
}

impl Wordlist {
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn load_embedded() -> Result<Self, serde_json::Error> {
        Self::from_json(EMBEDDED_WORDLIST)
    }

    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        Ok(Self::from_json(&json)?)
    }

    pub fn to_json_pretty(&self) -> String {
        let mut json = serde_json::to_string_pretty(self).expect("wordlist serializes");
        json.push('\n');
        json
    }

    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        crate::storage::atomic::write_atomic(path, self.to_json_pretty().as_bytes())
    }

    /// Build a word list from all stories. The first story (in global order)
    /// that lists a lemma as new owns it.
    pub fn build_from(stories: &[LoadedStory]) -> Self {
        let mut sorted: Vec<&LoadedStory> = stories.iter().collect();
        sorted.sort_by(|a, b| {
            a.story
                .level
                .cmp(&b.story.level)
                .then(a.story.order.cmp(&b.story.order))
                .then(a.story.id.cmp(&b.story.id))
        });

        let mut wordlist = Wordlist {
            version: WORDLIST_VERSION,
            stories: sorted
                .iter()
                .map(|loaded| WordlistStory {
                    id: loaded.story.id.clone(),
                    level: loaded.story.level,
                    order: loaded.story.order,
                })
                .collect(),
            words: BTreeMap::new(),
        };

        // Starter words are introduced by the fixed starter list, not by a
        // story. They are known from the very beginning.
        for word in &super::starter::StarterWords::embedded().words {
            wordlist
                .words
                .entry(normalize_word(&word.lemma))
                .or_insert_with(|| "starter".to_string());
        }

        for loaded in sorted {
            for word in &loaded.story.new_words {
                wordlist
                    .words
                    .entry(normalize_word(word))
                    .or_insert_with(|| loaded.story.id.clone());
            }
        }
        wordlist
    }

    pub fn introduced_by(&self, lemma: &str) -> Option<&str> {
        self.words.get(&normalize_word(lemma)).map(String::as_str)
    }

    pub fn contains(&self, lemma: &str) -> bool {
        self.words.contains_key(&normalize_word(lemma))
    }

    /// True when the word list attributes this lemma to the starter list.
    pub fn is_starter(&self, lemma: &str) -> bool {
        self.introduced_by(lemma) == Some("starter")
    }
}

/// Words used by a story that are neither new nor introduced by an earlier
/// story. The three sample stories still carry this debt; the list records it
/// and keeps it out of the owned vocabulary. Generated shelves never use it.
pub const ASSUMED_KNOWN_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssumedKnown {
    pub version: u32,
    pub note: String,
    /// lemma -> story ids in which it was used without being introduced.
    pub words: BTreeMap<String, Vec<String>>,
}

impl Default for AssumedKnown {
    fn default() -> Self {
        AssumedKnown {
            version: ASSUMED_KNOWN_VERSION,
            note: "Words used by sample stories before any story introduced them. \
                   When the missing earlier stories are written, make them introduce \
                   these words. Regenerate with `validate_stories --write-wordlist --allow-gaps`."
                .to_string(),
            words: BTreeMap::new(),
        }
    }
}

impl AssumedKnown {
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        Ok(Self::from_json(&json)?)
    }

    pub fn to_json_pretty(&self) -> String {
        let mut json = serde_json::to_string_pretty(self).expect("debt list serializes");
        json.push('\n');
        json
    }

    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        crate::storage::atomic::write_atomic(path, self.to_json_pretty().as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stories::loader::LoadedStory;

    fn story(id: &str, level: Level, order: u8, words: &[&str]) -> LoadedStory {
        LoadedStory {
            source: format!("{}/{id}.json", level.folder()),
            story: super::super::model::Story {
                id: id.to_string(),
                level,
                order,
                title: "Test".into(),
                blurb_nl: "Test".into(),
                blurb_en: "Test".into(),
                sections: vec!["test".into()],
                new_words: words.iter().map(|w| w.to_string()).collect(),
                glossary: vec![],
            },
        }
    }

    #[test]
    fn first_introduction_wins_and_is_deterministic() {
        let stories = vec![
            story("a2-01", Level::A2, 1, &["huis", "fiets"]),
            story("a1-01", Level::A1, 1, &["fiets", "de"]),
        ];
        let wordlist = Wordlist::build_from(&stories);
        assert_eq!(wordlist.introduced_by("fiets"), Some("a1-01"));
        assert_eq!(wordlist.introduced_by("huis"), Some("a2-01"));
        assert_eq!(wordlist.introduced_by("de"), Some("starter"));
        assert_eq!(wordlist.stories[0].id, "a1-01");
        assert_eq!(wordlist.stories[1].id, "a2-01");
    }

    #[test]
    fn round_trips_through_json() {
        let stories = vec![story("a1-01", Level::A1, 1, &["fiets"])];
        let wordlist = Wordlist::build_from(&stories);
        let json = wordlist.to_json_pretty();
        assert_eq!(Wordlist::from_json(&json).unwrap(), wordlist);
    }
}
