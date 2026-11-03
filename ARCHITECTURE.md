# DutchReader: Architecture and Build Plan

Project folder: `Documents/Code/DutchReader`
Stack: Rust + Slint, Android only (optional desktop preview for development)

---

## 0. How to use this document

This document is written for you, the AI coding agent building DutchReader. It has two parts:

1. **Sections 1 and 2** explain the whole app: what it is, how it behaves, and how it is structured. Read these completely before writing any code. They are the source of truth for every decision.
2. **Sections 3 and 4** split the work into **four build phases plus a fifth verification phase**. Each phase is self-contained, has a clear goal, a task list, deliverables, and exit criteria. A phase is only finished when all its exit criteria pass.

Rules for working through the phases:
- Work on the phase you have been assigned, and only that phase. Do not start the next one.
- Phases are done in order. Each phase builds on the previous one and must leave the project in a **compiling, running state**.
- At the end of each phase, report three things: what you finished, what you could not do as specified, and what you did instead.

---

## 1. Full app explanation

### 1.1 What the app is

**DutchReader** is an Android-only app that teaches Dutch to a near-complete beginner (slightly below A1) through two activities:

- **Reading** graded stories from A1 to B2, with tap-to-translate words and optional read-aloud.
- **Writing** the learner's own short Dutch stories in a frictionless editor, then sharing them through the Android share sheet to ChatGPT (or any app) to get mistakes checked.

All user data is stored **locally on the device**. There is no cloud, no account, and no sync. The only network use is the optional "Online dictionary" mode.

### 1.2 Non-negotiable principles

- **Offline first.** The app is fully usable with no internet. Online lookup is an optional extra with an automatic offline fallback.
- **Stories are data, not code.** Adding a story means dropping one JSON file into `assets/stories/<level>/` and rebuilding. No Rust or Slint changes.
- **Never lose user data.** Every file write is atomic (temp file, flush, rename). Drafts auto-save continuously.
- **Low friction writing.** One tap opens the editor. One tap shares. No timer, no countdown, no required fields.
- **Reading is for fun.** Stories must be entertaining as well as level-appropriate.

### 1.3 Content model (long-term plan)

| Level | Stories | New words per story | Cumulative new words |
|-------|---------|---------------------|----------------------|
| A1 | 25 | 20 | 500 |
| A2 | 25 | 20 | 1,000 |
| B1 | 25 | 20 | 1,500 |
| B2 | 25 | 20 | 2,000 |

- Stories get gradually harder within a level (A1-01 easiest, A1-25 hardest A1). The jump from the last story of one level to the first of the next must also be gentle.
- Length is measured in **total words**, not authored pages. Targets: A1 600-750, A2 650-850, B1 850-1,100, B2 1,050-1,300. Pages are computed from the text at one fixed reader font size.
- Every story must render as **at least 5 screens** on both reference phones (1080x2400 and 360dp wide). The validator checks this with the real layout engine.
- Each story introduces **exactly 20 new lemmas** (dictionary forms) that appeared in no earlier story. The fixed starter words are known from the start and never count as new.
- Each story has a mini-arc: hook in the first ~20% of the text, a small problem or surprise, and a satisfying or funny payoff in the last ~20%.
- **This first pass ships exactly 4 stories**: A1-01, A2-01, B1-01, B2-01.

### 1.4 Screens and navigation

| # | Screen | Purpose |
|---|--------|---------|
| 1 | **Home** | Level cards (A1-B2) with progress (e.g. 1/25 completed). Prominent **Write** button. Entry points to Personal Dictionary, My Writing, Settings. |
| 2 | **Level** | Ordered list of stories: title, blurb, page count computed for the current screen, progress, completed state. |
| 3 | **Reader** | Story text, one page at a time. |
| 4 | **Personal Dictionary** | All learned words with meanings. Searchable. Sort alphabetical or newest first. |
| 5 | **Writing Editor** | Distraction-free editor. |
| 6 | **My Writing** | History of written stories plus monthly statistics and streak. |
| 7 | **Settings** | Theme (light / dark / follow system), TTS speed, word meanings source (offline glossary or online dictionary). |

Navigation is a simple stack: Home -> Level -> Reader, and Home -> Editor / Dictionary / My Writing / Settings. Screens change instantly; there are no screen-transition animations.

### 1.5 Reader behavior

