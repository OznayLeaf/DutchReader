//! The fixed starter vocabulary.
//!
//! Starter words are known from the very beginning. They never count as a
//! story's new words, they are never underlined as new, and they are never
//! recorded in the assumed-known debt list. The loader copies their glossary
//! entries into every story so a beginner can tap them from the first page.
//!
//! `assets/starter_words.json` is deliberately small and hand-editable. It may
//! contain at most 25 lemmas and only function words plus the most basic forms
//! of `zijn` and `hebben`; nouns and adjectives are not allowed.

use crate::stories::model::{GlossaryEntry, normalize_word};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

pub const STARTER_VERSION: u32 = 1;
pub const MAX_STARTER_LEMMAS: usize = 25;

/// Compile-time copy of `assets/starter_words.json`.
pub const EMBEDDED_STARTER_WORDS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/starter_words.json"
));

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StarterWord {
    pub lemma: String,
    pub pos: String,
    pub meaning: String,
    /// Surface forms that resolve to this lemma. Defaults to the lemma itself.
    #[serde(default)]
    pub forms: Vec<String>,
    /// Optional form-specific meanings, e.g. `ben` = "am" while the lemma
    /// `zijn` means "to be".
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub form_meanings: HashMap<String, String>,
}

impl StarterWord {
    pub fn all_forms(&self) -> Vec<&str> {
        if self.forms.is_empty() {
            vec![self.lemma.as_str()]
        } else {
            self.forms.iter().map(String::as_str).collect()
        }
    }

    fn glossary_entries(&self) -> Vec<GlossaryEntry> {
        self.all_forms()
            .into_iter()
            .map(|form| GlossaryEntry {
                form: form.to_string(),
                parts: None,
                lemma: self.lemma.clone(),
                pos: self.pos.clone(),
                article: None,
                meaning: self
                    .form_meanings
                    .get(form)
                    .cloned()
                    .unwrap_or_else(|| self.meaning.clone()),
                new: false,
                occurrence: None,
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StarterWords {
    pub version: u32,
    #[serde(default)]
    pub note: String,
    pub words: Vec<StarterWord>,
}

impl Default for StarterWords {
    fn default() -> Self {
        StarterWords {
            version: STARTER_VERSION,
            note: String::new(),
            words: Vec::new(),
        }
    }
}

impl StarterWords {
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn embedded() -> &'static StarterWords {
        static STARTER: OnceLock<StarterWords> = OnceLock::new();
        STARTER.get_or_init(|| {
            StarterWords::from_json(EMBEDDED_STARTER_WORDS)
                .expect("assets/starter_words.json must be valid JSON")
        })
    }

    pub fn is_lemma(&self, lemma: &str) -> bool {
        let lemma = normalize_word(lemma);
        self.words
            .iter()
            .any(|word| normalize_word(&word.lemma) == lemma)
    }

    pub fn is_form(&self, form: &str) -> bool {
        let form = normalize_word(form);
        self.words.iter().any(|word| {
            word.all_forms()
                .into_iter()
                .any(|candidate| normalize_word(candidate) == form)
        })
    }

    pub fn lemma_set(&self) -> HashSet<String> {
        self.words
            .iter()
            .map(|word| normalize_word(&word.lemma))
            .collect()
    }

    /// One glossary entry per surface form. The loader appends these to every
    /// story, skipping forms the story already defines.
    pub fn glossary_entries(&self) -> Vec<GlossaryEntry> {
        self.words
            .iter()
            .flat_map(StarterWord::glossary_entries)
            .collect()
    }

    /// Problems that would make the starter list invalid. Used by tests and the
    /// validator's startup check.
    pub fn problems(&self) -> Vec<String> {
        let mut problems = Vec::new();
        if self.words.len() > MAX_STARTER_LEMMAS {
            problems.push(format!(
                "starter_words.json has {} lemmas; the maximum is {MAX_STARTER_LEMMAS}",
                self.words.len()
            ));
        }
        let mut lemmas = HashSet::new();
        let mut forms = HashSet::new();
        for word in &self.words {
            let lemma = normalize_word(&word.lemma);
            if lemma.is_empty() {
                problems.push("starter word has an empty lemma".into());
            } else if !lemmas.insert(lemma.clone()) {
                problems.push(format!("starter lemma `{lemma}` is listed twice"));
            }
            let pos = word.pos.to_ascii_lowercase();
            if !matches!(
                pos.as_str(),
                "article"
                    | "pronoun"
                    | "conjunction"
                    | "interjection"
                    | "verb"
                    | "adverb"
                    | "determiner"
                    | "preposition"
            ) {
                problems.push(format!(
                    "starter lemma `{lemma}` has pos `{}`; only function words and basic verbs are allowed",
                    word.pos
                ));
            }
            if word.meaning.trim().is_empty() {
                problems.push(format!("starter lemma `{lemma}` has no meaning"));
            }
            for form in word.all_forms() {
                let form = normalize_word(form);
                if form.is_empty() {
                    problems.push(format!("starter lemma `{lemma}` has an empty form"));
                } else if !forms.insert(form.clone()) {
                    problems.push(format!("starter form `{form}` is listed twice"));
                }
            }
        }
        problems
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_starter_list_is_valid_and_small() {
        let starter = StarterWords::embedded();
        assert!(starter.problems().is_empty(), "{:?}", starter.problems());
        assert!(starter.words.len() <= MAX_STARTER_LEMMAS);
    }

    #[test]
    fn starter_forms_resolve_to_their_lemmas() {
        let starter = StarterWords::embedded();
        assert!(starter.is_form("is"));
        assert!(starter.is_lemma("zijn"));
        assert!(starter.is_form("heeft"));
        assert!(starter.is_lemma("hebben"));
        assert!(!starter.is_lemma("fiets"));
    }

    #[test]
    fn glossary_entries_are_never_new() {
        let entries = StarterWords::embedded().glossary_entries();
        assert!(entries.iter().all(|entry| !entry.new));
        assert!(entries.iter().any(|entry| entry.form == "is"));
    }
}
