//! Word meaning lookup: bundled glossary, optional online provider, and cache.
//!
//! The UI talks to [`resolve`], which always returns a non-empty outcome. In
//! online mode the network call happens on a worker thread; this module itself
//! stays synchronous so it is easy to test with a fake provider.

use crate::storage::settings::MeaningsSource;
use crate::stories::model::{GlossaryEntry, Story, normalize_word};
use crate::stories::starter::StarterWords;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::path::Path;
use std::time::Duration;
use thiserror::Error;

/// One successful provider answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LookupHit {
    /// A bundled glossary entry.
    Glossary(GlossaryEntry),
    /// A machine translation.
    Machine(String),
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum LookupError {
    #[error("network error: {0}")]
    Network(String),
    #[error("HTTP {0}")]
    Http(u16),
    #[error("could not read the response: {0}")]
    Parse(String),
    #[error("no translation was returned")]
    Empty,
}

/// Swappable meaning provider. `MyMemoryProvider` implements this over HTTPS;
/// tests use a small fake.
pub trait WordLookup: Send + Sync {
    fn lookup(&self, word: &str) -> Result<LookupHit, LookupError>;
}

/// Offline provider backed by one story's bundled glossary.
#[derive(Debug, Clone)]
pub struct OfflineGlossary {
    by_form: HashMap<String, Vec<GlossaryEntry>>,
    by_part: HashMap<String, Vec<GlossaryEntry>>,
}

impl OfflineGlossary {
    pub fn new(story: &Story) -> Self {
        let mut glossary = OfflineGlossary {
            by_form: HashMap::new(),
            by_part: HashMap::new(),
        };
        for entry in &story.glossary {
            glossary
                .by_form
                .entry(normalize_word(&entry.form))
                .or_default()
                .push(entry.clone());
            if let Some(parts) = &entry.parts {
                for part in parts {
                    glossary
                        .by_part
                        .entry(normalize_word(part))
                        .or_default()
                        .push(entry.clone());
                }
            }
        }
        glossary
    }

    /// Look up a tapped surface form at a 1-based occurrence. There is no lemma
    /// fallback: a missing form is a missing form, never silently the base
    /// verb's translation.
    pub fn lookup_at(&self, word: &str, occurrence: u32) -> Result<LookupHit, LookupError> {
        let key = normalize_word(word);
        self.by_form
            .get(&key)
            .and_then(|entries| pick_entry(entries, occurrence))
            .or_else(|| {
                self.by_part
                    .get(&key)
                    .and_then(|entries| pick_entry(entries, occurrence))
            })
            .cloned()
            .map(LookupHit::Glossary)
            .ok_or(LookupError::Empty)
    }
}

fn pick_entry(entries: &[GlossaryEntry], occurrence: u32) -> Option<&GlossaryEntry> {
    entries
        .iter()
        .find(|entry| entry.occurrence == Some(occurrence))
        .or_else(|| entries.iter().find(|entry| entry.occurrence.is_none()))
        .or_else(|| entries.first())
}

impl WordLookup for OfflineGlossary {
    fn lookup(&self, word: &str) -> Result<LookupHit, LookupError> {
        self.lookup_at(word, 1)
    }
}

/// MyMemory's free, keyless Dutch -> English endpoint.
pub struct MyMemoryProvider {
    agent: ureq::Agent,
}

impl Default for MyMemoryProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl MyMemoryProvider {
    pub fn new() -> Self {
        let config = ureq::Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(5)))
            .http_status_as_error(false)
            .user_agent("DutchReader/0.1 (offline-first Dutch reader)")
            .build();
        MyMemoryProvider {
            agent: config.into(),
        }
    }
}

