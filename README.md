# DutchReader

DutchReader is an Android-only graded Dutch reader and writing app. It is
local-first: stories, progress, settings, and writing live on the device.

This repository contains the completed **Phase 5** build described in
[`APP_BRIEF.md`](APP_BRIEF.md) and [`ARCHITECTURE.md`](ARCHITECTURE.md): the
foundation, storage layer, story engine, validator, the complete A1, A2, B1,
and B2 shelves (25 stories each), the story-writing guide, the Home/Level
library UI, the complete reading loop, and the complete writing loop. The reading loop includes the paginated reader,
word lookup, page turns, text-to-speech, the 30-second dictionary rule, the
Personal Dictionary and Tapped words tabs, and Settings. The writing loop adds
the editor, draft autosave, native sharing, My Writing, and writing statistics.

Phase 5 added on-device verification, the local-time-zone fix, the bug log, and
the test report. See [`docs/bug-log.md`](docs/bug-log.md) and
[`docs/test-report.md`](docs/test-report.md).

See [`STORY_GUIDE.md`](STORY_GUIDE.md) for the complete story specification:
schema, difficulty curve, the 20-new-words rule, glossary rules, validator
workflow, checklist, and the AI writing prompt template.

## Prerequisites

- Rust 1.92+ with the Android targets:
  `rustup target add aarch64-linux-android x86_64-linux-android`
- Android SDK with platform 34, build-tools, and an NDK (r26 recommended)
- JDK 17
- `cargo-apk` 0.10: `cargo install cargo-apk --locked`
- On Windows, a linker. The project is currently developed with the GNU host
  toolchain (`rustup override set stable-x86_64-pc-windows-gnu`) and MSYS2
  UCRT64 (`C:\msys64\ucrt64\bin` on `PATH`).

Set the Android paths before building:

```powershell
$env:ANDROID_HOME = "C:\Android-SDK"
$env:ANDROID_NDK_ROOT = "C:\Android-SDK\ndk\26.1.10909125"
$env:JAVA_HOME = "C:\Program Files\Microsoft\jdk-17.0.19.10-hotspot"
```

## Desktop preview

```powershell
$env:PATH = "C:\msys64\ucrt64\bin;" + $env:PATH
cargo run -p dutchreader-desktop
```

The desktop app uses winit + femtovg. Settings and progress go to
`.dutchreader-data/` in the working directory, or to `DUTCHREADER_DATA_DIR` if
set. The Home screen has a Dark/Light toggle for checking both themes.

The desktop runner lives in its own workspace package (`desktop/`) because
`cargo-apk` treats every binary in the Android package as a native library.
For the same reason the validator CLI is its own package (`validator/`).

## Android build and install

```powershell
# Universal APK (arm64-v8a + x86_64), debug-signed
cargo apk build
adb install -r target\debug\apk\DutchReader.apk
adb shell am start -n com.dutchreader.app/android.app.NativeActivity

# One ABI only
cargo apk build --target x86_64-linux-android
cargo apk build --target aarch64-linux-android
```

`cargo apk run --target x86_64-linux-android` builds, installs, and launches on
a connected emulator or device. The manifest requests only the `INTERNET`
permission and locks the activity to portrait.

The current debug APK is debug-signed. `cargo apk build --release` needs a
release keystore, supplied either under
`[package.metadata.android.signing.release]` or through the
`CARGO_APK_RELEASE_KEYSTORE` and `CARGO_APK_RELEASE_KEYSTORE_PASSWORD`
environment variables:

```powershell
$env:CARGO_APK_RELEASE_KEYSTORE = "C:\path\to\release.jks"
$env:CARGO_APK_RELEASE_KEYSTORE_PASSWORD = "your-password"
cargo apk build --release
```

The release APK is signed with the APK Signature Scheme v2/v3, requests only
`INTERNET`, and contains `arm64-v8a` and `x86_64`. Phase 5 used a throwaway test
key; a real keystore is required before shipping.

## Reading loop

- **Reader:** one fixed font size (20 px) for every story. The app joins the
  story's sections and computes pages that fill the screen; only the last page
  may be partly filled, and pages are recomputed when the screen size changes.
  One `Text` per word is placed at the exact position measured with the bundled
  Crimson Text metrics. New words get a subtle accent underline; starter words
  never do. Tap a word for its meaning; tap the left third of the page for the
  previous page, the right third for the next page, or the middle to show/hide
  the overlay. The page indicator, Back button, and Listen/Stop button live in
  the overlay.
- **Page turns:** instant. The next or previous page and the page indicator
  change in the same frame; there is no slide, flip, shadow, or fade.
- **30-second rule:** a page counts as read after 30 seconds in the foreground.
  Its glossary words are added to `dictionary.json` once; starter words are
  saved too, attributed to the first story in reading order that uses them.
  Changing page, leaving the reader, or backgrounding the app resets the count.
  The last computed page also marks the story complete.