- **Tap zones**: left side = previous page, right side = next page, middle = toggle overlay (TTS button, back, page indicator).
- **Word taps**: a tap on a word opens a popup or bottom sheet with its meaning. A tap on empty space is handled as a page-turn or overlay tap. Word hit-testing always wins over the zones.
- **Page turn**: the next or previous page appears immediately, with no flip, slide, fade, or other animation. The page counter and text change in the same frame.
- **TTS**: a top-right button toggles a Dutch (nl-NL) narrator using Android's native `TextToSpeech` through JNI. It highlights the current sentence or word if feasible, and continues across page turns. The default speed is `0.88` (12% slower than the previous `1.0` default); the Settings slider still ranges from `0.5` to `2.0` and a saved speed always wins.
- **Typography**: bundled legible serif font at one fixed size (20 px) for every story, generous line height and paragraph spacing, comfortable margins, max line length. Good contrast in light and dark themes.
- **Pagination**: story sections are joined and paginated to fill the available content height. Only the last page may be partly filled; pages are recomputed when the screen size changes.
- **New-word marking**: the 20 new words of the story are visually marked (subtle underline or tint). Starter words are never marked.
- **Resume**: progress is stored as a text offset, so reopening a story goes to the page containing that offset even after repagination.

### 1.6 Personal dictionary rules

- A page counts as **read** only if it stays open for **at least 30 seconds**.
- The per-page timer **resets** when the page changes, the reader is left, or the app goes to the background.
- When a page reaches 30 seconds, all glossary words on that page are added to the dictionary.
- **No duplicates.** Each entry stores word, meaning, and the story and level where it was first learned.
- Words always come from the **bundled glossary**, regardless of lookup mode, so filler words from online lookups never pollute the dictionary.
- Function words are filtered out, with one exception: the fixed **starter words** are saved so a beginner can review them, attributed to the first story in reading order that uses them.

### 1.7 Word meaning lookup

- **Offline glossary (default)**: uses only the glossary bundled in the story file. A tap looks up the **surface form as written** at its occurrence in the story, not the lemma. If the form is missing, the popup says so instead of silently falling back to the lemma's translation.
- **Online dictionary (optional)**: Dutch -> English through a free, keyless API (MyMemory), behind a `WordLookup` trait so the provider can be swapped without touching the UI. The request is for the **tapped word**, not its lemma, and the cache is keyed by the tapped word.
  - Loading state in the popup, with a short timeout.
  - Every result is cached locally; the same word is never requested twice and works offline afterwards.
  - On failure (no internet, timeout, rate limit), automatically fall back to the glossary and show a small note. A tap never ends in an empty popup.
  - Online results are labeled "machine translated".
- **Popup contents**: the tapped word exactly as written (for example `ben`), the form-specific meaning for that occurrence (for example `am`), and a secondary line `form of: <lemma>, <lemma meaning>` (for example `form of: zijn, to be`). The article is shown only when the tapped form is the lemma itself. A phrase part shows the phrase's meaning.
- **Ambiguous forms**: when one surface form genuinely has different senses in different sentences (`zijn`, `haar`, `dat`, `als`, `te`, `uit`, `die`, `ze`, and so on), the glossary may give an entry an optional 1-based `occurrence`. The entry without `occurrence` is the default for that form.
- The chosen mode is stored in settings.

### 1.8 Writing feature

**Editor**
- Opened by the Home "Write" button. Large, readable text area using the reader's typography, light and dark themes.
- Works with the Android keyboard: text area scrolls, cursor stays visible above the keyboard, toolbar stays reachable.
- Live word count. Time spent is tracked **silently** (only while the editor is open and the app is in the foreground). **No timer UI, no limits.**
- Optional title (default: the date).
- Autocorrect, autocomplete, and prediction disabled where possible, so mistakes stay visible.
- Draft auto-saves every few seconds and when the app goes to the background. If an unfinished draft exists when the editor opens, offer **Continue** or **Start new**.

**Share**
- A Share button opens the native share sheet (`ACTION_SEND`, `text/plain`, wrapped in `Intent.createChooser`) via JNI.
- The shared text is built automatically in exactly this form (title is never included):

```
"<full story text, paragraphs and line breaks intact>"

Are there any grammar or vocabulary mistakes in this story, other than punctuation?
```

- The story is wrapped in double quotation marks on its own, then a blank line, then the exact question above.
- Sharing marks the story as finished and saves it. The Share button is disabled (with a hint) when the text is empty. Empty drafts are never saved.

**Saving and statistics**
- Each finished story stores: text, optional title, date and time, word count, time spent.
- **My Writing** lists stories newest first (title or date, preview, word count, time spent). Opening one shows the full text with: re-share (same format), copy, edit, delete (with confirmation).
- Statistics: total stories written, month-by-month overview (stories and total words per month, as a simple bar chart or list), and the current streak (days or weeks in which I wrote).
- Writing data lives in its **own files**, separate from reader data, and is never sent anywhere except through Share.

