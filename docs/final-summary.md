# DutchReader — final summary (through Phase 5 and the Tapped words follow-up)

## What is done

DutchReader is an Android-only, local-first graded Dutch reader and writing app.
All phases of `ARCHITECTURE.md` (1–5) are implemented and verified:

- **Foundation and storage:** atomic JSON writes with backup recovery, settings,
  reading progress, Personal Dictionary, tapped words, and lookup cache.
- **Story engine and content:** the complete A1, A2, B1, and B2 shelves (25
  stories each), the starter-word list, the generated word list and
  assumed-known debt list, and the `validate_stories` CLI with its fixture
  suite. The library now holds 100 stories and 2,025 owned lemmas.
- **Reading loop:** paginated reader with the reader typography, per-word hit
  testing, glossary and optional MyMemory lookup, the 30-second dictionary rule,
  text-to-speech with sentence highlighting, the Personal Dictionary and Tapped
  words tabs, and Settings (theme, TTS speed, meanings source).
- **Writing loop:** editor with optional title (default date), live word count,
  silent foreground time tracking, draft autosave every few seconds / on
  background / on exit, Continue or Start new, native Android sharing with the
  exact required text, My Writing list and detail (re-share, copy, edit, delete
  with confirmation), and statistics (totals, per-month stories and words,
  current streak).
- **Tapped words follow-up:** every successful lookup is saved to a separate
  `tapped_words.json` and shown in the Dictionary's **Tapped words** tab with
  the tapped form, meaning, optional `form of:` base line, story, level, date,
  source, and tap count. Repeat taps update the entry and increment the count.
  The tab shares the existing search and A–Z / Newest sort, and resets to
  **All words** on launch.
- **Phase 5 verification:** local-time-zone fix (DST-aware), bug log, test
  report, release build, and a full on-device pass. See
  [`bug-log.md`](bug-log.md) and [`test-report.md`](test-report.md).
- **Polish:** no animations anywhere, instant press feedback, empty states,
  consistent spacing, dark-mode contrast, inset handling, and a light-mode
  status-bar accent strip so the white system icons stay readable.
- **Documentation:** `README.md` covers prerequisites, build/install, desktop
  preview, validator use, adding a story, and current behaviour.

## How to build and install

Prerequisites: Rust 1.92+ with the `aarch64-linux-android` and
`x86_64-linux-android` targets, Android SDK platform 34 and build-tools, an NDK
(r26 recommended), JDK 17, and `cargo-apk` 0.10. On Windows the project uses the
GNU host toolchain and MSYS2 UCRT64 on `PATH`.

```powershell
$env:PATH = "C:\msys64\ucrt64\bin;$env:USERPROFILE\.cargo\bin;" + $env:PATH
$env:ANDROID_HOME = "C:\Android-SDK"
$env:ANDROID_NDK_ROOT = "C:\Android-SDK\ndk\26.1.10909125"
$env:JAVA_HOME = "C:\Program Files\Microsoft\jdk-17.0.19.10-hotspot"

cargo apk build
adb install -r target\debug\apk\DutchReader.apk
adb shell am start -n com.dutchreader.app/android.app.NativeActivity
```

The debug APK is debug-signed and universal (`arm64-v8a` + `x86_64`). For a
release build, supply a keystore through the environment:

```powershell
$env:CARGO_APK_RELEASE_KEYSTORE = "C:\path\to\release.jks"
$env:CARGO_APK_RELEASE_KEYSTORE_PASSWORD = "your-password"
cargo apk build --release
```

The release APK requests only `INTERNET`, contains both ABIs, and is signed with
APK Signature Scheme v2/v3. A real keystore is required to ship; the Phase 5
test key is not for distribution.

Desktop preview:

```powershell
cargo run -p dutchreader-desktop
```

Validator:

```powershell
cargo run -p validate_stories
cargo run -p validate_stories -- --strict
cargo run -p validate_stories -- --write-wordlist
```

## Test results