- **Resume:** progress stores a text offset, not a page number, so resuming
  survives repagination on a different screen size.
- **Word lookup:** Settings chooses the story glossary (offline, default) or
  MyMemory machine translation. A tap looks up the word **as written** at that
  occurrence, not its lemma. The popup shows the tapped form (for example
  `ben`), its form-specific meaning (for example `am`), and a secondary
  `form of: zijn, to be` line. Ambiguous forms use per-occurrence glossary
  entries where one meaning per form cannot work. Online lookups run on a
  worker thread, show a loading state, cache every result by tapped word in
  `lookup_cache.json`, and fall back to the story glossary with a note when the
  network is unavailable. A cached word works offline and is labelled
  "machine translated · cached".
- **Text-to-speech:** the Listen button speaks the page sentence by sentence in
  Dutch, highlights the current sentence, follows the speed slider (default
  `0.88`, range `0.5`–`2.0`, saved speed wins), survives page turns, and stops
  when the reader is left or the app is backgrounded. If no Dutch voice is
  installed, the reader explains how to add one.
- **Dictionary:** two tabs share one search field and the A–Z / Newest sort.
  **All words** lists learned words with meaning, part of speech, story, level,
  and date. **Tapped words** records every successful lookup exactly as it was
  tapped — surface form, meaning, optional `form of:` base line, story, level,
  date, and source. Repeating a tap updates the entry and increments its count
  instead of creating a duplicate; lookups with no meaning are not saved. Tapped
  words live in their own `tapped_words.json` and survive restarts. Both tabs
  have their own description and empty state, and the tab selection resets to
  All words on launch.
- **Settings:** theme (System/Light/Dark, applied live), TTS speed, and word
  meanings source. All settings persist in `settings.json` across restarts.

## Writing loop

- **Editor:** opened from the Home **Write** button in one tap. It uses the
  reader's serif typography, follows the light/dark theme, and has an optional
  title field whose placeholder is the current date. The word count updates
  live. Time spent is tracked silently while the editor is open and the app is
  in the foreground; there is no timer UI and no limit.
- **Keyboard:** the text area scrolls and the Share row stays above the
  on-screen keyboard. The editor asks the IME to disable autocorrect,
  autocomplete, and auto-capitalisation through Slint's `input-method-hints`, so
  typed mistakes stay visible. Some keyboards (for example Gboard) may still
  show suggestion chips, which the app cannot suppress through this API (see the
  Phase 4 notes for why no JNI keyboard bridge was needed).
- **Drafts:** `writing/draft.json` is written with the atomic write layer every
  few seconds, when the app goes to the background, and when the editor closes.
  Empty text never creates or keeps a draft. Opening the editor with an
  unfinished draft offers **Continue** or **Start new**.
- **Share:** one tap on Share opens the native Android share sheet
  (`ACTION_SEND`, `text/plain`, `Intent.createChooser`) through the JNI bridge in
  `src/platform/share.rs`. The text is exactly:

  ```
  "<story, line breaks preserved>"

  Are there any grammar or vocabulary mistakes in this story, other than punctuation?
  ```

  The title is never included. Share is disabled with a hint while the text is
  empty. On Android a successful share saves the story, clears the draft, and
  opens the saved story; the desktop preview shows a note instead and saves
  nothing.
- **My Writing:** newest-first list with title (or date), preview, word count,
  and time spent, plus a detail view with re-share, copy, edit, and delete with
  confirmation. Editing a saved story updates it in place rather than creating a
  duplicate.
- **Statistics:** total stories, total words, per-month stories and words with
  proportional bars, and the current streak. All dates use the phone's local
  time zone (DST-aware). A streak counts consecutive local days with at least one
  saved story; multiple stories on one day count once, and the streak stays alive
  through today when yesterday has a story.
- **Storage:** `writing/draft.json` and `writing/stories.json` live in the
  app-private `files/writing/` directory and use the same atomic write and
  backup recovery as the rest of the app.

## Story validator

```powershell
cargo run -p validate_stories
cargo run -p validate_stories -- --strict
cargo run -p validate_stories -- --write-wordlist
cargo run -p validate_stories -- --stories tests/fixtures/valid
```

Because the three sample stories still carry background vocabulary, the
validator defaults to `--allow-gaps`: words without an earlier introduction
become warnings and are written to `assets/assumed_known.json`. `--strict`
requires every used word to have been introduced by an earlier story; it
currently reports the expected sample-story debt as errors.

