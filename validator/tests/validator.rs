//! Fixture tests for the story validator.
//!
//! Each directory under `tests/fixtures/` is a complete story tree with its own
//! `wordlist.json`. The `valid` tree must pass; every `broken-*` tree must fail
//! with the specific diagnostic code named here.

use dutchreader::stories::loader::load_dir;
use dutchreader::stories::validate::{ValidateOptions, validate};
use dutchreader::stories::wordlist::Wordlist;
use std::path::{Path, PathBuf};
use std::process::Command;

const FIXTURES: &[(&str, &str)] = &[
    ("broken-19-words", "NEW_WORD_COUNT"),
    ("broken-missing-glossary", "MISSING_GLOSSARY"),
    ("broken-short-story", "MIN_SCREENS"),
    ("broken-old-word-new", "NEW_WORD_EARLIER"),
    ("broken-duplicate", "DUPLICATE_ID"),
    ("broken-identity", "IDENTITY"),
    ("broken-article", "GLOSSARY_ARTICLE"),
    ("broken-new-word-unused", "NEW_WORD_UNUSED"),
    ("broken-wordlist", "WORDLIST_OWNER"),
    ("broken-starter-as-new", "STARTER_AS_NEW"),
    ("broken-a1-extra-word", "A1_01_EXTRA_WORD"),
    ("broken-lemma-meaning", "GLOSSARY_LEMMA_MEANING"),
];

fn fixture_dir(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("validator lives inside the workspace")
        .join("tests")
        .join("fixtures")
        .join(name)
}

fn validate_fixture(name: &str, allow_gaps: bool) -> dutchreader::stories::validate::Report {
    let dir = fixture_dir(name);
    let stories = load_dir(&dir).unwrap_or_else(|e| panic!("{name}: {e}"));
    let wordlist =
        Wordlist::load(&dir.join("wordlist.json")).unwrap_or_else(|e| panic!("{name}: {e}"));
    let options = ValidateOptions {
        allow_gaps,
        coverage: true,
    };
    validate(&stories, &wordlist, &options)
}

#[test]
fn valid_fixture_passes_in_strict_mode() {
    let report = validate_fixture("valid", false);
    let messages: Vec<String> = report
        .diagnostics
        .iter()
        .map(|d| format!("{} [{}] {}", d.severity, d.code, d.message))
        .collect();
    assert!(
        report.is_ok(),
        "valid fixture must not have errors:\n{}",
        messages.join("\n")
    );
}

#[test]
fn every_broken_fixture_reports_its_expected_error() {
    for &(name, code) in FIXTURES {
        let report = validate_fixture(name, false);
        assert!(
            report.has_error(code),
            "{name} should report an error with code {code}, got:\n{}",
            report
                .diagnostics
                .iter()
                .map(|d| format!("{} [{}] {}", d.severity, d.code, d.message))
                .collect::<Vec<_>>()
                .join("\n")
        );
    }
}

#[test]
fn allow_gaps_turns_unintroduced_words_into_a_warning() {
    // Add two words that no story introduces, then validate without a word list.
    let dir = fixture_dir("valid");
    let mut stories = load_dir(&dir).unwrap();
    stories[0].source = "a2/01-de-fiets.json".into();
    let story = &mut stories[0].story;
    story.id = "a2-01".into();
    story.level = dutchreader::stories::model::Level::A2;
    story.sections[0].push_str(" De kat is vaak hier.");
    for (form, lemma, meaning) in [("kat", "kat", "cat"), ("vaak", "vaak", "often")] {
        story
            .glossary
            .push(dutchreader::stories::model::GlossaryEntry {
                form: form.into(),
                parts: None,
                lemma: lemma.into(),
                pos: if form == "kat" { "noun" } else { "adverb" }.into(),
                article: if form == "kat" {
                    Some("de".into())
                } else {
                    None
                },
                meaning: meaning.into(),
                new: false,
                occurrence: None,
            });
    }
    let report = validate(
        &stories,
        &Wordlist::default(),
        &ValidateOptions {
            allow_gaps: true,
            coverage: true,
        },
    );
    assert!(
        report.is_ok(),
        "allow-gaps mode should not produce errors for the valid fixture"
    );
    assert!(
        report.warnings().any(|d| d.code == "ASSUMED_KNOWN"),
        "allow-gaps mode should list assumed-known words"
    );
}

#[test]
fn cli_exits_nonzero_for_every_broken_fixture() {
    for &(name, code) in FIXTURES {
        let output = Command::new(env!("CARGO_BIN_EXE_validate_stories"))
            .arg("--stories")
            .arg(fixture_dir(name))
            .arg("--strict")
            .output()
            .unwrap_or_else(|e| panic!("cannot run validator for {name}: {e}"));
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            !output.status.success(),
            "validator should fail for {name}; stdout:\n{stdout}"
        );
        assert!(
            stdout.contains(&format!("[{code}]")),
            "validator output for {name} should mention [{code}]; stdout:\n{stdout}"
        );
    }
}

#[test]
fn cli_passes_the_valid_fixture() {
    let output = Command::new(env!("CARGO_BIN_EXE_validate_stories"))
        .arg("--stories")
        .arg(fixture_dir("valid"))
        .arg("--strict")
        .output()
        .expect("cannot run validator");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "validator should pass the valid fixture; stdout:\n{stdout}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(stdout.contains("0 error(s)"), "stdout:\n{stdout}");
}
