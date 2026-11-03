//! Reader typography and word rectangles.
//!
//! Slint has no flow layout with per-word hit-testing, so this module measures
//! every word with the bundled Crimson Text font (`ab_glyph`) and produces
//! absolute rectangles. The Slint reader draws one `Text` per word at those
//! coordinates and forwards taps back here for hit-testing.
//!
//! The layout is deliberately independent of Slint and JNI so it can be tested
//! on the desktop. Coordinates are logical pixels relative to the reader's
//! content rectangle.

use crate::stories::model::{Story, is_new_lemma, normalize_word};
use crate::stories::tokenize::tokenize;
use ab_glyph::{Font, FontArc, PxScale, ScaleFont};
use std::collections::HashMap;
use std::sync::OnceLock;

/// The bundled reader face. Embedded so Android never needs a system font.
pub const READER_FONT_BYTES: &[u8] = include_bytes!("../../assets/fonts/CrimsonText-Regular.ttf");

/// Parse the bundled face once. `FontArc` is cheap to clone, so every
/// `LayoutMetrics` can share it instead of re-parsing 130 KB of tables.
fn reader_font() -> FontArc {
    static FONT: OnceLock<FontArc> = OnceLock::new();
    FONT.get_or_init(|| {
        FontArc::try_from_slice(READER_FONT_BYTES)
            .expect("the bundled reader font must be a valid TrueType font")
    })
    .clone()
}

/// A word (or punctuation-delimited chunk) positioned on the page.
#[derive(Debug, Clone, PartialEq)]
pub struct WordBox {
    /// Text to draw, including attached punctuation and quotes.
    pub text: String,
    /// First word token inside `text`, used for lookup. Empty for punctuation.
    pub token: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    /// Zero-based sentence index, used by the TTS highlight.
    pub sentence: usize,
    /// 1-based occurrence of this token's surface form in the story. Used to
    /// pick a per-occurrence glossary sense for ambiguous forms.
    pub occurrence: u32,
    /// True when the token's glossary lemma is one of the story's new words.
    pub new_word: bool,
}

/// A rectangle used for a sentence highlight or a new-word underline.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RectBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub sentence: usize,
}

/// One laid-out line of text. The reader draws a single Slint `Text` per line;
/// per-word rectangles are kept separately for hit-testing and underlines.
#[derive(Debug, Clone, PartialEq)]
pub struct LineBox {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Everything the reader needs to draw one page.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PageLayout {
    pub words: Vec<WordBox>,
    pub lines: Vec<LineBox>,
    pub underlines: Vec<RectBox>,
    pub sentence_marks: Vec<RectBox>,
    /// Sentences in reading order, passed to the TTS queue.
    pub sentences: Vec<String>,
    /// Total height used by the text, in logical pixels.
    pub height: f32,
    pub font_size: f32,
    pub line_height: f32,
}

impl PageLayout {
    /// Find the word under a tap, with a small touch slop. The word hit-test
    /// always wins over the page-turn zones.
    pub fn hit_test(&self, x: f32, y: f32) -> Option<usize> {
        self.words.iter().position(|word| {
            x >= word.x - 2.0
                && x <= word.x + word.width + 2.0
                && y >= word.y - 1.0
                && y <= word.y + word.height + 1.0
        })
    }

    pub fn is_empty(&self) -> bool {
        self.words.is_empty()
    }
}

/// Convert a CSS-style em size (what Slint's `font-size` means) into the
/// `PxScale` that `ab_glyph` expects. `ab_glyph` scales ascent + descent to the
/// requested value, so the em size must be divided by that ratio first.
fn em_to_px_scale(font: &FontArc, em_size: f32) -> PxScale {
    let height = font.height_unscaled();
    let units = font.units_per_em().unwrap_or(height);
    if height <= 0.0 || units <= 0.0 {
        return PxScale::from(em_size);
    }
    PxScale::from(em_size * height / units)
}

/// Font metrics for one font size. Creating this is cheap enough to do once
/// per story/page; measuring is pure arithmetic on the font's advances.
pub struct LayoutMetrics {
    font: FontArc,
    /// The scale to hand to `ab_glyph`. This is not the em size: `PxScale`
    /// scales the font's ascent + descent to the requested pixel height, while
    /// Slint's `font-size` is the em size. `px_scale` converts between them so
    /// measured advances match what Slint draws.
    px_scale: PxScale,
    font_size: f32,
    line_height: f32,
    paragraph_gap: f32,
    space_advance: f32,
}

impl LayoutMetrics {
    pub fn new(font_size: f32) -> Self {
        let font = reader_font();
        let font_size = font_size.max(1.0);
        let px_scale = em_to_px_scale(&font, font_size);
        let space_advance = {
            let scaled = font.as_scaled(px_scale);
            scaled.h_advance(font.glyph_id(' '))
        };
        LayoutMetrics {
            font,
            px_scale,
            font_size,
            line_height: (font_size * 1.52).round(),
            paragraph_gap: (font_size * 0.92).round(),
            space_advance,
        }
    }