impl WordLookup for MyMemoryProvider {
    fn lookup(&self, word: &str) -> Result<LookupHit, LookupError> {
        let url = format!(
            "https://api.mymemory.translated.net/get?q={}&langpair=nl%7Cen",
            url_encode(word)
        );
        let mut response = self
            .agent
            .get(&url)
            .call()
            .map_err(|error| LookupError::Network(error.to_string()))?;
        let status = response.status().as_u16();
        if !(200..300).contains(&status) {
            return Err(LookupError::Http(status));
        }
        let body = response
            .body_mut()
            .read_to_string()
            .map_err(|error| LookupError::Network(error.to_string()))?;
        parse_mymemory(&body).map(LookupHit::Machine)
    }
}

#[derive(Debug, Deserialize)]
struct MyMemoryResponse {
    #[serde(default, rename = "responseData")]
    response_data: Option<MyMemoryData>,
}

#[derive(Debug, Deserialize)]
struct MyMemoryData {
    #[serde(default, rename = "translatedText")]
    translated_text: Option<String>,
}

/// Extract `responseData.translatedText`. MyMemory sometimes returns a warning
/// in that field when the daily quota is exhausted; treat it as a failure so
/// the UI can fall back to the glossary.
pub fn parse_mymemory(body: &str) -> Result<String, LookupError> {
    let parsed: MyMemoryResponse =
        serde_json::from_str(body).map_err(|error| LookupError::Parse(error.to_string()))?;
    let text = parsed
        .response_data
        .and_then(|data| data.translated_text)
        .unwrap_or_default();
    let text = decode_entities(text.trim());
    if text.is_empty() {
        return Err(LookupError::Empty);
    }
    let lower = text.to_lowercase();
    if lower.contains("mymemory warning")
        || lower.contains("query length limit")
        || lower.contains("please select two distinct languages")
        || lower.contains("invalid language pair")
    {
        return Err(LookupError::Empty);
    }
    Ok(text)
}

fn decode_entities(input: &str) -> String {
    let mut output = input
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&apos;", "'")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&nbsp;", " ");
    // A handful of numeric apostrophes appear in MyMemory output.
    for entity in ["&#x27;", "&#x2019;"] {
        output = output.replace(entity, "'");
    }
    output
}

/// Percent-encode a query value for the URL. Only unreserved bytes stay as-is.
pub fn url_encode(input: &str) -> String {
    let mut output = String::new();
    for byte in input.as_bytes() {
        match *byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                output.push(*byte as char)
            }
            b' ' => output.push_str("%20"),
            _ => output.push_str(&format!("%{byte:02X}")),
        }
    }
    output
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CacheEntry {
    /// Normalised lemma used as the key.
    pub word: String,
    pub meaning: String,
    /// Unix timestamp in seconds.
    pub fetched_at: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LookupCache {
    #[serde(default)]
    pub entries: BTreeMap<String, CacheEntry>,
}

impl LookupCache {
    pub fn load(path: &Path) -> Self {
        let loaded = crate::storage::atomic::load_json::<LookupCache>(path);
        if let Some(warning) = &loaded.warning {
            eprintln!("[lookup] {warning}");
        }
        loaded.value
    }

    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        crate::storage::atomic::write_json_atomic(path, self)
    }

    pub fn get(&self, word: &str) -> Option<&CacheEntry> {
        self.entries.get(&normalize_word(word))
    }

    pub fn insert(&mut self, entry: CacheEntry) {
        self.entries.insert(normalize_word(&entry.word), entry);
    }
}

/// Where a displayed meaning came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeaningSource {
    Glossary,
    Online,
    Cached,
}

impl MeaningSource {
    pub fn label(self) -> &'static str {
        match self {
            MeaningSource::Glossary => "Story glossary",
            MeaningSource::Online => "Machine translated",
            MeaningSource::Cached => "Machine translated · cached",
        }
    }
}

/// Everything the popup needs. `meaning` is never empty.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LookupOutcome {
    /// The tapped word exactly as it appears in the story.
    pub word: String,
    /// Article, shown only when the tapped form is the lemma itself.
    pub article: Option<String>,
    /// Meaning of the tapped surface form (not of the lemma).
    pub meaning: String,
    /// Secondary line such as `form of: zijn, to be`.
    pub base: Option<String>,
    pub source: MeaningSource,
    pub note: Option<String>,
}

