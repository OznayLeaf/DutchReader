//! Command-line story validator.
//!
//! Examples:
//!   cargo run -p validate_stories
//!   cargo run -p validate_stories -- --allow-gaps
//!   cargo run -p validate_stories -- --write-wordlist --allow-gaps
//!   cargo run -p validate_stories -- --strict
//!   cargo run -p validate_stories -- --stories tests/fixtures/valid

use dutchreader::stories::loader::{LoadedStory, load_dir, load_embedded};
use dutchreader::stories::validate::{ValidateOptions, build_assumed_known, validate};
use dutchreader::stories::wordlist::Wordlist;
use std::path::PathBuf;

fn main() {
    let mut write_wordlist = false;
    let mut allow_gaps: Option<bool> = None;
    let mut stories_dir: Option<PathBuf> = None;

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--write-wordlist" => write_wordlist = true,
            "--allow-gaps" => allow_gaps = Some(true),
            "--strict" => allow_gaps = Some(false),
            "--stories" => match args.next() {
                Some(dir) => stories_dir = Some(PathBuf::from(dir)),
                None => fail("--stories needs a directory"),
            },
            "-h" | "--help" => {
                print_help();
                return;
            }
            other => fail(&format!("unknown argument: {other}")),
        }
    }

    let stories: Vec<LoadedStory> = match &stories_dir {
        Some(dir) => load_dir(dir).unwrap_or_else(|e| fail(&e.to_string())),
        None => load_embedded().unwrap_or_else(|e| fail(&e.to_string())),
    };

    let assets_dir = match &stories_dir {
        Some(dir) => dir.clone(),
        None => PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("validator lives inside the workspace")
            .join("assets"),
    };
    let wordlist_path = assets_dir.join("wordlist.json");
    let assumed_path = assets_dir.join("assumed_known.json");

    let allow_gaps = allow_gaps.unwrap_or(stories.len() < 100);

    let mut wordlist = if wordlist_path.exists() {
        Wordlist::load(&wordlist_path)
            .unwrap_or_else(|e| fail(&format!("cannot read {}: {e}", wordlist_path.display())))
    } else {
        eprintln!(
            "note: {} does not exist yet; starting with an empty word list",
            wordlist_path.display()
        );
        Wordlist::default()
    };

    if write_wordlist {
        wordlist = Wordlist::build_from(&stories);
        wordlist
            .save(&wordlist_path)
            .unwrap_or_else(|e| fail(&format!("cannot write {}: {e}", wordlist_path.display())));
        let assumed = build_assumed_known(&stories, &wordlist);
        assumed
            .save(&assumed_path)
            .unwrap_or_else(|e| fail(&format!("cannot write {}: {e}", assumed_path.display())));
        // The atomic writer keeps a `.bak` of the previous file. These files are
        // generated assets that are packaged into the APK, so a stale backup
        // would just add dead weight and confuse the next reader.
        remove_backup(&wordlist_path);
        remove_backup(&assumed_path);
        println!(
            "wrote {} ({} stories, {} words) and {} ({} assumed-known words)",
            wordlist_path.display(),
            wordlist.stories.len(),
            wordlist.words.len(),
            assumed_path.display(),
            assumed.words.len()
        );
    }

    println!(
        "validating {} story/stories ({} mode)\n",
        stories.len(),
        if allow_gaps { "allow-gaps" } else { "strict" }
    );
    let options = ValidateOptions {
        allow_gaps,
        coverage: true,
    };
    let report = validate(&stories, &wordlist, &options);
    let ok = report.print();
    if !ok {
        std::process::exit(1);
    }
}

fn print_help() {
    println!(
        "DutchReader story validator

Usage: validate_stories [OPTIONS]

Options:
  --allow-gaps       Treat words with no introduction as 'assumed known'
                     (warnings + debt list) instead of errors.
  --strict           Require every used word to be introduced by an earlier
                     story. Use once all 100 stories exist.
  --write-wordlist   Regenerate wordlist.json and assumed_known.json from the
                     story files.
  --stories <DIR>    Validate a different story tree (for tests/fixtures).
  -h, --help         Show this help.

With fewer than 100 stories, allow-gaps is the default."
    );
}

fn fail(message: &str) -> ! {
    eprintln!("error: {message}");
    std::process::exit(2);
}

fn remove_backup(path: &std::path::Path) {
    let mut name = path.as_os_str().to_os_string();
    name.push(".bak");
    let _ = std::fs::remove_file(PathBuf::from(name));
}