    pub fn font_size(&self) -> f32 {
        self.font_size
    }

    pub fn line_height(&self) -> f32 {
        self.line_height
    }

    /// Width of `text` at this size, including kerning between characters.
    pub fn measure(&self, text: &str) -> f32 {
        let scaled = self.font.as_scaled(self.px_scale);
        let mut width = 0.0;
        let mut previous = None;
        for ch in text.chars() {
            let id = self.font.glyph_id(ch);
            if let Some(previous) = previous {
                width += scaled.kern(previous, id);
            }
            width += scaled.h_advance(id);
            previous = Some(id);
        }
        width
    }

    /// Lay out one page. `max_width` is the usable text width in logical pixels.
    pub fn layout_page(&self, story: &Story, page: &str, max_width: f32) -> PageLayout {
        let occurrences = occurrence_map(page);
        self.layout_page_at(story, page, max_width, 0, &occurrences)
    }

    /// Lay out one page whose first byte is `page_offset` in the full story.
    /// `occurrences` maps token byte offsets in the full story to 1-based
    /// occurrences of their surface form.
    fn layout_page_at(
        &self,
        story: &Story,
        page: &str,
        max_width: f32,
        page_offset: usize,
        occurrences: &HashMap<usize, u32>,
    ) -> PageLayout {
        let page = page.replace("\r\n", "\n");
        let max_width = max_width.max(40.0);
        let lexicon = story.lexicon();
        let sentences = split_sentences(&page);
        let sentence_ranges: Vec<(usize, usize)> = sentences
            .iter()
            .map(|(start, end, _)| (*start, *end))
            .collect();

        let mut words = Vec::new();
        let mut y = 0.0;

        for (paragraph_index, (paragraph_start, paragraph)) in
            split_paragraphs(&page).into_iter().enumerate()
        {
            if paragraph_index > 0 {
                y += self.paragraph_gap;
            }

            let mut x = 0.0;
            let mut has_line = false;

            for unit in split_units(paragraph, paragraph_start) {
                let width = self.measure(unit.text);
                if has_line && x + self.space_advance + width > max_width {
                    y += self.line_height;
                    x = 0.0;
                    has_line = false;
                }
                if has_line {
                    x += self.space_advance;
                }

                let tokens = tokenize(unit.text);
                let token = tokens.first().map(|token| token.text).unwrap_or("");
                let (new_word, sentence, occurrence) = if let Some(first) = tokens.first() {
                    let absolute = unit.offset + first.start;
                    let occurrence = occurrences
                        .get(&(page_offset + absolute))
                        .copied()
                        .unwrap_or(1);
                    let new_word = lexicon
                        .resolve_at(first.text, occurrence)
                        .is_some_and(|entry| is_new_lemma(story, &entry.lemma));
                    (
                        new_word,
                        sentence_at(absolute, &sentence_ranges),
                        occurrence,
                    )
                } else {
                    (false, sentence_at(unit.offset, &sentence_ranges), 1)
                };

                words.push(WordBox {
                    text: unit.text.to_string(),
                    token: token.to_string(),
                    x,
                    y,
                    width,
                    height: self.line_height,
                    sentence,
                    occurrence,
                    new_word,
                });

                x += width;
                has_line = true;
            }

            if has_line {
                y += self.line_height;
            }
        }

        let mut underlines = Vec::new();
        for word in &words {
            if word.new_word {
                underlines.push(RectBox {
                    x: word.x,
                    // The text is drawn top-aligned inside the line box, so the
                    // underline sits just below the glyphs rather than at the
                    // bottom of the full 1.52em line slot.
                    y: word.y + self.font_size * 1.04,
                    width: word.width,
                    height: 2.0,
                    sentence: word.sentence,
                });
            }
        }

        let mut sentence_marks = Vec::new();
        let mut index = 0;
        while index < words.len() {
            let word = &words[index];

            // Group consecutive words on the same line that belong to the same
            // sentence. This gives one soft background per sentence segment.
            let line_y = word.y;
            let sentence = word.sentence;
            let mut last = index;
            while last + 1 < words.len()
                && (words[last + 1].y - line_y).abs() < 0.1
                && words[last + 1].sentence == sentence
            {
                last += 1;
            }
            let start_x = words[index].x;
            let end_x = words[last].x + words[last].width;
            sentence_marks.push(RectBox {
                x: start_x,
                y: line_y,
                width: end_x - start_x,
                height: self.line_height,
                sentence,
            });
            index = last + 1;
        }

        let lines = build_lines(&words);

        PageLayout {
            words,
            lines,
            underlines,
            sentence_marks,
            sentences: sentences.into_iter().map(|(_, _, text)| text).collect(),
            height: y,
            font_size: self.font_size,
            line_height: self.line_height,
        }
    }
}

/// Group consecutive word boxes that share a baseline into drawable lines.
fn build_lines(words: &[WordBox]) -> Vec<LineBox> {
    let mut lines = Vec::new();
    let mut start = 0;
    while start < words.len() {
        let line_y = words[start].y;
        let mut end = start;
        while end + 1 < words.len() && (words[end + 1].y - line_y).abs() < 0.1 {
            end += 1;
        }

        let mut text = String::new();
        for (index, word) in words[start..=end].iter().enumerate() {
            if index > 0 {
                text.push(' ');
            }
            text.push_str(&word.text);
        }
        let last = &words[end];
        lines.push(LineBox {
            text,
            x: words[start].x,
            y: line_y,
            width: last.x + last.width - words[start].x,
            height: words[start].height,
        });
        start = end + 1;
    }
    lines
}

/// The one reader font size, in logical pixels, for every story and level.
pub const READER_FONT_SIZE: f32 = 20.0;

/// Reference content rectangles used by the validator's length check.
///
/// Large: a 1080x2400 phone at density 420 (about 411x914 dp) after the
/// 26 px side margins, 64 px top margin, 52 px bottom margin, and typical
/// 24 px system insets.
/// Small: a 360x800 dp phone with the same margins and insets.
pub const REFERENCE_LARGE_WIDTH: f32 = 359.0;
pub const REFERENCE_LARGE_HEIGHT: f32 = 750.0;
pub const REFERENCE_SMALL_WIDTH: f32 = 308.0;
pub const REFERENCE_SMALL_HEIGHT: f32 = 636.0;

/// A story must render as at least this many screens on both reference phones.
pub const MIN_SCREENS: usize = 5;

/// One computed page: a byte range in the story text plus its layout.
#[derive(Debug, Clone, PartialEq)]
pub struct PageSpan {
    /// Byte offset where this page starts in the normalized story text.
    pub start: usize,
    /// Byte offset one past the last character on this page.
    pub end: usize,
    pub layout: PageLayout,
}

impl PageSpan {
    pub fn text<'a>(&self, story_text: &'a str) -> &'a str {
        &story_text[self.start.min(story_text.len())..self.end.min(story_text.len())]
    }
}

