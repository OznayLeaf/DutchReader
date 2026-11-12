# -*- coding: utf-8 -*-
"""Report unknown tokens in the hand-written A1 core scenes."""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from a1_theme_data import THEMES
from a1_story_text import CORES
from generate_a1_stories import build_known, build_theme_lexicon, normalize, tokenize

known, _ = build_known()
problems = []
for theme in THEMES:
    order = theme["order"]
    lex = build_theme_lexicon(theme)
    unknown = set()
    for section in CORES[order]:
        for token in tokenize(section):
            key = normalize(token)
            if key not in lex and key not in known:
                unknown.add(token)
    if unknown:
        problems.append((order, sorted(unknown)))
    # New-word counts in the core.
    counts = {}
    for section in CORES[order]:
        for token in tokenize(section):
            entry = lex.get(normalize(token))
            if entry:
                counts[entry["lemma"]] = counts.get(entry["lemma"], 0) + 1
    missing = [
        lemma
        for lemma in [n[0] for n in theme["nouns"]]
        + [v[0] for v in theme["verbs"]]
        + [a[0] for a in theme["adjectives"]]
        + [d[0] for d in theme["adverbs"]]
        if counts.get(lemma, 0) < 2
    ]
    if missing:
        problems.append((order, ["missing<2: " + ", ".join(missing)]))

for order, items in problems:
    print(f"theme {order}:")
    for item in items:
        print("  -", item)
if not problems:
    print("all core scenes use known words and repeat every new word")
