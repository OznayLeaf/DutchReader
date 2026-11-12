# -*- coding: utf-8 -*-
"""Check the hand-written B1 cores before the generator runs."""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
import b1_forbidden as guard
from b1_theme_data import THEMES
from b1_story_text import CORES
from generate_b1_stories import build_known, build_theme_lexicon, normalize, tokenize


def main():
    known = build_known()
    problems = []
    for theme in THEMES:
        order = theme["order"]
        lex = build_theme_lexicon(theme)
        if order not in CORES:
            problems.append(f"{order}: no core")
            continue
        used = set()
        unknown = set()
        for section in CORES[order]:
            for token in tokenize(section):
                key = normalize(token)
                if key in lex:
                    used.add(lex[key]["lemma"])
                elif key in known:
                    pass
                else:
                    unknown.add(token)
        if unknown:
            problems.append(f"{order}: unknown token(s): {sorted(unknown)}")
        new_lemmas = (
            [n[0] for n in theme["nouns"]]
            + [v[0] for v in theme["verbs"]]
            + [a[0] for a in theme["adjectives"]]
            + [d[0] for d in theme["adverbs"]]
        )
        missing = [lemma for lemma in new_lemmas if lemma not in used]
        if missing:
            problems.append(f"{order}: core misses {missing}")
        for key, entry in lex.items():
            known[key] = entry
    if problems:
        print(f"{len(problems)} problem(s):")
        for problem in problems:
            print(" -", problem)
        sys.exit(1)
    print(f"OK: {len(CORES)} cores")


if __name__ == "__main__":
    main()