/// Join a story's authored sections into one text. Kept here so the reader and
/// validator always paginate exactly the same string.
pub fn story_text(story: &Story) -> String {
    story.text()
}

/// Map every token's byte offset in `text` to its 1-based occurrence of that
/// surface form. Used for per-occurrence glossary senses.
pub fn occurrence_map(text: &str) -> HashMap<usize, u32> {
    let mut counts: HashMap<String, u32> = HashMap::new();
    let mut map = HashMap::new();
    for token in tokenize(text) {
        let key = normalize_word(token.text);
        let count = counts.entry(key).or_insert(0);
        *count += 1;
        map.insert(token.start, *count);
    }
    map
}

/// Paginate the whole story into screens that fill `max_height`.
///
/// The last page may be partly filled. Every page starts at a word boundary and
/// the ranges never overlap, so no word is lost or duplicated when the screen
/// size changes and the story is paginated again.
pub fn paginate(
    story: &Story,
    text: &str,
    max_width: f32,
    max_height: f32,
    font_size: f32,
) -> Vec<PageSpan> {
    let metrics = LayoutMetrics::new(font_size);
    let max_width = max_width.max(40.0);
    let max_height = max_height.max(metrics.line_height);
    let occurrences = occurrence_map(text);

    let mut pages = Vec::new();
    let mut page_start: Option<usize> = None;
    // `y` is the bottom of the content laid out so far: the next line starts
    // here. `x` is the width used on the current line.
    let mut y = 0.0;
    let mut x = 0.0;
    let mut has_line = false;

    let finish_page = |pages: &mut Vec<PageSpan>, start: usize, end: usize, text: &str| {
        if end > start {
            let layout =
                metrics.layout_page_at(story, &text[start..end], max_width, start, &occurrences);
            pages.push(PageSpan { start, end, layout });
        }
    };

    let break_page = |pages: &mut Vec<PageSpan>,
                      page_start: &mut Option<usize>,
                      y: &mut f32,
                      x: &mut f32,
                      has_line: &mut bool,
                      end: usize| {
        if let Some(start) = page_start.take() {
            finish_page(pages, start, end, text);
        }
        *y = 0.0;
        *x = 0.0;
        *has_line = false;
    };

    for (paragraph_index, (paragraph_start, paragraph)) in
        split_paragraphs(text).into_iter().enumerate()
    {
        let units = split_units(paragraph, paragraph_start);
        if units.is_empty() {
            continue;
        }

        // A paragraph gap only applies when the page already has text. If the
        // gap does not fit, start the paragraph on the next page instead.
        if page_start.is_some() && paragraph_index > 0 {
            if y + metrics.paragraph_gap + metrics.line_height > max_height {
                break_page(
                    &mut pages,
                    &mut page_start,
                    &mut y,
                    &mut x,
                    &mut has_line,
                    units[0].offset,
                );
            } else {
                y += metrics.paragraph_gap;
            }
        }

        for unit in units {
            let width = metrics.measure(unit.text);

            if has_line && x + metrics.space_advance + width > max_width {
                // The current line is full. Start a new line, or a new page if
                // that line would not fit.
                if page_start.is_some() && y + metrics.line_height * 2.0 > max_height {
                    break_page(
                        &mut pages,
                        &mut page_start,
                        &mut y,
                        &mut x,
                        &mut has_line,
                        unit.offset,
                    );
                } else {
                    y += metrics.line_height;
                    x = 0.0;
                    has_line = false;
                }
            }

            if !has_line && page_start.is_some() && y + metrics.line_height > max_height {
                break_page(
                    &mut pages,
                    &mut page_start,
                    &mut y,
                    &mut x,
                    &mut has_line,
                    unit.offset,
                );
            }

            page_start.get_or_insert(unit.offset);
            if has_line {
                x += metrics.space_advance;
            }
            x += width;
            has_line = true;
        }

        if has_line {
            y += metrics.line_height;
            x = 0.0;
            has_line = false;
        }
    }

    if let Some(start) = page_start {
        finish_page(&mut pages, start, text.len(), text);
    }

    if pages.is_empty() {
        pages.push(PageSpan {
            start: 0,
            end: text.len(),
            layout: metrics.layout_page(story, text, max_width),
        });
    }
    pages
}

