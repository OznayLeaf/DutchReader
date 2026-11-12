# -*- coding: utf-8 -*-
"""Check A2 cores for unknown tokens and new-word usage."""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from a2_theme_data import THEMES
from a2_story_text import CORES
from generate_a2_stories import (
    build_known,
    build_theme_lexicon,
    normalize,
    tokenize,
)


def known_for(order):
    known = build_known()
    for theme in THEMES:
        if theme["order"] >= order:
            break
        lex = build_theme_lexicon(theme)
        for key, entry in lex.items():
            known[key] = dict(entry, starter=False, new=False)
    return known


def main(orders=None):
    problems = 0
    for theme in THEMES:
        order = theme["order"]
        if orders and order not in orders:
            continue
        if order not in CORES:
            print(f"a2-{order:02d}: NO CORE")
            problems += 1
            continue
        known = known_for(order)
        lex = build_theme_lexicon(theme)
        unknown = set()
        counts = {}
        for section in CORES[order]:
            for token in tokenize(section):
                key = normalize(token)
                entry = lex.get(key)
                if entry:
                    counts[entry["lemma"]] = counts.get(entry["lemma"], 0) + 1
                elif key not in known:
                    unknown.add(token)
        new_lemmas = (
            [n[0] for n in theme["nouns"]]
            + [v[0] for v in theme["verbs"]]
            + [a[0] for a in theme["adjectives"]]
            + [d[0] for d in theme["adverbs"]]
            + [c[0] for c in theme.get("conjunctions", [])]
            + [p[0] for p in theme.get("pronouns", [])]
        )
        missing = [lemma for lemma in new_lemmas if counts.get(lemma, 0) < 2]
        words = sum(len(tokenize(s)) for s in CORES[order])
        status = "ok" if not unknown and not missing else "PROBLEM"
        print(
            f"a2-{order:02d}: {len(CORES[order])} sections, {words} words, "
            f"{len(unknown)} unknown, missing<2={missing} [{status}]"
        )
        if unknown:
            print("   unknown:", ", ".join(sorted(unknown)))
            problems += 1
        if missing:
            print("   too rare:", ", ".join(missing))
            problems += 1
    return problems


if __name__ == "__main__":
    orders = [int(arg) for arg in sys.argv[1:]] or None
    sys.exit(1 if main(orders) else 0)