impl LookupOutcome {
    pub fn display_word(&self) -> String {
        match &self.article {
            Some(article) if !article.is_empty() => format!("{article} {}", self.word),
            _ => self.word.clone(),
        }
    }
}

/// Resolve one tapped word at a 1-based occurrence of its surface form.
/// Returns the outcome plus an optional cache entry the caller should persist.
pub fn resolve(
    story: &Story,
    word: &str,
    occurrence: u32,
    mode: MeaningsSource,
    cache: &LookupCache,
    online: Option<&dyn WordLookup>,
    now: u64,
) -> (LookupOutcome, Option<CacheEntry>) {
    let glossary = OfflineGlossary::new(story);
    let entry = match glossary.lookup_at(word, occurrence) {
        Ok(LookupHit::Glossary(entry)) => Some(entry),
        _ => None,
    };
    let base = entry.as_ref().and_then(|entry| base_line(story, entry));

    // Online lookups and the cache are keyed by the tapped word, never by the
    // lemma: `ben` must be looked up as `ben`, not as `zijn`.
    let key = normalize_word(word);

    if mode == MeaningsSource::Offline {
        return (glossary_outcome(entry.as_ref(), word, base, None), None);
    }

    if let Some(cached) = cache.get(&key) {
        return (
            glossary_outcome(
                entry.as_ref(),
                word,
                base,
                Some((cached.meaning.clone(), MeaningSource::Cached, None)),
            ),
            None,
        );
    }

    match online.map(|provider| provider.lookup(&key)) {
        Some(Ok(LookupHit::Machine(meaning))) => {
            let outcome = glossary_outcome(
                entry.as_ref(),
                word,
                base,
                Some((meaning.clone(), MeaningSource::Online, None)),
            );
            let cache_entry = CacheEntry {
                word: key,
                meaning,
                fetched_at: now,
            };
            (outcome, Some(cache_entry))
        }
        Some(Ok(LookupHit::Glossary(entry))) => {
            (glossary_outcome(Some(&entry), word, base, None), None)
        }
        Some(Err(error)) => {
            eprintln!("[lookup] online lookup failed: {error}");
            (
                glossary_outcome(
                    entry.as_ref(),
                    word,
                    base,
                    Some((
                        String::new(),
                        MeaningSource::Glossary,
                        Some(
                            "Could not reach the online dictionary. Showing the story glossary."
                                .into(),
                        ),
                    )),
                ),
                None,
            )
        }
        None => (
            glossary_outcome(
                entry.as_ref(),
                word,
                base,
                Some((
                    String::new(),
                    MeaningSource::Glossary,
                    Some("Online lookup is unavailable. Showing the bundled glossary.".into()),
                )),
            ),
            None,
        ),
    }
}

/// The secondary popup line: `form of: <lemma>, <lemma meaning>`.
fn base_line(story: &Story, entry: &GlossaryEntry) -> Option<String> {
    let form = normalize_word(&entry.form);
    let lemma = normalize_word(&entry.lemma);
    if form == lemma {
        return None;
    }
    let lemma_meaning = StarterWords::embedded()
        .words
        .iter()
        .find(|word| normalize_word(&word.lemma) == lemma)
        .map(|word| word.meaning.clone())
        .or_else(|| {
            story
                .lexicon()
                .lemma_entry(&entry.lemma)
                .map(|lemma_entry| lemma_entry.meaning.clone())
        });
    Some(match lemma_meaning {
        Some(meaning) => format!("form of: {}, {}", entry.lemma, meaning),
        None => format!("form of: {}", entry.lemma),
    })
}

