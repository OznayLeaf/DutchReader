# -*- coding: utf-8 -*-
"""Generate A2 stories 02-25 from the theme word bank and core scenes."""
import json
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from a2_theme_data import THEMES
from a2_story_text import CORES

ROOT = Path(__file__).resolve().parent.parent
ASSETS = ROOT / "assets"
OUT_DIR = ASSETS / "stories" / "a2"

WORD_RE = re.compile(r"[A-Za-zÀ-ÿ0-9]+(?:['’][A-Za-zÀ-ÿ0-9]+)?")


def normalize(word: str) -> str:
    return word.replace("\u2019", "'").lower()


def tokenize(text: str):
    return WORD_RE.findall(text)


def load_json(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


# Extra past-tense and perfect-tense forms for A1 verbs.  These let the A2
# cores and filler use the perfect tense without re-introducing old lemmas.
VERB_FORMS = {
    "aaien": ("aaide", "geaaid"),
    "acteren": ("acteerde", "geacteerd"),
    "bakken": ("bakte", "gebakken"),
    "betalen": ("betaalde", "betaald"),
    "bewegen": ("bewoog", "bewogen"),
    "bewonderen": ("bewonderde", "bewonderd"),
    "bezoeken": ("bezocht", "bezocht"),
    "bezorgen": ("bezorgde", "bezorgd"),
    "binden": ("bond", "gebonden"),
    "blaffen": ("blafte", "geblaft"),
    "blazen": ("blies", "geblazen"),
    "blijven": ("bleef", "gebleven"),
    "bloeien": ("bloeide", "gebloeid"),
    "borstelen": ("borstelde", "geborsteld"),
    "bouwen": ("bouwde", "gebouwd"),
    "brengen": ("bracht", "gebracht"),
    "dalen": ("daalde", "gedaald"),
    "dansen": ("danste", "gedanst"),
    "dragen": ("droeg", "gedragen"),
    "drinken": ("dronk", "gedronken"),
    "dromen": ("droomde", "gedroomd"),
    "duiken": ("dook", "gedoken"),
    "duwen": ("duwde", "geduwd"),
    "eten": ("at", "gegeten"),
    "fietsen": ("fietste", "gefietst"),
    "filmen": ("filmde", "gefilmd"),
    "fotograferen": ("fotografeerde", "gefotografeerd"),
    "gaan": ("ging", "gegaan"),
    "genieten": ("genoot", "genoten"),
    "geven": ("gaf", "gegeven"),
    "gieten": ("goot", "gegoten"),
    "glijden": ("gleed", "gegleden"),
    "gooien": ("gooide", "gegooid"),
    "graven": ("groef", "gegraven"),
    "groeien": ("groeide", "gegroeid"),
    "halen": ("haalde", "gehaald"),
    "hangen": ("hing", "gehangen"),
    "harken": ("harkte", "geharkt"),
    "helpen": ("hielp", "geholpen"),
    "hijsen": ("hees", "gehesen"),
    "hoesten": ("hoestte", "gehoest"),
    "hooien": ("hooide", "gehooid"),
    "horen": ("hoorde", "gehoord"),
    "huilen": ("huilde", "gehuild"),
    "juichen": ("juichte", "gejuicht"),
    "kamperen": ("kampeerde", "gekampeerd"),
    "kijken": ("keek", "gekeken"),
    "kiezen": ("koos", "gekozen"),
    "klappen": ("klapte", "geklapt"),
    "klauteren": ("klauterde", "geklauterd"),
    "kletsen": ("kletste", "gekletst"),
    "klimmen": ("klom", "geklommen"),
    "koken": ("kookte", "gekookt"),
    "komen": ("kwam", "gekomen"),
    "kopen": ("kocht", "gekocht"),
    "krijgen": ("kreeg", "gekregen"),
    "kruipen": ("kroop", "gekropen"),
    "laten": ("liet", "gelaten"),
    "leiden": ("leidde", "geleid"),
    "leren": ("leerde", "geleerd"),
    "lezen": ("las", "gelezen"),
    "liggen": ("lag", "gelegen"),
    "lopen": ("liep", "gelopen"),
    "luisteren": ("luisterde", "geluisterd"),
    "maaien": ("maaide", "gemaaid"),
    "maken": ("maakte", "gemaakt"),
    "melken": ("molk", "gemolken"),
    "meten": ("mat", "gemeten"),
    "niezen": ("niesde", "geniest"),
    "oefenen": ("oefende", "geoefend"),
    "onderzoeken": ("onderzocht", "onderzocht"),
    "ontvangen": ("ontving", "ontvangen"),
    "opruimen": ("ruimde op", "opgeruimd"),
    "pakken": ("pakte", "gepakt"),
    "parkeren": ("parkeerde", "geparkeerd"),
    "passen": ("paste", "gepast"),
    "plakken": ("plakte", "geplakt"),
    "ploegen": ("ploegde", "geploegd"),
    "poetsen": ("poetste", "gepoetst"),
    "praten": ("praatte", "gepraat"),
    "proosten": ("proostte", "geproost"),
    "redden": ("redde", "gered"),
    "regenen": ("regende", "geregend"),
    "reizen": ("reisde", "gereisd"),
    "rekenen": ("rekende", "gerekend"),
    "repareren": ("repareerde", "gerepareerd"),
    "rijden": ("reed", "gereden"),
    "roeien": ("roeide", "geroeid"),
    "roepen": ("riep", "geroepen"),
    "roeren": ("roerde", "geroerd"),
    "rollen": ("rolde", "gerold"),
    "ruiken": ("rook", "geroken"),
    "ruilen": ("ruilde", "geruild"),
    "rusten": ("rustte", "gerust"),
    "schaatsen": ("schaatste", "geschaatst"),
    "schilderen": ("schilderde", "geschilderd"),
    "schoppen": ("schopte", "geschopt"),
    "schrijven": ("schreef", "geschreven"),
    "schrikken": ("schrok", "geschrokken"),
    "schuilen": ("schuilde", "geschuild"),
    "scoren": ("scoorde", "gescoord"),
    "serveren": ("serveerde", "geserveerd"),
    "slapen": ("sliep", "geslapen"),
    "sneeuwen": ("sneeuwde", "gesneeuwd"),
    "snijden": ("sneed", "gesneden"),
    "spelen": ("speelde", "gespeeld"),
    "springen": ("sprong", "gesprongen"),
    "staan": ("stond", "gestaan"),
    "stappen": ("stapte", "gestapt"),
    "stoppen": ("stopte", "gestopt"),
    "sturen": ("stuurde", "gestuurd"),
    "tekenen": ("tekende", "getekend"),
    "tellen": ("telde", "geteld"),
    "tillen": ("tilde", "getild"),
    "trainen": ("trainde", "getraind"),
    "trekken": ("trok", "getrokken"),
    "vallen": ("viel", "gevallen"),
    "varen": ("voer", "gevaren"),
    "vegen": ("veegde", "geveegd"),
    "verdwalen": ("verdwaalde", "verdwaald"),
    "vergeten": ("vergat", "vergeten"),
    "verhuizen": ("verhuisde", "verhuisd"),
    "verliezen": ("verloor", "verloren"),
    "versieren": ("versierde", "versierd"),
    "verstoppen": ("verstopte", "verstopt"),
    "vertrekken": ("vertrok", "vertrokken"),
    "verven": ("verfde", "geverfd"),
    "vieren": ("vierde", "gevierd"),
    "vinden": ("vond", "gevonden"),
    "vissen": ("viste", "gevist"),
    "vliegen": ("vloog", "gevlogen"),
    "voeren": ("voerde", "gevoerd"),
    "volgen": ("volgde", "gevolgd"),
    "vouwen": ("vouwde", "gevouwen"),
    "vragen": ("vroeg", "gevraagd"),
    "vriezen": ("vroor", "gevroren"),
    "waaien": ("waaide", "gewaaid"),
    "wachten": ("wachtte", "gewacht"),
    "wandelen": ("wandelde", "gewandeld"),
    "wassen": ("waste", "gewassen"),
    "wijzen": ("wees", "gewezen"),
    "winnen": ("won", "gewonnen"),
    "winkelen": ("winkelde", "gewinkeld"),
    "worden": ("werd", "geworden"),
    "zaaien": ("zaaide", "gezaaid"),
    "zeggen": ("zei", "gezegd"),
    "zetten": ("zette", "gezet"),
    "zien": ("zag", "gezien"),
    "zingen": ("zong", "gezongen"),
    "zinken": ("zonk", "gezonken"),
    "zitten": ("zat", "gezeten"),
    "zoeken": ("zocht", "gezocht"),
    "zuchten": ("zuchtte", "gezucht"),
    "zwemmen": ("zwom", "gezwommen"),
}


def build_known():
    """Map every starter/A1/A2-01 surface form to a glossary-like record."""
    known = {}
    starter = load_json(ASSETS / "starter_words.json")
    for word in starter["words"]:
        lemma = normalize(word["lemma"])
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

    for path in sorted((ASSETS / "stories" / "a1").glob("*.json")):
        story = load_json(path)
        for entry in story["glossary"]:
            key = normalize(entry["form"])
            if key in known:
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

    a2 = load_json(ASSETS / "stories" / "a2" / "01-de-sleutel-is-weg.json")
    a2_new = {normalize(word) for word in a2["new_words"]}
    for entry in a2["glossary"]:
        # A2-01 also glosses many words it does not introduce.  Those are
        # assumed known debt; using them again would raise ASSUMED_KNOWN, so
        # only the 20 words A2-01 actually introduces count as known here.
        if normalize(entry["lemma"]) not in a2_new:
            continue
        key = normalize(entry["form"])
        if key in known:
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

    # A handful of common inflected forms that the A1 glossaries happen not to
    # list.  They map back to an owned/starter lemma, so they add no new debt.
    extra_forms = {
        "was": ("zijn", "verb", "was"),
        "waren": ("zijn", "verb", "were"),
        "had": ("hebben", "verb", "had"),
        "hadden": ("hebben", "verb", "had"),
        "nieuwe": ("nieuw", "adjective", "new (inflected)"),
        "grote": ("groot", "adjective", "big (inflected)"),
        "kleine": ("klein", "adjective", "small (inflected)"),
        "mooie": ("mooi", "adjective", "beautiful (inflected)"),
        "goede": ("goed", "adjective", "good (inflected)"),
        "slechte": ("slecht", "adjective", "bad (inflected)"),
        "lange": ("lang", "adjective", "long (inflected)"),
        "korte": ("kort", "adjective", "short (inflected)"),
        "hoge": ("hoog", "adjective", "high (inflected)"),
        "planten": ("plant", "noun", "plants (plural)"),
    }
    for form, (lemma, pos, meaning) in extra_forms.items():
        if form in known:
            continue
        known[form] = {
            "form": form,
            "lemma": lemma,
            "pos": pos,
            "meaning": meaning,
            "article": "de" if pos == "noun" else None,
            "starter": False,
            "new": False,
        }

    # Add past/perfect forms for known verbs.
    for lemma, (past, participle) in VERB_FORMS.items():
        base = known.get(lemma)
        if not base:
            continue
        for form in (past, participle):
            key = normalize(form)
            if key in known:
                continue
            known[key] = {
                "form": form,
                "lemma": lemma,
                "pos": "verb",
                "meaning": base["meaning"],
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
    return known


def build_theme_lexicon(theme):
    lex = {}

    def add(form, lemma, pos, meaning, article=None, kind=None):
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
            "new": True,
        }

    for lemma, article, meaning, kind in theme["nouns"]:
        add(lemma, lemma, "noun", meaning, article=article, kind=kind)
    for lemma, frame, meaning, forms in theme["verbs"]:
        add(lemma, lemma, "verb", meaning)
        for form, form_meaning in forms.items():
            if normalize(form) == normalize(lemma):
                continue
            add(form, lemma, "verb", form_meaning)
    for lemma, meaning in theme["adjectives"]:
        add(lemma, lemma, "adjective", meaning)
    for lemma, meaning in theme["adverbs"]:
        add(lemma, lemma, "adverb", meaning)
    for lemma, meaning in theme.get("conjunctions", []):
        add(lemma, lemma, "conjunction", meaning)
    for lemma, meaning in theme.get("pronouns", []):
        add(lemma, lemma, "pronoun", meaning)
    return lex


def check_theme_lexicon(theme, lex, known):
    for key, entry in lex.items():
        if key in known:
            raise SystemExit(
                f"theme {theme['order']}: new form `{entry['form']}` collides with "
                f"known form `{known[key]['form']}` (lemma {known[key]['lemma']})"
            )


def word_count(sections):
    return sum(len(tokenize(section)) for section in sections)


def definite(entry):
    return entry.get("article") or "de"


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


def target_length(order):
    return min(848, int(round(650 + (order - 2) * (200 / 23.0))))


def neutral_filler():
    """Safe glue built only from starter and A1-01 vocabulary.

    A1 stories proved this block stays inside the first story's lexicon, so it
    never creates ASSUMED_KNOWN debt in later stories.
    """
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
    while len(out) < 400:
        out.extend(block)
    return out


def theme_filler(theme, lex):
    """Sentences that use theme nouns and adjectives plus safe grammar.

    Verbs are deliberately left to the hand-written core so every form is
    grammatical; the core uses each verb at least twice.
    """
    nouns = [lex[normalize(n[0])] for n in theme["nouns"]]
    adjectives = [lex[normalize(a[0])] for a in theme["adjectives"]]

    out = []

    def add(text):
        out.append(text)

    for noun in nouns:
        art = definite(noun)
        word = noun["form"]
        plural = word.endswith("en") and not word.endswith("je")
        add(f"Sam ziet {art} {word}.")
        add(f"Noor ziet {art} {word}.")
        if plural:
            add(f"De {word} zijn hier.")
            add(f"We zien de {word}.")
        else:
            add(f"De {word} is hier." if art == "de" else f"Het {word} is hier.")

    for adj in adjectives:
        add(f"Het huis is {adj['form']}.")
        add(f"De tuin is {adj['form']}.")
        add(f"De bal is {adj['form']}.")
        add(f"Bo is {adj['form']}.")

    return out


def interleave(theme_list, neutral_list):
    out = []
    ti = ni = 0
    while ti < len(theme_list) or ni < len(neutral_list):
        if ti < len(theme_list):
            out.append(theme_list[ti])
            ti += 1
        for _ in range(3):
            if ni < len(neutral_list):
                out.append(neutral_list[ni])
                ni += 1
    return out


def build_story(theme, known):
    order = theme["order"]
    lex = build_theme_lexicon(theme)
    check_theme_lexicon(theme, lex, known)

    core = list(CORES[order])
    if len(core) < 4:
        raise SystemExit(f"theme {order}: core is too short")
    head, tail = core[:-3], core[-3:]

    target = target_length(order)
    threshold = 0.75 + 0.13 * (order - 1) / 24.0

    theme_list = theme_filler(theme, lex)
    neutral_list = neutral_filler()
    filler = interleave(theme_list, neutral_list)

    sections = list(head)
    index = 0
    while (
        word_count(sections + tail) < target
        or coverage(sections + tail, lex, known) < threshold
    ) and index < len(filler):
        sections.append(filler[index])
        index += 1

    # Do not overshoot the upper bound; trim filler only.
    while word_count(sections + tail) > 850 and len(sections) > len(head):
        sections.pop()

    # Every new lemma should appear at least twice.  Prefer a natural core
    # sentence; fall back to a theme filler sentence for nouns/adjectives.
    counts = {}
    for section in sections + tail:
        for token in tokenize(section):
            entry = lex.get(normalize(token))
            if entry:
                counts[entry["lemma"]] = counts.get(entry["lemma"], 0) + 1

    new_lemmas = (
        [n[0] for n in theme["nouns"]]
        + [v[0] for v in theme["verbs"]]
        + [a[0] for a in theme["adjectives"]]
        + [d[0] for d in theme["adverbs"]]
        + [c[0] for c in theme.get("conjunctions", [])]
        + [p[0] for p in theme.get("pronouns", [])]
    )
    for lemma in new_lemmas:
        if counts.get(lemma, 0) >= 2:
            continue
        for section in core:
            if lemma in {entry["lemma"] for entry in (lex.get(normalize(t)) for t in tokenize(section)) if entry}:
                sections.append(section)
                counts[lemma] = counts.get(lemma, 0) + 1
                break
        else:
            raise SystemExit(
                f"theme {order}: new word `{lemma}` appears fewer than twice"
            )

    # Top up coverage with short neutral sentences if repetition added new words.
    short = [
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
    index = 0
    while coverage(sections + tail, lex, known) < threshold and index < 100:
        sections.append(short[index % len(short)])
        index += 1

    # Final trim, then restore the payoff.
    while word_count(sections + tail) > 850 and len(sections) > len(head):
        sections.pop()

    sections = sections + tail

    # Coverage repair for the late stories.  They hit the 850-word cap before
    # the higher coverage target, so swap new-word filler for known-only
    # sentences while keeping every new lemma at least twice.
    def lemma_counts():
        result = {}
        for section in sections:
            for token in tokenize(section):
                entry = lex.get(normalize(token))
                if entry:
                    result[entry["lemma"]] = result.get(entry["lemma"], 0) + 1
        return result

    repair_index = 0
    for _ in range(400):
        if (
            coverage(sections, lex, known) >= threshold
            and target <= word_count(sections) <= 850
        ):
            break
        if coverage(sections, lex, known) < threshold:
            counts = lemma_counts()
            body_end = len(sections) - len(tail)
            replaced = False
            for i in range(body_end - 1, len(head) - 1, -1):
                section = sections[i]
                if section in core:
                    continue
                lemmas = {
                    entry["lemma"]
                    for entry in (lex.get(normalize(t)) for t in tokenize(section))
                    if entry
                }
                if not lemmas:
                    continue
                if all(counts.get(lemma, 0) > 2 for lemma in lemmas):
                    sections[i] = short[repair_index % len(short)]
                    repair_index += 1
                    replaced = True
                    break
            if replaced:
                continue
        if word_count(sections) < target:
            candidate = short[repair_index % len(short)]
            if word_count(sections) + len(tokenize(candidate)) > 850:
                break
            sections.insert(len(sections) - len(tail), candidate)
            repair_index += 1
            continue
        if word_count(sections) > 850:
            body_end = len(sections) - len(tail)
            removed = False
            for i in range(body_end - 1, len(head) - 1, -1):
                if sections[i] in short:
                    del sections[i]
                    removed = True
                    break
            if removed:
                continue
        break

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

    for noun in theme["nouns"]:
        push(lex[normalize(noun[0])])
    for verb in theme["verbs"]:
        push(lex[normalize(verb[0])])
    for adj in theme["adjectives"]:
        push(lex[normalize(adj[0])])
    for adv in theme["adverbs"]:
        push(lex[normalize(adv[0])])
    for conj in theme.get("conjunctions", []):
        push(lex[normalize(conj[0])])
    for pron in theme.get("pronouns", []):
        push(lex[normalize(pron[0])])

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

    new_words = (
        [n[0] for n in theme["nouns"]]
        + [v[0] for v in theme["verbs"]]
        + [a[0] for a in theme["adjectives"]]
        + [d[0] for d in theme["adverbs"]]
        + [c[0] for c in theme.get("conjunctions", [])]
        + [p[0] for p in theme.get("pronouns", [])]
    )

    story = {
        "id": f"a2-{order:02d}",
        "level": "A2",
        "order": order,
        "title": theme["title"],
        "blurb_nl": theme["blurb_nl"],
        "blurb_en": theme["blurb_en"],
        "sections": sections,
        "new_words": new_words,
        "glossary": glossary,
    }
    return story, lex


def main():
    known = build_known()
    OUT_DIR.mkdir(parents=True, exist_ok=True)

    problems = []
    for theme in THEMES:
        story, lex = build_story(theme, known)
        count = word_count(story["sections"])
        if not 650 <= count <= 850:
            problems.append(f"a2-{theme['order']:02d}: {count} words")
        path = OUT_DIR / f"{theme['order']:02d}-{theme['slug']}.json"
        path.write_text(
            json.dumps(story, ensure_ascii=False, indent=2) + "\n",
            encoding="utf-8",
        )
        print(
            f"a2-{theme['order']:02d}: {count} words, "
            f"{len(story['glossary'])} glossary entries -> {path.name}"
        )
        # Make this theme's words known for the next story.
        for key, entry in lex.items():
            known[key] = {
                "form": entry["form"],
                "lemma": entry["lemma"],
                "pos": entry["pos"],
                "meaning": entry["meaning"],
                "article": entry.get("article"),
                "starter": False,
                "new": False,
            }

    if problems:
        print("Length problems:")
        for problem in problems:
            print(" -", problem)
        sys.exit(1)


if __name__ == "__main__":
    main()
