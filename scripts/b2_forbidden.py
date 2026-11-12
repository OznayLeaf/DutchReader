# -*- coding: utf-8 -*-
"""Vocabulary guards for the B2 stories 02-25.

The B2 shelf has to respect four hard rules:

* a B2 story may not introduce a lemma that any earlier story owns;
* it may not introduce a lemma that A2-01, B1-01 or B2-01 uses as assumed-known
  debt, because those stories would then be using the word before its
  introduction;
* it must not re-introduce a word that B2-01 already owns;
* its text may only use starter, A1, A2, B1 and B2-01 owned vocabulary plus its
  own 20 new lemmas.

The helpers below build the forbidden lemma set and the known surface-form map
from the assets on disk.  They are shared by the theme check, the generator and
the text checker.
"""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
ASSETS = ROOT / "assets"
LEVELS = ("a1", "a2", "b1")


def normalize(word):
    return word.replace("\u2019", "'").lower()


def load_json(path):
    return json.loads(Path(path).read_text(encoding="utf-8"))


def starter_lemmas():
    data = load_json(ASSETS / "starter_words.json")
    return {normalize(word["lemma"]) for word in data["words"]}


def wordlist_owners():
    data = load_json(ASSETS / "wordlist.json")
    return {normalize(lemma): owner for lemma, owner in data["words"].items()}


def assumed_words():
    data = load_json(ASSETS / "assumed_known.json")
    return {normalize(lemma) for lemma in data["words"]}


def forbidden_lemmas():
    """Every lemma a B2-02..25 story must not list in new_words."""
    return set(wordlist_owners()) | starter_lemmas() | assumed_words()


def story_new_words(level, name):
    story = load_json(ASSETS / "stories" / level / name)
    return [normalize(word) for word in story["new_words"]]


def known_forms():
    """Map known surface forms to their lemma.

    Only glossary entries whose lemma is actually owned by an earlier story
    (or by the starter) count.  Extra glossary entries in the sample stories
    are assumed-known debt, not owned vocabulary.
    """
    owned = set(wordlist_owners())
    starter = starter_lemmas()
    known = {}

    starter_data = load_json(ASSETS / "starter_words.json")
    for word in starter_data["words"]:
        lemma = normalize(word["lemma"])
        for form in word["forms"]:
            known[normalize(form)] = lemma

    for level in LEVELS:
        for path in sorted((ASSETS / "stories" / level).glob("*.json")):
            story = load_json(path)
            for entry in story["glossary"]:
                lemma = normalize(entry["lemma"])
                if lemma in owned or lemma in starter:
                    known.setdefault(normalize(entry["form"]), lemma)

    # A few inflected forms the sample glossaries omit.
    extra = {
        "was": "zijn",
        "waren": "zijn",
        "had": "hebben",
        "hadden": "hebben",
    }
    for form, lemma in extra.items():
        known.setdefault(form, lemma)
    return known


def check_candidate(lemma, seen=None):
    """Return a list of problems for one candidate lemma."""
    lemma = normalize(lemma)
    problems = []
    if lemma in forbidden_lemmas():
        problems.append("forbidden lemma")
    if seen is not None and lemma in seen:
        problems.append("duplicate lemma")
    return problems
