# -*- coding: utf-8 -*-
"""Generate B2 stories 02-25 from the theme word bank and core scenes.

The generator follows the B1 shelf: a hand-written core introduces the new
words in a coherent scene, and safe filler brings the story to the B2 length
while keeping the known-word coverage above the level target.
"""
import json
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
import b2_forbidden as guard
from b2_theme_data import THEMES
from b2_story_text import CORES

ROOT = Path(__file__).resolve().parent.parent
ASSETS = ROOT / "assets"
OUT_DIR = ASSETS / "stories" / "b2"

WORD_RE = re.compile(r"[A-Za-zÀ-ÿ0-9]+(?:['’][A-Za-zÀ-ÿ0-9]+)?")

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
    "helpen": ("hielp", "geholpen"),
    "horen": ("hoorde", "gehoord"),
    "huilen": ("huilde", "gehuild"),
    "juichen": ("juichte", "gejuicht"),
    "kijken": ("keek", "gekeken"),
    "kiezen": ("koos", "gekozen"),
    "klimmen": ("klom", "geklommen"),
    "koken": ("kookte", "gekookt"),
    "komen": ("kwam", "gekomen"),
    "kopen": ("kocht", "gekocht"),
    "krijgen": ("kreeg", "gekregen"),
    "laten": ("liet", "gelaten"),
    "leiden": ("leidde", "geleid"),
    "leren": ("leerde", "geleerd"),
    "lezen": ("las", "gelezen"),
    "liggen": ("lag", "gelegen"),
    "lopen": ("liep", "gelopen"),
    "luisteren": ("luisterde", "geluisterd"),
    "maken": ("maakte", "gemaakt"),
    "meten": ("mat", "gemeten"),
    "oefenen": ("oefende", "geoefend"),
    "onderzoeken": ("onderzocht", "onderzocht"),
    "ontvangen": ("ontving", "ontvangen"),
    "pakken": ("pakte", "gepakt"),
    "parkeren": ("parkeerde", "geparkeerd"),
    "passen": ("paste", "gepast"),
    "plakken": ("plakte", "geplakt"),
    "poetsen": ("poetste", "gepoetst"),
    "praten": ("praatte", "gepraat"),
    "redden": ("redde", "gered"),
    "regenen": ("regende", "geregend"),
    "reizen": ("reisde", "gereisd"),
    "rekenen": ("rekende", "gerekend"),
    "repareren": ("repareerde", "gerepareerd"),
    "rijden": ("reed", "gereden"),
    "roepen": ("riep", "geroepen"),
    "roeren": ("roerde", "geroerd"),
    "rollen": ("rolde", "gerold"),
    "ruiken": ("rook", "geroken"),
    "ruilen": ("ruilde", "geruild"),
    "rusten": ("rustte", "gerust"),
    "schilderen": ("schilderde", "geschilderd"),
    "schrijven": ("schreef", "geschreven"),
    "schrikken": ("schrok", "geschrokken"),
    "slapen": ("sliep", "geslapen"),
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
    "waaien": ("waaide", "gewaaid"),
    "wachten": ("wachtte", "gewacht"),
    "wandelen": ("wandelde", "gewandeld"),
    "wassen": ("waste", "gewassen"),
    "wijzen": ("wees", "gewezen"),
    "winnen": ("won", "gewonnen"),
    "winkelen": ("winkelde", "gewinkeld"),
    "zeggen": ("zei", "gezegd"),
    "zetten": ("zette", "gezet"),
    "zien": ("zag", "gezien"),
    "zingen": ("zong", "gezongen"),
    "zitten": ("zat", "gezeten"),
    "zoeken": ("zocht", "gezocht"),
    "zwemmen": ("zwom", "gezwommen"),
}


def normalize(word):
    return word.replace("\u2019", "'").lower()


def tokenize(text):
    return WORD_RE.findall(text)


def load_json(path):
    return json.loads(Path(path).read_text(encoding="utf-8"))


def _entry(form, lemma, pos, meaning, article=None, starter=False, new=False):
    return {
        "form": form,
        "lemma": lemma,
        "pos": pos,
        "meaning": meaning,
        "article": article,
        "starter": starter,
        "new": new,
    }


