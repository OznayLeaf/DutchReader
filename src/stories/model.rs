//! Story data model, matching the contract in ARCHITECTURE.md section 2.3.
//!
//! These types are deliberately plain Rust + serde: no Slint, no JNI, so they
//! can be validated and unit-tested on the desktop.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// One of the four graded-reader levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Level {
    A1,
    A2,
    B1,
    B2,
}

impl Level {
    pub const ALL: [Level; 4] = [Level::A1, Level::A2, Level::B1, Level::B2];

    /// Lowercase folder name used under `assets/stories/`.
    pub fn folder(self) -> &'static str {
        match self {
            Level::A1 => "a1",
            Level::A2 => "a2",
            Level::B1 => "b1",
            Level::B2 => "b2",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Level::A1 => "A1",
            Level::A2 => "A2",
            Level::B1 => "B1",
            Level::B2 => "B2",
        }
    }

    pub fn from_folder(folder: &str) -> Option<Level> {
        match folder.to_ascii_lowercase().as_str() {
            "a1" => Some(Level::A1),
            "a2" => Some(Level::A2),
            "b1" => Some(Level::B1),
            "b2" => Some(Level::B2),
            _ => None,
        }
    }

    pub fn from_code(code: &str) -> Option<Level> {
        match code.to_ascii_uppercase().as_str() {
            "A1" => Some(Level::A1),
            "A2" => Some(Level::A2),
            "B1" => Some(Level::B1),
            "B2" => Some(Level::B2),
            _ => None,
        }
    }

    pub fn next(self) -> Option<Level> {
        match self {
            Level::A1 => Some(Level::A2),
            Level::A2 => Some(Level::B1),
            Level::B1 => Some(Level::B2),
            Level::B2 => None,
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A single glossary entry.
///
/// `parts` is used for separable verbs and other multi-word forms: the entry
/// `{ "form": "zet op", "parts": ["zet", "op"], "lemma": "opzetten", ... }`
/// makes both `zet` and `op` tappable and counts as glossing those tokens.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GlossaryEntry {
    pub form: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<String>>,
    pub lemma: String,
    /// Part of speech. Kept as a string so the schema can grow; the validator
    /// checks it against a known set.
    pub pos: String,
    /// Required for nouns: `de` or `het`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub article: Option<String>,
    pub meaning: String,
    #[serde(default)]
    pub new: bool,
    /// Optional 1-based occurrence of this surface form in the story. Used when
    /// one form has different meanings in different places (`zijn` = are / his /
    /// to be). Entries without `occurrence` are the default for that form.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub occurrence: Option<u32>,
}

impl GlossaryEntry {
    pub fn is_name(&self) -> bool {
        self.pos.eq_ignore_ascii_case("name")
    }

    pub fn is_noun(&self) -> bool {
        self.pos.eq_ignore_ascii_case("noun")
    }

    /// Human-readable surface form, including separated parts.
    pub fn display_form(&self) -> String {
        match &self.parts {
            Some(parts) if !parts.is_empty() => parts.join(" ... "),
            _ => self.form.clone(),
        }
    }
}

/// A story exactly as stored on disk.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Story {
    pub id: String,
    pub level: Level,
    pub order: u8,
    pub title: String,
    pub blurb_nl: String,
    #[serde(default)]
    pub blurb_en: String,
    /// Authored text sections. The reader joins them with blank lines and then
    /// paginates the result for the actual screen, so these are not pages.
    #[serde(alias = "pages")]
    pub sections: Vec<String>,
    pub new_words: Vec<String>,
    pub glossary: Vec<GlossaryEntry>,
}

impl Story {
    /// The whole story as one text, preserving paragraph breaks between and
    /// inside sections.
    pub fn text(&self) -> String {
        self.sections
            .iter()
            .map(|section| section.trim())
            .filter(|section| !section.is_empty())
            .collect::<Vec<_>>()
            .join("\n\n")
    }

    /// Append the shared starter glossary entries. Called by the loader so every
    /// story can resolve `de`, `is`, `heb`, and the other beginner words even
    /// though authors do not repeat them in each file.
    pub fn merge_starter_glossary(&mut self) {
        use std::collections::HashSet;

        // Only a default entry blocks the starter entry. A story may define
        // occurrence-specific senses for a starter form (for example `zijn`
        // meaning "are" in one place) and still rely on the starter entry for
        // the lemma's default meaning.
        let mut existing_defaults: HashSet<String> = self
            .glossary
            .iter()
            .filter(|entry| entry.occurrence.is_none())
            .map(|entry| normalize_word(&entry.form))
            .collect();
        for entry in super::starter::StarterWords::embedded().glossary_entries() {
            if existing_defaults.insert(normalize_word(&entry.form)) {
                self.glossary.push(entry);
            }
        }
    }

    pub fn new_word_set(&self) -> std::collections::HashSet<String> {
        self.new_words.iter().map(|w| normalize_word(w)).collect()
    }

