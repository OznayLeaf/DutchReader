//! Dutch-aware word tokenizer.
//!
//! The validator, the reader layout, and the personal dictionary all need to
//! agree on what a "word" is. This module is the single source of truth.
//!
//! Rules:
//! - Letters and digits form words.
//! - A straight or curly apostrophe stays inside a word when followed by a
//!   letter (`zo'n`, `'s ochtends`), but a closing quote (`'hond'`) is not
//!   included.
//! - Hyphens separate words, so `zwart-wit` yields `zwart` and `wit`.
//! - Byte offsets are returned so the reader can draw word rectangles later.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<'a> {
    pub text: &'a str,
    /// Byte offset of the first character.
    pub start: usize,
    /// Byte offset one past the last character.
    pub end: usize,
}

impl<'a> Token<'a> {
    pub fn new(text: &'a str, start: usize, end: usize) -> Self {
        Token { text, start, end }
    }
}

/// Split `text` into word tokens, preserving byte offsets.
pub fn tokenize(text: &str) -> Vec<Token<'_>> {
    let mut tokens = Vec::new();
    let mut word_start: Option<usize> = None;

    let mut chars = text.char_indices().peekable();
    while let Some((idx, ch)) = chars.next() {
        let is_word_char = ch.is_alphanumeric() || is_combining_mark(ch);
        let is_apostrophe = ch == '\'' || ch == '\u{2019}';

        if is_word_char {
            word_start.get_or_insert(idx);
            continue;
        }

        if is_apostrophe {
            // Keep the apostrophe only if it is followed by a letter/digit.
            let next = chars.peek().map(|(_, next)| *next);
            let next_is_word = next
                .map(|next| next.is_alphanumeric() || is_combining_mark(next))
                .unwrap_or(false);
            if next_is_word {
                if word_start.is_some() {
                    // Inside a word: `zo'n`, `'s ochtends`.
                    continue;
                }
                // A leading apostrophe is only a Dutch elision (`'s`, `'t`,
                // `'n`, `'k`, `'m`, `'d`, `'r`, `'ie`), never an opening quote.
                let is_elision = next
                    .map(|next| {
                        matches!(
                            next.to_ascii_lowercase(),
                            's' | 't' | 'n' | 'k' | 'm' | 'd' | 'r' | 'i'
                        )
                    })
                    .unwrap_or(false);
                if is_elision {
                    word_start = Some(idx);
                    continue;
                }
            }
        }

        if let Some(start) = word_start.take() {
            tokens.push(Token::new(&text[start..idx], start, idx));
        }
    }

    if let Some(start) = word_start {
        tokens.push(Token::new(&text[start..], start, text.len()));
    }

    tokens
}

fn is_combining_mark(ch: char) -> bool {
    matches!(ch, '\u{0300}'..='\u{036f}' | '\u{1ab0}'..='\u{1aff}' | '\u{20d0}'..='\u{20ff}')
}

#[cfg(test)]
mod tests {
    use super::*;

    fn words(text: &str) -> Vec<&str> {
        tokenize(text).into_iter().map(|t| t.text).collect()
    }

    #[test]
    fn splits_on_punctuation() {
        assert_eq!(
            words("De fiets is rood."),
            vec!["De", "fiets", "is", "rood"]
        );
        assert_eq!(words("Hallo, wereld!"), vec!["Hallo", "wereld"]);
    }

    #[test]
    fn keeps_apostrophes_inside_words() {
        assert_eq!(words("Zo'n fiets."), vec!["Zo'n", "fiets"]);
        assert_eq!(words("'s Ochtends"), vec!["'s", "Ochtends"]);
        assert_eq!(words("zo\u{2019}n"), vec!["zo\u{2019}n"]);
    }

    #[test]
    fn does_not_treat_quotes_as_word_parts() {
        assert_eq!(words("'hond'"), vec!["hond"]);
        assert_eq!(words("\u{2018}hond\u{2019}"), vec!["hond"]);
    }

    #[test]
    fn hyphens_separate_words() {
        assert_eq!(words("zwart-wit"), vec!["zwart", "wit"]);
    }

    #[test]
    fn numbers_are_words() {
        assert_eq!(words("Ik heb 3 appels."), vec!["Ik", "heb", "3", "appels"]);
    }

    #[test]
    fn offsets_point_at_the_original_text() {
        let text = "De fiets is rood.";
        let tokens = tokenize(text);
        assert_eq!(&text[tokens[1].start..tokens[1].end], "fiets");
        assert_eq!(tokens[1].start, 3);
        assert_eq!(tokens[1].end, 8);
    }

    #[test]
    fn empty_and_punctuation_only() {
        assert!(tokenize("").is_empty());
        assert!(tokenize("... !!! ---").is_empty());
    }
}