/// Build an outcome, optionally replacing the meaning with a machine result.
fn glossary_outcome(
    entry: Option<&GlossaryEntry>,
    tapped: &str,
    base: Option<String>,
    machine: Option<(String, MeaningSource, Option<String>)>,
) -> LookupOutcome {
    let article = entry
        .filter(|entry| normalize_word(&entry.form) == normalize_word(&entry.lemma))
        .and_then(|entry| entry.article.clone());

    match machine {
        Some((meaning, source, note)) => {
            let meaning = if meaning.is_empty() {
                entry
                    .map(|entry| entry.meaning.clone())
                    .unwrap_or_else(|| format!("No meaning found for “{tapped}”."))
            } else {
                meaning
            };
            LookupOutcome {
                word: tapped.to_string(),
                article,
                meaning,
                base,
                source,
                note,
            }
        }
        None => LookupOutcome {
            word: tapped.to_string(),
            article,
            meaning: entry
                .map(|entry| entry.meaning.clone())
                .unwrap_or_else(|| format!("No glossary entry for “{tapped}”.")),
            base,
            source: MeaningSource::Glossary,
            note: None,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stories::index::StoryIndex;

    fn story() -> Story {
        StoryIndex::load_embedded()
            .expect("embedded stories")
            .get("a1-01")
            .expect("a1-01")
            .story
            .clone()
    }

    struct Fake {
        result: Result<String, LookupError>,
    }

    impl WordLookup for Fake {
        fn lookup(&self, _word: &str) -> Result<LookupHit, LookupError> {
            self.result.clone().map(LookupHit::Machine)
        }
    }

    #[test]
    fn offline_mode_uses_the_bundled_glossary() {
        let story = story();
        let word = story.glossary[0].form.clone();
        let (outcome, cache) = resolve(
            &story,
            &word,
            1,
            MeaningsSource::Offline,
            &LookupCache::default(),
            None,
            0,
        );
        assert_eq!(outcome.meaning, story.glossary[0].meaning);
        assert_eq!(outcome.source, MeaningSource::Glossary);
        assert!(cache.is_none());
    }

    #[test]
    fn offline_unknown_word_still_returns_a_popup() {
        let story = story();
        let (outcome, _) = resolve(
            &story,
            "onbekendwoord",
            1,
            MeaningsSource::Offline,
            &LookupCache::default(),
            None,
            0,
        );
        assert!(!outcome.meaning.is_empty());
        assert!(outcome.meaning.contains("onbekendwoord"));
    }

    #[test]
    fn online_success_is_machine_translated_and_cached() {
        let story = story();
        let fake = Fake {
            result: Ok("bicycle".into()),
        };
        let (outcome, cache) = resolve(
            &story,
            "fiets",
            1,
            MeaningsSource::Online,
            &LookupCache::default(),
            Some(&fake),
            123,
        );
        assert_eq!(outcome.meaning, "bicycle");
        assert_eq!(outcome.source, MeaningSource::Online);
        assert_eq!(outcome.source.label(), "Machine translated");
        let cache = cache.expect("cache entry");
        assert_eq!(cache.word, "fiets");
        assert_eq!(cache.fetched_at, 123);
    }

    #[test]
    fn online_failure_falls_back_to_the_glossary_with_a_note() {
        let story = story();
        let fake = Fake {
            result: Err(LookupError::Network("airplane mode".into())),
        };
        let (outcome, cache) = resolve(
            &story,
            "fiets",
            1,
            MeaningsSource::Online,
            &LookupCache::default(),
            Some(&fake),
            0,
        );
        assert_eq!(outcome.source, MeaningSource::Glossary);
        assert!(outcome.note.as_deref().unwrap().contains("Could not reach"));
        assert!(!outcome.meaning.is_empty());
        assert!(cache.is_none());
    }

    #[test]
    fn cached_word_works_without_a_provider() {
        let story = story();
        let mut cache = LookupCache::default();
        cache.insert(CacheEntry {
            word: "fiets".into(),
            meaning: "bicycle".into(),
            fetched_at: 1,
        });
        let (outcome, new_cache) =
            resolve(&story, "fiets", 1, MeaningsSource::Online, &cache, None, 0);
        assert_eq!(outcome.meaning, "bicycle");
        assert_eq!(outcome.source, MeaningSource::Cached);
        assert!(new_cache.is_none());
    }

    #[test]
    fn cache_is_keyed_by_the_tapped_word_and_normalised() {
        let mut cache = LookupCache::default();
        cache.insert(CacheEntry {
            word: "Ben".into(),
            meaning: "am".into(),
            fetched_at: 1,
        });
        assert!(cache.get("ben").is_some());
        assert!(cache.get("BEN").is_some());
        assert!(cache.get("zijn").is_none());
    }

    #[test]
    fn inflected_form_shows_its_own_meaning_and_the_base_line() {
        let story = story();
        let (ben, _) = resolve(
            &story,
            "ben",
            1,
            MeaningsSource::Offline,
            &LookupCache::default(),
            None,
            0,
        );
        assert_eq!(ben.word, "ben");
        assert_eq!(ben.meaning, "am");
        assert_eq!(ben.base.as_deref(), Some("form of: zijn, to be"));
        assert!(ben.article.is_none());

        let (is, _) = resolve(
            &story,
            "is",
            1,
            MeaningsSource::Offline,
            &LookupCache::default(),
            None,
            0,
        );
        assert_eq!(is.word, "is");
        assert_eq!(is.meaning, "is");
        assert_eq!(is.base.as_deref(), Some("form of: zijn, to be"));

        let (heeft, _) = resolve(
            &story,
            "heeft",
            1,
            MeaningsSource::Offline,
            &LookupCache::default(),
            None,
            0,
        );
        assert_eq!(heeft.word, "heeft");
        assert_eq!(heeft.meaning, "has");
        assert_eq!(heeft.base.as_deref(), Some("form of: hebben, to have"));
    }

    #[test]
    fn lemma_itself_has_no_base_line_and_keeps_its_article() {
        let story = story();
        let (hond, _) = resolve(
            &story,
            "hond",
            1,
            MeaningsSource::Offline,
            &LookupCache::default(),
            None,
            0,
        );
        assert_eq!(hond.word, "hond");
        assert_eq!(hond.article.as_deref(), Some("de"));
        assert_eq!(hond.base, None);
    }

    #[test]
    fn online_provider_is_asked_about_the_tapped_word() {
        struct Recording {
            seen: std::sync::Mutex<Vec<String>>,
        }
        impl WordLookup for Recording {
            fn lookup(&self, word: &str) -> Result<LookupHit, LookupError> {
                self.seen.lock().unwrap().push(word.to_string());
                Ok(LookupHit::Machine(format!("translation of {word}")))
            }
        }
        let story = story();
        let recorder = Recording {
            seen: std::sync::Mutex::new(Vec::new()),
        };
        let (outcome, cache) = resolve(
            &story,
            "ben",
            1,
            MeaningsSource::Online,
            &LookupCache::default(),
            Some(&recorder),
            0,
        );
        assert_eq!(recorder.seen.lock().unwrap().as_slice(), ["ben"]);
        assert_eq!(outcome.meaning, "translation of ben");
        assert_eq!(cache.unwrap().word, "ben");
    }

    #[test]
    fn occurrence_picks_the_right_sense() {
        let story = story();
        // A1 occurrence 7 of `zijn` is the infinitive in
        // "We willen samen blij zijn."
        let (infinitive, _) = resolve(
            &story,
            "zijn",
            7,
            MeaningsSource::Offline,
            &LookupCache::default(),
            None,
            0,
        );
        assert_eq!(infinitive.meaning, "to be");
        let (finite, _) = resolve(
            &story,
            "zijn",
            1,
            MeaningsSource::Offline,
            &LookupCache::default(),
            None,
            0,
        );
        assert_eq!(finite.meaning, "are");
    }

    #[test]
    fn occurrence_can_change_the_part_of_speech_and_meaning() {
        let story = StoryIndex::load_embedded()
            .expect("embedded stories")
            .get("b2-01")
            .expect("b2-01")
            .story
            .clone();

        let (his, _) = resolve(
            &story,
            "zijn",
            1,
            MeaningsSource::Offline,
            &LookupCache::default(),
            None,
            0,
        );
        assert_eq!(his.meaning, "his");
        assert_eq!(his.base, None);

        let (to_be, _) = resolve(
            &story,
            "zijn",
            9,
            MeaningsSource::Offline,
            &LookupCache::default(),
            None,
            0,
        );
        assert_eq!(to_be.meaning, "to be");

        let (are, _) = resolve(
            &story,
            "zijn",
            18,
            MeaningsSource::Offline,
            &LookupCache::default(),
            None,
            0,
        );
        assert_eq!(are.meaning, "are");

        let (to, _) = resolve(
            &story,
            "te",
            1,
            MeaningsSource::Offline,
            &LookupCache::default(),
            None,
            0,
        );
        assert_eq!(to.meaning, "to");

        let (too, _) = resolve(
            &story,
            "te",
            2,
            MeaningsSource::Offline,
            &LookupCache::default(),
            None,
            0,
        );
        assert_eq!(too.meaning, "too");
    }

    #[test]
    fn phrase_parts_resolve_to_the_phrase_entry() {
        let story = StoryIndex::load_embedded()
            .expect("embedded stories")
            .get("b2-01")
            .expect("b2-01")
            .story
            .clone();

        let (s, _) = resolve(
            &story,
            "'s",
            1,
            MeaningsSource::Offline,
            &LookupCache::default(),
            None,
            0,
        );
        assert_eq!(s.word, "'s");
        assert_eq!(s.meaning, "in the morning");
        assert_eq!(s.base, None);

        let (af, _) = resolve(
            &story,
            "Af",
            1,
            MeaningsSource::Offline,
            &LookupCache::default(),
            None,
            0,
        );
        assert_eq!(af.word, "Af");
        assert_eq!(af.meaning, "now and then");
        assert_eq!(af.base, None);
    }

    #[test]
    fn missing_form_is_not_masked_by_the_lemma() {
        let story = story();
        // `loopt` is a real entry; a made-up inflected form must not silently
        // return the lemma `lopen` meaning.
        let (outcome, _) = resolve(
            &story,
            "loopten",
            1,
            MeaningsSource::Offline,
            &LookupCache::default(),
            None,
            0,
        );
        assert!(outcome.meaning.contains("No glossary entry"));
    }

    #[test]
    fn parses_mymemory_and_decodes_entities() {
        let body = r#"{"responseData":{"translatedText":"don&#39;t walk"},"responseStatus":200}"#;
        assert_eq!(parse_mymemory(body).unwrap(), "don't walk");
    }

    #[test]
    fn rejects_mymemory_quota_warnings() {
        let body = r#"{"responseData":{"translatedText":"MYMEMORY WARNING: YOU USED ALL AVAILABLE FREE TRANSLATIONS FOR TODAY"}}"#;
        assert_eq!(parse_mymemory(body), Err(LookupError::Empty));
    }

    #[test]
    fn url_encoding_handles_dutch_diacritics() {
        assert_eq!(url_encode("zo'n fiets"), "zo%27n%20fiets");
        assert_eq!(url_encode("café"), "caf%C3%A9");
    }

    #[test]
    fn offline_glossary_resolves_separable_parts() {
        let story = story();
        let glossary = OfflineGlossary::new(&story);
        if let Some(entry) = story
            .glossary
            .iter()
            .find(|entry| entry.parts.as_ref().is_some_and(|parts| !parts.is_empty()))
        {
            let part = entry.parts.as_ref().unwrap()[0].clone();
            match glossary.lookup(&part).unwrap() {
                LookupHit::Glossary(found) => assert_eq!(found.lemma, entry.lemma),
                LookupHit::Machine(_) => panic!("expected glossary"),
            }
        }
    }
}