The library contains all 100 stories: 25 A1, 25 A2, 25 B1, and 25 B2 stories,
with 2,025 owned lemmas and 244 assumed-known words. Every shelf passes in
allow-gaps mode with no coverage or repetition warnings; the remaining
expected warnings are the `ASSUMED_KNOWN` gaps for the sample stories `a2-01`,
`b1-01`, and `b2-01`.
Every story must render as at least 5 screens at the fixed reader font size on
both reference phones; the validator checks this with the real layout engine
(`MIN_SCREENS`).

Stories are JSON files under `assets/stories/<level>/<order>-<slug>.json`.
`build.rs` embeds every file at compile time, so adding a story needs no Rust or
Slint changes: drop the file in, regenerate the word list, and rebuild.

```powershell
cargo run -p validate_stories -- --write-wordlist --allow-gaps
cargo apk build
```

## Tests

```powershell
cargo test
```

This runs the root package and the `desktop` / `validator` default workspace
members, so the broken-fixture tests are included.

The suite includes:

- validator fixture tests in `tests/fixtures/` (one valid story tree and twelve
  deliberately broken ones),
- CLI tests proving the validator exits non-zero with the expected diagnostic,
- a real process-kill test that terminates a writer mid-write and verifies the
  JSON file still parses,
- storage recovery tests for corrupt main files, backups, empty files, and
  leftover temporary files,
- an auto-discovery test that compares the embedded story index with every JSON
  file under `assets/stories/`,
- app-level tests for Home/Level progress text and row counts,
- reader layout tests (word boxes, line breaking, paragraphs, hit-testing,
  fixed font, dynamic pagination, page-range coverage, offset lookup, the
  5-screen minimum on both reference phones, no lost or duplicated words),
- page-timer boundary tests at 29/30 seconds, including page-change,
  background, and leave resets,
- dictionary dedupe, search, sort, and date tests,
- tapped-word tests for recording, repeat-tap counting, article/base lines,
  skipping empty meanings, search, sort, and JSON round-trips,
- lookup tests for offline, online, fallback, cache, MyMemory parsing, and
  occurrence-aware form lookup,
- local-time tests for midnight, month boundaries, and DST, plus the Android
  millisecond-to-second offset conversion,
- writing tests for the exact share text (including line breaks and the title
  exclusion), word counting, draft round-trips and empty-draft clearing, story
  upsert/edit/delete ordering, and statistics/streak edge cases with fixed dates
  across month and year boundaries.

The Phase 5 run passed **131 tests** (124 library, 2 atomic-kill, 5 validator)
with `cargo fmt` and clippy clean. The Tapped words follow-up adds six library
tests, for **137 tests** (130 library, 2 atomic-kill, 5 validator). See
[`docs/test-report.md`](docs/test-report.md).

## Layout

```
assets/            fonts, stories, starter_words.json, wordlist.json, assumed_known.json
android/res/       launcher icon and Android resources
src/stories/       model, loader, tokenizer, wordlist, validator
src/storage/       atomic writes, settings, progress
src/reader/        pagination/layout and the 30-second page timer
src/dictionary/    personal dictionary and tapped words, dedupe, search, sort
src/lookup/        WordLookup trait, glossary, MyMemory, cache
src/writing/       share text, draft, story store, statistics
src/time.rs        local civil dates, month/streak maths, DST-aware
src/platform/      Android lifecycle, JNI text-to-speech, JNI share and time-zone bridges
src/app.rs         Slint <-> Rust wiring (the only bridge module)
ui/                Slint theme, shell, Home/Level/Reader/Editor/Writing/Dictionary/Settings
desktop/           desktop preview runner (workspace package)
validator/         validator CLI and fixture tests (workspace package)
tests/             process-kill and storage tests
docs/screenshots/  emulator captures (Home, level, reader, editor, writing, dictionary, settings)
docs/glossary-audit.md  every entry and occurrence checked in the sample stories
docs/bug-log.md    Phase 5 bug log (what, where, reproduction, fix, verification)
docs/test-report.md Phase 5 test report and known deviations
docs/final-summary.md   final summary, build/install, and deviations
STORY_GUIDE.md     how to write, validate, and extend the complete story library
```

## Phase 3 notes

- A1-01 is the real story now: `assets/stories/a1/01-de-rode-bal.json` ("De rode
  bal") introduces exactly 20 new lemmas and uses only starter words and proper
  names besides them, as required by the A1-01 exception in `ARCHITECTURE.md`
  section 2.5.
- `assets/starter_words.json` holds the 25 starter lemmas that are known from
  the start. They are merged into every story glossary, never count as new,
  never appear in `new_words`, and never enter the assumed-known debt list.
- `assets/wordlist.json` maps every introduced lemma to its story, or to
  `"starter"` for starter words; `assets/assumed_known.json` is the debt list of
  words the sample stories use before any story introduces them. Both are
  generated with `--write-wordlist`; never edit them by hand.
- Home shows completed/total counts and a progress bar per level, a "Continue
  reading" card when a story has been opened, and a large Write button (wired to
  the editor in Phase 4).
