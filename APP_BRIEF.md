Build an Android-only app called DutchReader in the folder Documents/Code/DutchReader, using Rust and Slint. It is a Dutch reading app for learners, with graded stories from A1 to B2. All user data is stored locally on the device, with no cloud and no accounts. The target user is a near-complete beginner (slightly below A1) who wants to learn Dutch by reading and by writing.

## Scope for this first pass
- Design and build the full app and a modular story system.
- Write exactly ONE story per level (A1, A2, B1, B2), 4 in total, so I can test everything.
- Do NOT write the other stories. Later I will add them myself, so adding a story must only require dropping in a new story file, with no code changes.
- Write a STORY_GUIDE.md (see below) that fully documents how every story must be made, so that any AI or person can later write the remaining 96 stories consistently.

## Content model (the long-term plan the system must support)
- 4 levels: A1, A2, B1, B2, with 25 stories each (100 total).
- Stories within a level get slightly harder one after another. A1 story 1 is the easiest and A1 story 25 is the hardest in A1. The jump from the last story of one level to the first of the next must also be gentle.
- Every story introduces exactly 20 new words (words that have not appeared in any earlier story). That is 2,000 new words across the 100 stories.
- Stories must render as at least 5 screens and should be fairly entertaining while fitting their level's vocabulary and grammar. Pages are computed from the text, not authored.
- For the 4 sample stories, write original, entertaining Dutch that follows STORY_GUIDE.md exactly. A1 story 1 is the easiest story in the whole app and is for a near-absolute beginner. The A2, B1 and B2 samples are story 1 of their level.

## Modular story system
- Each story is a separate data file (JSON or similar) in an assets/stories/<level>/ folder, with a manifest or auto-discovery so new files appear automatically.
- A story file contains: id, level, order number (1-25), title, short blurb, sections (an array of short text paragraphs), a list of the 20 new words (marked as "new" for this story), and a glossary.
- The glossary is bundled in the story file and works offline. It maps each word or form that appears in the story to its English meaning (include the base form, e.g. "liep" -> "walked (lopen)"). By default, tapping a word looks it up in this glossary.
- Visually mark the 20 new words of the story in the reader (e.g. a subtle underline or tint) so the learner can see what is new.
- Include a validation step or a clear schema doc so I can add stories later, with a startup check or script that warns about: words missing from the glossary, a story that does not have exactly 20 new words, words flagged "new" that appeared in earlier stories, and a story too short to fill 5 screens.
- Keep a cumulative word list per level (or global) that records which words were introduced in which story. This is how new-word checks and the validation work.

## STORY_GUIDE.md (required deliverable, lives in the app's repo)
Write a detailed STORY_GUIDE.md that tells a future story writer (an AI or a human) exactly how to write stories for this app. It must cover everything below.

### Purpose and learner
- The app teaches Dutch to a near-complete beginner through reading. Stories must be genuinely entertaining, because the learner reads them for fun, while staying inside the level's limits.
- Target total vocabulary: A1 ~600-800 words, A2 ~1,500, B1 ~2,500-3,000, B2 ~4,000+. Each story adds 20 new words, so the vocabulary the stories add is about 2,000 words in total; the guide must explain how the writer stays on a consistent curve toward this.

### Story structure
- At least 5 computed screens per story at one fixed reader font size. Pages fill the screen; only the last page may be partly filled.
- Length is measured in total words: A1 ~600-750, A2 ~650-850, B1 ~850-1,100, B2 ~1,050-1,300. Within a level, early stories are shorter and later stories longer.
- Every story has a clear mini-arc: a setup in the first ~20%, a small problem or surprise in the middle, and a satisfying or funny ending in the last ~20%.
- Each story has a title and a one-or-two-sentence blurb in simple Dutch (and an English version for the story list if useful).