### 1.9 Local persistence

Saved locally: last opened story and text offset, per-story progress and completion, settings, cached online lookups, personal dictionary, writing draft, all saved writings with statistics. Format: JSON files in app-private storage with atomic writes.

### 1.10 UI quality (no animations)

Modern, clean, rounded cards, cohesive palette, polished dark mode. **The app has no animations**: screen changes, page turns, the overlay, the popup / bottom sheet, the progress bar, toasts, and editor entry / exit all appear and disappear instantly. Pressed controls may change colour instantly, but nothing moves, fades, slides, flips, or scales. The removal is code-level: the animation blocks and their timing state are deleted, not set to zero duration, so no `flip_busy`-style flag can swallow a tap or leave a stuck state. Respect system insets and the on-screen keyboard. Portrait first.

### 1.11 Technical constraints

- Rust and Slint only. Slint Android backend (`backend-android-activity-06` at time of writing; confirm the current feature name in the Slint docs). Build with `cargo-apk` or `xbuild`, whichever is current and reliable.
- INTERNET permission only. Lightweight HTTP client (`ureq` with rustls, to avoid OpenSSL on Android). All requests run off the UI thread.
- JNI (`jni` + `ndk-context`) for TTS and the share sheet.
- A README with build and run instructions.

---

## 2. System architecture

### 2.1 Module map

```
DutchReader/
  Cargo.toml
  build.rs                    # scans assets/stories, generates the embedded story index
  README.md
  ARCHITECTURE.md
  STORY_GUIDE.md
  assets/
    fonts/                    # bundled serif + UI font (OFL licensed)
    starter_words.json        # fixed beginner words, merged into every glossary
    wordlist.json             # cumulative lemma -> story id or "starter"
    assumed_known.json        # debt list while fewer than 100 stories exist
    stories/
      a1/01-<slug>.json
      a2/01-<slug>.json
      b1/01-<slug>.json
      b2/01-<slug>.json
  ui/                         # Slint files
    app.slint
    theme.slint               # light/dark tokens
    components/               # card, button, bottom-sheet, overlay, progress-bar
    screens/                  # home, level, reader, dictionary, editor, my-writing, settings
  src/
    main.rs / lib.rs          # android_main + desktop preview entry
    app.rs                    # wiring: callbacks <-> modules, navigation state
    stories/                  # model.rs, loader.rs, index.rs, validate.rs, wordlist.rs, starter.rs
    storage/                  # atomic.rs (safe write), settings.rs, progress.rs
    dictionary/               # personal dictionary + 30-second rule
    lookup/                   # WordLookup trait, offline.rs, online.rs (MyMemory), cache.rs
    reader/                   # layout.rs (word rects + pagination), page_timer.rs, tts_controller.rs
    writing/                  # draft.rs, store.rs, stats.rs, share_text.rs
    platform/                 # tts.rs, share.rs, lifecycle.rs, keyboard.rs (JNI bridges)
  validator/                  # validate_stories CLI + fixture tests (workspace package)
  tests/                      # unit + fixture tests (good and deliberately broken stories)
```

Rule: **business logic lives in plain Rust modules with no Slint or JNI dependency** so it can be unit-tested on desktop. `platform/` is the only place that touches JNI. `app.rs` is the only place that touches both Slint and the logic modules.

### 2.2 Data flow

```
build.rs -> embedded story index -> stories::loader -> in-memory Story models
Story models -> reader::pagination/layout -> word rectangles -> Slint reader screen
Tap word -> lookup (offline | online + cache + fallback) -> popup
Page open -> page_timer (30s) -> dictionary::add_page_words (page slice + starter origins) -> storage
Editor text -> writing::draft (autosave) -> Share -> writing::share_text -> platform::share (JNI)
                                         -> writing::store (finished story + stats)
```

### 2.3 Story file contract

Path: `assets/stories/<level>/<nn>-<slug>.json` (level folder lowercase: `a1`, `a2`, `b1`, `b2`).

```json
{
  "id": "a1-01",
  "level": "A1",
  "order": 1,
  "title": "De rode bal",
  "blurb_nl": "Een korte zin over het verhaal.",
  "blurb_en": "A short line about the story.",
  "sections": ["Short paragraph...", "Another short paragraph...", "..."],
  "new_words": ["hond", "bal", "...exactly 20 lemmas..."],
  "glossary": [
    { "form": "hond",   "lemma": "hond", "pos": "noun", "article": "de", "meaning": "dog", "new": true },
    { "form": "loopt",  "lemma": "lopen", "pos": "verb", "meaning": "walks", "new": false },
    { "form": "zijn",   "lemma": "zijn", "pos": "determiner", "meaning": "his", "new": false },
    { "form": "zijn",   "lemma": "zijn", "pos": "verb", "meaning": "are", "occurrence": 4, "new": false },
    { "form": "zet",    "parts": ["zet", "op"], "lemma": "opzetten", "pos": "verb", "meaning": "puts on", "new": false }
  ]
}
```