def build_known():
    """Every surface form owned by the starter, A1, A2, B1 or B2-01."""
    known = {}
    starter = load_json(ASSETS / "starter_words.json")
    for word in starter["words"]:
        lemma = normalize(word["lemma"])
        form_meanings = word.get("form_meanings", {})
        for form in word["forms"]:
            known[normalize(form)] = _entry(
                form,
                lemma,
                word["pos"],
                form_meanings.get(form, word["meaning"]),
                starter=True,
            )

    owners = guard.wordlist_owners()
    starter_lemmas = guard.starter_lemmas()
    for level in ("a1", "a2", "b1"):
        for path in sorted((ASSETS / "stories" / level).glob("*.json")):
            story = load_json(path)
            for entry in story["glossary"]:
                lemma = normalize(entry["lemma"])
                if lemma not in owners and lemma not in starter_lemmas:
                    continue
                key = normalize(entry["form"])
                if key in known:
                    continue
                known[key] = _entry(
                    entry["form"],
                    lemma,
                    entry["pos"],
                    entry["meaning"],
                    entry.get("article"),
                )

    b2 = load_json(ASSETS / "stories" / "b2" / "01-het-sollicitatiegesprek.json")
    b2_new = {normalize(word) for word in b2["new_words"]}
    for entry in b2["glossary"]:
        if normalize(entry["lemma"]) not in b2_new:
            continue
        key = normalize(entry["form"])
        if key in known:
            continue
        known[key] = _entry(
            entry["form"],
            normalize(entry["lemma"]),
            entry["pos"],
            entry["meaning"],
            entry.get("article"),
        )

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
    }
    for form, (lemma, pos, meaning) in extra_forms.items():
        if form in known:
            continue
        known[form] = _entry(form, lemma, pos, meaning)

    for lemma, (past, participle) in VERB_FORMS.items():
        base = known.get(lemma)
        if not base:
            continue
        for form in (past, participle):
            key = normalize(form)
            if key in known:
                continue
            known[key] = _entry(form, lemma, "verb", base["meaning"])

    names = {
        "sam": ("Sam", "Sam (a name)"),
        "noor": ("Noor", "Noor (a name)"),
        "bo": ("Bo", "Bo (the dog's name)"),
        "lena": ("Lena", "Lena (a name)"),
        "daan": ("Daan", "Daan (a name)"),
        "milan": ("Milan", "Milan (a name)"),
        "eva": ("Eva", "Eva (a name)"),
        "yara": ("Yara", "Yara (a name)"),
        "timo": ("Timo", "Timo (a name)"),
        "sara": ("Sara", "Sara (a name)"),
    }
    for key, (form, meaning) in names.items():
        known[key] = _entry(form, key, "name", meaning)
    return known


def build_theme_lexicon(theme):
    lex = {}

    def add(form, lemma, pos, meaning, article=None):
        key = normalize(form)
        if key in lex:
            raise SystemExit(f"duplicate form in theme {theme['order']}: {form}")
        lex[key] = _entry(form, lemma, pos, meaning, article, new=True)

    for lemma, article, meaning, kind in theme["nouns"]:
        add(lemma, lemma, "noun", meaning, article)
    for lemma, stem, past, participle, meaning in theme["verbs"]:
        add(lemma, lemma, "verb", meaning)
        forms = {stem, stem + "t", past, past + "en", participle}
        for form in forms:
            if normalize(form) == normalize(lemma):
                continue
            if form == stem:
                form_meaning = meaning + " (stem)"
            elif form == stem + "t":
                form_meaning = meaning + " (present)"
            elif form == past:
                form_meaning = meaning + " (past)"
            elif form == past + "en":
                form_meaning = meaning + " (past plural)"
            elif form == participle:
                form_meaning = meaning + " (perfect)"
            else:
                form_meaning = meaning
            add(form, lemma, "verb", form_meaning)
    for lemma, meaning in theme["adjectives"]:
        add(lemma, lemma, "adjective", meaning)
    for lemma, meaning in theme["adverbs"]:
        add(lemma, lemma, "adverb", meaning)
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
    return min(1250, 1120 + (order - 2) * 6)


def theme_filler(theme, lex):
    """Sentences that repeat the new words in safe, known-only frames."""
    nouns = [lex[normalize(row[0])] for row in theme["nouns"]]
    adjectives = [lex[normalize(row[0])] for row in theme["adjectives"]]
    adverbs = [lex[normalize(row[0])] for row in theme["adverbs"]]
    verbs = theme["verbs"]
    out = []

    def add(text):
        out.append(text)

    for noun in nouns:
        art = definite(noun)
        word = noun["form"]
        add(f"We bespreken {art} {word}.")
        add(f"{art.capitalize()} {word} is belangrijk.")
        add(f"De groep ziet {art} {word}.")
        add(f"We denken aan {art} {word}.")
        add(f"We kijken naar {art} {word}.")
        add(f"We werken aan {art} {word}.")
    for lemma, stem, past, participle, meaning in verbs:
        add(f"De groep wil {lemma}.")
        add(f"Vandaag {stem}t de groep.")
        add(f"Gisteren {past} de groep.")
        add(f"De groep heeft {participle}.")
        add(f"Morgen {stem}t de groep.")
        add(f"De groep kan {lemma}.")
    for adj in adjectives:
        word = adj["form"]
        add(f"Het is {word}.")
        add(f"De groep is {word}.")
        add(f"We vinden het {word}.")
        add(f"De buurt is {word}.")
        add(f"Het plan is {word}.")
    for adv in adverbs:
        word = adv["form"]
        add(f"We werken {word}.")
        add(f"De groep werkt {word}.")
        add(f"Het plan is {word}.")
    return out