### Difficulty progression
- Story N is slightly harder than story N-1 within a level. Increase difficulty gradually through sentence length, grammar complexity, vocabulary rarity, and total story length, never all at once.
- A1 story 1 is extremely simple: present tense, very short sentences, high-frequency words, tons of repetition, and cognates where possible. A1 story 25 is the hardest A1 story but is still A1.
- Give concrete progression guidance by level:
  - A1: present tense (zijn, hebben, basic regular verbs, modal verbs like willen/kunnen), simple word order, basic questions, numbers, colors, family, food, daily routine, greetings, places in town. Very short sentences (3-8 words). Introduce simple separable verbs and negation (niet/geen) later in A1.
  - A2: past tense (perfectum first, then simple past), separable verbs, more connectors (omdat, maar, want, dan), comparatives, reflexive verbs, dialogue, common daily-life situations (shopping, travel, work, health). Sentences up to ~12 words.
  - B1: mixed tenses, subordinate clauses with correct verb-final word order, opinions and reasons, longer descriptions, some idioms, more abstract topics, plots with more characters.
  - B2: complex sentences, passive voice, conditional and subjunctive-like forms (zou, had...), formal and informal registers, idiomatic expressions, nuance and humor, more abstract and varied topics.

### The 20 new words per story
- Each story introduces exactly 20 new words, meaning words that have not appeared in any earlier story. Count lemmas (dictionary forms), not every inflected form. A new word's conjugations and plural forms later do not count as new.
- Choose new words so that they are useful (high-frequency first), fit the story's topic naturally, and mix word types (nouns with their article de/het, verbs, adjectives, a few function words), never only nouns.
- Every new word should appear at least 2-3 times within the story that introduces it, so the learner sees it in context and repetition starts inside the story.
- Previously introduced words must be recycled in later stories, so vocabulary is reinforced. A good rule: at least 85-90% of the running words in a story should be words the learner has already met, with the 20 new words making up the rest.
- Nouns are always glossed with their article (de/het) because Dutch gender is hard to guess.
- Don't use rare or off-level words outside the 20 new ones. If a story needs a name or a unique word, keep it minimal and gloss it.

### Glossary requirements
- Every word that appears in the story must be in the glossary, including inflected forms (e.g. "liep" -> "walked (lopen)"), separable verbs ("zet ... op" -> "puts on (opzetten)"), and fixed expressions where it helps.
- Each glossary entry: surface form, base form (lemma), part of speech, article for nouns, and a short English meaning that fits the story's context.
- The 20 new words are flagged as new for this story.