Field rules:
- `id` = `<level lowercase>-<order zero-padded>`; must match folder and order.
- `sections` holds short paragraphs. The app joins them with blank lines and computes pages; there is no authored page count. The legacy key `pages` is accepted as an alias.
- `new_words` holds exactly 20 lemmas; each must have at least one glossary entry with `"new": true`.
- Starter words must never appear in `new_words` and do not need glossary entries; the loader merges them into every story.
- `article` is required for nouns (`de` / `het`).
- `parts` (optional) marks separable or multi-word forms; tapping any part shows the entry.
- `occurrence` (optional, 1-based) picks a sense for one occurrence of the surface form. A form may have at most one default entry (no `occurrence`) plus entries with unique occurrence numbers. Use it only when one meaning per form genuinely cannot work; otherwise one entry per form is still the rule.
- A non-lemma `form` must have its own form-specific `meaning`, not the lemma's meaning. The validator rejects a non-lemma entry whose meaning equals its lemma entry's meaning (`GLOSSARY_LEMMA_MEANING`).
- Every referenced lemma must have a lemma entry (or be a starter word) so the popup's `form of:` line always has a meaning.
- Proper names are glossed with `"pos": "name"` and are excluded from new-word counting.
- Paragraph breaks inside a section use `\n\n`.

Cumulative list: `assets/wordlist.json` maps each lemma to the story id that introduced it, or to `"starter"`, plus the global story order. The validator verifies it and can regenerate it with `--write-wordlist`.

**Auto-discovery**: Android assets cannot be listed through `std::fs`, so `build.rs` scans `assets/stories/*/*.json` and generates an index using `include_str!`. New stories appear automatically after a rebuild, with no code changes. Optionally also load extra story files from app-private storage at runtime.

### 2.4 Local storage files (app-private directory)

| File | Contents |
|------|----------|
| `settings.json` | theme, TTS speed, meanings source |
| `progress.json` | last story, per-story text offset and completion |
| `dictionary.json` | word, meaning, first story and level, date added |
| `lookup_cache.json` | cached online translations |
| `writing/draft.json` | current draft (title, text, accumulated seconds) |
| `writing/stories.json` | all finished writings with stats |

All writes: serialize -> write `*.tmp` -> flush and fsync -> rename over the target -> keep `*.bak`. On load, if the main file is corrupt, fall back to `.bak`, else start empty and log.

### 2.5 Spec tensions and design decisions

These are places where the requirements pull against each other. The decisions below are binding for all phases and must be documented in `STORY_GUIDE.md`.

1. **A1-01 has no earlier story.** With nothing known yet, every word in A1-01 would be new, which collides with "exactly 20 new words". Decision: the app has a fixed list of **25 starter lemmas** in `assets/starter_words.json` (basic function words plus the basic forms of `zijn` and `hebben`). They are known from the start, merged into every story glossary, never count as new, never enter the debt list, and are never underlined. A1-01 introduces exactly 20 new lemmas and may use only starter words and proper names besides them. The 85-90% known-word rule is waived for A1-01.
2. **The 85-90% known-token rule is token-based and depends on story length.** A short early story with 20 new words repeated 2-3 times each cannot be 85% known. Decision: the validator uses **level- and order-aware thresholds** (config in `validate.rs`): starts low in early A1 and ramps up to 85-90% as stories grow longer. Below-threshold is a warning, not an error.
3. **Sample stories have gaps.** A2-01, B1-01, and B2-01 are written without the stories before them. Decision: the validator has an **`--allow-gaps` mode** (default while fewer than 100 stories exist). Words that are not new and not in the cumulative list are reported as **"assumed known"** and written to a **debt list** (`assumed_known.json`). When the missing earlier stories are written later, the AI prompt template includes the debt list so those stories introduce those words. Strict mode (no gaps) is used once all 100 exist.
4. **Vocabulary targets vs. the 20-word rule.** The stories add 500 lemmas per level (500 / 1,000 / 1,500 / 2,000 cumulative). The per-level totals in the brief (A1 about 600-800, A2 about 1,500, B1 about 2,500-3,000, B2 4,000+) describe the learner's overall level, not only what the stories teach. Decision: the 20-new-words rule is never relaxed; `STORY_GUIDE.md` explains this and how to stay on the curve.
5. **"Drop a file, no code changes" still needs a rebuild** because Android assets are embedded in the APK (see 2.3).

