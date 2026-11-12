# -*- coding: utf-8 -*-
"""Check the B1 theme word bank before generating stories."""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
import b1_forbidden as guard
from b1_theme_data import THEMES

EXPECTED = 20


def main():
    forbidden = guard.forbidden_lemmas()
    forms = guard.known_forms()
    problems = []
    all_lemmas = {}
    for theme in THEMES:
        order = theme["order"]
        lemmas = []
        for lemma, article, meaning, kind in theme["nouns"]:
            lemmas.append(lemma)
            if not article:
                problems.append(f"{order}: noun {lemma} has no article")
        for lemma, stem, past, participle, meaning in theme["verbs"]:
            lemmas.append(lemma)
            for form in (stem, stem + "t", lemma, past, past + "en", participle):
                key = guard.normalize(form)
                if key in forms:
                    problems.append(f"{order}: verb {lemma} form {form} is already known")
        for lemma, meaning in theme["adjectives"]:
            lemmas.append(lemma)
        for lemma, meaning in theme["adverbs"]:
            lemmas.append(lemma)
        if len(lemmas) != EXPECTED:
            problems.append(f"{order}: {len(lemmas)} lemmas instead of {EXPECTED}")
        if len(set(lemmas)) != len(lemmas):
            problems.append(f"{order}: duplicate lemma in theme")
        for lemma in lemmas:
            key = guard.normalize(lemma)
            if key in forbidden:
                problems.append(f"{order}: {lemma} is forbidden")
            if key in forms:
                problems.append(f"{order}: {lemma} is already a known form")
            if key in all_lemmas:
                problems.append(f"{order}: {lemma} also used in theme {all_lemmas[key]}")
            all_lemmas[key] = order
    if problems:
        print(f"{len(problems)} problem(s):")
        for problem in problems:
            print(" -", problem)
        sys.exit(1)
    print(f"OK: {len(THEMES)} themes, {len(all_lemmas)} unique lemmas")


if __name__ == "__main__":
    main()
