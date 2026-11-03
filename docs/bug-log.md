# DutchReader — Phase 5 bug log

Phase 5 was a verification pass, so this log records every defect found while
running the checks in `ARCHITECTURE.md` 5.1–5.6 and the follow-up review.
Severity follows the Phase 5 rule:
**critical** = data loss, crash, or a wrong result the user would act on;
**major** = a visible functional failure with a workaround or a clearly wrong
display; **minor** = cosmetic or documentation-only.

All critical and major defects are fixed. The full automated suite and the
on-device script were re-run after the last fix.

| # | Severity | Area | Status |
|---|----------|------|--------|
| 1 | Critical | Local dates / statistics | Fixed, verified |
| 2 | Major | Editor default title | Fixed, verified |
| 3 | Minor | My Writing empty state | Fixed, verified |
| 4 | Minor | Story validator assets | Fixed, verified |
| 5 | Minor | Android status-bar icons (light theme) | Worked around, verified |
| 6 | — | Android status-bar icon JNI attempt | Reverted (not shipped) |
| 7 | Major | Display scale after a background density change | Fixed, verified |

---

## 1. Critical — dates and statistics used UTC instead of the phone's time zone

**What.** Story timestamps, the default story title, the My Writing list, the
per-month totals, and the streak all used UTC civil dates. The Phase 4 code
converted a Unix timestamp to a UTC calendar date with a small hand-written
routine.

**Where.** `src/writing/stats.rs`, `src/dictionary.rs` (dictionary entry dates),
and the date/title paths in `src/app.rs`.

**How to reproduce.** Set the phone to a positive-offset zone (for example
`Africa/Cairo`, UTC+2/+3 with DST) and write a story after local midnight but
before UTC midnight. The list showed the previous day, the default title showed
the previous date, and a story written late in the evening could land in the
next UTC day for negative offsets. Around month boundaries this also moved a
story between month buckets and could break a streak.

**Why it mattered.** The brief asks for local dates and a local streak. A learner
writing at 00:30 local time saw "yesterday", and month totals disagreed with the
phone's calendar.

**Fix.** Dates are now computed in the phone's local time zone:

- `src/time.rs` holds the shared civil-date maths. Every function takes the UTC
  offset that applies **at that timestamp**, so DST transitions are handled per
  entry rather than with one fixed offset for the whole list.
- `src/platform/timezone.rs` asks Android for the offset through JNI:
  `java.util.TimeZone.getDefault().getOffset(millis)`. The desktop preview uses
  `chrono::Local`.
- Stats, dictionary dates, and the display title take an offset parameter, so
  each stored timestamp is converted with its own offset.
- If the JNI lookup fails, the app falls back to offset 0 (UTC) and keeps
  working instead of crashing.

**Second defect found while fixing this (same bug family).** Java's
`TimeZone.getOffset(long)` returns **milliseconds**, not seconds. The first
implementation used the raw value as if it were seconds, which shifted dates by
months. `src/platform/timezone.rs` now divides by 1 000, and a unit test pins the
millisecond-to-second conversion.

**Verification.**

- Unit tests in `src/time.rs` cover midnight boundaries, month boundaries, and a
  DST transition (a fixed instant before and after the clock change gets
  different offsets).
- On the emulator (`Africa/Cairo`, device date 31 Oct 2026) the My Writing list
  showed `31 Oct 2026 01:30`, `30 Oct 2026 01:30`, `29 Oct 2026 00:30` (DST +3),
  `1 Oct 2026 02:30`, and `1 Sep 2026 01:30`; month rows were `Oct 2026`
  (4 stories, 21 words) and `Sep 2026` (1 story, 5 words) with a `3 days`
  streak.
- An untitled story created from the editor showed the local date
  `1 Sep 2026` as its fallback title.

## 2. Major — default title for an existing draft used the wrong date

**What.** When the editor reopened an existing draft, the empty title
placeholder was computed from `max(now)`, so it showed the current date instead
of the date the draft was created. A draft started before midnight and reopened
after midnight displayed the new day, which did not match the story that would be
saved.

**Where.** `src/app.rs` (editor open / placeholder wiring) and
`ui/screens/editor.slint`.

**How to reproduce.** Start a draft on day 1, leave the editor, change the phone
date to day 2 (or wait past midnight), reopen the draft. The title placeholder
showed day 2 while the draft's `created_at` was day 1.

**Fix.** The placeholder uses the draft's creation date (converted with the local
offset for that timestamp), not the current time. New drafts still use the
current local date.

**Verification.** On device the placeholder showed `31 Oct 2026` for a new draft
and the draft's own creation date after reopening; the saved story's list
timestamp and the fallback title agreed.