### 2.6 Known platform risks and planned fallbacks

| Risk | Plan | Fallback if impossible |
|------|------|------------------------|
| Tappable words in wrapped text: Slint has no flow layout with per-word hit-testing | Rust lays out words into lines using bundled-font metrics (`fontdue` or `ab_glyph`), emits word rectangles to Slint as a model | Per-line `TouchArea` with approximate word boundaries |
| Page turn: the brief asked for a page curl / flip | Slint has no shaders or true 3D; a fake flip was prototyped | **Removed**: pages now change instantly, with no flip or slide, per the no-animation requirement |
| TTS word highlight: `UtteranceProgressListener` needs a Java class | Speak **sentence by sentence** through the utterance queue and poll `isSpeaking()` to advance the highlight | Highlight only the current page; no live highlight |
| Disabling autocorrect/prediction in Slint `TextInput` | Set `InputType` flags via JNI on the activity view (e.g. no suggestions) | Document the limitation |
| Keyboard insets in Slint | Read insets via JNI or backend events; resize the editor area | Fixed bottom padding plus scrolling |
| App lifecycle (pause/resume) | Use the android-activity event listener exposed by Slint (verify current API name) | Poll window focus |

---

## 3. Phase overview

| Phase | Name | Outcome |
|-------|------|---------|
| 1 | Foundation, storage, and story engine | Project builds an APK, themed app shell runs, stories load and validate, storage is safe |
| 2 | Content and library UI | `STORY_GUIDE.md`, 4 sample stories, Home and Level screens with progress |
| 3 | Reader and learning | Reader with tap zones, word popup, instant page turns, TTS, 30-second rule, dictionary, lookup modes, Settings |
| 4 | Writing feature and polish | Editor, share sheet, My Writing and stats, no-animation polish pass, README |
| 5 | Verification and bug fixing | Everything tested on a device, bugs fixed, final report |

Dependencies: 1 -> 2 -> 3 -> 4 -> 5. Phase 3 needs the stories from Phase 2; Phase 4 reuses Phase 3 typography and storage; Phase 5 needs everything.

---

## 4. Phases

### Phase 1: Foundation, storage, and story engine

**Goal:** a project that builds to an APK and runs on a device or emulator, with a working data layer and a working story pipeline (load, validate, track words), using a test fixture story.

**Tasks**
- [ ] Create the project in `Documents/Code/DutchReader` with the module layout from 2.1 (empty modules compile).
- [ ] Set up the Android build (Slint Android backend + `cargo-apk` or `xbuild`). Add `android_main` and an optional desktop preview entry for development.
- [ ] Manifest: INTERNET permission only, portrait orientation, app name and icon placeholder.
- [ ] Theme system in Slint: light and dark tokens (colors, radii, spacing, font sizes), bundled fonts loaded.
- [ ] Minimal app shell: Home placeholder, navigation stack, and system inset handling (status and navigation bars).
- [ ] `storage/atomic.rs`: atomic write with temp + fsync + rename + `.bak`, and corruption-safe load. Unit-tested, including a simulated interrupted write.
- [ ] `storage/settings.rs` and `storage/progress.rs` with serde models and defaults.
- [ ] `stories/model.rs`: `Story`, `GlossaryEntry`, `Level` (serde, schema from 2.3).
- [ ] `build.rs`: scan `assets/stories/*/*.json`, generate the embedded index; fail the build on invalid JSON.
- [ ] `stories/loader.rs` + `stories/index.rs`: load all stories, sort globally by (level, order).
- [ ] `stories/wordlist.rs`: read and write `assets/wordlist.json`; query "who introduced this lemma".
- [ ] `stories/validate.rs` with these checks (errors and warnings, per 2.5):
  - minimum length (error below 5 computed screens on both reference phones),
  - exactly 20 new words and each has a `"new": true` glossary entry,
  - every word in the text is in the glossary (tokenizer handles case, punctuation, apostrophes, `parts`),
  - new words that appeared in earlier stories,
  - new words used fewer than 2 times in their own story,
  - known-word coverage versus the level and order threshold,
  - id, level, order, and filename consistency,
  - wordlist consistency, plus `--allow-gaps` and the debt list.
- [ ] `src/bin/validate_stories.rs`: CLI with `--write-wordlist`, `--allow-gaps`, and readable output. A debug-build startup check logs the same warnings.
- [ ] Test fixtures: one valid story and several deliberately broken ones (19 new words, missing glossary word, old word flagged new, too short to reach 5 screens, starter word listed as new, etc.) with tests proving each is caught.

