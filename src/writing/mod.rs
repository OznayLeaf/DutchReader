//! Writing feature: draft autosave, finished-story store, statistics, share text.
//!
//! Writing data lives in its own files under `writing/`, separate from reader
//! data. All writes go through [`crate::storage::atomic`].

pub mod draft;
pub mod share_text;
pub mod stats;
pub mod store;

/// Count words the way the editor does: whitespace-separated tokens. This is
/// intentionally simple so the live count, the saved story, and the statistics
/// always agree.
pub fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

/// A story is empty when it has no visible text. Whitespace-only drafts are
/// treated as empty and are never saved or shared.
pub fn is_empty_text(text: &str) -> bool {
    text.trim().is_empty()
}

/// A one-line preview for the My Writing list. Runs of whitespace collapse to a
/// single space and long text is truncated on a character boundary.
pub fn preview(text: &str, max_chars: usize) -> String {
    let mut collapsed = String::new();
    let mut last_was_space = false;
    for ch in text.chars() {
        if ch.is_whitespace() {
            if !collapsed.is_empty() && !last_was_space {
                collapsed.push(' ');
                last_was_space = true;
            }
        } else {
            collapsed.push(ch);
            last_was_space = false;
        }
    }
    let trimmed = collapsed.trim();
    if trimmed.chars().count() <= max_chars {
        return trimmed.to_string();
    }
    let mut result: String = trimmed.chars().take(max_chars.saturating_sub(1)).collect();
    result.push('…');
    result
}

/// Human-readable time spent, e.g. `42 s`, `7 min`, `1 h 12 min`.
pub fn format_duration(seconds: u64) -> String {
    if seconds < 60 {
        format!("{seconds} s")
    } else if seconds < 3_600 {
        format!("{} min", seconds / 60)
    } else {
        format!("{} h {} min", seconds / 3_600, (seconds % 3_600) / 60)
    }
}

/// The title shown in lists and detail views. An empty title falls back to the
/// local date the story was started, as specified for the optional title field.
pub fn display_title(title: &str, created_at: u64, offset_seconds: i32) -> String {
    let title = title.trim();
    if title.is_empty() {
        stats::format_date(created_at, offset_seconds)
    } else {
        title.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_count_ignores_extra_whitespace_and_newlines() {
        assert_eq!(word_count(""), 0);
        assert_eq!(word_count("   \n\t "), 0);
        assert_eq!(word_count("Hallo wereld."), 2);
        assert_eq!(word_count("Een  verhaal\nmet\tregels."), 4);
    }

    #[test]
    fn empty_text_is_whitespace_only() {
        assert!(is_empty_text(""));
        assert!(is_empty_text(" \n\t "));
        assert!(!is_empty_text("Hallo"));
    }

    #[test]
    fn preview_collapses_whitespace_and_truncates_on_a_char_boundary() {
        assert_eq!(
            preview("Een  verhaal\nmet\tregels.", 40),
            "Een verhaal met regels."
        );
        assert_eq!(preview("één twee drie", 8), "één twe…");
        assert_eq!(preview("kort", 40), "kort");
    }

    #[test]
    fn duration_has_clear_units() {
        assert_eq!(format_duration(0), "0 s");
        assert_eq!(format_duration(42), "42 s");
        assert_eq!(format_duration(60), "1 min");
        assert_eq!(format_duration(3_599), "59 min");
        assert_eq!(format_duration(3_600), "1 h 0 min");
        assert_eq!(format_duration(4_320), "1 h 12 min");
    }

    #[test]
    fn empty_title_falls_back_to_the_local_date() {
        assert_eq!(display_title("", 0, 0), "1 Jan 1970");
        assert_eq!(display_title("  ", 0, 0), "1 Jan 1970");
        assert_eq!(display_title("De fiets", 0, 0), "De fiets");
        // 23:30 UTC on 12 May is 00:30 on 13 May at +01:00.
        assert_eq!(display_title("", 1_778_628_600, 3_600), "13 May 2026");
    }
}
