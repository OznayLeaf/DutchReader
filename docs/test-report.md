# DutchReader — Phase 5 test report

Verification of `ARCHITECTURE.md` Phase 5 (5.1–5.7) plus the extra checks
requested for this pass. Every result below was produced in this working tree.

## Environment

| Item | Value |
|------|-------|
| Host | Windows, GNU host toolchain, MSYS2 UCRT64 |
| Rust | 1.92 (edition 2024) |
| Android SDK | platform 34, build-tools, NDK 26.1.10909125 |
| JDK | 17 |
| `cargo-apk` | 0.10.0 |
| Device | Android emulator `DutchReaderTest`, 1080×2400 @420 dpi |
| Emulator time zone | `Africa/Cairo` (EET/EEST, UTC+2/+3 with DST) |
| Device date | Sat 31 Oct – Tue 10 Nov 2026 (B2 shelf re-run on 10 Nov) |

**Limitation:** no physical phone was available. All device checks ran on the
emulator above. The 60 fps target cannot be measured reliably on the software
emulator; the app deliberately has no animations, so this was not profiled.
"Read each story aloud for natural flow" is a subjective judgement that an
automated agent cannot make; TTS playback was exercised instead (see 5.2).

---

## 5.1 Static and automated checks

All commands were run from the repository root.

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p validate_stories -- --allow-gaps
```

| Check | Result |
|-------|--------|
| `cargo fmt --all -- --check` | Pass (no diff) |
| `cargo clippy --workspace --all-targets -- -D warnings` | Pass (no warnings) |
| `cargo test --workspace` | Pass — **137 tests** |
| Validator on the 100-story library | Pass — `0 error(s), 3 warning(s)` |

Test breakdown:

| Suite | Tests | Result |
|-------|-------|--------|
| `dutchreader` library unit tests | 130 | Pass |
| `tests/atomic_kill.rs` (real process kill mid-write) | 2 | Pass |
| `validator` fixture/CLI tests | 5 | Pass |
| desktop unit tests | 0 | — |
| doc tests | 0 | — |

The three validator warnings are `ASSUMED_KNOWN` for `a2-01` (44 words),
`b1-01` (109 words), and `b2-01` (173 words). They are expected because those
three samples were written before the stories that would introduce their
background vocabulary. All four shelves are complete and ordered, and no
story produces a coverage or repetition warning. The broken fixtures in
`tests/fixtures/` still fail with their expected diagnostics (covered by the
5 validator tests).

The Tapped words follow-up added six library tests (`dictionary::tapped`),
raising the library suite from 124 to 130. The total is now 137. The follow-up
was re-verified with `cargo fmt`, clippy, the full test suite, and the device
checks below.

Coverage explicitly required by 5.1 is present: atomic write recovery, page
timer 29/30 s and reset rules, dictionary dedupe, lookup fallback and cache,
exact share text, draft autosave, stats/streak edge cases, pagination with no
lost or duplicated words, and occurrence-aware glossary lookup. Local-date
midnight, month-boundary, and DST cases were added with the time-zone fix.

### Release APK

Built with a throwaway test keystore:

```powershell
$env:CARGO_APK_RELEASE_KEYSTORE = "...\dutchreader-release-test.jks"
$env:CARGO_APK_RELEASE_KEYSTORE_PASSWORD = "dutchreader-test"
cargo apk build --release
```

| Property | Result |
|----------|--------|
| Output | `target\release\apk\DutchReader.apk` |
| Size | 21,596,049 bytes (20.60 MiB) |
| ABIs | `arm64-v8a`, `x86_64` |
| Permissions | `android.permission.INTERNET` only |
| Signature | v2 and v3 (APK Signature Scheme); v1 disabled |
| Signer | `CN=DutchReader Test` (throwaway key) |
| Orientation | portrait |
| Activity `configChanges` | `orientation`, `keyboardHidden`, `screenSize`, `uiMode` (no `density`/`fontScale`; see bug log #7) |
| Stray assets | none (no `.bak` files) |

Without a release keystore, `cargo apk build --release` fails with
`Configure a release keystore via [package.metadata.android.signing.release]`.
A real keystore is required for shipping; the test key is not for distribution.

---

## 5.2 Story content verification

| Story | New words | Non-starter words glossed | Min screens (both reference phones) | New word ≥ 2× | No new word in an earlier story |
|-------|-----------|---------------------------|-------------------------------------|---------------|----------------------------------|
| A1-01 De rode bal | 20 | Yes | Pass | Pass | Pass |
| A2-01 De sleutel is weg | 20 | Yes | Pass | Pass | Pass |
| A2-02 to A2-25 (24 stories) | 20 each | Yes | Pass | Pass | Pass |
| B1-01 De laatste stroopwafel | 20 | Yes | Pass | Pass | Pass |
| B1-02 to B1-25 (24 stories) | 20 each | Yes | Pass | Pass | Pass |
| B2-01 Het sollicitatiegesprek | 20 | Yes | Pass | Pass | Pass |
| B2-02 to B2-25 (24 stories) | 20 each | Yes | Pass | Pass | Pass |

Word counts (sections joined): A1 650-697, A2-01 622, A2-02 to A2-25
657-849, B1-01 846, B1-02 to B1-25 850-1,028, B2-01 1,053, B2-02 to B2-25
1,122-1,288. A1, A2-02 through A2-25, B1-02 through B1-25, and all B2 stories
sit inside the brief's ranges; A2-01 (622 vs 650-850) and B1-01 (846 vs
850-1100) are slightly below, recorded as a deviation.

- **TTS flow:** Listen spoke each page sentence by sentence, highlighted the
  current sentence, followed the speed slider, survived page turns, stopped on
  leaving the reader, and reopened on a later page. Natural-flow judgement is
  left to a human reader.
- **A1-01 constraint:** exactly 20 new lemmas; every other text word is a
  starter word or a proper name (checked by the validator).
- **Difficulty step:** the validator's word-debt counts rise from A1 to B2
  (0 / 44 / 109 / 173 assumed-known words), consistent with a graded step.
- **Copy-a-story test:** added `assets/stories/a1/02-copy-test.json`, rebuilt,
  and the A1 level screen showed `STORY 02` with no Rust or Slint change. The
  file was then removed and the final debug APK rebuilt.

---

## 5.3 On-device functional test script

Run on the emulator at 1080×2400 @420 unless noted.

| Area | Check | Result |
|------|-------|--------|
| Home | Progress counts (`1 of 1 story` per level, completed state) | Pass |
| Home | All shelves show 25 stories each (A1/A2/B2 `0 of 25`, B1 `1 of 25` from the stored resume; 100 in total) | Pass |
| B1 shelf | Stories 01-25 listed with title, blurb, and page counts (15-18 pages) | Pass |
| B1 reader | Story 24 `De reünie` opens at `Page 1 of 18`; last page is `Page 18 of 18` | Pass |
| B1 glossary | Tapping `oudleerling` shows `de oudleerling`, `Story glossary`, and `former pupil` | Pass |
| B2 shelf | Stories 01-25 listed with title, blurb, and page counts | Pass |
| B2 reader | Story 25 opens with pagination; glossary shows `de`/`het` and the new-word badge | Pass |
| Home | Continue Reading card opens the stored resume position | Pass |
| Home | Write / Dictionary / My Writing / Settings route correctly | Pass |
| Level | Ordered story cards, page counts, in-progress/completed state | Pass |
| Reader | Page turns left/right, middle overlay toggle | Pass |
| Reader | Word tap shows meaning; zone tap turns page (no double action) | Pass |
| Reader | Page indicator and completed marker | Pass |
| Reader | Rapid right/left arrow taps reach page 6 / page 1 with no stuck state | Pass |
| Reader | 30-second page rule adds glossary words once | Pass |
| Lookup | Offline glossary meaning | Pass |
| Lookup | Online MyMemory (`Machine translated`) | Pass |
| Lookup | Cached offline (`Machine translated · cached`) | Pass |
| Lookup | Airplane-mode fallback with note | Pass |
| Lookup | Slow network shows a loading state without freezing the UI | Pass |
| Dictionary | Words from a 30-second page, dedupe, local date | Pass |
| Dictionary | Search, A–Z / Newest sort, empty search state | Pass |
| Dictionary | Tapped words tab saves a lookup with meaning, story, date, source | Pass |
| Dictionary | `form of:` base line shown as a note under the meaning | Pass |
| Dictionary | Repeat tap increments the count instead of duplicating | Pass |
| Dictionary | Tapped words survive force-stop and app restart | Pass |
| Dictionary | Tab resets to All words on launch; empty state per tab | Pass |
| Settings | Dark theme applies live and persists | Pass |
| Settings | TTS speed `1.9x` persists | Pass |
| Settings | Meanings source `Online` persists | Pass |
| TTS | Start / Stop, sentence highlight | Pass |
| TTS | Survives page turn; stops on leaving reader | Pass |
| TTS | Reopens on page 4 with Listen | Pass |
| Editor | Empty state, local title placeholder, 0-word count | Pass |
| Editor | Share disabled note while empty | Pass |
| Editor | Cursor and text area stay above the keyboard | Pass |
| Editor | IME suggestions shown but no autocorrect; text stayed lowercase | Pass (see limitation) |
| Share | Detail Share opens the Android chooser | Pass |
| Share | Preview shows quoted story, blank line, exact question, no title | Pass |
| Share | Chooser copy pasted into the editor matches exactly | Pass |
| Share | Editor Share saves the story and opens My Writing detail behind the chooser | Pass |
| My Writing | List, detail, Copy toast, re-share | Pass |
| My Writing | Edit updates in place without a duplicate | Pass |
| My Writing | Delete confirmation; Cancel keeps, Delete removes | Pass |
| My Writing | Stats return to 5 stories / 26 words after deleting the test story | Pass |

**Final re-run (2 Nov 2026, after the display-scale fix):** app data was cleared
and the script was repeated on the rebuilt debug APK: Home/Level/Reader,
page 1 → 6 → 1 by rapid taps, offline and online lookup, airplane-mode fallback,
TTS start/stop with highlight, Dictionary timer entries/search/empty state,
Settings dark/light, Editor typing and Share chooser, My Writing copy and
delete (Cancel and Delete), draft force-stop/Continue, rotation request,
small/large screens, and both themes on every screen. All passed.

**B2 shelf re-run (10 Nov 2026, after completing the 100-story library):** the
debug APK was rebuilt with all 25 B2 story files, reinstalled over the existing
app data, and launched on the same emulator. The startup validator logged
`100 stories, 0 error(s), 3 warning(s)`. Home showed 25 stories per shelf
(A1/A2/B2 `0 of 25`; B1 `1 of 25` from the stored resume). The B2 shelf listed
stories 01–25 with page counts (7 for the `b2-01` sample, 17–20 for the
generated stories). `b2-25` opened as `B2 · De promotie`; tapping `promotie`
showed `de promotie`, the `Story glossary` badge, and `promotion`, and tapping
`vooraanstaand` showed the adjective meaning without an article. No
`FATAL EXCEPTION`, `ANR in com.dutchreader.app`, or `panic` appeared in logcat;
`dumpsys meminfo` reported PSS 99,442 kB / RSS 185,560 kB for the session.

---

## 5.4 Robustness and edge cases

| Check | Result |
|-------|--------|
| Force-stop and relaunch during reading; progress intact | Pass |
| Emulator restart (unexpected, during the pass); data intact on relaunch | Pass |
| Kill the app during writing; `draft.json` intact | Pass |
| Force-stop/relaunch offers **Continue your draft?** and restores text | Pass |
| **Start new** opens a clean editor | Pass |
| Corrupt `dictionary.json` → recovered from `.bak` (34 words, log warning) | Pass |
| Corrupt dictionary main + `.bak` → empty dictionary, no crash | Pass |
| Corrupt `settings.json` main + `.bak` → defaults (System, `0.9x`, Story glossary), no crash | Pass |
| Corrupt `progress.json` main + `.bak` → no Continue Reading, 0 progress, no crash | Pass |
| Corrupt `writing/stories.json` → recovered 5 stories from `.bak` | Pass |
| Corrupt stories main + `.bak` → 0-story empty state, no crash | Pass |
| Corrupt `writing/draft.json` main + `.bak` → empty editor, no prompt, no crash | Pass |
| Rotation requested while locked portrait → activity stayed `ROTATION_0` | Pass |
| Small screen `wm size 720x1280`, density 320 (foreground) → all screens render; home scrolls; editor usable with keyboard; page count 6 → 10 | Pass |
| Large screen `wm size 1440x3200`, density 560 → all screens render; page count stays 6 | Pass |
| Backgrounded, then `wm size 720x1280` + `wm density 320`, then resumed → activity recreated, correct 720-wide layout, no clipping, 10 pages | Pass (fixed in bug log #7) |
| Draft survives that recreate → `Continue your draft?` restores the text | Pass |
| Display-cutout emulation (`tall`, 126 px top inset) → Home and Reader content stayed below the cutout; no clipped controls | Pass |
| `wm size reset` / `wm density reset` → 1080×2400 @420 | Pass |

The background-resume case first failed (clipped layout, 16 pages). The cause was
a cached display scale factor in Slint's software renderer combined with
`density|fontScale` in the activity's `config_changes`. Removing those two flags
lets Android recreate the activity on a density change; see bug log #7. This is
also why the activity can restart after a display-density change while a draft
is open: the draft is read back from disk.

### Long text and editor behaviour

- Seeded a 5,040-word (~26.8 KB) draft. The prompt showed `5040 words`; the
  editor opened, scrolled, and autosaved without freezing; force-stop/relaunch
  restored the text.
- Typed `hbd`: the IME showed composing suggestions but did not auto-replace;
  after committing, only the typed word remained and capitalisation stayed
  lowercase. This confirms the Slint `input-method-hints` settings
  (`auto-correct: false`, `auto-complete: false`, `capitalization: none`).
- **Known limitation:** Gboard still draws its suggestion strip. Slint 1.18.1
  exposes no `TYPE_TEXT_FLAG_NO_SUGGESTIONS` hint, so the app cannot hide the
  strip. Autocorrect itself stayed off.
- Glide-typing artefacts appeared only when a test swipe started on the Gboard
  keyboard; that is a test-harness artefact, not an app defect.

### Storage recovery detail

The atomic write layer keeps `<file>.bak`. Corrupting the main file makes the
loader fall back to the backup and log a warning; corrupting both yields the
documented empty/default state. No corruption case produced a crash, `FATAL
EXCEPTION`, or `panic`.

---

## 5.5 Performance

| Check | Result |
|-------|--------|
| Screen changes and page turns | Instant; no animation to profile |
| No tap swallowed by a timing guard | Pass (rapid navigation test) |
| Cold start | `TotalTime: 631 ms` (final pass; earlier pass 657 ms) |
| ANR in lookup / TTS / share | None observed |
| Memory after the session | `TOTAL PSS: 80469`, `TOTAL RSS: 163008` (an earlier long session measured 83920 / 165868) |
| `ANR in com.dutchreader.app`, `FATAL EXCEPTION`, `panic` in logcat | None |

---

## 5.6 Fix loop

Bugs found and fixed are recorded in [`bug-log.md`](bug-log.md):

1. Critical — dates/statistics used UTC instead of the phone's local zone
   (including the Java `getOffset` millisecond/seconds conversion).
2. Major — editor default title for an existing draft used the wrong date.
3. Minor — My Writing empty-state gap.
4. Minor — `.bak` files left in packaged `assets/`.
5. Minor — unreadable status-bar icons on the light theme (accent strip
   workaround; the JNI attempt was reverted).
6. Major — stale display scale after a size/density change while backgrounded
   (clipped layout, wrong page count); fixed by letting Android recreate the
   activity on density/font-scale changes.

After the last fix (bug log #7) the static suite (5.1), the validator, the release
build, and the full on-device script were re-run from cleared app data on
2 Nov 2026. No open critical or major bugs remain.

---

## Extra checks requested for this pass

| Request | Result |
|---------|--------|
| Editor autocorrect/suggestions and several-thousand-word text | Pass, with the Gboard suggestion-strip limitation above |
| Kill the app during writing | Pass; draft intact and restored |
| Corrupt storage by hand | Pass; recovers or starts clean, no crash |
| Rapid navigation | Pass; no stuck page or lost tap |
| Online lookup slow / airplane mode | Pass; loading state, fallback note, cache |
| Release APK permissions, size, signing | Pass; INTERNET only, both ABIs, v2/v3; a real keystore is still required to ship |

---

## Known limitations and deviations from `APP_BRIEF.md`

1. **Gboard suggestion strip.** The editor disables autocorrect, autocomplete,
   and auto-capitalisation through `input-method-hints`, but the IME still shows
   suggestion chips. Slint has no flag to suppress them. Autocorrect stayed off.
2. **Personal Dictionary includes function words.** The brief says filler words
   should be excluded, but all glossary words are added, so `de`, `het`, `een`,
   and `en` appear (103 words in the test state). This was chosen so every
   glossary word is reviewable; it is a deviation from the "filler words" note.
3. **A2 and B1 story lengths.** A2-01 is 622 words (brief: 650-850); A2-02
   through A2-25 are 657-849 words and are in range. B1-01 is 846 words
   (brief: 850-1100) and is slightly below. A1 and all B2 stories are in range
   (B2-02 through B2-25 are 1,122-1,288 words).
4. **TTS highlights sentences, not individual words.** This matches the
   `ARCHITECTURE.md` sentence-level reader but differs from a word-by-word
   karaoke reading.
5. **Middle tap toggles the overlay.** `APP_BRIEF.md` lines 110 and 113 conflict
   (middle tap toggles controls vs. turns the page); the overlay toggle was
   chosen, matching `ARCHITECTURE.md`.
6. **Editing a saved story and pressing Back keeps the changes as a draft.**
   Tapping Share updates the saved story. Back does not silently discard edits.
7. **`ASSUMED_KNOWN` warnings.** Expected because A2-01, B1-01, and B2-01
   were written before the stories that would introduce their background
   vocabulary. The generated shelves never use that debt, so the 100-story
   library still defaults to allow-gaps mode; strict mode becomes the default
   once the three samples are cleaned up.
8. **Status-bar icon appearance** is a visual workaround (accent strip in light
   mode), not a true system-icon-colour API fix.
9. **Release signing.** A real keystore is required; the test key is not for
   shipping.
10. **No physical device.** All on-device results are from the emulator, and the
    60 fps target was not measured.
11. **Auto-discovery needs a rebuild.** Story files are embedded at compile time
    (Android assets cannot be listed at runtime), so dropping a JSON file is
    enough but `cargo apk build` must be re-run. No Rust or Slint change is
    needed.
12. **New-word repetition.** The validator enforces each new word appearing at
    least twice; the brief's "2-3 times" is treated as a minimum of two.

---

## Build note

The release APK was rebuilt after the final status-bar change and verified:
21,596,049 bytes (20.60 MiB), `INTERNET` only, both ABIs, v2 + v3 signatures,
signer `CN=DutchReader Test`. The on-device pass used the debug APK built from
the same source; the release APK was verified statically because it is signed
with a different key and installing it would have required wiping the restored
test data.