**Deliverables:** buildable APK (placeholder UI), working validator CLI, passing unit tests, `wordlist.json` mechanism.

**Exit criteria**
- APK installs and launches on an emulator or device and shows the themed Home shell in light and dark.
- `cargo test` passes, including every broken-fixture test.
- The validator reports the expected error for each broken fixture and passes the valid fixture.
- Killing the process during a write never corrupts a storage file.

---

### Phase 2: Content and library UI

**Goal:** the learner can see real stories: the guide for future writers exists, the 4 samples are written and validated, and Home and Level screens show them with progress.

**Tasks**
- [ ] Write **`STORY_GUIDE.md`** with every section from the brief:
  - purpose and learner, vocabulary curve toward about 2,000 words, and the spec decisions from 2.5;
  - story structure (sections, total words per level, the 5-screen minimum, mini-arc by proportion, titles and blurbs);
  - difficulty progression (general rules and the A1 / A2 / B1 / B2 grammar and topic guidance);
  - the 20 new words (lemma counting, word-type mix, articles for nouns, 2-3 repetitions, recycling, coverage targets, rare-word and name rules);
  - the fixed starter words (list, forms, ownership, why they are never new);
  - glossary requirements, including automatic starter entries;
  - style and content (entertaining, natural Netherlands Dutch, Dutch cultural touches, all-ages, dialogue from A1, short paragraphs);
  - exact JSON schema with an annotated example, folder and file naming, how to run the validator;
  - the pre-submission checklist;
  - the **AI writing prompt template** taking: level, story number, cumulative word list (and debt list), starter words, previous story, and returning the new story plus glossary plus 20 new words as valid JSON.
- [ ] Write **A1-01** first: the easiest story in the app. Present tense, very short sentences (3-8 words), heavy repetition, **exactly 20 new lemmas plus starter words and proper names only** (see 2.5), at least 5 screens. Validate.
- [ ] Write **A2-01, B1-01, B2-01** as story 1 of each level, following the guide exactly, each with a complete glossary and exactly 20 new words. Validate with `--allow-gaps` and generate `wordlist.json` and the debt list.
- [ ] Do a read-through quality pass on each story: entertaining, natural Dutch, difficulty matches the level, jump from A1 to A2 etc. is plausible.
- [ ] **Home screen:** level cards with progress (completed / total stories) and a progress bar, large **Write** button (routes to a placeholder until Phase 4), and entry points to Dictionary, My Writing, Settings.
- [ ] **Level screen:** ordered story list with title, blurb, computed page count, progress, completed state, and instant press feedback (colour only).
- [ ] Progress model wired to `progress.json` (last opened story, per-story text offset, completion, resume target).
- [ ] Screen navigation is instant, with no transition animation.

**Deliverables:** `STORY_GUIDE.md`, 4 validated story files, `wordlist.json`, working Home and Level screens driven by real data.

**Exit criteria**
- The validator passes on all 4 stories (gap warnings only), each with exactly 20 new words and no missing glossary words.
- Adding a fifth (copied) story file and rebuilding makes it appear in the Level list with **no code change**.
- Home shows correct progress counts; Level shows the story and opens a placeholder reader route.
- `STORY_GUIDE.md` can be followed by someone who has not seen this document.

---

### Phase 3: Reader and learning

**Goal:** the full reading experience: computed pages, word lookup, instant page turns, TTS, the 30-second dictionary rule, the Personal Dictionary screen, and Settings.