    /// Every token in the story, in reading order.
    pub fn tokens(&self) -> Vec<crate::stories::tokenize::Token<'_>> {
        self.sections
            .iter()
            .flat_map(|section| crate::stories::tokenize::tokenize(section))
            .collect()
    }

    /// Total number of word tokens across all sections.
    pub fn token_count(&self) -> usize {
        self.sections
            .iter()
            .map(|section| crate::stories::tokenize::tokenize(section).len())
            .sum()
    }

    /// Build an efficient lookup table for this story's glossary.
    pub fn lexicon(&self) -> StoryLexicon<'_> {
        StoryLexicon::new(self)
    }
}

/// Lookup table from surface forms (and separable parts) to glossary entries.
///
/// A form can have several entries: at most one default entry (no
/// `occurrence`) plus entries for specific occurrences. The validator enforces
/// that occurrence numbers are unique per form.
#[derive(Debug, Default)]
pub struct StoryLexicon<'a> {
    by_form: HashMap<String, Vec<&'a GlossaryEntry>>,
    by_part: HashMap<String, Vec<&'a GlossaryEntry>>,
    by_lemma: HashMap<String, Vec<&'a GlossaryEntry>>,
}

impl<'a> StoryLexicon<'a> {
    pub fn new(story: &'a Story) -> Self {
        let mut lexicon = StoryLexicon::default();
        for entry in &story.glossary {
            lexicon
                .by_form
                .entry(normalize_word(&entry.form))
                .or_default()
                .push(entry);
            if let Some(parts) = &entry.parts {
                for part in parts {
                    lexicon
                        .by_part
                        .entry(normalize_word(part))
                        .or_default()
                        .push(entry);
                }
            }
            lexicon
                .by_lemma
                .entry(normalize_word(&entry.lemma))
                .or_default()
                .push(entry);
        }
        lexicon
    }

    /// Resolve a surface token to its default glossary entry. Exact surface
    /// forms win over separable parts.
    pub fn resolve(&self, token: &str) -> Option<&'a GlossaryEntry> {
        self.resolve_inner(token, None)
    }

    /// Resolve a surface token at a 1-based occurrence of that form. An entry
    /// with a matching `occurrence` wins; otherwise the default entry is used.
    pub fn resolve_at(&self, token: &str, occurrence: u32) -> Option<&'a GlossaryEntry> {
        self.resolve_inner(token, Some(occurrence))
    }

    fn resolve_inner(&self, token: &str, occurrence: Option<u32>) -> Option<&'a GlossaryEntry> {
        let key = normalize_word(token);
        self.by_form
            .get(&key)
            .and_then(|entries| pick_entry(entries, occurrence))
            .or_else(|| {
                self.by_part
                    .get(&key)
                    .and_then(|entries| pick_entry(entries, occurrence))
            })
    }

    /// The entry that represents the lemma itself. Prefers an entry whose form
    /// equals the lemma and which is not occurrence-specific, so `zijn` keeps
    /// its lemma meaning even when a story overrides some occurrences.
    pub fn lemma_entry(&self, lemma: &str) -> Option<&'a GlossaryEntry> {
        let key = normalize_word(lemma);
        self.by_form
            .get(&key)
            .and_then(|entries| {
                entries
                    .iter()
                    .copied()
                    .find(|entry| entry.occurrence.is_none() && normalize_word(&entry.lemma) == key)
                    .or_else(|| {
                        entries
                            .iter()
                            .copied()
                            .find(|entry| normalize_word(&entry.lemma) == key)
                    })
            })
            .or_else(|| {
                self.by_lemma
                    .get(&key)
                    .and_then(|entries| entries.first().copied())
            })
    }

    pub fn has_form(&self, form: &str) -> bool {
        self.resolve(form).is_some()
    }
}

/// Choose the occurrence-specific entry if present, otherwise the default entry
/// (no `occurrence`), otherwise the first entry.
fn pick_entry<'a>(
    entries: &[&'a GlossaryEntry],
    occurrence: Option<u32>,
) -> Option<&'a GlossaryEntry> {
    if let Some(occurrence) = occurrence
        && let Some(entry) = entries
            .iter()
            .copied()
            .find(|entry| entry.occurrence == Some(occurrence))
    {
        return Some(entry);
    }
    entries
        .iter()
        .copied()
        .find(|entry| entry.occurrence.is_none())
        .or_else(|| entries.first().copied())
}

/// Lowercase a word and normalise the two common apostrophe characters.
///
/// Punctuation is not stripped here; the tokenizer already removes it. This
/// function is also used for glossary keys, so `"Zo'n"` and `"zo’n"` match.
pub fn normalize_word(word: &str) -> String {
    word.replace('\u{2019}', "'").to_lowercase()
}

/// True if `lemma` is listed as one of the story's new words.
pub fn is_new_lemma(story: &Story, lemma: &str) -> bool {
    let lemma = normalize_word(lemma);
    story
        .new_words
        .iter()
        .any(|word| normalize_word(word) == lemma)
}