Static and automated (all pass):

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace` — **137 tests** (130 library, 2 atomic-kill,
  5 validator)
- `cargo run -p validate_stories -- --allow-gaps` — `0 error(s), 3 warning(s)`
  (the expected `ASSUMED_KNOWN` debt for `a2-01`, `b1-01`, and `b2-01`; all 100
  stories have no coverage or repetition warnings)

On-device (`DutchReaderTest` emulator, 1080×2400 @420, `Africa/Cairo`):

- Cold start 631 ms; no ANR, `FATAL EXCEPTION`, or `panic`.
- Home shows 25 stories on every shelf (A1/A2/B2 `0 of 25`; B1 `1 of 25` from
  the stored resume), for 100 stories in total.
- The B1 and B2 shelves list stories 01–25 with their page counts. B1 story 24
  (`De reünie`) opens at `Page 1 of 18`; B2 story 25 (`De promotie`) opens as a
  generated story with 20 pages. Tapping a new word shows its `de`/`het`
  article, the `Story glossary` badge, and the English meaning.
- Memory after the session: PSS 80,469 kB, RSS 163,008 kB (an earlier long
  session measured 83,920 / 165,868; the 100-story B2 session measured 99,442 /
  185,560).
- Home, Level, Reader, Dictionary, Settings, Editor, and My Writing all route
  and render in both themes.
- Local dates verified across midnight and month boundaries, including DST:
  `31 Oct 2026 01:30`, `30 Oct 2026 01:30`, `29 Oct 2026 00:30`,
  `1 Oct 2026 02:30`, `1 Sep 2026 01:30`.
- Lookup offline / online / cached / airplane-mode fallback / slow-network
  loading all behave.
- TTS start/stop, sentence highlight, page-turn survival, and stop-on-leave.
- Editor autocorrect off, suggestions shown (IME limitation), 5,040-word draft
  handled and restored after a kill.
- Share text exact; copy/paste round-trip exact.
- My Writing add/edit/delete and statistics correct.
- Tapped words: offline lookup saved, `form of:` note shown, repeat tap counted,
  and all entries survived a force-stop and emulator restart.
- Corrupt storage recovers from `.bak` or starts clean, never crashes.
- Small (720×1280) and large (1440×3200) screens render; portrait lock holds.
- A size/density change while the app is backgrounded now relayouts correctly
  (Android recreates the activity; page count and draft survive).

The full matrix is in [`test-report.md`](test-report.md).

## Not implemented exactly as asked, and what was done instead

1. **Keyboard suggestions.** The editor disables autocorrect, autocomplete, and
   auto-capitalisation through Slint's `input-method-hints`. Gboard still draws
   its suggestion strip; Slint 1.18.1 has no `TYPE_TEXT_FLAG_NO_SUGGESTIONS`
   hint. Autocorrect itself stayed off. No `platform/keyboard.rs` bridge exists.
2. **Personal Dictionary function words.** All glossary words are added, so
   `de`, `het`, `een`, and `en` appear despite the brief's "filler words" note.
   This keeps every glossary word reviewable.
3. **Story lengths.** A2-01 is 622 words (brief 650-850); A2-02 through
   A2-25 are 657-849 words and are in range. B1-01 is 846 words (brief
   850-1100); B1-02 through B1-25 are 850-1,028 words and are in range. A1 (697)
   is in range. B2-01 is 1,053 words; B2-02 through B2-25 are 1,122-1,288 words,
   all inside the 1,050-1,300 range.
4. **TTS highlighting.** Sentences are highlighted, not individual words.
5. **Middle tap.** Tapping the middle toggles the overlay rather than turning a
   page. `APP_BRIEF.md` lines 110 and 113 conflict; the overlay toggle matches
   `ARCHITECTURE.md`.
6. **Editing and Back.** Editing a saved story and pressing Back keeps the
   changes as a draft; tapping Share updates the saved story.
7. **`ASSUMED_KNOWN` warnings.** Expected because A2-01, B1-01, and B2-01 were
   written before the stories that would introduce their background vocabulary.
   The generated shelves never use that debt. The 100-story library therefore
   still defaults to allow-gaps mode; strict mode becomes the default once the
   three samples are cleaned up.
8. **Status-bar icons.** The light theme paints an accent strip behind the
   status bar. A JNI attempt to set dark icons was reverted because
   `android_main` is not the Android UI thread
   (`CalledFromWrongThreadException`).
9. **Release signing.** A real keystore is required; the test key is not for
   shipping.
10. **Physical device and 60 fps.** All device checks ran on the emulator. The
    software emulator cannot measure frame rate reliably, and the app has no
    animations to profile.
11. **Read-aloud judgement.** Natural-flow judgement is subjective and was left
    to a human reader; TTS playback was exercised instead.
12. **Auto-discovery needs a rebuild.** Stories are embedded at compile time, so
    adding a JSON file needs `cargo apk build` but no code change.
13. **New-word repetition.** The validator enforces at least two appearances;
    the brief's "2-3 times" is treated as a minimum of two.
