//! Story validation.
//!
//! The validator is intentionally independent of the UI so it can run in unit
//! tests, in the CLI, and as a debug-build startup check.
//!
//! Errors mean a story must not ship. Warnings mean a story is suspicious but
//! still playable (for example a new word used only once, or known-word
//! coverage below the target for its level).

use super::loader::{LoadedStory, split_source};
use super::model::{Level, Story, StoryLexicon, normalize_word};
use super::starter::StarterWords;
use super::tokenize::{Token, tokenize};
use super::wordlist::{AssumedKnown, Wordlist};
use crate::reader::layout::{
    MIN_SCREENS, REFERENCE_LARGE_HEIGHT, REFERENCE_LARGE_WIDTH, REFERENCE_SMALL_HEIGHT,
    REFERENCE_SMALL_WIDTH, paginate_story,
};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Error => f.write_str("error"),
            Severity::Warning => f.write_str("warning"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub severity: Severity,
    pub code: &'static str,
    pub story_id: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Default)]
pub struct Report {
    pub diagnostics: Vec<Diagnostic>,
}

impl Report {
    pub fn is_ok(&self) -> bool {
        !self
            .diagnostics
            .iter()
            .any(|d| d.severity == Severity::Error)
    }

    pub fn errors(&self) -> impl Iterator<Item = &Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
    }

    pub fn warnings(&self) -> impl Iterator<Item = &Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Warning)
    }

    pub fn error_count(&self) -> usize {
        self.errors().count()
    }

    pub fn warning_count(&self) -> usize {
        self.warnings().count()
    }

    pub fn has_code(&self, code: &str) -> bool {
        self.diagnostics.iter().any(|d| d.code == code)
    }

    pub fn has_error(&self, code: &str) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.code == code && d.severity == Severity::Error)
    }

    pub fn push(
        &mut self,
        severity: Severity,
        code: &'static str,
        story_id: Option<&str>,
        message: impl Into<String>,
    ) {
        self.diagnostics.push(Diagnostic {
            severity,
            code,
            story_id: story_id.map(str::to_string),
            message: message.into(),
        });
    }

    fn error(&mut self, code: &'static str, story_id: Option<&str>, message: impl Into<String>) {
        self.push(Severity::Error, code, story_id, message);
    }

    fn warning(&mut self, code: &'static str, story_id: Option<&str>, message: impl Into<String>) {
        self.push(Severity::Warning, code, story_id, message);
    }

    /// Print a readable report to stdout. Returns true when there are no errors.
    pub fn print(&self) -> bool {
        for diagnostic in &self.diagnostics {
            match &diagnostic.story_id {
                Some(id) => println!(
                    "{} [{}] {}: {}",
                    diagnostic.severity, diagnostic.code, id, diagnostic.message
                ),
                None => println!(
                    "{} [{}]: {}",
                    diagnostic.severity, diagnostic.code, diagnostic.message
                ),
            }
        }
        let errors = self.error_count();
        let warnings = self.warning_count();
        println!("\n{errors} error(s), {warnings} warning(s)");
        errors == 0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ValidateOptions {
    /// While the app has fewer than 100 stories, words without an introduction
    /// are "assumed known" (warnings) instead of hard errors. `--strict` turns
    /// this off.
    pub allow_gaps: bool,
    /// Report known-word coverage warnings.
    pub coverage: bool,
}

impl ValidateOptions {
    pub fn for_story_count(count: usize) -> Self {
        ValidateOptions {
            allow_gaps: count < 100,
            coverage: true,
        }
    }
}

impl Default for ValidateOptions {
    fn default() -> Self {
        ValidateOptions {
            allow_gaps: true,
            coverage: true,
        }
    }
}

/// Known-word coverage target by level and story number.
///
/// The brief's 85-90% target cannot hold in early A1 (a 20-word story with 20
/// new words has no known words at all), so the threshold ramps up. Below the
/// threshold is a warning, never an error.
pub fn coverage_threshold(level: Level, order: u8) -> f64 {
    let t = (f64::from(order.clamp(1, 25)) - 1.0) / 24.0;
    match level {
        Level::A1 => 0.80 * t,
        Level::A2 => 0.75 + 0.13 * t,
        Level::B1 => 0.82 + 0.08 * t,
        Level::B2 => 0.85 + 0.05 * t,
    }
}

const KNOWN_POS: &[&str] = &[
    "noun",
    "verb",
    "adjective",
    "adverb",
    "pronoun",
    "preposition",
    "conjunction",
    "interjection",
    "numeral",
    "article",
    "determiner",
    "name",
    "phrase",
    "expression",
    "other",
];

struct StoryFacts<'a> {
    loaded: &'a LoadedStory,
    lexicon: StoryLexicon<'a>,
    missing_forms: BTreeMap<String, usize>,
    lemma_counts: HashMap<String, usize>,
    used_lemmas: BTreeSet<String>,
    non_name_tokens: usize,
}