/// Paginate at the app's fixed font size.
pub fn paginate_story(story: &Story, max_width: f32, max_height: f32) -> Vec<PageSpan> {
    let text = story_text(story);
    paginate(story, &text, max_width, max_height, READER_FONT_SIZE)
}

/// Page counts on the two reference phones. Used by the validator and by the
/// Level/Home screens when they show a computed page number.
pub fn reference_page_counts(story: &Story) -> (usize, usize) {
    (
        paginate_story(story, REFERENCE_LARGE_WIDTH, REFERENCE_LARGE_HEIGHT).len(),
        paginate_story(story, REFERENCE_SMALL_WIDTH, REFERENCE_SMALL_HEIGHT).len(),
    )
}

/// The page containing `offset`, clamped to the available pages.
pub fn page_for_offset(pages: &[PageSpan], offset: usize) -> usize {
    if pages.is_empty() {
        return 0;
    }
    let mut index = 0;
    for (candidate, page) in pages.iter().enumerate() {
        if page.start <= offset {
            index = candidate;
        } else {
            break;
        }
    }
    index
}

/// Page number and page count on the large reference phone. Home and Level use
/// this so their labels stay stable when the reader is not open.
pub fn reference_position(story: &Story, offset: usize) -> (usize, usize) {
    let pages = paginate_story(story, REFERENCE_LARGE_WIDTH, REFERENCE_LARGE_HEIGHT);
    (page_for_offset(&pages, offset), pages.len())
}