## 3. Minor — gap under the empty state on My Writing

**What.** With no saved stories, the empty-state block did not fill the remaining
height, leaving an uneven band under it.

**Where.** `ui/screens/writing.slint`.

**Fix.** Added a trailing `Rectangle { vertical-stretch: 1; }` so the layout
consumes the leftover space.

**Verification.** On device the empty state filled the screen at 1080×2400 and
at the reduced 720×1280 size.

## 4. Minor — `validate_stories --write-wordlist` left `.bak` files in `assets/`

**What.** Rewriting `assets/wordlist.json` and `assets/assumed_known.json`
created atomic-write backups (`*.bak`) inside the packaged `assets/` tree. Those
files were then embedded in the APK and shipped as stray assets.

**Where.** `validator/src/main.rs`.

**Fix.** After a successful `--write-wordlist` run the validator removes the
backup files it created.

**Verification.** `cargo run -p validate_stories -- --allow-gaps` printed
`0 error(s), 3 warning(s)` and `Get-ChildItem assets -Recurse -Filter *.bak`
returned nothing.

## 5. Minor — status-bar icons were unreadable on the light theme

**What.** Slint's Android backend draws edge to edge and does not expose
status-bar icon appearance. Android keeps the status-bar icons white, so on the
light theme's cream background the clock and battery icons were nearly invisible.

**Where.** `ui/app.slint` (app shell).

**Fix (workaround).** In light mode the shell paints an accent-coloured strip
behind the status-bar inset; the white system icons stay readable, and dark mode
needs no strip. This is a visual workaround, not a system-icon-colour API fix.

**Verification.** Home and reader were checked in both themes: light mode showed
the accent strip with readable icons, dark mode showed no strip and white icons
on the dark background.

## 6. Not shipped — JNI status-bar icon attempt

The first attempt at #5 kept the activity pointer and called
`View.setSystemUiVisibility` through JNI to request dark icons. It failed on
device with:

```
CalledFromWrongThreadException: Only the original thread that created a view
hierarchy can touch its views. Expected: main Calling: android_main
```

`android_main` is not the Android UI thread, so the call was unsafe. The module
and its callbacks were removed and replaced with the accent strip described in
#5. Nothing from the failed attempt ships.

## 7. Major — stale display scale after changing size/density while backgrounded

**What.** If the display size or density changed while DutchReader was in the
background, resuming drew the whole UI at the old scale: the right and bottom of
every screen were clipped (the Home `Light` button was half off-screen) and the
page count was wrong (`16 pages` at 720×1280 instead of `10`). Resizing while the
app was in the foreground relaid out correctly, so only the background-resume
path was affected.

**Where.** `Cargo.toml` (`android.application.activity.config_changes`) plus
Slint's Android software renderer, which caches the display scale factor for the
life of the activity.

**How to reproduce.** Launch the app at 1080×2400 @420, press Home, run
`adb shell wm size 720x1280` and `adb shell wm density 320`, then reopen the app.
The activity was not recreated (because `density`/`fontScale` were listed in
`config_changes`), so Slint kept the 420 dpi scale factor and laid out a
1080-wide surface inside a 720-wide window.

**Fix.** Removed `density` and `fontScale` from `config_changes` so Android
recreates the activity when either changes; a recreate re-reads the scale
factor. `orientation`, `keyboardHidden`, `screenSize`, and `uiMode` stay in the
list, so rotation, the keyboard, multi-window resizes, and theme changes still
do not recreate the activity. Draft autosave means a recreate is safe: the draft
was still on disk and the `Continue your draft?` prompt restored it.

**Verification.** Rebuilt the debug APK, backgrounded it, changed to
720×1280 @320, and resumed: Home rendered at the correct width with no clipping,
the page count changed from 6 to 10, and the editor opened with the keyboard,
accepted text (`hallo`, `1 word`), and kept the text after a second
density-change recreate. Returning to 1080×2400 @420 restored the 6-page count.
The keyboard does not change density, so opening it does not recreate the
activity.

---

## Checks that found no defect

The following were exercised without finding a critical or major bug; details
and results are in `docs/test-report.md`:

- atomic write recovery, including a real process-kill test;
- page timer at 29/30 seconds and its reset rules;
- dictionary dedupe, search, sort, and date display;
- lookup offline / online / cached / airplane-mode fallback and the slow-network
  loading state;
- exact share text and title exclusion;
- draft autosave, force-stop, and kill during writing;
- corrupt `dictionary.json`, `settings.json`, `progress.json`,
  `writing/stories.json`, and `writing/draft.json` (main and backup);
- rotation and small/large screen sizes;
- rapid reader navigation and tap-zone handling;
- release APK permissions, size, and signature.