impl<'a> StoryFacts<'a> {
    fn new(loaded: &'a LoadedStory) -> Self {
        let lexicon = loaded.story.lexicon();
        let tokens: Vec<Token<'a>> = loaded
            .story
            .sections
            .iter()
            .flat_map(|section| tokenize(section))
            .collect();

        let mut missing_forms = BTreeMap::new();
        let mut lemma_counts: HashMap<String, usize> = HashMap::new();
        let mut used_lemmas = BTreeSet::new();
        let mut non_name_tokens = 0;
        let mut form_counts: HashMap<String, u32> = HashMap::new();

        for token in &tokens {
            let key = normalize_word(token.text);
            let occurrence = {
                let count = form_counts.entry(key).or_insert(0);
                *count += 1;
                *count
            };
            let Some(entry) = lexicon.resolve_at(token.text, occurrence) else {
                *missing_forms.entry(normalize_word(token.text)).or_insert(0) += 1;
                continue;
            };
            if entry.is_name() {
                continue;
            }
            non_name_tokens += 1;
            let lemma = normalize_word(&entry.lemma);
            *lemma_counts.entry(lemma.clone()).or_insert(0) += 1;
            used_lemmas.insert(lemma);
        }

        StoryFacts {
            loaded,
            lexicon,
            missing_forms,
            lemma_counts,
            used_lemmas,
            non_name_tokens,
        }
    }

    fn story(&self) -> &'a Story {
        &self.loaded.story
    }

    fn new_lemma_tokens(&self) -> usize {
        self.story()
            .new_words
            .iter()
            .map(|word| {
                self.lemma_counts
                    .get(&normalize_word(word))
                    .copied()
                    .unwrap_or(0)
            })
            .sum()
    }
}

