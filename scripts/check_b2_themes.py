# -*- coding: utf-8 -*-
"""Check the B2 theme bank before the generator runs."""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
import b2_forbidden as guard
from b2_theme_data import THEMES


def main():
    known = guard.known_forms()
    problems = []
    seen = {}
    for theme in THEMES:
        order = theme["order"]
        lemmas = []
        lemmas += [row[0] for row in theme["nouns"]]
        lemmas += [row[0] for row in theme["verbs"]]
        lemmas += [row[0] for row in theme["adjectives"]]
        lemmas += [row[0] for row in theme["adverbs"]]
        if len(lemmas) != 20:
            problems.append(f"{order}: {len(lemmas)} lemmas, expected 20")
        if len(set(lemmas)) != len(lemmas):
            problems.append(f"{order}: duplicate lemma inside theme")
        if len(theme["nouns"]) != 10:
            problems.append(f"{order}: {len(theme['nouns'])} nouns")
        if len(theme["verbs"]) != 3:
            problems.append(f"{order}: {len(theme['verbs'])} verbs")
        if len(theme["adjectives"]) != 5:
            problems.append(f"{order}: {len(theme['adjectives'])} adjectives")
        if len(theme["adverbs"]) != 2:
            problems.append(f"{order}: {len(theme['adverbs'])} adverbs")
        for lemma in lemmas:
            key = guard.normalize(lemma)
            if key in seen:
                problems.append(f"{order}: {lemma} also used in theme {seen[key]}")
            seen[key] = order
            if key in guard.forbidden_lemmas():
                problems.append(f"{order}: lemma {lemma} is forbidden/known")
            if key in known:
                problems.append(f"{order}: lemma {lemma} is already a known form")
        for lemma, stem, past, participle, meaning in theme["verbs"]:
            key = guard.normalize(lemma)
            forms = {stem, stem + "t", past, past + "en", participle}
            for form in forms:
                form_key = guard.normalize(form)
                if form_key == key:
                    continue
                if form_key in known:
                    problems.append(
                        f"{order}: verb {lemma} form {form} is already known"
                    )
    if problems:
        print(f"{len(problems)} problem(s):")
        for problem in problems:
            print(" -", problem)
        sys.exit(1)
    print(f"OK: {len(THEMES)} themes, {len(seen)} unique lemmas")


if __name__ == "__main__":
    main()