**Tasks**
- [ ] **Pagination and layout** (`reader/layout.rs`): join the story sections, paginate at one fixed font size to fill the content height, and lay out each page's words into lines with the bundled font metrics, respecting max line length, margins, line height, and paragraph spacing. Recompute on screen-size changes. Output word rectangles (with flags: new word, glossary key) to Slint.
- [ ] **Reader screen:** typography, page indicator, new-word marking (subtle underline or tint), light and dark contrast check.
- [ ] **Tap handling:** word hit-test first (opens popup); otherwise left = previous, right = next, middle = overlay toggle. No conflicts.
- [ ] **Overlay:** TTS button (top-right), back button, page indicator. It appears and disappears instantly, with no fade.
- [ ] **Page turn:** the next / previous page and the page indicator change in the same frame. No flip, slide, fade, or timing flag.
- [ ] **Resume:** save the text offset on every change; open on the computed page that contains it.
- [ ] **Page timer** (`reader/page_timer.rs`, pure Rust, injectable clock): 30 seconds (`READ_MILLIS = 30_000`), resets on page change, leaving the reader, and app background. Wire to lifecycle events (`platform/lifecycle.rs`).
- [ ] **Personal dictionary** (`dictionary/`): on timer completion, add all glossary words on that page from the **bundled glossary**; no duplicates; store word, meaning, story and level, date. Function words are filtered out except starter words, which are attributed to the first story in reading order that uses them.
- [ ] **Story completion:** mark complete when the last page has been read per the 30-second rule (or per a documented, consistent rule) and update Home and Level progress.
- [ ] **Lookup** (`lookup/`): `WordLookup` trait; `OfflineGlossary` provider; `MyMemoryProvider` (ureq + rustls, short timeout, off the UI thread); local cache; automatic fallback with a small note; "machine translated" label; loading state in the popup. Look up the tapped surface form at its occurrence, never the lemma.
- [ ] **Popup / bottom sheet** with no animation, showing the tapped word as written, the form-specific meaning, the article (only when the form is the lemma), the `form of: <lemma>, <meaning>` base line, and the source label.
- [ ] **TTS bridge** (`platform/tts.rs` via JNI): init `TextToSpeech`, nl-NL locale, speed from settings, start / stop, sentence-by-sentence queue, highlight current sentence, continue across page turns, stop when leaving the reader. Handle a missing Dutch voice with a clear message.
- [ ] **Personal Dictionary screen:** list, search, sort (alphabetical / newest first), empty state.
- [ ] **Settings screen:** theme (light / dark / follow system, applied live), TTS speed slider, meanings source (offline default / online); all persisted.

**Deliverables:** a complete reading and learning loop on the 4 stories.

**Exit criteria**
- Reading a page for 30 seconds adds its glossary words once; re-reading adds nothing; a page left at 29 seconds adds nothing; switching page, backgrounding, or leaving resets the timer.
- Word tap works in both lookup modes; in online mode with airplane mode on, the popup falls back to the glossary and shows the note; a cached word works offline.
- TTS reads Dutch, survives page turns, and its speed follows Settings (default `0.88`).
- Page turns are instant: no animation, no stuck state, and the correct page after rapid taps.
- Theme setting applies immediately and persists across restarts.

---

### Phase 4: Writing feature and polish

**Goal:** the complete writing loop (write, share, save, review stats), plus a final polish pass and documentation.

**Tasks**
- [ ] **Editor screen:** large text area with reader typography, light and dark, optional title field (default = date), live word count, **no visible timer**.
- [ ] **Keyboard handling:** text area scrolls, cursor stays visible, toolbar stays reachable; apply the IME flags to disable autocorrect and prediction via JNI (`platform/keyboard.rs`), or document the fallback.
- [ ] **Silent time tracking:** accumulate seconds only while the editor is open **and** the app is in the foreground; store with the draft.
- [ ] **Draft autosave** (`writing/draft.rs`): every few seconds, on background, and on leaving the editor; never save empty drafts. On open with an existing draft, show **Continue / Start new**.
- [ ] **Share text builder** (`writing/share_text.rs`): produces exactly `"<story>"` + blank line + `Are there any grammar or vocabulary mistakes in this story, other than punctuation?`, keeps line breaks inside the quotes, never includes the title. Unit-tested against the exact string.
- [ ] **Share bridge** (`platform/share.rs`): `ACTION_SEND` + `text/plain` + `Intent.createChooser`, launched through JNI from the activity context.
- [ ] **Share flow:** button disabled with a hint when empty; sharing saves the story as finished (text, title, timestamp, word count, time spent) and clears the draft.
- [ ] **My Writing screen:** newest-first list (title or date, preview, word count, time spent); detail view with re-share (same format), copy, edit, delete with confirmation.
- [ ] **Statistics** (`writing/stats.rs`): total stories, per-month stories and words (bar chart or list), current streak in days or weeks (define "current" clearly, e.g. a streak stays alive through today). Unit-tested with fixed dates, including month and year boundaries.
- [ ] Writing data in its own files (`writing/draft.json`, `writing/stories.json`) with the atomic write layer.
- [ ] **Polish pass:** no animations anywhere; instant editor entry / exit, instant press feedback on all cards and buttons (colour change only), instant progress bar, empty states, consistent spacing, dark-mode contrast review, inset review on all screens.
- [ ] **README.md:** prerequisites (Rust, Android SDK / NDK, build tool), build and install commands for device and emulator, desktop preview, running the validator, adding a story.
- [ ] Draft the **final summary** (what is done, how to build and install, and a list of anything not implemented as asked with what was done instead).

**Deliverables:** complete app feature set, README, final-summary draft.

**Exit criteria**
- From Home, one tap opens the editor; typing, then one tap on Share opens the Android share sheet with the exact required text; the story appears in My Writing.
- Closing the app mid-draft and reopening offers Continue; no text is lost.
- Stats match the saved stories (verified with test data across at least 3 months).
- Empty text: Share is disabled, nothing is saved.

