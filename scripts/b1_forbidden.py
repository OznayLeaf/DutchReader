# -*- coding: utf-8 -*-
"""Vocabulary guards for the B1 stories 02-25.

The B1 shelf has to respect three hard rules:

* a B1 story may not introduce a lemma that any earlier story owns;
* it may not introduce a lemma that B1-01 uses as assumed-known debt,
  because B1-01 would then be using the word before its introduction;
* it may not introduce a B2-01 word either.

The helpers below build the forbidden lemma set and the known surface-form
map from the assets on disk.  They are shared by the theme data check, the
generator and the text checker.
"""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
ASSETS = ROOT / "assets"


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
    """Every lemma a B1-02..25 story must not list in new_words."""
    return set(wordlist_owners()) | starter_lemmas() | assumed_words()


def b1_01_new_words():
    story = load_json(ASSETS / "stories" / "b1" / "01-de-laatste-stroopwafel.json")
    return [normalize(word) for word in story["new_words"]]


def known_forms():
    """Map known surface forms to their lemma.

    Only glossary entries whose lemma is actually owned by an earlier story
    (or by the starter, or by B1-01) count.  Extra glossary entries in the
    sample stories are assumed-known debt, not owned vocabulary.
    """
    owned = set(wordlist_owners())
    starter = starter_lemmas()
    known = {}

    starter_data = load_json(ASSETS / "starter_words.json")
    for word in starter_data["words"]:
        lemma = normalize(word["lemma"])
        for form in word["forms"]:
            known[normalize(form)] = lemma

    for level in ("a1", "a2"):
        for path in sorted((ASSETS / "stories" / level).glob("*.json")):
            story = load_json(path)
            for entry in story["glossary"]:
                lemma = normalize(entry["lemma"])
                if lemma in owned or lemma in starter:
                    known.setdefault(normalize(entry["form"]), lemma)

    story = load_json(ASSETS / "stories" / "b1" / "01-de-laatste-stroopwafel.json")
    b1_new = set(b1_01_new_words())
    for entry in story["glossary"]:
        lemma = normalize(entry["lemma"])
        if lemma in b1_new:
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


def check_candidate(lemma, seen_forms=None):
    """Return a list of problems for one candidate lemma."""
    lemma = normalize(lemma)
    problems = []
    if lemma in forbidden_lemmas():
        problems.append("forbidden lemma")
    if seen_forms and lemma in seen_forms:
        problems.append("surface form already known")
    return problems