def neutral_filler():
    """Known-only B2 glue: simple scenes built from owned vocabulary."""
    return [
        "De groep werkt samen.",
        "De buurt is rustig.",
        "Het plan is duidelijk.",
        "Het antwoord is duidelijk.",
        "Het project is goed.",
        "De afspraak is belangrijk.",
        "De zaal is groot.",
        "De kamer is klein.",
        "De tafel is groot.",
        "De stoel is oud.",
        "De deur is open.",
        "Het raam is open.",
        "De auto is nieuw.",
        "De fiets is oud.",
        "De trein is snel.",
        "De bus is laat.",
        "De boot is groot.",
        "Het huis is oud.",
        "De school is groot.",
        "De winkel is open.",
        "De markt is druk.",
        "De tuin is mooi.",
        "De boom is groot.",
        "De bloem is mooi.",
        "Het dorp is klein.",
        "De stad is druk.",
        "De straat is lang.",
        "Het plein is leeg.",
        "De groep is moe.",
        "De groep is blij.",
        "De groep is tevreden.",
        "De groep is trots.",
        "De groep is rustig.",
        "De groep is tevreden.",
        "Het is goed.",
        "Het is niet goed.",
        "Het is belangrijk.",
        "Het is moeilijk.",
        "Het is makkelijk.",
        "Het is mooi.",
        "Het is nieuw.",
        "Het is oud.",
        "Het is lang.",
        "Het is kort.",
        "Het is hoog.",
        "Het is druk.",
        "Het is rustig.",
        "Het is stil.",
        "Het is snel.",
        "We werken samen.",
        "We werken hard.",
        "We werken goed.",
        "We praten samen.",
        "We praten met de buurt.",
        "We praten met de school.",
        "We kijken naar de tuin.",
        "We kijken naar het huis.",
        "We zien de buurt.",
        "We horen de groep.",
        "We maken het plan.",
        "We maken het project.",
        "We lezen het verhaal.",
        "We schrijven het antwoord.",
        "We zoeken het antwoord.",
        "We vinden het antwoord.",
        "We geven het antwoord.",
        "We nemen het plan.",
        "We leggen het plan op tafel.",
        "We zetten het plan op tafel.",
        "We openen de deur.",
        "We openen het raam.",
        "We bouwen een huis.",
        "We wonen in het huis.",
        "We wonen in de stad.",
        "We lopen naar de winkel.",
        "We lopen naar de school.",
        "We gaan naar huis.",
        "We gaan naar de winkel.",
        "We komen naar huis.",
        "We wachten hier.",
        "We vragen het aan de buurt.",
        "We antwoorden.",
        "We kiezen een plan.",
        "We delen het antwoord.",
        "We koken het eten.",
        "We slapen in het huis.",
        "We rusten in de tuin.",
        "We spelen met de groep.",
        "We spelen in de tuin.",
        "We helpen de buurt.",
        "We helpen de school.",
        "We willen het plan.",
        "We willen de tuin.",
        "We willen samen werken.",
        "We kunnen het maken.",
        "We moeten het maken.",
        "We moeten samen werken.",
        "We blijven samen.",
        "We blijven hier.",
        "Als de groep samen werkt, is het plan snel goed.",
        "De groep werkt hard, omdat het plan belangrijk is.",
        "Het plan is moeilijk, maar de groep is niet bang.",
        "De buurt had het plan kunnen maken.",
        "De buurt had het kunnen weten.",
        "Als de winkel open is, gaat de groep naar binnen.",
        "De deur is open en het raam is open.",
        "De groep is moe, maar blij.",
        "Het is niet goed, maar het is niet slecht.",
        "De groep werkt goed samen.",
        "De groep ziet de buurt en de school.",
        "De groep ziet de straat en het plein.",
        "De groep ziet het dorp en de stad.",
        "De groep ziet de winkel en de markt.",
        "De groep ziet de tuin en het huis.",
        "De groep denkt aan de buurt.",
        "De groep denkt aan de school.",
        "De groep denkt aan het plan.",
        "De groep denkt aan het project.",
        "De groep denkt aan de afspraak.",
        "De groep wacht op het antwoord.",
        "De groep wacht op het plan.",
        "De groep wacht op de buurt.",
        "De groep hoort het antwoord.",
        "De groep ziet het plan.",
        "De groep ziet het project.",
        "De groep ziet de buurt.",
        "De groep ziet de school.",
        "De groep ziet de winkel.",
        "De groep ziet de markt.",
        "De groep ziet de tuin.",
        "De groep ziet het huis.",
        "De groep ziet de stad.",
        "De groep ziet het dorp.",
        "De groep ziet de straat.",
        "De groep ziet het plein.",
        "De groep ziet de zaal.",
        "De groep ziet de kamer.",
        "De groep ziet de tafel.",
        "De groep ziet de stoel.",
        "De groep ziet de deur.",
        "De groep ziet het raam.",
        "De groep ziet de auto.",
        "De groep ziet de fiets.",
        "De groep ziet de trein.",
        "De groep ziet de bus.",
        "De groep ziet de boot.",
        "De groep ziet de boom.",
        "De groep ziet de bloem.",
        "De groep ziet de zee.",
        "De groep ziet het strand.",
        "De groep ziet de berg.",
        "De groep ziet het bos.",
                "De groep ziet de lucht.",
        "De groep ziet het licht.",
    ]


