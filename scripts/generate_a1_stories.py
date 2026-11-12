# -*- coding: utf-8 -*-
"""Generate A1 stories 02-25 from the theme word bank and core scenes."""
import json
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from a1_theme_data import THEMES
from a1_story_text import CORES

ROOT = Path(__file__).resolve().parent.parent
ASSETS = ROOT / "assets"
OUT_DIR = ASSETS / "stories" / "a1"

WORD_RE = re.compile(r"[A-Za-zÀ-ÿ0-9]+(?:['’][A-Za-zÀ-ÿ0-9]+)?")


def normalize(word: str) -> str:
    return word.replace("\u2019", "'").lower()


def tokenize(text: str):
    return WORD_RE.findall(text)


def load_json(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


def build_known():
    """Map every starter/A1-01 surface form to a glossary-like record."""
    known = {}
    starter = load_json(ASSETS / "starter_words.json")
    starter_lemmas = set()
    for word in starter["words"]:
        lemma = normalize(word["lemma"])
        starter_lemmas.add(lemma)
        form_meanings = word.get("form_meanings", {})
        for form in word["forms"]:
            known[normalize(form)] = {
                "form": form,
                "lemma": lemma,
                "pos": word["pos"],
                "meaning": form_meanings.get(form, word["meaning"]),
                "article": None,
                "starter": True,
                "new": False,
            }

    a1 = load_json(ASSETS / "stories" / "a1" / "01-de-rode-bal.json")
    for entry in a1["glossary"]:
        key = normalize(entry["form"])
        if key in known and known[key]["starter"]:
            continue
        known[key] = {
            "form": entry["form"],
            "lemma": normalize(entry["lemma"]),
            "pos": entry["pos"],
            "meaning": entry["meaning"],
            "article": entry.get("article"),
            "starter": False,
            "new": False,
        }

    # A1-01 does not list every regular present-tense form it uses.
    extra = {
        "speelt": ("spelen", "verb", "plays"),
        "gaat": ("gaan", "verb", "goes"),
        "loopt": ("lopen", "verb", "walks"),
        "kijkt": ("kijken", "verb", "looks"),
        "komt": ("komen", "verb", "comes"),
        "pakt": ("pakken", "verb", "takes / grabs"),
        "geeft": ("geven", "verb", "gives"),
        "wil": ("willen", "verb", "wants"),
        "ziet": ("zien", "verb", "sees"),
        "gaan": ("gaan", "verb", "to go"),
        "lopen": ("lopen", "verb", "to walk"),
        "kijken": ("kijken", "verb", "to look"),
        "komen": ("komen", "verb", "to come"),
        "pakken": ("pakken", "verb", "to take / grab"),
        "geven": ("geven", "verb", "to give"),
        "willen": ("willen", "verb", "to want"),
        "zien": ("zien", "verb", "to see"),
        "spelen": ("spelen", "verb", "to play"),
    }
    for form, (lemma, pos, meaning) in extra.items():
        key = normalize(form)
        if key not in known:
            known[key] = {
                "form": form,
                "lemma": lemma,
                "pos": pos,
                "meaning": meaning,
                "article": None,
                "starter": False,
                "new": False,
            }

    names = {
        "sam": ("Sam", "Sam (a name)"),
        "noor": ("Noor", "Noor (a name)"),
        "bo": ("Bo", "Bo (the dog's name)"),
    }
    for key, (form, meaning) in names.items():
        known[key] = {
            "form": form,
            "lemma": key,
            "pos": "name",
            "meaning": meaning,
            "article": None,
            "starter": False,
            "new": False,
        }
    return known, starter_lemmas


def build_theme_lexicon(theme):
    lex = {}

    def add(form, lemma, pos, meaning, article=None, kind=None, frame=None):
        key = normalize(form)
        if key in lex:
            raise SystemExit(f"duplicate form in theme {theme['order']}: {form}")
        lex[key] = {
            "form": form,
            "lemma": lemma,
            "pos": pos,
            "meaning": meaning,
            "article": article,
            "kind": kind,
            "frame": frame,
            "new": True,
        }

    for lemma, article, meaning, kind in theme["nouns"]:
        add(lemma, lemma, "noun", meaning, article=article, kind=kind)
    for lemma, frame, meaning, forms in theme["verbs"]:
        add(lemma, lemma, "verb", meaning, frame=frame)
        for form, form_meaning in forms.items():
            if normalize(form) == normalize(lemma):
                continue
            add(form, lemma, "verb", form_meaning, frame=frame)
    for lemma, meaning in theme["adjectives"]:
        add(lemma, lemma, "adjective", meaning)
    for lemma, meaning in theme["adverbs"]:
        add(lemma, lemma, "adverb", meaning)
    return lex


def check_theme_lexicon(theme, lex, known):
    for key, entry in lex.items():
        if key in known and not known[key]["starter"]:
            raise SystemExit(
                f"theme {theme['order']}: new form `{entry['form']}` collides with "
                f"known form `{known[key]['form']}` (lemma {known[key]['lemma']})"
            )
        if key in known and known[key]["starter"]:
            raise SystemExit(
                f"theme {theme['order']}: new form `{entry['form']}` collides with starter `{key}`"
            )


def word_count(sections):
    return sum(len(tokenize(section)) for section in sections)


def definite(entry):
    return entry.get("article") or "de"


# Open or surface places take `op`; enclosed places take `in`.
OP_PLACES = {
    "plein",
    "station",
    "perron",
    "strand",
    "pad",
    "veld",
    "feest",
    "berg",
    "top",
    "brug",
    "boerderij",
    "meer",
    "oever",
}


def place_prep(word):
    return "op" if word in OP_PLACES else "in"


def known_only_filler():
    """Neutral sentences that use only starter and A1-01 vocabulary."""
    block = [
        "Sam is hier. Noor is hier. Bo is hier.",
        "We zijn hier. We zijn daar. We zijn samen.",
        "Sam is blij. Noor is blij. Bo is blij.",
        "We zijn blij. We zijn samen. We zijn altijd samen.",
        "Sam en Noor zijn samen. Bo is samen.",
        "Sam en Bo zijn samen. Noor is hier.",
        "Noor en Bo zijn samen. Sam is hier.",
        "Sam ziet Noor. Noor ziet Sam. Bo ziet Sam en Noor.",
        "We kijken. We zien Sam en Noor.",
        "Sam kijkt naar de hond. Noor kijkt naar de bal.",
        "We kijken naar het huis. We kijken naar de tuin.",
        "We lopen samen. We zijn samen.",
        "Sam loopt naar het huis. Noor loopt naar het huis.",
        "Bo loopt naar het huis. Bo is blij.",
        "Sam loopt met de hond. Noor loopt met de bal.",
        "We lopen met de hond. We lopen met de bal.",
        "We gaan naar huis. We gaan naar de tuin.",
        "We gaan naar het huis. We gaan naar de tuin.",
        "Sam komt naar het huis. Noor komt naar de tuin.",
        "Bo komt naar Sam. Bo is blij.",
        "Sam komt. Noor komt. Bo komt.",
        "We komen naar het huis. We komen naar de tuin.",
        "De hond is groot. De hond is niet klein.",
        "Het huis is groot. De tuin is groot.",
        "De bal is rood. De bal is niet groot.",
        "De hond is in de tuin. De bal is in het huis.",
        "Sam is in het huis. Noor is in de tuin.",
        "Bo is in de tuin. De hond is in de tuin.",
        "We spelen samen. We zijn blij.",
        "We spelen met de bal. We zijn blij.",
        "Sam speelt met de hond. Noor speelt met de bal.",
        "Bo speelt met de bal. Bo is blij.",
        "We spelen in de tuin. We zijn blij.",
        "Sam pakt de bal. Noor pakt de bal.",
        "Bo pakt de bal. Bo is blij.",
        "We pakken de bal. We zijn blij.",
        "Sam geeft de bal aan Noor. Noor is blij.",
        "Noor geeft de bal aan Sam. Sam is blij.",
        "Sam geeft de bal aan Bo. Bo is blij.",
        "Bo geeft de bal aan Sam. Sam is blij.",
        "We geven de bal aan de hond. We zijn blij.",
        "Sam heeft de bal. Noor heeft de bal.",
        "Bo heeft de bal. Bo is blij.",
        "We hebben de bal. We zijn blij.",
        "Sam wil de hond zien. Noor wil de bal zien.",
        "We willen de hond zien. We zijn blij.",
        "Sam ziet de hond. Noor ziet de bal.",
        "Bo ziet Sam. Bo is blij.",
        "We zien de hond. We zien de bal.",
        "We zien Sam en Noor. We zijn blij.",
        "Sam is hier. Noor is daar. Bo is hier.",
        "We zijn niet in het huis. We zijn in de tuin.",
        "De hond is niet in het huis. De hond is in de tuin.",
        "Sam heeft geen bal. Noor heeft de bal.",
        "We hebben geen hond. We hebben een bal.",
        "Hallo, Sam! Hallo, Noor! Hallo, Bo!",
        "Hallo, hond! Hallo, bal! Hallo, huis!",
        "Hallo! Ik ben Sam. Hier is Noor. Hier is Bo.",
    ]
    out = []
    while len(out) < 240:
        out.extend(block)
    return out


def coverage(sections, lex, known):
    total = 0
    new = 0
    for section in sections:
        for token in tokenize(section):
            key = normalize(token)
            if key in lex:
                total += 1
                new += 1
            elif key in known and known[key]["pos"] != "name":
                total += 1
    if total == 0:
        return 1.0
    return (total - new) / total


def short_known_filler():
    """Very short known-word sentences for a final coverage top-up."""
    block = [
        "We zijn blij.",
        "Sam is hier.",
        "Noor is hier.",
        "Bo is blij.",
        "We zijn samen.",
        "Sam ziet Noor.",
        "Noor ziet Sam.",
        "We zijn hier.",
        "We gaan naar huis.",
        "Sam is blij.",
        "Noor is blij.",
        "We zijn altijd samen.",
        "De hond is groot.",
        "De bal is rood.",
        "Het huis is groot.",
        "De tuin is groot.",
    ]
    out = []
    while len(out) < 240:
        out.extend(block)
    return out


def is_theme_sentence(sentence, lex):
    return any(normalize(token) in lex for token in tokenize(sentence))


def interleave_filler(theme_list, glue_list):
    """Alternate one theme sentence with two neutral sentences."""
    out = []
    ti = gi = 0
    while ti < len(theme_list) or gi < len(glue_list):
        if ti < len(theme_list):
            out.append(theme_list[ti])
            ti += 1
        for _ in range(2):
            if gi < len(glue_list):
                out.append(glue_list[gi])
                gi += 1
    return out


def ensure_theme_ending(sections, head_len, lex):
    """Move a theme sentence to the end of the filler, if one exists."""
    body = sections[head_len:]
    if not body or is_theme_sentence(body[-1], lex):
        return
    for index in range(len(body) - 1, -1, -1):
        if is_theme_sentence(body[index], lex):
            body.append(body.pop(index))
            break
    sections[head_len:] = body


def late_filler(theme, lex):
    """A theme-aware block for the late A1 stories.

    Every sentence uses a theme noun plus only starter/A1-01 words, so the
    block stays on topic without spending the coverage budget on new verbs.
    """
    nouns = [lex[normalize(n[0])] for n in theme["nouns"]]
    out = []
    for noun in nouns:
        art = definite(noun)
        word = noun["form"]
        kind = noun["kind"]
        if kind == "place":
            prep = place_prep(word)
            out.append(f"We zijn {prep} {art} {word}.")
            out.append(f"We gaan naar {art} {word}.")
            out.append(f"Sam loopt naar {art} {word}.")
            out.append(f"{art.capitalize()} {word} is groot.")
        elif kind == "object":
            out.append(f"We zien {art} {word}.")
            out.append(f"Sam ziet {art} {word}.")
            out.append(f"Noor ziet {art} {word}.")
            out.append(f"{art.capitalize()} {word} is groot.")
        elif kind == "animal":
            out.append(f"We zien {art} {word}.")
            out.append(f"Sam ziet {art} {word}.")
            out.append(f"Noor ziet {art} {word}.")
            out.append(f"We kijken naar {art} {word}.")
        elif kind == "person":
            out.append(f"{art.capitalize()} {word} komt.")
            out.append(f"We zien {art} {word}.")
            out.append(f"{art.capitalize()} {word} is hier.")
        else:
            out.append(f"We zien {art} {word}.")
            out.append(f"{art.capitalize()} {word} is groot.")
            out.append(f"{art.capitalize()} {word} is hier.")

    first = nouns[0]
    out.append(f"Sam ziet {definite(first)} {first['form']}. Sam is blij.")
    out.append(f"Noor ziet {definite(first)} {first['form']}. Noor is blij.")
    out.append("We zijn samen. We zijn blij. We zijn altijd samen.")
    return out


def filler_sentences(theme, lex):
    """Return an endless-ish list of safe filler sentences using theme nouns."""
    nouns = [lex[normalize(n[0])] for n in theme["nouns"]]
    by_kind = {}
    for noun in nouns:
        by_kind.setdefault(noun["kind"], []).append(noun)

    out = []

    def add(text):
        out.append(text)

    # Per-noun patterns, grouped by kind so the grammar stays simple.
    for noun in nouns:
        art = definite(noun)
        word = noun["form"]
        kind = noun["kind"]
        if kind == "place":
            prep = place_prep(word)
            add(f"We zijn {prep} {art} {word}.")
            add(f"We gaan naar {art} {word}.")
            add(f"Sam loopt naar {art} {word}.")
            add(f"Noor gaat naar {art} {word}.")
            add(f"{art.capitalize()} {word} is hier.")
        elif kind == "object":
            add(f"We zien {art} {word}.")
            add(f"Sam ziet {art} {word}.")
            add(f"Noor ziet {art} {word}.")
            add(f"Sam pakt {art} {word}.")
            add(f"We spelen met {art} {word}.")
            add(f"{art.capitalize()} {word} is hier.")
        elif kind == "animal":
            add(f"We zien {art} {word}.")
            add(f"Sam ziet {art} {word}.")
            add(f"Noor ziet {art} {word}.")
            add(f"We kijken naar {art} {word}.")
            add(f"{art.capitalize()} {word} is hier.")
            add(f"{art.capitalize()} {word} is groot.")
        elif kind == "person":
            add(f"{art.capitalize()} {word} komt.")
            add(f"We zien {art} {word}.")
            add(f"{art.capitalize()} {word} is hier.")
            add(f"{art.capitalize()} {word} is in het huis.")
        else:  # abstract
            add(f"We zien {art} {word}.")
            add(f"{art.capitalize()} {word} is hier.")

    # A few cross-noun sentences.
    first = nouns[0]
    second = nouns[1]
    fifth = nouns[4]
    add(f"Hallo, {first['form']}! Hallo, {second['form']}! Hallo, {fifth['form']}!")
    add(f"Sam geeft {definite(first)} {first['form']} aan Noor.")
    add(f"Noor geeft {definite(first)} {first['form']} aan Sam.")
    add(f"Bo ziet {definite(first)} {first['form']}. Bo is blij.")

    # Generic known-word glue at the end, so the story stays on theme while
    # the filler is being consumed.
    add("Sam is in het huis. Noor is in het huis. Bo is in de tuin.")
    add("We zijn hier. We zijn daar. We zijn samen.")
    add("We kijken. We zien de hond. De hond is groot.")
    add("We lopen naar het huis. We lopen naar de tuin.")
    add("We spelen samen. We zijn blij. We zijn altijd samen.")
    add("We zijn samen. We zijn blij. We zijn altijd samen.")
    add("Hallo! Ik ben Sam. Hier is Noor. Hier is Bo.")

    # Repeat the block until it is comfortably longer than any target.
    block = list(out)
    while len(out) < 400:
        out.extend(block)
    return out


def build_story(theme, known, starter_lemmas):
    order = theme["order"]
    lex = build_theme_lexicon(theme)
    check_theme_lexicon(theme, lex, known)

    # Keep the final three core sentences as the payoff.  Filler goes between
    # the body and the payoff so every story still ends on its own resolution.
    core = list(CORES[order])
    head = core[:-3]
    tail = core[-3:]
    sections = list(head)

    def add_filler(filler_list):
        index = 0
        while word_count(sections + tail) < 650 and index < len(filler_list):
            sections.append(filler_list[index])
            index += 1

    add_filler(filler_sentences(theme, lex))

    # Late A1 stories need a high known-word ratio.  If the themed filler
    # pushes coverage below the target, rebuild the filler from theme-aware
    # sentences followed by pure A1-01 glue.
    target = 0.80 * (order - 1) / 24.0
    if coverage(sections + tail, lex, known) < target:
        sections = list(head)
        theme_block = late_filler(theme, lex)
        while len(theme_block) < 160:
            theme_block.extend(late_filler(theme, lex))
        for sentence in interleave_filler(theme_block, known_only_filler()):
            if (
                word_count(sections + tail) >= 650
                and coverage(sections + tail, lex, known) >= target
            ):
                break
            sections.append(sentence)

    # Do not overshoot the upper bound; trim filler only, never core.
    core_len = len(head)
    while word_count(sections + tail) > 740 and len(sections) > core_len:
        sections.pop()

    # Every new lemma should appear at least twice.  Repetition sentences go
    # before the payoff so the story still ends on its resolution.
    counts = {}
    for section in sections + tail:
        for token in tokenize(section):
            key = normalize(token)
            entry = lex.get(key)
            if entry:
                counts[entry["lemma"]] = counts.get(entry["lemma"], 0) + 1
    for lemma in [n[0] for n in theme["nouns"]] + [v[0] for v in theme["verbs"]] + [
        a[0] for a in theme["adjectives"]
    ] + [d[0] for d in theme["adverbs"]]:
        if counts.get(lemma, 0) < 2:
            # Add a direct repetition from the core that mentions this lemma.
            for section in CORES[order]:
                if lemma in {normalize(t) for t in tokenize(section)}:
                    sections.append(section)
                    break
            else:
                raise SystemExit(f"theme {order}: new word `{lemma}` appears fewer than twice")

    # Repetition sentences can be new-word heavy, so top up coverage again.
    # Short sentences keep this top-up from pushing the story over 750 words.
    glue = short_known_filler()
    index = 0
    while coverage(sections + tail, lex, known) < target and index < len(glue):
        sections.append(glue[index])
        index += 1

    ensure_theme_ending(sections, len(head), lex)
    sections.extend(tail)

    # Build the glossary.
    glossary = []
    seen_forms = set()

    def push(entry):
        key = normalize(entry["form"])
        if key in seen_forms:
            return
        seen_forms.add(key)
        item = {
            "form": entry["form"],
            "lemma": entry["lemma"],
            "pos": entry["pos"],
            "meaning": entry["meaning"],
            "new": entry["new"],
        }
        if entry.get("article"):
            item["article"] = entry["article"]
        glossary.append(item)

    # New words first, including their lemma entries even when the lemma form
    # itself is not used in the text.
    for noun in theme["nouns"]:
        push(lex[normalize(noun[0])])
    for verb in theme["verbs"]:
        push(lex[normalize(verb[0])])
    for adj in theme["adjectives"]:
        push(lex[normalize(adj[0])])
    for adv in theme["adverbs"]:
        push(lex[normalize(adv[0])])

    # Then every used form: new inflections, A1-01 words and names.
    unknown = []
    for section in sections:
        for token in tokenize(section):
            key = normalize(token)
            if key in lex:
                push(lex[key])
            elif key in known:
                if known[key]["starter"]:
                    continue
                push(known[key])
            else:
                unknown.append(token)
    if unknown:
        raise SystemExit(f"theme {order}: unknown token(s): {sorted(set(unknown))}")

    new_words = [n[0] for n in theme["nouns"]] + [v[0] for v in theme["verbs"]] + [
        a[0] for a in theme["adjectives"]
    ] + [d[0] for d in theme["adverbs"]]

    story = {
        "id": f"a1-{order:02d}",
        "level": "A1",
        "order": order,
        "title": theme["title"],
        "blurb_nl": theme["blurb_nl"],
        "blurb_en": theme["blurb_en"],
        "sections": sections,
        "new_words": new_words,
        "glossary": glossary,
    }
    return story


def main():
    known, starter_lemmas = build_known()
    OUT_DIR.mkdir(parents=True, exist_ok=True)

    problems = []
    for theme in THEMES:
        story = build_story(theme, known, starter_lemmas)
        count = word_count(story["sections"])
        if not 600 <= count <= 750:
            problems.append(f"a1-{theme['order']:02d}: {count} words")
        path = OUT_DIR / f"{theme['order']:02d}-{theme['slug']}.json"
        path.write_text(
            json.dumps(story, ensure_ascii=False, indent=2) + "\n",
            encoding="utf-8",
        )
        print(f"a1-{theme['order']:02d}: {count} words, {len(story['glossary'])} glossary entries -> {path.name}")

    if problems:
        print("Length problems:")
        for problem in problems:
            print(" -", problem)
        sys.exit(1)


if __name__ == "__main__":
    main()