### Style and content
- Entertaining: use humor, mystery, small adventures, relatable everyday situations, likeable recurring characters and mini cliffhangers. A mix of genres across the 25 stories keeps the learner interested (slice of life, mystery, comedy, light fantasy, travel, etc.).
- Natural Dutch in the Netherlands standard variety. Avoid translation-ese. Keep cultural touches (bikes, weather, food like stroopwafels and kaas, King's Day, markets) because they make the learning feel authentic.
- Appropriate for all ages. No hateful, explicit, or disturbing content.
- Use dialogue from A1 onward to show spoken language and punctuation.
- Avoid dense walls of text. Short paragraphs make pages easier to read.

### File format and workflow
- Document the exact JSON schema, with a full annotated example, and the folder and file naming convention (assets/stories/<level>/<nn>-<slug>.json).
- Document the validation checks and how to run them.
- Include a checklist the writer must pass before adding a story: exactly 20 new words, all non-starter words in the glossary, at least 5 screens, difficulty slightly above the previous story, new words repeated within the story, old words recycled, and the story is entertaining.
- Include a ready-to-use writing prompt template for an AI so that I can generate the remaining 96 stories one at a time. The template takes: level, story number, the cumulative list of previously introduced words, and the previous story, and asks for the new story plus its glossary and 20 new words in the correct JSON format, including form-specific meanings and optional per-occurrence entries for ambiguous forms.

## Screens and flow
1. Home screen: level cards for A1, A2, B1, B2 (with progress, e.g. stories completed), a prominent "Write" button that opens the writing editor (see "Writing feature"), plus entry points to the Personal Dictionary, My Writing, and Settings.
2. Level screen: the list of stories for that level in order (story 1, 2, ...), showing title, blurb, computed page count, progress/resume position, and a completed state.
3. Reader screen: the story text, one computed screen at a time.
4. Personal Dictionary screen: all learned words with meanings, searchable and sorted (alphabetical or newest first).
5. Writing editor screen: the distraction-free editor (see "Writing feature").
6. My Writing screen: the history of everything I've written, with monthly stats (see "Writing feature").
7. Settings screen: dark mode (light, dark, optionally follow system), TTS speed, and a "Word meanings source" option:
   - "Offline glossary (default)": uses only the glossary bundled in the story file.
   - "Online dictionary": uses an online API.

## Writing feature
The core idea: I tap one button, write my own Dutch story, tap Share, pick ChatGPT (or any app) from the Android share sheet, and send it to get my mistakes checked. It is a quick, simple routine, so the flow must have as little friction as possible. There is no timer or time limit of any kind.

### Editor
- The "Write" button on the home screen opens a clean, distraction-free text editor with a large, comfortable, readable text area, using the same typography quality as the reader, with light and dark themes. It must work well with the Android keyboard (the text area scrolls and keeps the cursor visible above the keyboard, and the toolbar stays reachable).
- Show a live word count. Quietly track how long I spent in the editor (only while it is open and the app is in the foreground), without showing any countdown or timer UI while I write. This is only saved with the story for statistics.
- Optional title field (default: the date). Disable autocorrect, autocomplete, and prediction where possible, so my mistakes stay visible for checking.
- Auto-save the draft continuously (e.g. every few seconds and when the app goes to the background), so nothing is ever lost if the app closes. If I reopen the editor with an unfinished draft, offer to continue it or start a new one.

### Share
- A Share button in the editor opens the native Android share sheet (an ACTION_SEND intent with text/plain via a JNI bridge, wrapped in Intent.createChooser) so I can pick ChatGPT or any other app and send it.
- The shared text is built automatically in exactly this form, so I never have to type anything:

  "<my full story text>"

  Are there any grammar or vocabulary mistakes in this story, other than punctuation?

  That is: my story wrapped in double quotation marks on its own, followed by a blank line and then the question. The wording of the question should be exactly: Are there any grammar or vocabulary mistakes in this story, other than punctuation? Keep my paragraphs and line breaks intact inside the quotes. Don't include the title in the shared text.
- Sharing marks the story as finished and saves it. The Share button is disabled (with a hint) if the text is empty.

### Saving and statistics
- Every finished story is saved locally and permanently, with: the text, optional title, date and time, word count, and time spent writing. Drafts that are empty are never saved.
- The My Writing screen lists all stories I've written, newest first. Each entry shows title/date, a preview, word count, and time spent. Tapping opens the full text, where I can re-share (same formatted text), copy it, edit it, or delete it (with confirmation).
- Show clear statistics on this screen: the total number of stories written, a month-by-month overview (stories written per month, e.g. as a simple bar chart or list, with total words per month), and the current streak of days or weeks in which I wrote, so I can see how much I've written over the months.
- Writing data lives in its own storage file or table, separate from reader data, using the same safe-write approach as the rest of the app, and is never sent anywhere except when I use Share.

## Reader behavior
- Tap zones: tap the left side of the screen for the previous page, the right side for the next page, and the middle to toggle an overlay with options (TTS, back, page indicator).
- Page turning is instant: the next or previous page and the page indicator change in the same frame. There is no page curl, flip, slide, or fade.
- A top-right button toggles a TTS narrator that reads the story aloud in Dutch (nl-NL) using Android's native TextToSpeech through JNI. Highlight the word or sentence being read if feasible, and keep TTS across page turns. The default speed is `0.88`; the slider still goes from `0.5` to `2.0`, and a saved speed wins.
- Tapping a word opens a small popup or bottom sheet showing the word **as written** (for example `ben`), the form-specific meaning for that occurrence (for example `am`), and a secondary line with the base form (for example `form of: zijn, to be`). Tapping must not conflict with the left/middle/right page-turn zones, so a tap on a word shows its meaning and a tap on empty space turns the page.
- Typography is optimized for readability: a clean serif or highly legible font, a comfortable size, generous line height and paragraph spacing, comfortable margins, and a max line length. Both light and dark themes must have good contrast.

## Local persistence
- Save locally: last opened story and page, per-story progress and completion, settings (theme, TTS speed, meanings source), the cached online lookups, the personal dictionary, the writing draft, and all saved writings with their statistics.
- Resuming a story opens it on the last page.
- Use a simple, robust local format (JSON files in the app's private storage, or SQLite), with safe writes so data doesn't corrupt.

## Personal dictionary rules
- A page counts as read only if it stays open for at least 30 seconds. Run a timer per page that resets when I change page, leave the reader, or the app goes to the background, so that just browsing doesn't count.
- When a page reaches 30 seconds, add all glossary words on that page to the dictionary.
- Never add duplicates. If a word is already in the dictionary, skip it. Store word, meaning, and the story/level it was first learned in.

## Word meaning lookup modes
- The default mode is the bundled offline glossary. The app must be fully usable with no internet in this mode.
- If the user switches to "Online dictionary", a tapped word is looked up through an online Dutch-to-English API. Choose a free option that needs no API key if one is reliable (e.g. MyMemory), and put the provider behind a trait/interface so it can be swapped later without touching the UI.
- Online mode behavior:
  - Show a loading state in the popup while the request runs, with a short timeout.
  - Cache every result locally so the same word is never requested twice and works offline afterwards.
  - If the request fails (no internet, timeout, rate limit), automatically fall back to the bundled glossary and show a small note, so a tap never ends in an empty popup.
  - Label online results as machine translated so the user knows the source.
- The personal dictionary rules don't change. Words added after the 30-second rule come from the page's bundled glossary regardless of mode, so the dictionary stays clean and doesn't fill up with filler words like "de" and "het".
- Save the chosen mode in local settings.

## UI quality (no animations)
- It should look and feel like a modern app: clean layout, rounded cards, a cohesive color palette, good spacing and typography, and a polished dark mode.
- There are **no animations**: screen transitions, page turns, the overlay, the popup/bottom sheet, the progress bar, toasts, and editor entry/exit all appear and disappear instantly. Pressed cards and buttons may change colour instantly, but nothing moves, fades, slides, flips, or scales. Remove the animation code itself, not just set durations to zero, so no timing flag can swallow a tap or leave a stuck state.
- Respect Android system insets (status/navigation bars) and the on-screen keyboard, and support portrait orientation first.

## Technical requirements
- Rust and Slint only for the app, built for Android (use the Slint Android backend, cargo-apk or xbuild, or whatever is current and reliable). Android only, with no desktop or iOS targets needed beyond optional desktop preview for development.
- Clean modular architecture: separate modules for stories (loading/validation), storage, dictionary, word lookup (offline glossary and online provider), TTS bridge, share bridge (Android share sheet via JNI), writing (editor state, drafts, saved writings, statistics), reader logic (pagination, timers), and UI.
- Network access is needed only for the optional online mode. Request the INTERNET permission, use a lightweight HTTP client crate, and run requests off the UI thread so the interface never freezes. The writing feature itself needs no network, because sharing goes through the Android share sheet.
- Keep the code well organized and commented where non-obvious. Include a README with build/run instructions for a device or emulator.

## Deliverables
- A working app, buildable to an APK, with the 4 sample stories and the working writing feature.
- The modular story system and validator.
- STORY_GUIDE.md as specified above, including the JSON schema, the checklist, and the AI writing prompt template.
- At the end, a short summary of what's done, how to build/install it, and anything that couldn't be implemented as asked (e.g. any TTS, keyboard, or share-sheet limitations in Slint) along with what you did instead. The page flip was removed on purpose, not left as a limitation.