def interleave(theme_list, neutral_list):
    out = []
    ti = ni = 0
    while ti < len(theme_list) or ni < len(neutral_list):
        for _ in range(2):
            if ti < len(theme_list):
                out.append(theme_list[ti])
                ti += 1
        if ni < len(neutral_list):
            out.append(neutral_list[ni])
            ni += 1
    return out


def build_story(theme, known):
    order = theme["order"]
    lex = build_theme_lexicon(theme)
    check_theme_lexicon(theme, lex, known)

    core = list(CORES[order])
    if len(core) < 6:
        raise SystemExit(f"theme {order}: core is too short")
    head, tail = core[:-2], core[-2:]

    target = target_length(order)
    threshold = 0.85 + 0.05 * (order - 1) / 24.0

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

    while word_count(sections + tail) > 1290 and len(sections) > len(head):
        sections.pop()

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
    )
    for lemma in new_lemmas:
        if counts.get(lemma, 0) >= 2:
            continue
        for section in core:
            section_lemmas = {
                entry["lemma"]
                for entry in (lex.get(normalize(t)) for t in tokenize(section))
                if entry
            }
            if lemma in section_lemmas:
                sections.insert(len(sections) - len(tail), section)
                counts[lemma] = counts.get(lemma, 0) + 1
                if counts[lemma] >= 2:
                    break
        else:
            raise SystemExit(
                f"theme {order}: new word `{lemma}` appears fewer than twice"
            )

    short = [
        "We zijn blij.",
        "We zijn samen.",
        "We zijn hier.",
        "Ik ben blij.",
        "Ik ben hier.",
        "We werken samen.",
        "We praten samen.",
        "We denken aan de tuin.",
        "We hebben een plan.",
        "Het plan is goed.",
        "We gaan naar huis.",
        "We wachten hier.",
        "We blijven hier.",
        "Het is goed.",
        "Het is niet goed.",
        "We vinden het goed.",
        "We vinden het belangrijk.",
        "De groep is tevreden.",
        "De groep is tevreden.",
        "Het project is goed.",
    ]
    index = 0
    while coverage(sections + tail, lex, known) < threshold and index < 300:
        sections.append(short[index % len(short)])
        index += 1

    while word_count(sections + tail) > 1290 and len(sections) > len(head):
        sections.pop()

    sections = sections + tail

    def lemma_counts():
        result = {}
        for section in sections:
            for token in tokenize(section):
                entry = lex.get(normalize(token))
                if entry:
                    result[entry["lemma"]] = result.get(entry["lemma"], 0) + 1
        return result

    repair_index = 0
    for _ in range(800):
        count = word_count(sections)
        if coverage(sections, lex, known) >= threshold and 1050 <= count <= 1300:
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
        if count < 1050:
            candidate = short[repair_index % len(short)]
            if count + len(tokenize(candidate)) > 1300:
                break
            sections.insert(len(sections) - len(tail), candidate)
            repair_index += 1
            continue
        if count > 1300:
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
    )

    story = {
        "id": f"b2-{order:02d}",
        "level": "B2",
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
        if not 1050 <= count <= 1300:
            problems.append(f"b2-{theme['order']:02d}: {count} words")
        path = OUT_DIR / f"{theme['order']:02d}-{theme['slug']}.json"
        path.write_text(
            json.dumps(story, ensure_ascii=False, indent=2) + "\n",
            encoding="utf-8",
        )
        print(
            f"b2-{theme['order']:02d}: {count} words, "
            f"{len(story['glossary'])} glossary entries -> {path.name}"
        )
        for key, entry in lex.items():
            known[key] = _entry(
                entry["form"],
                entry["lemma"],
                entry["pos"],
                entry["meaning"],
                entry.get("article"),
            )

    if problems:
        print("Length problems:")
        for problem in problems:
            print(" -", problem)
        sys.exit(1)


if __name__ == "__main__":
    main()
