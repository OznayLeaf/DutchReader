# -*- coding: utf-8 -*-
"""Check the A1 theme word bank against the cumulative word list."""
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from a1_theme_data import THEMES

ROOT = Path(__file__).resolve().parent.parent
ASSETS = ROOT / "assets"

wordlist = json.loads((ASSETS / "wordlist.json").read_text(encoding="utf-8"))
owned = wordlist["words"]
starter = {
    w["lemma"]
    for w in json.loads((ASSETS / "starter_words.json").read_text(encoding="utf-8"))["words"]
}
a1_01 = {
    "hond", "bal", "huis", "tuin", "groot", "klein", "rood", "blij",
    "gaan", "lopen", "kijken", "komen", "pakken", "geven", "willen",
    "zien", "spelen", "samen", "altijd", "hallo",
}

errors = []
seen = {}
for theme in THEMES:
    nouns = theme["nouns"]
    verbs = theme["verbs"]
    adjs = theme["adjectives"]
    advs = theme["adverbs"]
    if (len(nouns), len(verbs), len(adjs), len(advs)) != (8, 6, 4, 2):
        errors.append(f"{theme['order']}: wrong counts {(len(nouns), len(verbs), len(adjs), len(advs))}")
    lemmas = [n[0] for n in nouns] + [v[0] for v in verbs] + [a[0] for a in adjs] + [d[0] for d in advs]
    if len(set(lemmas)) != 20:
        errors.append(f"{theme['order']}: duplicate lemma inside theme: {lemmas}")
    for lemma in lemmas:
        if lemma in seen:
            errors.append(f"{theme['order']}: lemma `{lemma}` also in theme {seen[lemma]}")
        seen[lemma] = theme["order"]
        if lemma in starter:
            errors.append(f"{theme['order']}: `{lemma}` is a starter word")
        if lemma in a1_01:
            errors.append(f"{theme['order']}: `{lemma}` is an A1-01 word")
        if lemma in owned:
            errors.append(f"{theme['order']}: `{lemma}` is owned by {owned[lemma]}")

print(f"themes: {len(THEMES)}")
print(f"new lemmas: {len(seen)}")
if errors:
    print(f"{len(errors)} problem(s):")
    for e in errors:
        print(" -", e)
    sys.exit(1)
print("word bank is clean")