struct Unit<'a> {
    text: &'a str,
    offset: usize,
}

/// Split a paragraph into whitespace-delimited chunks. Keeping a whole chunk
/// (e.g. `"zwart-wit,"` or `"“Hallo!”"`) together preserves punctuation and
/// hyphenation, which is exactly what a reader expects to see and tap.
fn split_units(paragraph: &str, paragraph_start: usize) -> Vec<Unit<'_>> {
    let mut units = Vec::new();
    let mut start = None;
    for (index, ch) in paragraph.char_indices() {
        if ch.is_whitespace() {
            if let Some(start) = start.take() {
                units.push(Unit {
                    text: &paragraph[start..index],
                    offset: paragraph_start + start,
                });
            }
        } else if start.is_none() {
            start = Some(index);
        }
    }
    if let Some(start) = start {
        units.push(Unit {
            text: &paragraph[start..],
            offset: paragraph_start + start,
        });
    }
    units
}

/// Split a page into paragraphs at blank lines. Returns each paragraph with its
/// byte offset in the normalised page text.
fn split_paragraphs(text: &str) -> Vec<(usize, &str)> {
    let bytes = text.as_bytes();
    let mut paragraphs = Vec::new();
    let mut start = 0;
    let mut index = 0;

    while index + 1 < bytes.len() {
        if bytes[index] == b'\n' && bytes[index + 1] == b'\n' {
            let paragraph = &text[start..index];
            if !paragraph.trim().is_empty() {
                paragraphs.push((start, paragraph));
            }
            index += 2;
            while index < bytes.len() && matches!(bytes[index], b'\n' | b'\r') {
                index += 1;
            }
            start = index;
        } else {
            index += 1;
        }
    }

    let paragraph = &text[start..];
    if !paragraph.trim().is_empty() {
        paragraphs.push((start, paragraph));
    }
    paragraphs
}

/// Split text into sentences for the TTS queue. Closing quotes stay with the
/// sentence they close.
pub fn split_sentences(text: &str) -> Vec<(usize, usize, String)> {
    let chars: Vec<(usize, char)> = text.char_indices().collect();
    let mut sentences = Vec::new();
    let mut start = 0;
    let mut index = 0;

    while index < chars.len() {
        let (_, ch) = chars[index];
        if matches!(ch, '.' | '!' | '?' | '…') {
            let mut end_index = index + 1;
            while end_index < chars.len() {
                let next = chars[end_index].1;
                if matches!(
                    next,
                    '.' | '!' | '?' | '…' | '”' | '’' | '"' | '\'' | ')' | ']' | '»'
                ) {
                    end_index += 1;
                } else {
                    break;
                }
            }
            let next_is_break = end_index >= chars.len() || chars[end_index].1.is_whitespace();
            if next_is_break {
                let end = if end_index < chars.len() {
                    chars[end_index].0
                } else {
                    text.len()
                };
                let sentence = text[start..end].trim();
                if !sentence.is_empty() {
                    sentences.push((start, end, sentence.to_string()));
                }
                start = end;
                index = end_index;
                continue;
            }
        }
        index += 1;
    }

    let tail = text[start..].trim();
    if !tail.is_empty() {
        sentences.push((start, text.len(), tail.to_string()));
    }
    sentences
}