- Level shows ordered story cards with title, blurb, a page count computed for
  the current screen, an in-progress or completed state, and the stored resume
  position. Home and Level use the same pagination as the reader, so their page
  labels agree with it. Opening a story records a text offset in
  `progress.json` and opens the reader on the page that contains it.
- Home, Level, and the reader change instantly; there are no screen
  transitions, page flips, fades, or moving press effects. Cards and buttons
  may change colour on press, but nothing animates.
- The dictionary adds content words only. Articles, determiners, names,
  conjunctions, prepositions, numerals, and interjections are glossed in the
  reader but deliberately kept out of the Personal Dictionary, which would
  otherwise fill with function words. Starter words are the one exception: they
  are saved so a beginner can review them, and are attributed to the first story
  that uses them. This is the one documented deviation from "all glossary
  words" in `ARCHITECTURE.md` section 1.6.
- The page flip was removed on purpose: Slint's public API has no arbitrary 3D
  transform, and the Phase 3 no-animation requirement replaced the flip with an
  instant page change. This is no longer a limitation or a section 2.6 fallback.
- The Android system status bar is drawn by the platform. Slint's Android
  backend draws edge-to-edge and does not expose status-bar icon appearance, so
  Android keeps the icons white. The light theme therefore paints an
  accent-coloured strip behind the status-bar inset; the dark theme needs no
  strip. The app content itself is fully themed in both modes.
- Changing the display density or font scale recreates the Android activity,
  because Slint's software renderer caches the scale factor. Saved progress,
  settings, and the draft are read back from disk, so the app returns to Home
  with the new scale. Screen-size changes, rotation, the keyboard, and theme
  changes do not recreate the activity.
- The desktop preview window can clip at 150% Windows display scaling because
  the OS client area stays at the requested logical size while Slint renders at
  1.5x. Use the emulator for layout verification.

## Phase 4 notes

- **Local dates.** Story timestamps, the default title, the My Writing list, and
  all statistics use the phone's local time zone, including DST. Android supplies
  the offset for each timestamp through `java.util.TimeZone` (JNI); the desktop
  preview uses `chrono::Local`. The shared maths lives in `src/time.rs`. This
  replaced the Phase 4 UTC behaviour after the Phase 5 fix; see
  [`docs/bug-log.md`](docs/bug-log.md).
- **Keyboard flags without a JNI bridge.** The Phase 4 task allowed either a
  `platform/keyboard.rs` JNI bridge or a documented fallback. Slint 1.18.1
  exposes `TextInput.input-method-hints` with `auto-correct`, `auto-complete`,
  and `capitalization`, so the editor sets those to `false`/`none` and needs no
  Java keyboard code. Gboard still displays suggestion chips because that is an
  IME-side feature; autocorrect itself did not alter the test text. This is the
  documented fallback: no separate `platform/keyboard.rs` exists.
- **Keyboard layout.** The editor keeps its full height and reserves the
  keyboard height as bottom padding instead of shrinking the layout, because a
  shrunk Slint layout is vertically centred and left a gap under the toolbar.
  The Share row therefore stays directly above the keyboard.
- **Share chooser flag.** `startActivity` is called from Slint's native
  event-loop thread, where the JNI context is not always the Activity itself.
  The chooser intent therefore carries `FLAG_ACTIVITY_NEW_TASK`; without it
  Android throws `AndroidRuntimeException` and the sheet never appears. The
  bridge prints Java stack traces to logcat if the call fails.
- **Titles.** The title field is optional. Leaving it blank stores an empty
  title and displays the local date instead; the date is never sent in the share
  text. Reopening an existing draft uses the draft's creation date.
- **Edit in place.** Editing from a story's detail view puts the story id in the
  draft (`editing_id`), so the next successful share updates that story instead
  of adding a duplicate.
- **Streak definition.** The current streak is the number of consecutive local
  days ending today that have at least one saved story. If today has none yet,
  the streak still counts through yesterday. Multiple stories on one day count
  once.
- **Share on desktop.** The desktop preview cannot open an Android share sheet,
  so it shows a note instead. It does not save the story or clear the draft;
  the save-and-clear path is Android-only.
- **No animations.** The editor, My Writing list, detail view, delete dialog,
  and share entry are instant. There are no transitions or moving press effects;
  buttons and cards only change colour while pressed.
- **Verified statistics.** The emulator was seeded with five stories across
  September and October 2026. My Writing showed 5 stories, 26 words,
  the expected Oct 4/21 and Sep 1/5 month rows with proportional bars, and a
  3-day streak. A sixth story written through the editor updated all totals, and
  deleting it restored 5 stories / 26 words.