/// Validate every story against the cumulative word list.
pub fn validate(stories: &[LoadedStory], wordlist: &Wordlist, options: &ValidateOptions) -> Report {
    let mut report = Report::default();
    let facts: Vec<StoryFacts<'_>> = stories.iter().map(StoryFacts::new).collect();

    // Global reading order, independent of the order the caller supplied.
    let mut order: Vec<usize> = (0..facts.len()).collect();
    order.sort_by(|&a, &b| {
        facts[a]
            .story()
            .level
            .cmp(&facts[b].story().level)
            .then(facts[a].story().order.cmp(&facts[b].story().order))
            .then(facts[a].story().id.cmp(&facts[b].story().id))
    });

    let mut position_by_id: HashMap<&str, usize> = HashMap::new();
    for (position, &idx) in order.iter().enumerate() {
        position_by_id
            .entry(facts[idx].story().id.as_str())
            .or_insert(position);
    }

    check_identity_and_duplicates(&facts, &mut report);
    check_story_structure(&facts, &mut report);
    check_glossary(&facts, &mut report);

    let mut earlier_used: BTreeMap<String, String> = BTreeMap::new();
    let mut assumed: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for &idx in &order {
        let fact = &facts[idx];
        let story = fact.story();
        let id = story.id.as_str();
        let new_set = story.new_word_set();

        // --- new words that appeared earlier -------------------------------
        for word in &story.new_words {
            let lemma = normalize_word(word);
            if StarterWords::embedded().is_lemma(&lemma) {
                report.error(
                    "STARTER_AS_NEW",
                    Some(id),
                    format!("starter word `{lemma}` must not be listed in new_words"),
                );
                // One clear error is enough; the starter is known from the
                // start, so do not also report it as an earlier word or as a
                // wordlist ownership problem.
                continue;
            }
            if let Some(previous) = earlier_used.get(&lemma) {
                report.error(
                    "NEW_WORD_EARLIER",
                    Some(id),
                    format!("new word `{lemma}` already appeared in earlier story `{previous}`"),
                );
            }
            if let Some(owner) = wordlist.introduced_by(&lemma)
                && owner != id
            {
                let owner_position = position_by_id.get(owner).copied();
                let relation = match owner_position {
                    Some(p) if p < position_by_id[id] => "an earlier",
                    Some(_) => "a later",
                    None => "a missing",
                };
                report.error(
                    "NEW_WORD_WORDLIST_OWNER",
                    Some(id),
                    format!(
                        "new word `{lemma}` is recorded in wordlist.json as introduced by {relation} story `{owner}`"
                    ),
                );
            }
        }

        // --- repetition of new words ---------------------------------------
        for word in &story.new_words {
            let lemma = normalize_word(word);
            let count = fact.lemma_counts.get(&lemma).copied().unwrap_or(0);
            if count == 0 {
                report.error(
                    "NEW_WORD_UNUSED",
                    Some(id),
                    format!("new word `{lemma}` never appears in the story text"),
                );
            } else if count == 1 {
                report.warning(
                    "NEW_WORD_REPEAT",
                    Some(id),
                    format!("new word `{lemma}` appears only once; aim for 2-3 uses"),
                );
            }
        }

        // --- known-word coverage -------------------------------------------
        if options.coverage && fact.non_name_tokens > 0 {
            let new_tokens = fact.new_lemma_tokens();
            let known = fact.non_name_tokens.saturating_sub(new_tokens);
            let coverage = known as f64 / fact.non_name_tokens as f64;
            let threshold = coverage_threshold(story.level, story.order);
            if coverage + f64::EPSILON < threshold {
                report.warning(
                    "COVERAGE",
                    Some(id),
                    format!(
                        "known-word coverage {:.1}% is below the {:.0}% target for {} story {}",
                        coverage * 100.0,
                        threshold * 100.0,
                        story.level,
                        story.order
                    ),
                );
            }
        }

        // --- words used without an introduction -----------------------------
        let mut unintroduced: Vec<String> = Vec::new();
        for lemma in &fact.used_lemmas {
            if new_set.contains(lemma) {
                continue;
            }
            if wordlist.is_starter(lemma) || StarterWords::embedded().is_lemma(lemma) {
                // Starter words are known from the first screen.
                continue;
            }
            match wordlist.introduced_by(lemma) {
                Some(owner)
                    if position_by_id.get(owner).copied().unwrap_or(usize::MAX)
                        < position_by_id[id] =>
                {
                    // Properly introduced before this story.
                }
                Some(owner) => {
                    report.error(
                        "WORD_INTRODUCED_LATER",
                        Some(id),
                        format!(
                            "word `{lemma}` is used before the story that introduces it (`{owner}`)"
                        ),
                    );
                }
                None => {
                    unintroduced.push(lemma.clone());
                    assumed
                        .entry(lemma.clone())
                        .or_default()
                        .push(story.id.clone());
                }
            }
        }
        unintroduced.sort();
        if !unintroduced.is_empty() {
            if options.allow_gaps {
                report.warning(
                    "ASSUMED_KNOWN",
                    Some(id),
                    format!(
                        "{} word(s) assumed known (no earlier story introduces them): {}",
                        unintroduced.len(),
                        preview_list(&unintroduced, 12)
                    ),
                );
            } else {
                report.error(
                    "UNINTRODUCED_WORD",
                    Some(id),
                    format!(
                        "{} word(s) are not introduced by this or any earlier story: {}",
                        unintroduced.len(),
                        preview_list(&unintroduced, 20)
                    ),
                );
            }
        }

        // A1-01 is the first story: exactly 20 new words, and every other word
        // must be a starter word or a proper name (ARCHITECTURE.md 2.5).
        if story.id == "a1-01" {
            let starter_set = StarterWords::embedded().lemma_set();
            let extra: Vec<&String> = fact
                .used_lemmas
                .iter()
                .filter(|lemma| !new_set.contains(*lemma) && !starter_set.contains(*lemma))
                .collect();
            if !extra.is_empty() {
                report.error(
                    "A1_01_EXTRA_WORD",
                    Some(id),
                    format!(
                        "A1-01 may only use its 20 new words plus starter words and proper names; extra: {}",
                        extra
                            .iter()
                            .map(|s| s.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                );
            }
        }

        // This story's words become "earlier" for all following stories.
        for lemma in &fact.used_lemmas {
            earlier_used
                .entry(lemma.clone())
                .or_insert_with(|| story.id.clone());
        }
    }

    check_wordlist(stories, &facts, wordlist, options, &mut report);

    // Deterministic output: story order, errors before warnings, then code.
    report.diagnostics.sort_by(|a, b| {
        let pos = |id: &Option<String>| {
            id.as_deref()
                .and_then(|id| position_by_id.get(id).copied())
                .unwrap_or(usize::MAX)
        };
        pos(&a.story_id)
            .cmp(&pos(&b.story_id))
            .then((a.severity == Severity::Warning).cmp(&(b.severity == Severity::Warning)))
            .then(a.code.cmp(b.code))
            .then(a.message.cmp(&b.message))
    });

    report
}

fn check_identity_and_duplicates(facts: &[StoryFacts<'_>], report: &mut Report) {
    let mut ids: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut positions: HashMap<(Level, u8), Vec<&str>> = HashMap::new();

    for fact in facts {
        let story = fact.story();
        let id = story.id.as_str();
        ids.entry(id).or_default().push(fact.loaded.source.as_str());
        positions
            .entry((story.level, story.order))
            .or_default()
            .push(id);

        if !(1..=25).contains(&story.order) {
            report.error(
                "ORDER_RANGE",
                Some(id),
                format!("order must be between 1 and 25, got {}", story.order),
            );
        }

        let expected_id = format!("{}-{:02}", story.level.folder(), story.order);
        if story.id != expected_id {
            report.error(
                "IDENTITY",
                Some(id),
                format!(
                    "id must be `{expected_id}` for {} story {}",
                    story.level, story.order
                ),
            );
        }

        let (folder, file) = split_source(&fact.loaded.source);
        if let Some(folder) = folder
            && Level::from_folder(folder) != Some(story.level)
        {
            report.error(
                "IDENTITY",
                Some(id),
                format!(
                    "file is in folder `{folder}` but story level is {}",
                    story.level
                ),
            );
        }
        if let Some(file) = file {
            let expected_prefix = format!("{:02}-", story.order);
            if !file.starts_with(&expected_prefix) {
                report.error(
                    "IDENTITY",
                    Some(id),
                    format!("file name `{file}` must start with `{expected_prefix}`"),
                );
            }
            if !file.ends_with(".json") {
                report.error(
                    "IDENTITY",
                    Some(id),
                    format!("file name `{file}` must end with .json"),
                );
            }
        }

        if story.title.trim().is_empty() {
            report.error("IDENTITY", Some(id), "title must not be empty");
        }
        if story.blurb_nl.trim().is_empty() {
            report.warning("IDENTITY", Some(id), "blurb_nl is empty");
        }
    }

    for (id, sources) in ids {
        if sources.len() > 1 {
            report.error(
                "DUPLICATE_ID",
                Some(id),
                format!("story id is used more than once: {}", sources.join(", ")),
            );
        }
    }
    for ((level, order), ids) in positions {
        if ids.len() > 1 {
            report.error(
                "DUPLICATE_ORDER",
                ids.first().copied(),
                format!(
                    "{} story {} is used by more than one story: {}",
                    level,
                    order,
                    ids.join(", ")
                ),
            );
        }
    }
}

fn check_story_structure(facts: &[StoryFacts<'_>], report: &mut Report) {
    for fact in facts {
        let story = fact.story();
        let id = story.id.as_str();
        let large = paginate_story(story, REFERENCE_LARGE_WIDTH, REFERENCE_LARGE_HEIGHT).len();
        let small = paginate_story(story, REFERENCE_SMALL_WIDTH, REFERENCE_SMALL_HEIGHT).len();
        if large < MIN_SCREENS || small < MIN_SCREENS {
            report.error(
                "MIN_SCREENS",
                Some(id),
                format!(
                    "a story must render as at least {MIN_SCREENS} screens; it renders as {large} on a 1080x2400 phone and {small} on a 360dp-wide phone"
                ),
            );
        }

        let unique: HashSet<String> = story.new_words.iter().map(|w| normalize_word(w)).collect();
        if story.new_words.len() != 20 {
            report.error(
                "NEW_WORD_COUNT",
                Some(id),
                format!(
                    "a story must introduce exactly 20 new words, got {}",
                    story.new_words.len()
                ),
            );
        }
        if unique.len() != story.new_words.len() {
            report.error(
                "DUPLICATE_NEW_WORD",
                Some(id),
                "new_words contains duplicates",
            );
        }
        for word in &story.new_words {
            if word.trim().is_empty() {
                report.error(
                    "NEW_WORD_COUNT",
                    Some(id),
                    "new_words contains an empty entry",
                );
            }
        }

        // Missing glossary words: report each unique surface form once.
        if !fact.missing_forms.is_empty() {
            let list = fact
                .missing_forms
                .iter()
                .map(|(form, count)| format!("{form} ({count}x)"))
                .collect::<Vec<_>>();
            report.error(
                "MISSING_GLOSSARY",
                Some(id),
                format!(
                    "{} word form(s) in the text have no glossary entry: {}",
                    list.len(),
                    preview_list(&list, 20)
                ),
            );
        }
    }
}

fn check_glossary(facts: &[StoryFacts<'_>], report: &mut Report) {
    for fact in facts {
        let story = fact.story();
        let id = story.id.as_str();
        let new_set = story.new_word_set();
        let mut form_entries: HashMap<String, Vec<(usize, Option<u32>)>> = HashMap::new();
        let mut token_counts: HashMap<String, u32> = HashMap::new();
        for token in story.tokens() {
            *token_counts.entry(normalize_word(token.text)).or_insert(0) += 1;
        }

        for (index, entry) in story.glossary.iter().enumerate() {
            let where_ = format!("glossary[{index}] (`{}`)", entry.form);
            if entry.form.trim().is_empty() {
                report.error(
                    "GLOSSARY_INVALID",
                    Some(id),
                    format!("{where_}: form is empty"),
                );
            }
            if entry.lemma.trim().is_empty() {
                report.error(
                    "GLOSSARY_INVALID",
                    Some(id),
                    format!("{where_}: lemma is empty"),
                );
            }
            if entry.meaning.trim().is_empty() {
                report.error(
                    "GLOSSARY_INVALID",
                    Some(id),
                    format!("{where_}: meaning is empty"),
                );
            }
            if entry.pos.trim().is_empty() {
                report.error(
                    "GLOSSARY_INVALID",
                    Some(id),
                    format!("{where_}: pos is empty"),
                );
            } else if !KNOWN_POS.contains(&entry.pos.to_ascii_lowercase().as_str()) {
                report.warning(
                    "GLOSSARY_POS",
                    Some(id),
                    format!("{where_}: unknown part of speech `{}`", entry.pos),
                );
            }

            if entry.is_noun() {
                match entry.article.as_deref().map(str::to_ascii_lowercase) {
                    Some(article) if article == "de" || article == "het" => {}
                    Some(article) => report.error(
                        "GLOSSARY_ARTICLE",
                        Some(id),
                        format!("{where_}: noun article must be `de` or `het`, got `{article}`"),
                    ),
                    None => report.error(
                        "GLOSSARY_ARTICLE",
                        Some(id),
                        format!("{where_}: nouns must include an article (de/het)"),
                    ),
                }
            } else if entry.article.is_some() {
                report.warning(
                    "GLOSSARY_ARTICLE_EXTRA",
                    Some(id),
                    format!("{where_}: article is only expected for nouns"),
                );
            }

            let key = normalize_word(&entry.form);
            if !key.is_empty() {
                form_entries
                    .entry(key.clone())
                    .or_default()
                    .push((index, entry.occurrence));
            }

            if let Some(occurrence) = entry.occurrence {
                if occurrence == 0 {
                    report.error(
                        "GLOSSARY_OCCURRENCE",
                        Some(id),
                        format!("{where_}: occurrence must be 1 or greater"),
                    );
                } else {
                    let count = token_counts.get(&key).copied().unwrap_or(0);
                    if occurrence > count {
                        report.error(
                            "GLOSSARY_OCCURRENCE",
                            Some(id),
                            format!(
                                "{where_}: occurrence {occurrence} is never reached; `{}` appears {count} time(s)",
                                entry.form
                            ),
                        );
                    }
                }
            }

            if entry.new {
                let lemma = normalize_word(&entry.lemma);
                if StarterWords::embedded().is_lemma(&lemma) {
                    report.error(
                        "STARTER_AS_NEW",
                        Some(id),
                        format!("{where_}: starter word `{lemma}` must not be flagged new"),
                    );
                }
                if !new_set.contains(&lemma) {
                    report.error(
                        "GLOSSARY_NEW_NOT_LISTED",
                        Some(id),
                        format!("{where_}: flagged new, but `{lemma}` is not in new_words"),
                    );
                }
            }

            // A non-lemma form must carry its own meaning, not a copy of the
            // base entry's meaning. This catches `gaat` showing "to go" and
            // similar base-form leaks.
            let form = normalize_word(&entry.form);
            let lemma = normalize_word(&entry.lemma);
            if form != lemma
                && let Some(lemma_entry) = story.glossary.iter().find(|candidate| {
                    candidate.occurrence.is_none()
                        && normalize_word(&candidate.form) == lemma
                        && normalize_word(&candidate.lemma) == lemma
                })
                && entry
                    .meaning
                    .trim()
                    .eq_ignore_ascii_case(lemma_entry.meaning.trim())
            {
                report.error(
                    "GLOSSARY_LEMMA_MEANING",
                    Some(id),
                    format!(
                        "{where_}: meaning `{}` is the lemma's meaning; give the surface form its own meaning",
                        entry.meaning
                    ),
                );
            }
        }

        for (form, entries) in form_entries {
            let defaults = entries
                .iter()
                .filter(|(_, occurrence)| occurrence.is_none())
                .count();
            let mut seen_occurrences = HashSet::new();
            for (index, occurrence) in &entries {
                if let Some(occurrence) = occurrence
                    && !seen_occurrences.insert(*occurrence)
                {
                    report.error(
                        "GLOSSARY_DUPLICATE_FORM",
                        Some(id),
                        format!(
                            "glossary form `{form}` uses occurrence {occurrence} more than once (glossary[{index}])"
                        ),
                    );
                }
            }
            if defaults > 1 {
                report.error(
                    "GLOSSARY_DUPLICATE_FORM",
                    Some(id),
                    format!(
                        "glossary form `{form}` has {defaults} default entries; only one may omit `occurrence`"
                    ),
                );
            }
            if entries.len() > 1 && defaults == 0 {
                report.warning(
                    "GLOSSARY_DUPLICATE_FORM",
                    Some(id),
                    format!(
                        "glossary form `{form}` has only occurrence-specific entries; occurrences not listed fall back to the first"
                    ),
                );
            }
        }

        for word in &story.new_words {
            let lemma = normalize_word(word);
            let has_new_entry = story
                .glossary
                .iter()
                .any(|entry| normalize_word(&entry.lemma) == lemma && entry.new);
            if !has_new_entry {
                report.error(
                    "NEW_WORD_GLOSSARY",
                    Some(id),
                    format!("new word `{lemma}` has no glossary entry flagged `\"new\": true`"),
                );
            }
            if fact.lexicon.lemma_entry(&lemma).is_none() {
                report.error(
                    "NEW_WORD_GLOSSARY",
                    Some(id),
                    format!("new word `{lemma}` is not in the glossary at all"),
                );
            }
        }
    }
}

fn check_wordlist(
    stories: &[LoadedStory],
    facts: &[StoryFacts<'_>],
    wordlist: &Wordlist,
    options: &ValidateOptions,
    report: &mut Report,
) {
    if wordlist.version != super::wordlist::WORDLIST_VERSION {
        report.warning(
            "WORDLIST_VERSION",
            None,
            format!(
                "wordlist.json version is {}, expected {}",
                wordlist.version,
                super::wordlist::WORDLIST_VERSION
            ),
        );
    }

    let by_id: HashMap<&str, &StoryFacts<'_>> = facts
        .iter()
        .map(|fact| (fact.story().id.as_str(), fact))
        .collect();

    // Every recorded introduction must point at a real story that lists it.
    for (lemma, owner) in &wordlist.words {
        if owner == "starter" {
            if !StarterWords::embedded().is_lemma(lemma) {
                report.error(
                    "STARTER_WORDLIST",
                    None,
                    format!("wordlist.json says `{lemma}` is a starter word, but it is not in starter_words.json"),
                );
            }
            continue;
        }
        match by_id.get(owner.as_str()) {
            Some(fact) => {
                let listed = fact
                    .story()
                    .new_words
                    .iter()
                    .any(|word| normalize_word(word) == *lemma);
                if !listed {
                    report.error(
                        "WORDLIST_STALE",
                        Some(owner),
                        format!(
                            "wordlist.json says `{lemma}` is introduced here, but it is not in new_words"
                        ),
                    );
                }
            }
            None => {
                let message =
                    format!("wordlist.json references unknown story `{owner}` for word `{lemma}`");
                if options.allow_gaps {
                    report.warning("WORDLIST_UNKNOWN_STORY", None, message);
                } else {
                    report.error("WORDLIST_UNKNOWN_STORY", None, message);
                }
            }
        }
    }

    // Every new word must be recorded, owned by its own story.
    for fact in facts {
        let story = fact.story();
        for word in &story.new_words {
            let lemma = normalize_word(word);
            if StarterWords::embedded().is_lemma(&lemma) {
                continue;
            }
            match wordlist.introduced_by(&lemma) {
                Some(owner) if owner == story.id => {}
                Some(owner) => report.error(
                    "WORDLIST_OWNER",
                    Some(&story.id),
                    format!(
                        "new word `{lemma}` is recorded as introduced by `{owner}`, not by this story"
                    ),
                ),
                None => {
                    let message = format!(
                        "new word `{lemma}` is missing from wordlist.json; run with --write-wordlist"
                    );
                    if options.allow_gaps {
                        report.warning("WORDLIST_STALE", Some(&story.id), message);
                    } else {
                        report.error("WORDLIST_STALE", Some(&story.id), message);
                    }
                }
            }
        }
    }

    // The wordlist's story order should match the actual story set.
    let mut actual_ids: Vec<&str> = facts.iter().map(|f| f.story().id.as_str()).collect();
    actual_ids.sort_unstable();
    let mut listed_ids: Vec<&str> = wordlist.stories.iter().map(|s| s.id.as_str()).collect();
    listed_ids.sort_unstable();
    if actual_ids != listed_ids {
        let message = format!(
            "wordlist.json lists {} stories but {} are present; run with --write-wordlist",
            listed_ids.len(),
            actual_ids.len()
        );
        if options.allow_gaps {
            report.warning("WORDLIST_STORIES", None, message);
        } else {
            report.error("WORDLIST_STORIES", None, message);
        }
    }

    // Duplicate introductions (same lemma marked new by two stories).
    let mut introductions: HashMap<String, Vec<&str>> = HashMap::new();
    for story in stories.iter().map(|s| &s.story) {
        for word in &story.new_words {
            introductions
                .entry(normalize_word(word))
                .or_default()
                .push(story.id.as_str());
        }
    }
    for (lemma, owners) in introductions {
        if owners.len() > 1 {
            report.error(
                "DUPLICATE_INTRODUCTION",
                Some(owners[1]),
                format!(
                    "word `{lemma}` is introduced more than once: {}",
                    owners.join(", ")
                ),
            );
        }
    }
}

/// Build the debt list of words the sample stories use without introducing.
pub fn build_assumed_known(stories: &[LoadedStory], wordlist: &Wordlist) -> AssumedKnown {
    let mut debt = AssumedKnown::default();
    for loaded in stories {
        let lexicon = loaded.story.lexicon();
        let new_set = loaded.story.new_word_set();
        let mut used: BTreeSet<String> = BTreeSet::new();
        for section in &loaded.story.sections {
            for token in tokenize(section) {
                let Some(entry) = lexicon.resolve(token.text) else {
                    continue;
                };
                if entry.is_name() {
                    continue;
                }
                let lemma = normalize_word(&entry.lemma);
                if new_set.contains(&lemma)
                    || wordlist.contains(&lemma)
                    || StarterWords::embedded().is_lemma(&lemma)
                {
                    continue;
                }
                used.insert(lemma);
            }
        }
        for lemma in used {
            debt.words
                .entry(lemma)
                .or_default()
                .push(loaded.story.id.clone());
        }
    }
    debt
}

/// Debug-build startup check. Never panics and never blocks the UI.
pub fn startup_check() {
    let stories = match super::loader::load_embedded() {
        Ok(stories) => stories,
        Err(error) => {
            eprintln!("[story validation] could not load embedded stories: {error}");
            return;
        }
    };
    let wordlist = Wordlist::load_embedded().unwrap_or_default();
    let options = ValidateOptions::for_story_count(stories.len());
    let report = validate(&stories, &wordlist, &options);
    for diagnostic in &report.diagnostics {
        eprintln!(
            "[story validation] {} [{}] {}: {}",
            diagnostic.severity,
            diagnostic.code,
            diagnostic.story_id.as_deref().unwrap_or("-"),
            diagnostic.message
        );
    }
    eprintln!(
        "[story validation] {} stories, {} error(s), {} warning(s)",
        stories.len(),
        report.error_count(),
        report.warning_count()
    );
}

fn preview_list(items: &[String], limit: usize) -> String {
    if items.len() <= limit {
        items.join(", ")
    } else {
        format!(
            "{}, ... and {} more",
            items[..limit].join(", "),
            items.len() - limit
        )
    }
}
