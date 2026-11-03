//! The exact text sent to the Android share sheet.
//!
//! The title is deliberately never included: the learner sends only the story
//! plus the fixed question, so the receiving app sees a clean request.

/// The question appended after the quoted story.
pub const QUESTION: &str =
    "Are there any grammar or vocabulary mistakes in this story, other than punctuation?";

/// Wrap the story in double quotation marks, add a blank line, then the fixed
/// question. Paragraphs and line breaks inside `story` are preserved exactly.
pub fn share_text(story: &str) -> String {
    format!("\"{story}\"\n\n{QUESTION}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_text_for_a_single_line_story() {
        assert_eq!(
            share_text("De fiets is rood."),
            "\"De fiets is rood.\"\n\nAre there any grammar or vocabulary mistakes in this story, other than punctuation?"
        );
    }

    #[test]
    fn paragraphs_and_line_breaks_are_preserved_inside_the_quotes() {
        let story = "De fiets is rood.\n\nIk ga naar de winkel.\nDe winkel is dicht.";
        assert_eq!(
            share_text(story),
            "\"De fiets is rood.\n\nIk ga naar de winkel.\nDe winkel is dicht.\"\n\nAre there any grammar or vocabulary mistakes in this story, other than punctuation?"
        );
    }

    #[test]
    fn the_title_is_not_part_of_the_builder_at_all() {
        // The builder has no title parameter. This test documents the contract:
        // passing only the story can never add a title.
        let text = share_text("Hallo.");
        assert!(!text.contains("Mijn verhaal"));
        assert!(text.starts_with("\"Hallo.\""));
    }

    #[test]
    fn question_wording_is_exact() {
        assert_eq!(
            QUESTION,
            "Are there any grammar or vocabulary mistakes in this story, other than punctuation?"
        );
        assert_eq!(share_text("x").split("\n\n").nth(1), Some(QUESTION));
    }
}