fn sentence_at(offset: usize, ranges: &[(usize, usize)]) -> usize {
    let mut result = 0;
    for (index, (start, _)) in ranges.iter().enumerate() {
        if offset >= *start {
            result = index;
        } else {
            break;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stories::index::StoryIndex;

    fn stories() -> Vec<Story> {
        StoryIndex::load_embedded()
            .expect("embedded stories")
            .stories()
            .iter()
            .map(|loaded| loaded.story.clone())
            .collect()
    }

    #[test]
    fn every_story_renders_at_least_five_screens() {
        let mut failures = Vec::new();
        for story in stories() {
            let (large, small) = reference_page_counts(&story);
            if large < MIN_SCREENS || small < MIN_SCREENS {
                failures.push(format!(
                    "{}: {large} large / {small} small ({} words)",
                    story.id,
                    story.token_count()
                ));
            }
        }
        assert!(
            failures.is_empty(),
            "stories below {MIN_SCREENS} screens:\n{}",
            failures.join("\n")
        );
    }

    #[test]
    fn every_computed_page_fits_both_reference_screens() {
        for story in stories() {
            for (width, height) in [
                (REFERENCE_LARGE_WIDTH, REFERENCE_LARGE_HEIGHT),
                (REFERENCE_SMALL_WIDTH, REFERENCE_SMALL_HEIGHT),
            ] {
                for (index, page) in paginate_story(&story, width, height).iter().enumerate() {
                    assert!(
                        page.layout.height <= height + 0.01,
                        "{} page {} is {}px (max {height})",
                        story.id,
                        index + 1,
                        page.layout.height
                    );
                    assert!(!page.layout.words.is_empty(), "{} page {index}", story.id);
                }
            }
        }
    }

    #[test]
    fn every_story_uses_the_same_fixed_font_size() {
        for story in stories() {
            let pages = paginate_story(&story, REFERENCE_LARGE_WIDTH, REFERENCE_LARGE_HEIGHT);
            assert!(!pages.is_empty());
            for page in pages {
                assert_eq!(page.layout.font_size, READER_FONT_SIZE);
            }
        }
    }

    #[test]
    fn page_ranges_cover_the_text_in_order() {
        for story in stories() {
            let text = story_text(&story);
            let pages = paginate_story(&story, REFERENCE_LARGE_WIDTH, REFERENCE_LARGE_HEIGHT);
            assert_eq!(pages[0].start, 0, "{} does not start at offset 0", story.id);
            assert_eq!(
                pages.last().unwrap().end,
                text.len(),
                "{} does not cover the whole text",
                story.id
            );
            for pair in pages.windows(2) {
                assert!(
                    pair[0].end <= pair[1].start,
                    "{} pages overlap at {}..{}",
                    story.id,
                    pair[0].start,
                    pair[1].start
                );
            }
        }
    }

    #[test]
    fn page_for_offset_finds_each_computed_page() {
        let story = &stories()[0];
        let pages = paginate_story(story, REFERENCE_LARGE_WIDTH, REFERENCE_LARGE_HEIGHT);
        for (index, page) in pages.iter().enumerate() {
            assert_eq!(page_for_offset(&pages, page.start), index);
            assert_eq!(page_for_offset(&pages, page.start + 1), index);
        }
    }

    #[test]
    fn smaller_screens_produce_at_least_as_many_pages() {
        for story in stories() {
            let large = paginate_story(&story, REFERENCE_LARGE_WIDTH, REFERENCE_LARGE_HEIGHT).len();
            let small = paginate_story(&story, REFERENCE_SMALL_WIDTH, REFERENCE_SMALL_HEIGHT).len();
            assert!(
                small >= large,
                "{} has {small} pages on the small phone but {large} on the large one",
                story.id
            );
        }
    }

    #[test]
    fn words_stay_inside_the_text_column() {
        let story = &stories()[0];
        for page in paginate_story(story, REFERENCE_LARGE_WIDTH, REFERENCE_LARGE_HEIGHT) {
            for word in &page.layout.words {
                assert!(
                    word.x + word.width <= REFERENCE_LARGE_WIDTH + 1.0,
                    "`{}` overflows at x={} w={}",
                    word.text,
                    word.x,
                    word.width
                );
                assert!(word.x >= 0.0);
            }
        }
    }

    #[test]
    fn hit_test_finds_the_word_under_the_centre() {
        let story = &stories()[0];
        let page = &paginate_story(story, REFERENCE_LARGE_WIDTH, REFERENCE_LARGE_HEIGHT)[0];
        let word = &page.layout.words[3];
        let hit = page
            .layout
            .hit_test(word.x + word.width / 2.0, word.y + word.height / 2.0)
            .expect("word hit");
        assert_eq!(page.layout.words[hit].text, word.text);
    }

    #[test]
    fn new_words_are_underlined_but_starter_words_are_not() {
        let story = &stories()[0];
        let starter = crate::stories::starter::StarterWords::embedded();
        let lexicon = story.lexicon();
        for page in paginate_story(story, REFERENCE_LARGE_WIDTH, REFERENCE_LARGE_HEIGHT) {
            let new_words = page
                .layout
                .words
                .iter()
                .filter(|word| word.new_word)
                .count();
            assert_eq!(page.layout.underlines.len(), new_words);
            for word in &page.layout.words {
                if word.new_word {
                    let entry = lexicon.resolve(&word.token).expect("new word is glossed");
                    assert!(
                        !starter.is_lemma(&entry.lemma),
                        "starter `{}` was underlined as new",
                        entry.lemma
                    );
                }
            }
            for underline in &page.layout.underlines {
                assert!(underline.width > 0.0);
            }
        }
    }

    #[test]
    fn measured_advances_use_the_em_size() {
        use ab_glyph::{Font, ScaleFont};
        // Slint's `font-size` is the em size. The bundled face has a taller
        // ascent+descent than one em, so a naive `PxScale::from(font_size)`
        // would measure every word about 30% too narrow.
        let metrics = LayoutMetrics::new(28.0);
        let scaled = metrics.font.as_scaled(metrics.px_scale);
        let em = metrics.font.units_per_em().unwrap_or(1024.0);
        let expected = 318.0 / em * 28.0;
        let actual = scaled.h_advance(metrics.font.glyph_id('I'));
        assert!(
            (actual - expected).abs() < 0.01,
            "I advance {actual} should be {expected}"
        );
    }

    #[test]
    fn sentences_cover_the_page_and_keep_closing_quotes() {
        let text = "Hallo. “Waar is de fiets?” Ik weet het niet!";
        let sentences = split_sentences(text);
        assert_eq!(sentences.len(), 3);
        assert_eq!(sentences[1].2, "“Waar is de fiets?”");
        assert!(sentences[0].2.starts_with("Hallo."));
    }

    #[test]
    fn paragraph_breaks_add_vertical_space() {
        let story = &stories()[0];
        let metrics = LayoutMetrics::new(18.0);
        let one = metrics.layout_page(story, "Een twee drie.", 340.0);
        let two = metrics.layout_page(story, "Een twee drie.\n\nEen twee drie.", 340.0);
        assert!(two.height > one.height * 1.5);
    }

    #[test]
    fn lines_group_words_by_baseline_and_join_with_spaces() {
        let story = &stories()[0];
        let pages = paginate_story(story, REFERENCE_LARGE_WIDTH, REFERENCE_LARGE_HEIGHT);
        let layout = &pages[0].layout;
        assert!(!layout.lines.is_empty());
        for line in &layout.lines {
            assert!(!line.text.is_empty());
            assert!(
                line.width <= REFERENCE_LARGE_WIDTH + 1.0,
                "line overflows: {}",
                line.text
            );
            assert_eq!(line.height, layout.line_height);
        }

        let words: Vec<&str> = layout
            .lines
            .iter()
            .flat_map(|line| line.text.split(' '))
            .collect();
        let boxes: Vec<&str> = layout.words.iter().map(|word| word.text.as_str()).collect();
        assert_eq!(words, boxes);
    }

    #[test]
    fn sentence_indices_are_monotonic() {
        let story = &stories()[0];
        for page in paginate_story(story, REFERENCE_LARGE_WIDTH, REFERENCE_LARGE_HEIGHT) {
            let layout = &page.layout;
            let mut previous = 0;
            for word in &layout.words {
                assert!(word.sentence >= previous, "sentence went backwards");
                previous = word.sentence;
            }
            assert!(previous < layout.sentences.len());
        }
    }
}
