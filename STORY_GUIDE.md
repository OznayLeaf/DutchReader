# STORY_GUIDE.md

**How to write a DutchReader story**

This guide is the complete specification for the 100 graded stories in DutchReader.
It is written for a future writer who has not read the app's architecture document:
everything you need is here. It is also the instruction set for an AI that writes
stories one at a time.

Read this whole document before writing. Then follow the workflow in
[§9](#9-validation-workflow) and the checklist in [§10](#10-pre-submission-checklist).

---

## Contents

1. [Purpose and learner](#1-purpose-and-learner)
2. [The vocabulary curve](#2-the-vocabulary-curve)
3. [Story structure](#3-story-structure)
4. [Difficulty progression by level](#4-difficulty-progression-by-level)
5. [The 20 new words](#5-the-20-new-words)
6. [The glossary](#6-the-glossary)
7. [Style and content](#7-style-and-content)
8. [File format and naming](#8-file-format-and-naming)
9. [Validation workflow](#9-validation-workflow)
10. [Pre-submission checklist](#10-pre-submission-checklist)
11. [AI writing prompt template](#11-ai-writing-prompt-template)
12. [Reference stories](#12-reference-stories)
13. [Appendix: coverage table and budgets](#13-appendix-coverage-table-and-budgets)

---

## 1. Purpose and learner

DutchReader teaches Dutch through reading. The learner is a **near-complete
beginner** (slightly below A1) who wants to learn by reading and by writing.
They are not studying for an exam: they read because the stories are fun. That
is the central tension of this project, and the writer's job is to satisfy both
sides of it:

- stay strictly inside the level's vocabulary and grammar, and
- write something a person would actually want to finish.

A story that is easy but boring fails. A story that is entertaining but full of
unknown words also fails, because the learner cannot read it. The rules below
exist to make "easy" and "entertaining" compatible.

The app ships 4 levels (A1, A2, B1, B2) with 25 stories each: 100 stories in
total. Every story is a separate JSON file. Adding a story must never require a
code change.

### The reader's experience

- One screen at a time. Pages are **computed from the text**, not authored: the
  app joins the story's sections and paginates them to fill the screen at one
  fixed reader font size. Only the last page may be partly filled.
- A story must render as **at least 5 screens** on a 1080x2400 phone and on a
  smaller 360dp-wide phone. There is no upper limit, but 5–7 screens is the
  comfortable range.
- Every word in the story is in the bundled glossary and works offline. The
  starter words (see [§5.9](#59-starter-words)) are already known and are in
  every story automatically.
- The 20 new words are visually marked as new; starter words are never marked.
- A page counts as "read" only after it stays open for 30 seconds; the glossary
  words on that page are then added to the learner's Personal Dictionary.
- Progress is saved as a **text offset**, not a page number, so resuming still
  works after the screen size changes and the text is repaginated.
- Each computed page should feel like a comfortable screen, not a wall of text,
  and each page should make the learner want to tap "next".

---

## 2. The vocabulary curve

### 2.1 The numbers

The learner's **overall** vocabulary grows roughly like this:

| Level | Overall vocabulary target | New lemmas taught by the 25 stories | Typical learner label |
|-------|---------------------------|--------------------------------------|-----------------------|
| A1 | about 600–800 words | 25 × 20 = 500 | Beginner |
| A2 | about 1,500 words | 25 × 20 = 500 | Elementary |
| B1 | about 2,500–3,000 words | 25 × 20 = 500 | Intermediate |
| B2 | 4,000+ words | 25 × 20 = 500 | Upper intermediate |

The stories teach about **2,000 new lemmas** in total (100 × 20). The rest of the
overall target comes from function words, cognates, names, and vocabulary the
learner meets elsewhere. Do not try to close that gap by adding extra new words
to a story: the 20-word rule is absolute (see [§2.4](#24-binding-spec-decisions)).

### 2.2 How to stay on the curve

- Use the cumulative word list (`assets/wordlist.json`) as the source of truth
  for what the learner already knows.
- Recycle old words constantly. A story should feel like it is *using* the
  learner's vocabulary, not just adding to it.
- Within a level, increase difficulty gradually (sentence length, grammar,
  page length, abstraction), never all at once.
- At the start of a new level, stay close to the end of the previous level for
  the first two or three stories, then ramp up.

### 2.3 Known-word coverage

Coverage means: of all the ordinary word tokens in the story (proper names
excluded), what percentage belongs to words the learner has already met?

The validator computes:

```
known_tokens   = all non-name tokens − tokens belonging to this story's 20 new words
coverage       = known_tokens / all non-name tokens
```

The long-term target is 85–90% known. That cannot hold in early A1: a
short A1 story with 20 new words repeated a few times has very few known words.
The validator therefore uses **level- and order-aware thresholds** and reports a
warning (never an error) when coverage is below the target:

| Level | Story 1 target | Story 25 target | Formula |
|-------|----------------|-----------------|---------|
| A1 | 0% | 80% | `0.80 × (order − 1) / 24` |
| A2 | 75% | 88% | `0.75 + 0.13 × (order − 1) / 24` |
| B1 | 82% | 90% | `0.82 + 0.08 × (order − 1) / 24` |
| B2 | 85% | 90% | `0.85 + 0.05 × (order − 1) / 24` |

A1-01 is a special case: its target is 0% because it is the first story. For
every other story, aim at or above the target. Coverage is a warning rather than
an error because a good story should not be rejected for a few percentage
points, but consistently low coverage means the story is too hard and must be
rewritten.

Starter words count as known for coverage. They are not new, they are not in the
assumed-known debt list, and every story has them from the first page (see
[§5.9](#59-starter-words)).

### 2.4 Binding spec decisions

These decisions resolve places where the requirements pull against each other.
They are binding for every story and every phase. They are repeated here so this
guide is self-contained.

1. **A1-01 has no earlier stories.** With nothing known yet, every word in
   A1-01 would be "new", which collides with "exactly 20 new words". Decision:
   the app has a fixed list of **starter words** that are known from the start
   (see [§5.9](#59-starter-words)). A1-01 introduces exactly **20 new words**;
   every other word must be a starter word or a proper name. It repeats those 20
   heavily. The 85–90% coverage rule is waived for A1-01.
2. **Coverage is token-based and depends on story length.** A short early story
   cannot be 85% known. Decision: the validator uses the level- and
   order-aware thresholds in [§2.3](#23-known-word-coverage). Below-threshold is
   a warning, not an error.
3. **The sample stories have gaps.** A2-01, B1-01, and B2-01 are written before
   the stories that should introduce their background vocabulary. Decision: the
   validator keeps running in **`--allow-gaps` mode** for the 100-story library.
   Words that are neither new nor previously introduced are reported as
   **"assumed known"** and collected in the **debt list**
   (`assets/assumed_known.json`). The debt belongs only to those three sample
   stories; the generated shelves never use it. Paying the debt down is separate
   cleanup work, after which `--strict` can become the default.
4. **Vocabulary targets vs. the 20-word rule.** The level totals in
   [§2.1](#21-the-numbers) describe the learner's overall level, not only what
   the stories teach.
   Decision: the 20-new-words rule is never relaxed. Stay on the curve through
   recycling and through the level's grammar and page-length progression.
5. **"Drop a file, no code changes" still needs a rebuild.** Story files are
   embedded in the APK at build time (Android cannot list assets at runtime).
   Decision: dropping a JSON file into `assets/stories/<level>/` and rebuilding
   is the whole workflow. No Rust or Slint source changes are needed.

---

## 3. Story structure

### 3.1 Screens and sections

You do **not** author pages. You author **sections**: an array of paragraph
strings. The app joins them with a blank line between sections, measures the
text with the bundled font at one fixed size, and computes pages that fill the
available screen height. The last page may be partly filled; every other page is
full.

- A story must render as **at least 5 screens** on both reference phones:
  a 1080x2400 phone (359x750 logical pixels of text) and a 360dp-wide phone
  (308x636). The validator computes this with the real layout engine and reports
  `MIN_SCREENS` below 5.
- There is no fixed page count and no page-count warning. Aim for 5–7 screens.
- Each section is one short paragraph, normally 1–3 sentences. Use `\n\n`
  inside a section only when you need a deliberate break within a paragraph.
- You cannot control exactly where a computed page breaks. Pagination splits at
  word boundaries, so a sentence may continue on the next page. Keep sentences
  and sections short enough that the break still reads comfortably.
- The first screen must hook the reader. The last screen must pay off the hook.

### 3.2 Story length

Length is measured in **total word tokens**, not words per page. These are the
targets that reliably produce at least 5 screens at the fixed reader font size:

| Level | Total words | Screens | Notes |
|-------|-------------|---------|-------|
| A1 | 600–750 | 5–6 | A1-01 is deliberately repetitive and sits near the top |
| A2 | 650–850 | 5–6 | 600+ words is the safe floor |
| B1 | 850–1,100 | 5–6 | longer sentences fill screens faster |
| B2 | 1,050–1,300 | 5–7 | dense prose still needs enough total text |

Within a level, early stories sit at the low end and later stories at the high
end. The four sample stories are calibrated this way:

- A1-01: 697 words, 62 short sections.
- A2-01: 622 words, 65 sections; A2-02 through A2-25 run from 657 to 849 words.
- B1-01: 846 words; B1-02 through B1-25 run from 850 to 1,028 words.
- B2-01: 1,053 words; B2-02 through B2-25 run from 1,122 to 1,288 words.

The validator is the final judge: if a story is below 5 screens, add real story
material (a scene, a beat of dialogue, a consequence), not filler.

### 3.3 The mini-arc

Every story needs a clear arc. Because pages are computed, think in
**proportions of the total text**, not page numbers:

1. **Setup (first ~20%):** a likeable character, a place, and a hook. Something
   is about to happen.
2. **Problem or surprise (~20–60%):** the plan goes wrong, a mystery appears, a
   character behaves unexpectedly. Small cliffhangers are welcome.
3. **Turn (~60–80%):** the character understands something or makes a choice.
4. **Payoff (last ~20%):** a satisfying, funny, or warm ending that resolves the
   hook. The last line should land.

Keep the cast small: one or two main characters plus a minor character is
enough. Recurring characters across a level are encouraged (the man and the dog
in A1/A2; Noor in B1), because they make the reader feel at home.

### 3.4 Titles and blurbs

- `title`: short, concrete, and inviting. Dutch, no spoilers. Examples: *De rode
  bal*, *De sleutel is weg*, *De laatste stroopwafel*.
- `blurb_nl`: one or two simple Dutch sentences that set up the hook without
  giving away the ending. It appears on the Level screen, so it must be readable
  at the story's level.
- `blurb_en`: an optional English translation of the blurb. It is not shown in
  the reader but is useful for the story list and for the writer. Include it.

---

## 4. Difficulty progression by level

### 4.1 General rule

Story *N* is slightly harder than story *N−1*. Increase **one or two** of these
at a time, never all at once:

- sentence length,
- grammar complexity (tenses, subordinate clauses, passive, conditionals),
- vocabulary rarity and abstraction,
- story length (total words) and section length,
- number of characters and plot threads.

The jump from the last story of one level to the first story of the next must be
gentle. A2-01 should feel like a natural next step after A1-25, not like a
different app.

### 4.2 A1

**Grammar**

- Present tense only: `zijn`, `hebben`, regular verbs, modals `willen` / `kunnen`
  / `moeten` / `mogen`.
- Simple main-clause word order. Basic questions (`Waar is...?`, `Wat doe je?`).
- Negation with `niet` and `geen`.
- Simple separable verbs only late in A1.
- No perfect tense, no simple past, no passive, no subordinate clauses with
  verb-final order.

**Sentences**

- A1-01: 3–6 words per sentence.
- A1-25: 6–10 words per sentence, still mostly main clauses.

**Topics**

Numbers, colours, family, food, daily routine, greetings, places in town,
shopping, weather, pets, bicycles. Cognates where possible (`fiets`, `hond`,
`man`, `huis`).

**Repetition**

Heavy. A1-01 repeats its 20 new lemmas dozens of times across 62 short sections.
Later A1 stories still repeat new words 3–4 times each.

### 4.3 A2

**Grammar**

- Perfect tense first (`heeft gelegd`, `is gegaan`), then simple past for common
  verbs (`ging`, `zag`, `kwam`).
- Separable verbs (`doet ... open`, `belt ... op`).
- More connectors: `omdat`, `maar`, `want`, `dan`, `als`.
- Comparatives (`sneller`, `beter`), reflexives (`zich voelen`).
- Dialogue with reporting verbs (`zegt hij`, `vraagt ze`).
- Sentences up to about 12 words.

**Topics**

Shopping, travel, work, health, neighbours, small mysteries, everyday
misunderstandings. A2-01 is a small domestic mystery: a lost key and a dog who
knows more than he lets on.

### 4.4 B1

**Grammar**

- Mixed tenses, including simple past and perfect tense in the same story.
- Subordinate clauses with correct verb-final order (`... omdat hij de sleutel
  niet kon vinden`).
- Opinions and reasons (`Ik vind ... omdat ...`).
- Longer descriptions, some idioms (`de schuld geven`, `zich schamen`).
- Sentences up to about 16 words, with variety.

**Topics**

More abstract ideas: fairness, luck, honesty, friendship, small ethical
dilemmas, community. Plots can have more characters and a secondary thread.
B1-01 is a market story about a stolen stroopwafel, a cheeky seagull, and a baker
who rewards honesty.

### 4.5 B2

**Grammar**

- Complex sentences with multiple clauses.
- Passive voice (`wordt uitgesteld`, `werd overtuigd`).
- Conditional and hypothetical forms (`zou`, `had ... kunnen`).
- Formal and informal registers (`u` vs. `jij`), especially in dialogue.
- Idiomatic expressions and nuance (`een indruk maken`, `onder controle
  hebben`, `het tempo bepalen`).
- Humour through register and understatement.

**Topics**

Work, institutions, interviews, bureaucracy, ambition, responsibility, social
norms. B2-01 is a job interview where the boss is a dog and the applicant has to
prove he can do more than talk.

### 4.6 Cross-level jumps

- A1-25 → A2-01: add perfect tense and longer dialogue, but keep a similar total
  length and familiar topics.
- A2-25 → B1-01: add subordinate clauses and opinions, but keep the cast small.
- B1-25 → B2-01: add passive/conditional structures and a more formal register,
  but keep the plot concrete.

---

## 5. The 20 new words

### 5.1 Exactly 20, counted by lemma

Every story introduces **exactly 20 new words**. A "new word" is a **lemma**
(dictionary form) that has not appeared in any earlier story in the global order
(A1-01 → A1-25 → A2-01 → ... → B2-25).

- Count lemmas, not surface forms. If `lopen` is introduced, then `loopt`,
  `liep`, and `gelopen` are old words in later stories.
- Inflected forms and plurals of an introduced lemma are never new again.
- The validator normalises case and the two apostrophe characters, but it does
  **not** lemmatise for you. You declare the lemma in `new_words` and map every
  surface form to it in the glossary. Be consistent: the same lemma spelling
  must be used in `new_words`, in `glossary[].lemma`, and in
  `wordlist.json` (which is generated for you).
- Examples:
  - `liep` → lemma `lopen`
  - `fietsen` (plural) → lemma `fiets`
  - `beter` → lemma `goed` (comparative)
  - `opende` → lemma `openen`
  - `deed ... open` → lemma `opendoen` (separable verb)
- The 20 entries in `new_words` must be distinct. Duplicates are an error.
- A new word must actually appear in the text. A new word used only once gets a
  warning; aim for 2–3 uses minimum.

### 5.2 Choosing the words

Pick words that are:

1. **useful** — high-frequency for the level, likely to recur in later stories;
2. **natural to the story** — the plot should need them, not the other way
   around;
3. **teachable from context** — the surrounding sentence should make the meaning
   guessable;
4. **not already known** — check `wordlist.json` and the previous stories.

Do not choose 20 nouns just because nouns are easy to illustrate. A story made
only of nouns reads like a vocabulary list.

### 5.3 Word-type mix

A good default mix for a story:

| Type | Count | Examples |
|------|-------|----------|
| Nouns (with `de`/`het`) | 8–10 | `sleutel`, `keuken`, `buurman` |
| Verbs | 5–6 | `zoeken`, `vergeten`, `overtuigen` |
| Adjectives / adverbs | 3–4 | `plotseling`, `eerlijk`, `zenuwachtig` |
| Function words | 1–3 | `omdat`, `tijdens`, `zonder` |

A1-01 is the exception: the basic function words it needs are starter words, not
new words (see [§5.9](#59-starter-words)). Its 20 new words are ordinary nouns,
verbs, adjectives, and adverbs.

### 5.4 Articles for nouns

Dutch gender is hard to guess, so **every noun must be glossed with `de` or
`het`** in the glossary. This is enforced: a noun without an article is an error,
and an article other than `de`/`het` is an error. Do not put an article on
non-nouns; that produces a warning.

If a noun can take both (`de bank` = sofa/bench), choose the article that fits
the story's sense. If the story uses two senses with different articles, split
them into two glossary entries with different forms or lemmas and explain the
sense in `meaning`.

### 5.5 Repetition inside the story

Each new word should appear **at least 2–3 times** in the story that introduces
it. Repetition is how the learner consolidates it. The validator warns when a
new word appears only once.

Ways to repeat naturally:

- use the word in dialogue and in narration;
- use a different inflection (`zoeken` → `zoekt`, `zocht`);
- let another character react to the same object or idea;
- return to the word in the payoff.

Do not pad with unnatural repetition. If a word cannot be used three times
without sounding silly, it may be the wrong word for this story.

### 5.6 Recycling old words and coverage

- At least 85–90% of the running words should be words the learner already
  knows, once the level is established (see the thresholds in
  [§2.3](#23-known-word-coverage)).
- Recycle old vocabulary deliberately: the same places, objects, and verbs from
  earlier stories. Recurring characters help.
- Do not introduce a second new word for a concept the learner already has
  (`auto` when `wagen` is known, unless the distinction matters).
- The 20 new words should feel like the only new things in the story.

### 5.7 Rare words, names, and one-offs

- Do not use rare or off-level words outside the 20 new words. If a word is not
  in `wordlist.json`, it is either new (count it) or a gap (put it in the debt
  list, and prefer to avoid it).
- Proper names are allowed but must be minimal. Gloss them with
  `"pos": "name"`; they are excluded from new-word counting and coverage. Do not
  put names in `new_words`.
- Brand names, place names, and company names follow the same rule: gloss them
  as names, keep them few.
- Avoid abbreviations, digits, and symbols. Write `negen uur`, not `9.00`; write
  `een`, not `1`. The tokenizer treats digits as words, so `9.00` becomes two
  tokens (`9` and `00`) that both need glossary entries.
- Avoid hyphenated compounds (`zwart-wit`): the tokenizer splits on the hyphen,
  so both halves need entries.

### 5.8 The A1-01 exception

A1-01 is the easiest story in the app and the only story with no earlier story
to draw vocabulary from. It must:

- introduce **exactly 20 new lemmas**, listed in `new_words`;
- use **no other lemma except the starter words and proper names**. A non-starter
  extra word is an error (`A1_01_EXTRA_WORD`);
- never list a starter word in `new_words` (`STARTER_AS_NEW`);
- render as **at least 5 screens** on both reference phones, like every story;
- repeat its 20 new lemmas heavily in short sections, with 3–8 word sentences;
- use present tense only, with `zijn`, `hebben`, `gaan`, `zien`, `willen`, and
  simple main clauses.

A1-01 is intentionally repetitive: with 20 content lemmas, 25 starter lemmas,
and five screens to fill, the same words must recur. Repetition is the teaching
method here; keep the scenes varied so it still feels like a story.

The 20 new lemmas in the canonical A1-01 are:

```
hond, bal, huis, tuin, groot, klein, rood, blij, gaan, lopen,
kijken, komen, pakken, geven, willen, zien, spelen, samen, altijd, hallo
```

The coverage rule is waived for A1-01. The canonical example is
`assets/stories/a1/01-de-rode-bal.json` ("De rode bal").

### 5.9 Starter words

Some words are known from the very first screen. They live in
`assets/starter_words.json`, are loaded at startup, and are added to **every**
story's glossary automatically. You do not repeat them in a story file.

The list is deliberately small (25 lemmas maximum) and hand-editable. It may
contain only basic function words and the basic forms of `zijn` and `hebben`.
No nouns, no adjectives.

| Lemma | Forms | Type | Meaning |
|-------|-------|------|---------|
| `de` | de | article | the |
| `het` | het | article | the |
| `een` | een | article | a / an |
| `ik` | ik | pronoun | I |
| `je` | je | pronoun | you |
| `hij` | hij | pronoun | he |
| `zij` | zij | pronoun | she / they |
| `we` | we | pronoun | we |
| `jullie` | jullie | pronoun | you (plural) |
| `en` | en | conjunction | and |
| `of` | of | conjunction | or |
| `maar` | maar | conjunction | but |
| `ja` | ja | interjection | yes |
| `nee` | nee | interjection | no |
| `zijn` | ben, bent, is, zijn | verb | to be |
| `hebben` | heb, hebt, heeft, hebben | verb | to have |
| `hier` | hier | adverb | here |
| `daar` | daar | adverb | there |
| `niet` | niet | adverb | not |
| `geen` | geen | determiner | no / not a |
| `in` | in | preposition | in |
| `op` | op | preposition | on |
| `aan` | aan | preposition | on / at |
| `naar` | naar | preposition | to |
| `met` | met | preposition | with |

Rules for starter words:

- They are known from the start and are **never new**. Listing one in
  `new_words` is an error (`STARTER_AS_NEW`).
- They appear in every story's glossary automatically, so a beginner can tap
  them on the first screen. Do not duplicate their entries in a story file.
- They are never underlined as new.
- They count as known for coverage.
- They are never added to the assumed-known debt list.
- `wordlist.json` records them with the owner `"starter"`, not a story id.
- Do not add a word casually. Every addition changes the baseline for all 100
  stories. If a word is not a basic function word or a basic form of `zijn` /
  `hebben`, it belongs in a story's 20 new words instead.

---

## 6. The glossary

### 6.1 Every token must be covered

The glossary is bundled with the story and is the offline dictionary. **Every
word token in the story must resolve to a glossary entry**, either directly via
`form` or via a `parts` entry. The validator tokenises the text with the app's
tokenizer and reports every missing surface form (`MISSING_GLOSSARY`). This is
an error, not a warning.

The starter words are already covered: the loader appends their entries to every
story, so you never write `de`, `het`, `is`, `heeft`, `niet`, `in`, and the rest
of the list in [§5.9](#59-starter-words) yourself. If a story file also defines
one of those forms, the story's own entry wins.

Cover everything else:

- every inflected form (`gaat`, `ziet`, `wil`, `gelegd`, ...);
- every plural (`kramen`, `mensen`, `veren`, ...);
- every comparative/superlative (`sneller`, `beste`, ...);
- separable verbs and their split parts (see [§6.4](#64-separable-verbs-and-parts));
- fixed expressions where the literal words would mislead;
- names (`"pos": "name"`);
- numbers and interjections if they appear.

### 6.2 Entry fields

| Field | Required | Meaning |
|-------|----------|---------|
| `form` | yes | The exact surface form as it appears in the text. Case is ignored for matching, but keep the text's case for readability. |
| `lemma` | yes | Dictionary form. Must match `new_words` exactly (case-insensitive) for new words. |
| `pos` | yes | Part of speech. See the list below. |
| `article` | nouns only | `de` or `het`. Required for nouns, forbidden (warning) for other types. |
| `meaning` | yes | Short English meaning that fits **this** context. Keep it to a few words. For a non-lemma form this must be the form's own meaning, never a copy of the lemma's meaning. |
| `new` | yes | `true` for entries of this story's 20 new words, `false` otherwise. |
| `parts` | optional | For separable/multi-word forms: the individual tokens that should open this entry. |
| `occurrence` | optional | 1-based occurrence of `form` in the story, used when one form genuinely has different senses in different sentences. A form may have at most one default entry (no `occurrence`) plus entries with unique occurrence numbers. |

Allowed `pos` values (anything else produces a warning):

`noun`, `verb`, `adjective`, `adverb`, `pronoun`, `preposition`, `conjunction`,
`interjection`, `numeral`, `article`, `determiner`, `name`, `phrase`,
`expression`, `other`.

Use `name` only for proper names. Use `other` for anything that does not fit;
prefer a more specific value when possible.

### 6.3 Meanings for inflected forms

The `lemma` field carries the base form, so `meaning` must be a direct English
gloss of the **surface form**:

```json
{ "form": "liep", "lemma": "lopen", "pos": "verb", "meaning": "walked", "new": false }
```

The popup shows the tapped word as written, this form-specific meaning, and a
secondary line built from the lemma entry:

```
liep
walked
form of: lopen, to walk
```

So every lemma that a form points to needs its own lemma entry (or must be a
starter word) with a usable meaning. A non-lemma form may **not** repeat its
lemma's meaning: the validator reports `GLOSSARY_LEMMA_MEANING`. If English has
no different word (for example an inflected adjective such as `rode` = `red`),
add an explicit marker: `"red (inflected)"`.

If the same surface form has genuinely different meanings in different
sentences, do **not** write a combined gloss. Use one default entry plus an
`occurrence` entry for the sentence that differs:

```json
{ "form": "zijn", "lemma": "zijn", "pos": "determiner", "meaning": "his", "new": false },
{ "form": "zijn", "lemma": "zijn", "pos": "verb", "meaning": "are", "occurrence": 4, "new": false }
```

Occurrence numbers count the surface form in the story, starting at 1, across
all sections. Use this only for the smallest set of occurrences that need it;
one entry per form remains the default. The validator checks that occurrences
are unique per form, in range, and that a form has at most one default entry.

### 6.4 Separable verbs and parts

For a separable verb, add a `parts` array containing the tokens that should open
the entry. Example:

```json
{
  "form": "doet",
  "parts": ["doet", "open"],
  "lemma": "opendoen",
  "pos": "verb",
  "meaning": "opens",
  "new": true
}
```

Both `doet` and `open` then resolve to this entry. The `form` itself is also a
key, so `doet` works even without `parts`; `parts` is what makes the separated
half (`open`) tappable.

Use `parts` for:

- separable verbs: `opendoen`, `uitstellen`, `aannemen`, `terugkomen`,
  `schoonmaken`, `uitlaten`, `weggeven`, `opeten`;
- fixed multi-word expressions where a single word is misleading.

Be careful with a part that is also an independent word with a different
meaning. The exact `form` always wins over `parts`, so a separate entry for the
independent word can coexist. If the independent word itself has two senses,
use the smallest set of `occurrence` entries described in §6.3.

### 6.5 Tokenizer rules you must know

The app's tokenizer (`src/stories/tokenize.rs`) is the single source of truth
for what counts as a word. It behaves like this:

- Letters and digits form words. Accents and combining marks stay inside words.
- A straight or curly apostrophe stays inside a word when followed by a letter:
  `zo'n`, `'s ochtends`, `collega's`.
- A leading apostrophe is treated as a Dutch elision only before
  `s`, `t`, `n`, `k`, `m`, `d`, `r`, or `i` (`'s`, `'t`, `'n`, `'k`, `'m`,
  `'d`, `'r`, `'ie`). Any other leading apostrophe is punctuation and is
  dropped.
- A closing quote is punctuation, not part of the word: `'hond'` → `hond`.
- Hyphens split words: `zwart-wit` → `zwart`, `wit`.
- Numbers are words: `3` is a token, `9.00` is `9` and `00`.
- Punctuation (`. , ! ? : ; “ ” ( ) —`) is not part of a token.

**Dialogue quoting.** Use curly double quotes for dialogue:

```json
"“Waar is mijn sleutel?” vraagt hij."
```

Do **not** use straight single quotes around dialogue (`'Waar is...?'`). A
leading `'` before a lowercase word beginning with one of the elision letters is
kept as part of the token, which creates phantom words such as `'maar` that then
need glossary entries. Curly double quotes are always safe.

### 6.6 Generating the glossary in practice

Writing glossaries by hand is where most mistakes happen. A reliable workflow:

1. Write the story text and the 20 lemmas first.
2. Tokenise the text (the validator does this for you; run it and read the
   `MISSING_GLOSSARY` list).
3. Add entries for every missing form, using the same lemma spelling as
   `new_words`. Give each non-lemma form its own surface-form meaning.
4. Add a lemma entry for every lemma a form points to (unless it is a starter
   word), so the popup's `form of:` line has a meaning.
5. Re-read the text for a form with two genuinely different senses and add the
   smallest set of `occurrence` entries. Never use a combined gloss as a
   shortcut.
6. Run the validator again. Repeat until `MISSING_GLOSSARY` is gone and
   `GLOSSARY_LEMMA_MEANING` is silent.
7. Check that every entry flagged `"new": true` belongs to one of the 20 lemmas,
   and that every one of the 20 has at least one `"new": true` entry.

A small script that walks the tokenised text and requires a mapping for every
surface form is the safest way to guarantee completeness. The validator is the
final judge.

### 6.7 Duplicates and conflicts

- A surface form has one default entry. A second entry for the same `form` is
  allowed only with a unique `occurrence`; without one the validator reports
  `GLOSSARY_DUPLICATE_FORM`.
- Occurrence numbers must be 1-based, within the number of times the form
  appears, and unique per form.
- A `"new": true` entry whose lemma is not in `new_words` is an error
  (`GLOSSARY_NEW_NOT_LISTED`).
- A `"new": true` entry for a starter word is an error (`STARTER_AS_NEW`).
- A lemma in `new_words` with no `"new": true` entry is an error
  (`NEW_WORD_GLOSSARY`).
- Keep the glossary ordered by first appearance in the text. This is not
  required, but it makes review much easier.

---

## 7. Style and content

### 7.1 Entertaining first

Use humour, mystery, small adventures, relatable everyday situations, likeable
characters, and mini cliffhangers. A mix of genres across the 25 stories keeps
the level fresh: slice of life, comedy, mystery, light fantasy, travel, work,
family. The sample stories show the range: a bicycle-and-dog comedy (A1), a
domestic mystery (A2), a market comedy with a moral (B1), and a workplace farce
(B2).

### 7.2 Natural Netherlands Dutch

- Write standard Netherlands Dutch, not translation-ese.
- Use natural word order and contractions where they fit (`'s ochtends`,
  `zo'n`, `dat is 't`).
- Dialogue should sound spoken, not written.
- Avoid Flemish-only words and constructions unless a character specifically
  needs them.

### 7.3 Cultural touches

Bikes, weather, coffee, cheese, stroopwafels, markets, canals, King's Day,
directness, and small everyday rituals make the Dutch feel authentic. Use them
when they serve the story, not as decoration.

### 7.4 All ages

No hateful, explicit, or disturbing content. Conflict should be small-scale and
resolved with warmth or humour. Characters can be angry or disappointed, but the
story should not leave the reader uneasy.

### 7.5 Dialogue from A1 onward

Use dialogue in every story, including A1. It shows spoken language,
punctuation, and the difference between narration and speech. Keep dialogue
lines short at low levels.

### 7.6 Short paragraphs

Avoid dense walls of text. Author short sections (normally 1–3 sentences); each
section becomes a paragraph, and the app paginates them. A section with one
150-word paragraph is hard to read on a phone even at B2.

### 7.7 Titles that work

Concrete nouns and a hint of trouble work well: *De rode bal*, *De sleutel is
weg*, *De laatste stroopwafel*, *Het sollicitatiegesprek*. Avoid abstract titles
and puns that depend on vocabulary the learner does not have yet.

---

## 8. File format and naming

### 8.1 Folder and file naming

```
assets/stories/<level>/<nn>-<slug>.json
```

- `<level>` is lowercase: `a1`, `a2`, `b1`, `b2`.
- `<nn>` is the two-digit order, `01`–`25`.
- `<slug>` is a short lowercase kebab-case version of the title, ASCII only:
  `01-de-rode-bal.json`.
- The `id` is `<level>-<nn>`: `a1-01`, `a2-07`, `b2-25`.
- The `order` field must equal `<nn>` and be between 1 and 25.
- The folder must match `level`, and the file name must start with `<nn>-`.
  The validator checks all of this (`IDENTITY`).

Examples:

```
assets/stories/a1/01-de-rode-bal.json
assets/stories/a2/01-de-sleutel-is-weg.json
assets/stories/b1/01-de-laatste-stroopwafel.json
assets/stories/b2/01-het-sollicitatiegesprek.json
```

### 8.2 The JSON schema

A story file is one JSON object:

| Field | Type | Rules |
|-------|------|-------|
| `id` | string | `<level>-<nn>`, must match folder and order. |
| `level` | string | `"A1"`, `"A2"`, `"B1"`, or `"B2"`. |
| `order` | number | 1–25, must match the file name and `id`. |
| `title` | string | Non-empty. |
| `blurb_nl` | string | One or two simple Dutch sentences. |
| `blurb_en` | string | Optional English blurb; include it. |
| `sections` | array of strings | Short paragraphs, normally 1–3 sentences. The app joins them with blank lines and paginates them. |
| `new_words` | array of 20 strings | Lemmas, distinct, none introduced earlier and none from the starter list. |
| `glossary` | array of objects | Every non-starter token covered; see [§6](#6-the-glossary). |

The legacy key `pages` is still accepted as an alias for `sections` so old files
keep loading, but every new story must use `sections`. Do not add a page count:
pages are computed at runtime.

Glossary object:

| Field | Type | Rules |
|-------|------|-------|
| `form` | string | Exact surface form. |
| `lemma` | string | Dictionary form. |
| `pos` | string | One of the allowed parts of speech. |
| `article` | string | `"de"` or `"het"` for nouns only. |
| `meaning` | string | Short English gloss for this context; for a non-lemma form, its own meaning, not the lemma's. |
| `new` | boolean | `true` only for this story's 20 new lemmas. |
| `parts` | array of strings | Optional; tokens that open this entry. |
| `occurrence` | number | Optional; 1-based occurrence of `form` for a sense that differs from the default entry. |

### 8.3 Annotated example (abridged)

Below is the beginning of the canonical A1-01 story with documentation comments.
**The comments are for explanation only.** A real file is strict JSON: remove
every `//` comment. The complete file is
`assets/stories/a1/01-de-rode-bal.json`; copy its structure.

```jsonc
{
  // Identity. `id` must be <level>-<two-digit order>, the file must live in
  // assets/stories/a1/ and its name must start with "01-".
  "id": "a1-01",
  "level": "A1",
  "order": 1,

  // Title and blurbs. blurb_nl is shown on the Level screen.
  "title": "De rode bal",
  "blurb_nl": "Sam, Noor en de hond Bo spelen met een rode bal in de tuin.",
  "blurb_en": "Sam, Noor and the dog Bo play with a red ball in the garden.",

  // Short sections. The app joins them with a blank line and computes pages.
  // The real story has 62 sections; these are the first six.
  "sections": [
    "Hallo! Ik ben Sam. Hier is Noor.",
    "Hier is Bo. Bo is een hond. Bo is groot.",
    "Bo is niet klein.",
    "Bo heeft een bal. De bal is rood. De bal is niet groot.",
    "De bal is klein. Bo is blij met de bal.",
    "Bo wil de bal. Bo pakt de bal. Bo loopt met de bal in de tuin."
  ],

  // Exactly 20 distinct lemmas. A1-01 may use no other lemma except starter
  // words and proper names.
  "new_words": [
    "hond", "bal", "huis", "tuin", "groot", "klein", "rood", "blij",
    "gaan", "lopen", "kijken", "komen", "pakken", "geven", "willen",
    "zien", "spelen", "samen", "altijd", "hallo"
  ],

  // Every non-starter surface form in the text has an entry. Nouns carry
  // de/het. Entries for the 20 new lemmas are flagged "new": true. Starter
  // words such as "ik", "ben", "hier", "is", "een", "de", "niet" and "met"
  // are added automatically and must not be repeated here.
  // Non-lemma forms carry their own meaning (gaat = goes, not "to go"), and
  // each referenced lemma has a lemma entry so the popup's "form of:" line
  // always has a meaning.
  "glossary": [
    { "form": "hond", "lemma": "hond", "pos": "noun", "article": "de", "meaning": "dog", "new": true },
    { "form": "bal", "lemma": "bal", "pos": "noun", "article": "de", "meaning": "ball", "new": true },
    { "form": "huis", "lemma": "huis", "pos": "noun", "article": "het", "meaning": "house / home", "new": true },
    { "form": "tuin", "lemma": "tuin", "pos": "noun", "article": "de", "meaning": "garden", "new": true },
    { "form": "groot", "lemma": "groot", "pos": "adjective", "meaning": "big / large", "new": true },
    { "form": "klein", "lemma": "klein", "pos": "adjective", "meaning": "small / little", "new": true },
    { "form": "rood", "lemma": "rood", "pos": "adjective", "meaning": "red", "new": true },
    { "form": "blij", "lemma": "blij", "pos": "adjective", "meaning": "happy / glad", "new": true },
    { "form": "gaat", "lemma": "gaan", "pos": "verb", "meaning": "goes", "new": true },
    { "form": "gaan", "lemma": "gaan", "pos": "verb", "meaning": "to go", "new": true },
    { "form": "loopt", "lemma": "lopen", "pos": "verb", "meaning": "walks", "new": true },
    { "form": "lopen", "lemma": "lopen", "pos": "verb", "meaning": "to walk", "new": true },
    { "form": "Sam", "lemma": "sam", "pos": "name", "meaning": "Sam (a name)", "new": false },
    { "form": "Noor", "lemma": "noor", "pos": "name", "meaning": "Noor (a name)", "new": false },
    { "form": "Bo", "lemma": "bo", "pos": "name", "meaning": "Bo (the dog's name)", "new": false }
  ]
}
```

For a story with old vocabulary, the glossary will contain many entries with
`"new": false`, for example:

```json
{ "form": "liep", "lemma": "lopen", "pos": "verb", "meaning": "walked", "new": false }
```

An ambiguous form uses an occurrence entry only for the sense that differs:

```json
{ "form": "zijn", "lemma": "zijn", "pos": "determiner", "meaning": "his", "new": false },
{ "form": "zijn", "lemma": "zijn", "pos": "verb", "meaning": "are", "occurrence": 4, "new": false }
```

and a separable verb:

```json
{
  "form": "doet",
  "parts": ["doet", "open"],
  "lemma": "opendoen",
  "pos": "verb",
  "meaning": "opens",
  "new": true
}
```

### 8.4 `wordlist.json` and the debt list

Two generated files live next to the story folders:

- `assets/wordlist.json` — the cumulative list. It records the global story
  order and maps each introduced lemma to the story that introduced it:

  ```json
  {
    "version": 1,
    "stories": [{ "id": "a1-01", "level": "A1", "order": 1 }],
    "words": { "hond": "a1-01", "bal": "a1-01", "de": "starter" }
  }
  ```

  Starter words use the owner `"starter"`, not a story id. They are recorded
  once and never change owner.

- `assets/assumed_known.json` — the debt list. It records words that the sample
  stories use without any earlier story introducing them, and which stories use
  them:

  ```json
  {
    "version": 1,
    "note": "...",
    "words": { "aan": ["a2-01", "b1-01"] }
  }
  ```

**Never edit either file by hand.** They are regenerated with
`--write-wordlist`. The wordlist is what the AI prompt uses to know what the
learner already knows; the debt list tells future writers which words the
missing early stories still have to introduce.

---

## 9. Validation workflow

### 9.1 Commands

Run from the repository root:

```bash
# Validate all stories in allow-gaps mode (the default for this library).
cargo run -p validate_stories -- --allow-gaps

# Regenerate wordlist.json and assumed_known.json, then validate.
cargo run -p validate_stories -- --write-wordlist --allow-gaps

# Strict mode: every used word must be introduced by an earlier story.
cargo run -p validate_stories -- --strict

# Validate a different story tree (used by the test fixtures).
cargo run -p validate_stories -- --stories tests/fixtures/valid
```

The CLI exits with code 0 when there are no errors, 1 when there are errors,
and 2 for a usage or I/O failure. Warnings do not fail the build.

In debug builds the app also runs a non-blocking startup check and prints the
same diagnostics to logcat. A broken story file therefore shows up immediately
when the app starts.

### 9.2 Modes

- **allow-gaps** (the default for the current library): words that are neither
  new nor introduced earlier are collected as "assumed known" and reported as
  one warning per story. Only the three sample stories still produce these
  warnings.
- **strict** (use after the sample-story debt is paid down): the same situation
  is an error. Every used lemma must be introduced by an earlier story.

### 9.3 What the validator checks

| Code | Severity | Meaning |
|------|----------|---------|
| `MIN_SCREENS` | error | The story renders as fewer than 5 screens on a reference phone. |
| `NEW_WORD_COUNT` | error | Not exactly 20 new words, or an empty entry. |
| `DUPLICATE_NEW_WORD` | error | The same lemma appears twice in `new_words`. |
| `NEW_WORD_UNUSED` | error | A new word never appears in the text. |
| `NEW_WORD_REPEAT` | warning | A new word appears only once. |
| `NEW_WORD_EARLIER` | error | A "new" word already appeared in an earlier story. |
| `NEW_WORD_WORDLIST_OWNER` | error | The wordlist assigns the lemma to a different story. |
| `NEW_WORD_GLOSSARY` | error | No `"new": true` entry, or the lemma is absent from the glossary. |
| `MISSING_GLOSSARY` | error | A surface form in the text has no entry (directly or via `parts`). |
| `GLOSSARY_INVALID` | error | Empty form/lemma/meaning/pos. |
| `GLOSSARY_ARTICLE` | error | A noun lacks `de`/`het`, or has another article. |
| `GLOSSARY_ARTICLE_EXTRA` | warning | A non-noun has an article. |
| `GLOSSARY_POS` | warning | Unknown part of speech. |
| `GLOSSARY_DUPLICATE_FORM` | error / warning | Error when a form has more than one default entry or a duplicate `occurrence`; warning when a form has only occurrence-specific entries (unlisted occurrences fall back to the first). |
| `GLOSSARY_OCCURRENCE` | error | An `occurrence` is 0 or greater than the number of times the form appears. |
| `GLOSSARY_LEMMA_MEANING` | error | A non-lemma form repeats its lemma entry's meaning; give the surface form its own meaning. |
| `GLOSSARY_NEW_NOT_LISTED` | error | Flagged `"new": true` but not in `new_words`. |
| `STARTER_AS_NEW` | error | A starter word is listed in `new_words` or flagged `"new": true`. |
| `STARTER_WORDLIST` | error | `wordlist.json` marks a lemma as `"starter"` that is not in `starter_words.json`. |
| `IDENTITY` | error | id/level/order/file name/folder mismatch, or empty title. |
| `ORDER_RANGE` | error | `order` outside 1–25. |
| `DUPLICATE_ID` | error | Two files use the same id. |
| `DUPLICATE_ORDER` | error | Two files use the same level + order. |
| `DUPLICATE_INTRODUCTION` | error | Two stories list the same lemma as new. |
| `WORD_INTRODUCED_LATER` | error | A word is used before the story that introduces it. |
| `UNINTRODUCED_WORD` | error (strict) | A used word is never introduced. |
| `ASSUMED_KNOWN` | warning (allow-gaps) | A used word is not introduced; added to the debt list. |
| `COVERAGE` | warning | Known-word coverage below the level/order target. |
| `A1_01_EXTRA_WORD` | error | A1-01 uses a non-starter lemma outside its 20 new words. |
| `WORDLIST_STALE` | error (strict) / warning (allow-gaps) | `wordlist.json` is out of date. |
| `WORDLIST_OWNER` | error | A new word is recorded under another story. |
| `WORDLIST_STORIES` | error (strict) / warning (allow-gaps) | The story list in `wordlist.json` does not match the files. |
| `WORDLIST_VERSION` | warning | Unexpected wordlist version. |
| `WORDLIST_UNKNOWN_STORY` | error (strict) / warning (allow-gaps) | The wordlist references a story that is not present. |

### 9.4 Recommended workflow

1. Write the story and the 20 lemmas.
2. Run `cargo run -p validate_stories -- --allow-gaps`.
3. Fix every **error**. Read `MISSING_GLOSSARY` carefully; it lists every
   surface form you still need.
4. Fix the warnings that indicate a real problem (`NEW_WORD_REPEAT`,
   `COVERAGE`). Leave only the three expected gap warnings for `a2-01`,
   `b1-01`, and `b2-01`.
5. Run `cargo run -p validate_stories -- --write-wordlist --allow-gaps` to
   regenerate the cumulative list and the debt list.
6. Run the validator once more with `--allow-gaps` and confirm
   `0 error(s)`.
7. Rebuild the app. The new story appears automatically in its level; no code
   change is needed.

### 9.5 Auto-discovery

`build.rs` scans `assets/stories/*/*.json` at build time and embeds every file.
Android cannot list assets at runtime, so the index is generated at compile
time. Dropping in a new JSON file and rebuilding is enough. The build also
watches `assets/wordlist.json`, so regenerating the wordlist triggers a rebuild.

---

## 10. Pre-submission checklist

Before you add a story file, confirm every item:

**Identity and structure**

- [ ] File is at `assets/stories/<level>/<nn>-<slug>.json` with lowercase level
      folder and two-digit order.
- [ ] `id` is `<level>-<nn>` and matches the folder, file name, and `order`.
- [ ] `order` is between 1 and 25.
- [ ] The file uses `sections` (not `pages`), and every section is a short
      paragraph.
- [ ] Total word count matches the level (see [§3.2](#32-story-length)) and sits
      at the right point in the level's progression.
- [ ] The story renders as at least 5 screens on both reference phones
      (`MIN_SCREENS` is absent).
- [ ] Title is short and concrete; `blurb_nl` and `blurb_en` are present.
- [ ] The story has a hook in the first screen and a payoff in the last.

**The 20 words**

- [ ] Exactly 20 distinct lemmas in `new_words`.
- [ ] None of them appeared in an earlier story (check `wordlist.json`).
- [ ] None of them is a starter word (check [§5.9](#59-starter-words)).
- [ ] Each appears at least 2–3 times in the text.
- [ ] A1-01 uses no non-starter word outside its 20 (`A1_01_EXTRA_WORD`).
- [ ] The mix includes nouns, verbs, adjectives/adverbs, and function words,
      not only nouns.
- [ ] Every noun has `de` or `het` in the glossary.
- [ ] No rare or off-level words outside the 20 (or they are glossed and
      justified).
- [ ] Names are glossed with `"pos": "name"` and are not in `new_words`.

**Glossary**

- [ ] Every non-starter surface form in the text has an entry (run the
      validator; no `MISSING_GLOSSARY`). Starter words are added automatically
      and are not repeated in the file.
- [ ] Inflected forms, plurals, and comparatives are covered.
- [ ] Separable verbs use `parts` so both halves are tappable.
- [ ] Each of the 20 new lemmas has at least one `"new": true` entry.
- [ ] Meanings are short, English, and fit the story's context.
- [ ] Every non-lemma form has its own meaning, not a copy of its lemma's
      meaning (`GLOSSARY_LEMMA_MEANING`).
- [ ] Every referenced lemma has a lemma entry (or is a starter word), so the
      `form of:` line always has a meaning.
- [ ] Ambiguous forms use the smallest set of `occurrence` entries; no combined
      gloss stands in for two senses.
- [ ] No duplicate default `form` entries and no duplicate `occurrence` numbers.
- [ ] Dialogue uses curly double quotes, not straight single quotes.

**Difficulty and coverage**

- [ ] Grammar matches the level (see [§4](#4-difficulty-progression-by-level)).
- [ ] Coverage is at or above the level/order target (or the shortfall is
      understood and acceptable).
- [ ] Old vocabulary is recycled; the story does not feel like a word list.
- [ ] The jump from the previous story is gentle.

**Quality**

- [ ] The story is entertaining: a real problem, a turn, and a satisfying
      ending.
- [ ] Dutch is natural Netherlands Dutch, not translation-ese.
- [ ] Paragraphs are short; no wall of text.
- [ ] Content is appropriate for all ages.
- [ ] You have read the story aloud once and fixed anything that sounds wrong.

**Validation**

- [ ] `cargo run -p validate_stories -- --allow-gaps` reports 0 errors.
- [ ] `cargo run -p validate_stories -- --write-wordlist --allow-gaps` has been
      run.
- [ ] The app rebuilds and the story appears in its level.

---

## 11. AI writing prompt template

Use this template to generate one story at a time. Fill in the four inputs,
paste the template plus this guide into the AI, and ask for a single JSON object.

### 11.1 Inputs to provide

1. **Level** — `A1`, `A2`, `B1`, or `B2`.
2. **Story number** — `1`–`25` within that level.
3. **Cumulative word list** — the contents of `assets/wordlist.json` (or just
   the `words` map and the list of story ids), plus the contents of
   `assets/assumed_known.json` so the AI knows which words are still owed.
4. **Starter words** — the contents of `assets/starter_words.json`. Paste the
   list into the prompt so the AI never puts one in `new_words`.
5. **Previous story** — the full JSON of the previous story in global order
   (for A1-01, use the A1-01 exception and the starter list).

Optionally provide the next story's title idea, but let the AI choose a fresh
one.

### 11.2 Prompt

```text
You are writing story <N> of level <LEVEL> for DutchReader, a graded Dutch
reading app for a near-complete beginner. Read the attached STORY_GUIDE.md and
follow it exactly. Read the attached wordlist.json, assumed_known.json, and the
previous story JSON.

Write one original, entertaining story in Netherlands Dutch.

Starter words (known from the start; never put them in "new_words"):
<STARTER_LIST>

Hard requirements:
- Author "sections": an array of short paragraphs, normally 1-3 sentences each.
  Do not author pages and do not include a page count; the app computes pages
  from the text.
- Total length: <TOTAL_WORDS> words, appropriate for <LEVEL> story <N>. The
  finished story must render as at least 5 screens on a 1080x2400 phone and on
  a 360dp-wide phone at the app's fixed reader font size.
- Exactly 20 new words (lemmas) that do not appear in wordlist.json, are not
  used by any earlier story, and are not in the starter list. List them in
  "new_words".
- Each new word appears at least 2-3 times in the story.
- Every non-starter surface form in the story has a glossary entry (including
  inflected forms, plurals, and separable verbs). Nouns include "de" or "het".
  Do not add glossary entries for starter words; the app adds them.
- Every non-lemma form gets its own form-specific "meaning" (for example
  "gaat": "goes"), never a copy of its lemma's meaning. Add a lemma entry for
  every referenced lemma (unless it is a starter word) so the popup can show
  "form of: <lemma>, <meaning>".
- If one surface form genuinely has two different senses in the story, use the
  smallest set of "occurrence" entries (1-based) instead of one combined gloss.
  A form has at most one default entry (no "occurrence") plus unique occurrence
  entries.
- Use curly double quotes for dialogue (“...”).
- Do not use digits, abbreviations, or hyphenated compounds.
- Proper names are glossed with "pos": "name" and are not in "new_words".
- Recycled vocabulary should make up at least <COVERAGE>% of the running words
  (proper names excluded), excluding the 20 new words.
- The story must have a hook in the first ~20%, a problem or surprise in the
  middle, and a satisfying or funny payoff in the last ~20%.
- Keep the cast small and the sections short.
- All content must be appropriate for all ages.

Output format:
Return ONLY one JSON object, with no markdown fences and no commentary, using
exactly this shape:

{
  "id": "<level>-<nn>",
  "level": "<LEVEL>",
  "order": <N>,
  "title": "...",
  "blurb_nl": "...",
  "blurb_en": "...",
  "sections": ["...", "...", "..."],
  "new_words": ["...20 lemmas..."],
  "glossary": [
    { "form": "...", "lemma": "...", "pos": "...", "article": "de|het", "meaning": "...", "new": true|false },
    { "form": "...", "lemma": "...", "pos": "...", "meaning": "...", "occurrence": 1, "new": false }
  ]
}

Use "parts": ["...", "..."] for separable verbs, for example:
{ "form": "doet", "parts": ["doet", "open"], "lemma": "opendoen", "pos": "verb", "meaning": "opens", "new": true }

Before returning, check:
1. exactly 20 distinct new_words;
2. no starter word appears in new_words;
3. every new word appears at least twice;
4. every non-starter token in the text has a glossary entry;
5. every noun has an article;
6. every non-lemma form has its own meaning, not its lemma's meaning, and every
   referenced lemma has a lemma entry;
7. ambiguous forms use the smallest set of occurrence entries, with no combined
   gloss;
8. all dialogue uses curly double quotes;
9. the text is long enough to fill at least 5 screens (use the total-word
   target), and there is no page count.
```

### 11.3 Repair prompt

If the validator reports errors, paste them back with the story and ask for a
minimal patch:

```text
The DutchReader validator reports these errors for the story below:

<paste the validator output>

Fix ONLY what is needed to clear the errors. Keep the plot, the 20 new words,
and the section structure unchanged unless an error requires changing them.
Return the complete corrected JSON object and nothing else. Re-check that every
non-starter surface form still has a glossary entry, that no starter word is in
new_words, that every non-lemma form has its own meaning (not its lemma's), that
ambiguous forms use occurrence entries instead of a combined gloss, and that
exactly 20 new words remain.
```

### 11.4 What to do with the output

1. Save the JSON to the correct path.
2. Run the validator and the repair loop until there are 0 errors.
3. Regenerate the wordlist.
4. Read the story yourself. The AI can satisfy the validator and still write
   dull or unnatural prose; the quality pass is your job.

---

## 12. Reference stories

All four shelves are complete: `a1/01` through `a1/25`, `a2/01` through
`a2/25`, `b1/01` through `b1/25`, and `b2/01` through `b2/25` are present.
Every story introduces exactly 20 new lemmas and uses only previously known
vocabulary plus its own new words. A2-02 through A2-25, B1-02 through B1-25,
and B2-02 through B2-25 avoid the sample-story debt and every earlier story's
owned lemmas, and none of them adds a coverage or repetition warning.

The four sample stories below remain the reference implementations for
structure, difficulty, and glossary style. Read them before writing a new story
at the same level.

| File | Level | Title | Sections | Screens (large/small) | Tokens | New words | Notes |
|------|-------|-------|----------|-----------------------|--------|-----------|-------|
| `a1/01-de-rode-bal.json` | A1 | De rode bal | 62 | 5+ / 5+ | 697 | 20 | A1-01 exception: 20 new lemmas plus starter words only; heavy repetition. |
| `a2/01-de-sleutel-is-weg.json` | A2 | De sleutel is weg | 65 | 5+ / 5+ | 622 | 20 | Perfect tense, separable verbs, domestic mystery, dialogue. |
| `b1/01-de-laatste-stroopwafel.json` | B1 | De laatste stroopwafel | 15 | 5+ / 5+ | 846 | 20 | Mixed tenses, opinions, humour, cultural setting. |
| `b2/01-het-sollicitatiegesprek.json` | B2 | Het sollicitatiegesprek | 14 | 5+ / 5+ | 1053 | 20 | Formal register, conditionals, passive, workplace farce. |

All 100 stories pass the validator in `--allow-gaps` mode with 0 errors. The
only warnings are the expected `ASSUMED_KNOWN` gaps for `a2-01`, `b1-01`, and
`b2-01`; every other shelf is clean. The generated `wordlist.json` (100 stories,
2,025 owned lemmas) and `assumed_known.json` (244 sample-story debt words) are
the current source of truth for what the learner knows and what the sample
stories still owe.

---

## 13. Appendix: coverage table and budgets

### 13.1 Coverage targets by story number

| Story | A1 | A2 | B1 | B2 |
|-------|----|----|----|----|
| 1 | 0% | 75% | 82% | 85% |
| 5 | 13% | 77% | 83% | 86% |
| 10 | 30% | 80% | 85% | 87% |
| 15 | 47% | 83% | 87% | 88% |
| 20 | 63% | 85% | 88% | 89% |
| 25 | 80% | 88% | 90% | 90% |

These are warnings, not hard errors. Aim at or above them.

### 13.2 Story and vocabulary budgets

| Level | Total words | Minimum screens | New words per story | Cumulative new lemmas after 25 stories |
|-------|-------------|-----------------|---------------------|----------------------------------------|
| A1 | 600–750 | 5 | 20 | 500 |
| A2 | 650–850 | 5 | 20 | 500 |
| B1 | 850–1,100 | 5 | 20 | 500 |
| B2 | 1,050–1,300 | 5 | 20 | 500 |

Pages are computed at runtime and are not a budget. The 5-screen minimum is
checked on both reference phones at the fixed reader font size.

### 13.3 Common mistakes

1. **Counting surface forms instead of lemmas.** `liep` is not a new word if
   `lopen` was introduced.
2. **Forgetting an inflected form in the glossary.** The validator catches it,
   but only if you run it.
3. **Using straight single quotes for dialogue.** Use curly double quotes.
4. **Writing 20 nouns.** Mix word types.
5. **Using a word from a later story.** Check `wordlist.json`, not memory.
6. **Introducing the same lemma twice.** `DUPLICATE_INTRODUCTION` is an error.
7. **Making A1-01 too rich.** It may use only its 20 new lemmas plus starter
   words and proper names.
8. **Ignoring coverage.** A story can be error-free and still too hard.
9. **Hand-editing `wordlist.json`.** Regenerate it.
10. **Forgetting to rebuild after adding a file.** The APK embeds stories at
    build time.

---

*End of guide. When in doubt, follow the validator and the four sample files.*