---

### Phase 5: Verification and bug fixing

**Goal:** prove the app works end to end and fix everything found. This phase produces a **bug log** and a **test report**. Nothing is "done" until this phase passes.

**5.1 Static and automated checks**
- [ ] `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` all pass.
- [ ] Validator passes on all 4 stories; every broken fixture still fails as expected.
- [ ] Unit tests cover: atomic write recovery, page timer (reset rules, 30-second boundary at 29 / 30 s), dictionary dedupe, lookup fallback and cache, share text exact string, draft autosave, stats and streak edge cases, pagination (no word lost or duplicated across lines and pages), and occurrence-aware glossary lookup.
- [ ] Release build succeeds; APK declares **only** the INTERNET permission; APK size noted.

**5.2 Story content verification**
- [ ] Each story: exactly 20 new words, all non-starter text words in the glossary, at least 5 computed screens on both reference phones, every new word appears at least twice, no new word appears in an earlier story.
- [ ] Read each story aloud (via TTS) for natural flow; confirm the A1-01 constraint (20 new lemmas plus starter words and names only) and the difficulty step between levels.
- [ ] Copy-a-story test: adding a new file and rebuilding makes it appear with no code change (then remove it).

**5.3 On-device functional test script** (run on a real device if possible, plus an emulator; record pass or fail for each)

| Area | Checks |
|------|--------|
| Home | Progress counts correct; every button routes correctly |
| Level | Order, blurb, computed page count, progress and completed states |
| Reader | Left / right / middle zones; word tap vs. zone tap never conflict; new words marked, starter words not; resume at the stored text offset |
| Page turn | Instant, no flicker, correct page after rapid taps; no stuck state |
| Page timer / dictionary | 30-second add; no duplicates; reset on page change, background, leaving reader |
| Lookup | Offline default; online success; online timeout; airplane mode fallback with note; cached word offline; label shown |
| TTS | Starts, stops, speed change, survives page turns, stops on leaving reader, missing-voice message |
| Dictionary screen | Search, both sorts, empty state |
| Settings | Theme live apply and persist, TTS speed persist, source persist |
| Editor | Typing, word count, long text scroll, cursor above keyboard, autocorrect off, silent timer (no UI) |
| Draft | Autosave, kill app, Continue / Start new |
| Share | Exact text in ChatGPT or another app, line breaks kept, no title; disabled when empty; saved afterwards |
| My Writing | List order, detail, re-share, copy, edit, delete confirmation, stats and streak |

**5.4 Robustness and edge cases**
- [ ] Force-stop and reboot during reading and writing; data intact.
- [ ] Kill the app during a write (repeat several times); no corrupted file.
- [ ] Corrupt a storage file by hand; app recovers from `.bak` or starts clean without crashing.
- [ ] Rotation and screen-size changes (portrait first; must not crash).
- [ ] Dark and light mode on every screen; contrast acceptable.
- [ ] Very long writing (several thousand words); no lag.
- [ ] Rapid navigation (back-and-forth spam) with no crash or leaked timers.
- [ ] Low-connectivity and slow-response lookups; the UI never freezes.
- [ ] Notches, gesture navigation, and 3-button navigation insets.

**5.5 Performance**
- [ ] Screen changes and page turns are instant; no animation is present to profile, and no tap is swallowed by a timing guard.
- [ ] Cold start time noted; no ANR in the lookup, TTS, or share paths.
- [ ] Memory stable over a long reading session.

**5.6 Fix loop**
- [ ] Log every bug (what, where, how to reproduce, severity). Fix all critical and major bugs; re-run the affected tests and the full automated suite after each fix.
- [ ] Re-run the full on-device script once after the last fix.

**5.7 Final report** (the end-of-project summary)
- What was done, in short.
- How to build and install (exact commands).
- Test results (what passed, any known minor issues left).
- Anything **not implemented as asked** (TTS highlighting, page-curl, keyboard flags, share-sheet details, auto-discovery needing a rebuild, etc.) and what was done instead.
- The debt list (words assumed known by the sample stories) and next steps for writing the remaining 96 stories.

**Exit criteria (project done)**
- All 5.1-5.5 checks pass; no open critical or major bugs.
- The final report is delivered.

---

## 5. Definition of done (whole project)

- A working APK with 4 validated sample stories and the full writing feature.
- The modular story system and validator, with a cumulative word list and debt list.
- `STORY_GUIDE.md` with schema, checklist, and AI writing prompt template.
- `README.md` with build and run instructions.
- A completed bug log, test report, and final summary including every deviation from the brief.
