# Glossary audit

Full audit of every glossary entry and every word occurrence in the four bundled stories. Generated from the story JSON, the embedded starter list, and the same lookup rules the app uses (`StoryLexicon::resolve_at`): an occurrence-specific entry wins, otherwise the default entry for the surface form is used, and a phrase part resolves to the phrase entry.

## How the audit was done

1. Every token in every sentence was looked up exactly as a tap would look it up, including the 1-based occurrence number.
2. Each result was compared with the sentence: does the shown meaning fit the word as written in this sentence?
3. The lemma entry was checked for a usable base-form line (`form of: <lemma>, <meaning>`).
4. Noun entries were checked for the correct article, separable verbs and idioms for a phrase entry, and each entry for a correct `new` flag.
5. Suspicious results were fixed, and the validator rule `GLOSSARY_LEMMA_MEANING` now rejects a non-lemma form whose meaning is identical to its lemma's meaning.

## Summary

| Story | Tokens | Entries | Distinct forms | Missing | Same-meaning flags |
| --- | ---: | ---: | ---: | ---: | ---: |
| A1 · De rode bal (`a1-01`) | 697 | 72 | 62 | 0 | 0 |
| A2 · De sleutel is weg (`a2-01`) | 621 | 170 | 167 | 0 | 0 |
| B1 · De laatste stroopwafel (`b1-01`) | 846 | 338 | 328 | 0 | 0 |
| B2 · Het sollicitatiegesprek (`b2-01`) | 1053 | 461 | 445 | 0 | 0 |

After the fixes: **3,217 occurrences, 1,041 glossary entries, 0 missing forms, 0 same-meaning flags** (the validator's `GLOSSARY_LEMMA_MEANING` check also passes).

## Every occurrence

`occ` is the 1-based occurrence of the tapped surface form in that story. The **Shown meaning** column is exactly what the popup shows for that tap; the **Base line** column is the secondary line under it.

### A1 · De rode bal (`a1-01`)

| # | Sentence | Tapped (occ) | Shown meaning | Lemma | Base line | Flag |
| ---: | --- | --- | --- | --- | --- | --- |
| 1 | Hallo! | Hallo (1) | hello / hi | hallo |  |  |
| 2 | Ik ben Sam. | Ik (1) | I | ik |  |  |
| 3 | Ik ben Sam. | ben (1) | am | zijn | form of: zijn, to be |  |
| 4 | Ik ben Sam. | Sam (1) | Sam (a name) | sam |  |  |
| 5 | Hier is Noor. | Hier (1) | here | hier |  |  |
| 6 | Hier is Noor. | is (1) | is | zijn | form of: zijn, to be |  |
| 7 | Hier is Noor. | Noor (1) | Noor (a name) | noor |  |  |
| 8 | Hier is Bo. | Hier (2) | here | hier |  |  |
| 9 | Hier is Bo. | is (2) | is | zijn | form of: zijn, to be |  |
| 10 | Hier is Bo. | Bo (1) | Bo (the dog's name) | bo |  |  |
| 11 | Bo is een hond. | Bo (2) | Bo (the dog's name) | bo |  |  |
| 12 | Bo is een hond. | is (3) | is | zijn | form of: zijn, to be |  |
| 13 | Bo is een hond. | een (1) | a / an | een |  |  |
| 14 | Bo is een hond. | hond (1) | dog | hond |  |  |
| 15 | Bo is groot. | Bo (3) | Bo (the dog's name) | bo |  |  |
| 16 | Bo is groot. | is (4) | is | zijn | form of: zijn, to be |  |
| 17 | Bo is groot. | groot (1) | big / large | groot |  |  |
| 18 | Bo is niet klein. | Bo (4) | Bo (the dog's name) | bo |  |  |
| 19 | Bo is niet klein. | is (5) | is | zijn | form of: zijn, to be |  |
| 20 | Bo is niet klein. | niet (1) | not | niet |  |  |
| 21 | Bo is niet klein. | klein (1) | small / little | klein |  |  |
| 22 | Bo heeft een bal. | Bo (5) | Bo (the dog's name) | bo |  |  |
| 23 | Bo heeft een bal. | heeft (1) | has | hebben | form of: hebben, to have |  |
| 24 | Bo heeft een bal. | een (2) | a / an | een |  |  |
| 25 | Bo heeft een bal. | bal (1) | ball | bal |  |  |
| 26 | De bal is rood. | De (1) | the | de |  |  |
| 27 | De bal is rood. | bal (2) | ball | bal |  |  |
| 28 | De bal is rood. | is (6) | is | zijn | form of: zijn, to be |  |
| 29 | De bal is rood. | rood (1) | red | rood |  |  |
| 30 | De bal is niet groot. | De (2) | the | de |  |  |
| 31 | De bal is niet groot. | bal (3) | ball | bal |  |  |
| 32 | De bal is niet groot. | is (7) | is | zijn | form of: zijn, to be |  |
| 33 | De bal is niet groot. | niet (2) | not | niet |  |  |
| 34 | De bal is niet groot. | groot (2) | big / large | groot |  |  |
| 35 | De bal is klein. | De (3) | the | de |  |  |
| 36 | De bal is klein. | bal (4) | ball | bal |  |  |
| 37 | De bal is klein. | is (8) | is | zijn | form of: zijn, to be |  |
| 38 | De bal is klein. | klein (2) | small / little | klein |  |  |
| 39 | Bo is blij met de bal. | Bo (6) | Bo (the dog's name) | bo |  |  |
| 40 | Bo is blij met de bal. | is (9) | is | zijn | form of: zijn, to be |  |
| 41 | Bo is blij met de bal. | blij (1) | happy / glad | blij |  |  |
| 42 | Bo is blij met de bal. | met (1) | with | met |  |  |
| 43 | Bo is blij met de bal. | de (4) | the | de |  |  |
| 44 | Bo is blij met de bal. | bal (5) | ball | bal |  |  |
| 45 | Bo wil de bal. | Bo (7) | Bo (the dog's name) | bo |  |  |
| 46 | Bo wil de bal. | wil (1) | wants | willen | form of: willen, to want |  |
| 47 | Bo wil de bal. | de (5) | the | de |  |  |
| 48 | Bo wil de bal. | bal (6) | ball | bal |  |  |
| 49 | Bo pakt de bal. | Bo (8) | Bo (the dog's name) | bo |  |  |
| 50 | Bo pakt de bal. | pakt (1) | takes | pakken | form of: pakken, to take / grab |  |
| 51 | Bo pakt de bal. | de (6) | the | de |  |  |
| 52 | Bo pakt de bal. | bal (7) | ball | bal |  |  |
| 53 | Bo loopt met de bal in de tuin. | Bo (9) | Bo (the dog's name) | bo |  |  |
| 54 | Bo loopt met de bal in de tuin. | loopt (1) | walks | lopen | form of: lopen, to walk |  |
| 55 | Bo loopt met de bal in de tuin. | met (2) | with | met |  |  |
| 56 | Bo loopt met de bal in de tuin. | de (7) | the | de |  |  |
| 57 | Bo loopt met de bal in de tuin. | bal (8) | ball | bal |  |  |
| 58 | Bo loopt met de bal in de tuin. | in (1) | in | in |  |  |
| 59 | Bo loopt met de bal in de tuin. | de (8) | the | de |  |  |
| 60 | Bo loopt met de bal in de tuin. | tuin (1) | garden | tuin |  |  |
| 61 | De tuin is groot. | De (9) | the | de |  |  |
| 62 | De tuin is groot. | tuin (2) | garden | tuin |  |  |
| 63 | De tuin is groot. | is (10) | is | zijn | form of: zijn, to be |  |
| 64 | De tuin is groot. | groot (3) | big / large | groot |  |  |
| 65 | De tuin is niet klein. | De (10) | the | de |  |  |
| 66 | De tuin is niet klein. | tuin (3) | garden | tuin |  |  |
| 67 | De tuin is niet klein. | is (11) | is | zijn | form of: zijn, to be |  |
| 68 | De tuin is niet klein. | niet (3) | not | niet |  |  |
| 69 | De tuin is niet klein. | klein (3) | small / little | klein |  |  |
| 70 | Sam is in het huis. | Sam (2) | Sam (a name) | sam |  |  |
| 71 | Sam is in het huis. | is (12) | is | zijn | form of: zijn, to be |  |
| 72 | Sam is in het huis. | in (2) | in | in |  |  |
| 73 | Sam is in het huis. | het (1) | the | het |  |  |
| 74 | Sam is in het huis. | huis (1) | house / home | huis |  |  |
| 75 | Noor is in het huis. | Noor (2) | Noor (a name) | noor |  |  |
| 76 | Noor is in het huis. | is (13) | is | zijn | form of: zijn, to be |  |
| 77 | Noor is in het huis. | in (3) | in | in |  |  |
| 78 | Noor is in het huis. | het (2) | the | het |  |  |
| 79 | Noor is in het huis. | huis (2) | house / home | huis |  |  |
| 80 | Sam ziet Bo in de tuin. | Sam (3) | Sam (a name) | sam |  |  |
| 81 | Sam ziet Bo in de tuin. | ziet (1) | sees | zien | form of: zien, to see |  |
| 82 | Sam ziet Bo in de tuin. | Bo (10) | Bo (the dog's name) | bo |  |  |
| 83 | Sam ziet Bo in de tuin. | in (4) | in | in |  |  |
| 84 | Sam ziet Bo in de tuin. | de (11) | the | de |  |  |
| 85 | Sam ziet Bo in de tuin. | tuin (4) | garden | tuin |  |  |
| 86 | Noor ziet Bo in de tuin. | Noor (3) | Noor (a name) | noor |  |  |
| 87 | Noor ziet Bo in de tuin. | ziet (2) | sees | zien | form of: zien, to see |  |
| 88 | Noor ziet Bo in de tuin. | Bo (11) | Bo (the dog's name) | bo |  |  |
| 89 | Noor ziet Bo in de tuin. | in (5) | in | in |  |  |
| 90 | Noor ziet Bo in de tuin. | de (12) | the | de |  |  |
| 91 | Noor ziet Bo in de tuin. | tuin (5) | garden | tuin |  |  |
| 92 | Bo ziet Sam en Noor. | Bo (12) | Bo (the dog's name) | bo |  |  |
| 93 | Bo ziet Sam en Noor. | ziet (3) | sees | zien | form of: zien, to see |  |
| 94 | Bo ziet Sam en Noor. | Sam (4) | Sam (a name) | sam |  |  |
| 95 | Bo ziet Sam en Noor. | en (1) | and | en |  |  |
| 96 | Bo ziet Sam en Noor. | Noor (4) | Noor (a name) | noor |  |  |
| 97 | Sam gaat naar de tuin. | Sam (5) | Sam (a name) | sam |  |  |
| 98 | Sam gaat naar de tuin. | gaat (1) | goes | gaan | form of: gaan, to go |  |
| 99 | Sam gaat naar de tuin. | naar (1) | to | naar |  |  |
| 100 | Sam gaat naar de tuin. | de (13) | the | de |  |  |
| 101 | Sam gaat naar de tuin. | tuin (6) | garden | tuin |  |  |
| 102 | Noor gaat naar de tuin. | Noor (5) | Noor (a name) | noor |  |  |
| 103 | Noor gaat naar de tuin. | gaat (2) | goes | gaan | form of: gaan, to go |  |
| 104 | Noor gaat naar de tuin. | naar (2) | to | naar |  |  |
| 105 | Noor gaat naar de tuin. | de (14) | the | de |  |  |
| 106 | Noor gaat naar de tuin. | tuin (7) | garden | tuin |  |  |
| 107 | Sam wil de bal. | Sam (6) | Sam (a name) | sam |  |  |
| 108 | Sam wil de bal. | wil (2) | wants | willen | form of: willen, to want |  |
| 109 | Sam wil de bal. | de (15) | the | de |  |  |
| 110 | Sam wil de bal. | bal (9) | ball | bal |  |  |
| 111 | Noor wil de bal. | Noor (6) | Noor (a name) | noor |  |  |
| 112 | Noor wil de bal. | wil (3) | wants | willen | form of: willen, to want |  |
| 113 | Noor wil de bal. | de (16) | the | de |  |  |
| 114 | Noor wil de bal. | bal (10) | ball | bal |  |  |
| 115 | Bo wil de bal. | Bo (13) | Bo (the dog's name) | bo |  |  |
| 116 | Bo wil de bal. | wil (4) | wants | willen | form of: willen, to want |  |
| 117 | Bo wil de bal. | de (17) | the | de |  |  |
| 118 | Bo wil de bal. | bal (11) | ball | bal |  |  |
| 119 | Bo is niet blij. | Bo (14) | Bo (the dog's name) | bo |  |  |
| 120 | Bo is niet blij. | is (14) | is | zijn | form of: zijn, to be |  |
| 121 | Bo is niet blij. | niet (4) | not | niet |  |  |
| 122 | Bo is niet blij. | blij (2) | happy / glad | blij |  |  |
| 123 | Sam pakt de bal. | Sam (7) | Sam (a name) | sam |  |  |
| 124 | Sam pakt de bal. | pakt (2) | takes | pakken | form of: pakken, to take / grab |  |
| 125 | Sam pakt de bal. | de (18) | the | de |  |  |
| 126 | Sam pakt de bal. | bal (12) | ball | bal |  |  |
| 127 | Bo is niet blij. | Bo (15) | Bo (the dog's name) | bo |  |  |
| 128 | Bo is niet blij. | is (15) | is | zijn | form of: zijn, to be |  |
| 129 | Bo is niet blij. | niet (5) | not | niet |  |  |
| 130 | Bo is niet blij. | blij (3) | happy / glad | blij |  |  |
| 131 | Noor pakt de bal. | Noor (7) | Noor (a name) | noor |  |  |
| 132 | Noor pakt de bal. | pakt (3) | takes | pakken | form of: pakken, to take / grab |  |
| 133 | Noor pakt de bal. | de (19) | the | de |  |  |
| 134 | Noor pakt de bal. | bal (13) | ball | bal |  |  |
| 135 | Bo is niet blij. | Bo (16) | Bo (the dog's name) | bo |  |  |
| 136 | Bo is niet blij. | is (16) | is | zijn | form of: zijn, to be |  |
| 137 | Bo is niet blij. | niet (6) | not | niet |  |  |
| 138 | Bo is niet blij. | blij (4) | happy / glad | blij |  |  |
| 139 | Bo wil de bal altijd. | Bo (17) | Bo (the dog's name) | bo |  |  |
| 140 | Bo wil de bal altijd. | wil (5) | wants | willen | form of: willen, to want |  |
| 141 | Bo wil de bal altijd. | de (20) | the | de |  |  |
| 142 | Bo wil de bal altijd. | bal (14) | ball | bal |  |  |
| 143 | Bo wil de bal altijd. | altijd (1) | always | altijd |  |  |
| 144 | Bo is niet blij. | Bo (18) | Bo (the dog's name) | bo |  |  |
| 145 | Bo is niet blij. | is (17) | is | zijn | form of: zijn, to be |  |
| 146 | Bo is niet blij. | niet (7) | not | niet |  |  |
| 147 | Bo is niet blij. | blij (5) | happy / glad | blij |  |  |
| 148 | Bo loopt naar het huis. | Bo (19) | Bo (the dog's name) | bo |  |  |
| 149 | Bo loopt naar het huis. | loopt (2) | walks | lopen | form of: lopen, to walk |  |
| 150 | Bo loopt naar het huis. | naar (3) | to | naar |  |  |
| 151 | Bo loopt naar het huis. | het (3) | the | het |  |  |
| 152 | Bo loopt naar het huis. | huis (3) | house / home | huis |  |  |
| 153 | Bo heeft de bal. | Bo (20) | Bo (the dog's name) | bo |  |  |
| 154 | Bo heeft de bal. | heeft (2) | has | hebben | form of: hebben, to have |  |
| 155 | Bo heeft de bal. | de (21) | the | de |  |  |
| 156 | Bo heeft de bal. | bal (15) | ball | bal |  |  |
| 157 | Sam en Noor gaan naar het huis. | Sam (8) | Sam (a name) | sam |  |  |
| 158 | Sam en Noor gaan naar het huis. | en (2) | and | en |  |  |
| 159 | Sam en Noor gaan naar het huis. | Noor (8) | Noor (a name) | noor |  |  |
| 160 | Sam en Noor gaan naar het huis. | gaan (1) | to go | gaan |  |  |
| 161 | Sam en Noor gaan naar het huis. | naar (4) | to | naar |  |  |
| 162 | Sam en Noor gaan naar het huis. | het (4) | the | het |  |  |
| 163 | Sam en Noor gaan naar het huis. | huis (4) | house / home | huis |  |  |
| 164 | We kijken in het huis. | We (1) | we | we |  |  |
| 165 | We kijken in het huis. | kijken (1) | to look | kijken |  |  |
| 166 | We kijken in het huis. | in (6) | in | in |  |  |
| 167 | We kijken in het huis. | het (5) | the | het |  |  |
| 168 | We kijken in het huis. | huis (5) | house / home | huis |  |  |
| 169 | De bal is niet in het huis. | De (22) | the | de |  |  |
| 170 | De bal is niet in het huis. | bal (16) | ball | bal |  |  |
| 171 | De bal is niet in het huis. | is (18) | is | zijn | form of: zijn, to be |  |
| 172 | De bal is niet in het huis. | niet (8) | not | niet |  |  |
| 173 | De bal is niet in het huis. | in (7) | in | in |  |  |
| 174 | De bal is niet in het huis. | het (6) | the | het |  |  |
| 175 | De bal is niet in het huis. | huis (6) | house / home | huis |  |  |
| 176 | We kijken in de tuin. | We (2) | we | we |  |  |
| 177 | We kijken in de tuin. | kijken (2) | to look | kijken |  |  |
| 178 | We kijken in de tuin. | in (8) | in | in |  |  |
| 179 | We kijken in de tuin. | de (23) | the | de |  |  |
| 180 | We kijken in de tuin. | tuin (8) | garden | tuin |  |  |
| 181 | De bal is niet in de tuin. | De (24) | the | de |  |  |
| 182 | De bal is niet in de tuin. | bal (17) | ball | bal |  |  |
| 183 | De bal is niet in de tuin. | is (19) | is | zijn | form of: zijn, to be |  |
| 184 | De bal is niet in de tuin. | niet (9) | not | niet |  |  |
| 185 | De bal is niet in de tuin. | in (9) | in | in |  |  |
| 186 | De bal is niet in de tuin. | de (25) | the | de |  |  |
| 187 | De bal is niet in de tuin. | tuin (9) | garden | tuin |  |  |
| 188 | We kijken hier. | We (3) | we | we |  |  |
| 189 | We kijken hier. | kijken (3) | to look | kijken |  |  |
| 190 | We kijken hier. | hier (3) | here | hier |  |  |
| 191 | We kijken daar. | We (4) | we | we |  |  |
| 192 | We kijken daar. | kijken (4) | to look | kijken |  |  |
| 193 | We kijken daar. | daar (1) | there | daar |  |  |
| 194 | We zien de bal niet. | We (5) | we | we |  |  |
| 195 | We zien de bal niet. | zien (1) | to see | zien |  |  |
| 196 | We zien de bal niet. | de (26) | the | de |  |  |
| 197 | We zien de bal niet. | bal (18) | ball | bal |  |  |
| 198 | We zien de bal niet. | niet (10) | not | niet |  |  |
| 199 | Bo is in de tuin. | Bo (21) | Bo (the dog's name) | bo |  |  |
| 200 | Bo is in de tuin. | is (20) | is | zijn | form of: zijn, to be |  |
| 201 | Bo is in de tuin. | in (10) | in | in |  |  |
| 202 | Bo is in de tuin. | de (27) | the | de |  |  |
| 203 | Bo is in de tuin. | tuin (10) | garden | tuin |  |  |
| 204 | Bo heeft de bal. | Bo (22) | Bo (the dog's name) | bo |  |  |
| 205 | Bo heeft de bal. | heeft (3) | has | hebben | form of: hebben, to have |  |
| 206 | Bo heeft de bal. | de (28) | the | de |  |  |
| 207 | Bo heeft de bal. | bal (19) | ball | bal |  |  |
| 208 | Bo geeft de bal niet. | Bo (23) | Bo (the dog's name) | bo |  |  |
| 209 | Bo geeft de bal niet. | geeft (1) | gives | geven | form of: geven, to give |  |
| 210 | Bo geeft de bal niet. | de (29) | the | de |  |  |
| 211 | Bo geeft de bal niet. | bal (20) | ball | bal |  |  |
| 212 | Bo geeft de bal niet. | niet (11) | not | niet |  |  |
| 213 | Bo is altijd in de tuin met de bal. | Bo (24) | Bo (the dog's name) | bo |  |  |
| 214 | Bo is altijd in de tuin met de bal. | is (21) | is | zijn | form of: zijn, to be |  |
| 215 | Bo is altijd in de tuin met de bal. | altijd (2) | always | altijd |  |  |
| 216 | Bo is altijd in de tuin met de bal. | in (11) | in | in |  |  |
| 217 | Bo is altijd in de tuin met de bal. | de (30) | the | de |  |  |
| 218 | Bo is altijd in de tuin met de bal. | tuin (11) | garden | tuin |  |  |
| 219 | Bo is altijd in de tuin met de bal. | met (3) | with | met |  |  |
| 220 | Bo is altijd in de tuin met de bal. | de (31) | the | de |  |  |
| 221 | Bo is altijd in de tuin met de bal. | bal (21) | ball | bal |  |  |
| 222 | Bo is niet in het huis. | Bo (25) | Bo (the dog's name) | bo |  |  |
| 223 | Bo is niet in het huis. | is (22) | is | zijn | form of: zijn, to be |  |
| 224 | Bo is niet in het huis. | niet (12) | not | niet |  |  |
| 225 | Bo is niet in het huis. | in (12) | in | in |  |  |
| 226 | Bo is niet in het huis. | het (7) | the | het |  |  |
| 227 | Bo is niet in het huis. | huis (7) | house / home | huis |  |  |
| 228 | Sam kijkt. | Sam (9) | Sam (a name) | sam |  |  |
| 229 | Sam kijkt. | kijkt (1) | looks | kijken | form of: kijken, to look |  |
| 230 | Noor kijkt. | Noor (9) | Noor (a name) | noor |  |  |
| 231 | Noor kijkt. | kijkt (2) | looks | kijken | form of: kijken, to look |  |
| 232 | We kijken samen. | We (6) | we | we |  |  |
| 233 | We kijken samen. | kijken (5) | to look | kijken |  |  |
| 234 | We kijken samen. | samen (1) | together | samen |  |  |
| 235 | We zien de bal niet. | We (7) | we | we |  |  |
| 236 | We zien de bal niet. | zien (2) | to see | zien |  |  |
| 237 | We zien de bal niet. | de (32) | the | de |  |  |
| 238 | We zien de bal niet. | bal (22) | ball | bal |  |  |
| 239 | We zien de bal niet. | niet (13) | not | niet |  |  |
| 240 | De bal is niet hier. | De (33) | the | de |  |  |
| 241 | De bal is niet hier. | bal (23) | ball | bal |  |  |
| 242 | De bal is niet hier. | is (23) | is | zijn | form of: zijn, to be |  |
| 243 | De bal is niet hier. | niet (14) | not | niet |  |  |
| 244 | De bal is niet hier. | hier (4) | here | hier |  |  |
| 245 | De bal is niet daar. | De (34) | the | de |  |  |
| 246 | De bal is niet daar. | bal (24) | ball | bal |  |  |
| 247 | De bal is niet daar. | is (24) | is | zijn | form of: zijn, to be |  |
| 248 | De bal is niet daar. | niet (15) | not | niet |  |  |
| 249 | De bal is niet daar. | daar (2) | there | daar |  |  |
| 250 | De bal is hier! | De (35) | the | de |  |  |
| 251 | De bal is hier! | bal (25) | ball | bal |  |  |
| 252 | De bal is hier! | is (25) | is | zijn | form of: zijn, to be |  |
| 253 | De bal is hier! | hier (5) | here | hier |  |  |
| 254 | Bo heeft de bal! | Bo (26) | Bo (the dog's name) | bo |  |  |
| 255 | Bo heeft de bal! | heeft (4) | has | hebben | form of: hebben, to have |  |
| 256 | Bo heeft de bal! | de (36) | the | de |  |  |
| 257 | Bo heeft de bal! | bal (26) | ball | bal |  |  |
| 258 | Bo komt naar Sam. | Bo (27) | Bo (the dog's name) | bo |  |  |
| 259 | Bo komt naar Sam. | komt (1) | comes | komen | form of: komen, to come |  |
| 260 | Bo komt naar Sam. | naar (5) | to | naar |  |  |
| 261 | Bo komt naar Sam. | Sam (10) | Sam (a name) | sam |  |  |
| 262 | Bo komt naar Noor. | Bo (28) | Bo (the dog's name) | bo |  |  |
| 263 | Bo komt naar Noor. | komt (2) | comes | komen | form of: komen, to come |  |
| 264 | Bo komt naar Noor. | naar (6) | to | naar |  |  |
| 265 | Bo komt naar Noor. | Noor (10) | Noor (a name) | noor |  |  |
| 266 | Bo geeft de bal aan Sam. | Bo (29) | Bo (the dog's name) | bo |  |  |
| 267 | Bo geeft de bal aan Sam. | geeft (2) | gives | geven | form of: geven, to give |  |
| 268 | Bo geeft de bal aan Sam. | de (37) | the | de |  |  |
| 269 | Bo geeft de bal aan Sam. | bal (27) | ball | bal |  |  |
| 270 | Bo geeft de bal aan Sam. | aan (1) | to | aan |  |  |
| 271 | Bo geeft de bal aan Sam. | Sam (11) | Sam (a name) | sam |  |  |
| 272 | Sam is blij. | Sam (12) | Sam (a name) | sam |  |  |
| 273 | Sam is blij. | is (26) | is | zijn | form of: zijn, to be |  |
| 274 | Sam is blij. | blij (6) | happy / glad | blij |  |  |
| 275 | Bo is blij. | Bo (30) | Bo (the dog's name) | bo |  |  |
| 276 | Bo is blij. | is (27) | is | zijn | form of: zijn, to be |  |
| 277 | Bo is blij. | blij (7) | happy / glad | blij |  |  |
| 278 | Noor is blij. | Noor (11) | Noor (a name) | noor |  |  |
| 279 | Noor is blij. | is (28) | is | zijn | form of: zijn, to be |  |
| 280 | Noor is blij. | blij (8) | happy / glad | blij |  |  |
| 281 | Sam geeft de bal aan Noor. | Sam (13) | Sam (a name) | sam |  |  |
| 282 | Sam geeft de bal aan Noor. | geeft (3) | gives | geven | form of: geven, to give |  |
| 283 | Sam geeft de bal aan Noor. | de (38) | the | de |  |  |
| 284 | Sam geeft de bal aan Noor. | bal (28) | ball | bal |  |  |
| 285 | Sam geeft de bal aan Noor. | aan (2) | to | aan |  |  |
| 286 | Sam geeft de bal aan Noor. | Noor (12) | Noor (a name) | noor |  |  |
| 287 | Noor geeft de bal aan Bo. | Noor (13) | Noor (a name) | noor |  |  |
| 288 | Noor geeft de bal aan Bo. | geeft (4) | gives | geven | form of: geven, to give |  |
| 289 | Noor geeft de bal aan Bo. | de (39) | the | de |  |  |
| 290 | Noor geeft de bal aan Bo. | bal (29) | ball | bal |  |  |
| 291 | Noor geeft de bal aan Bo. | aan (3) | to | aan |  |  |
| 292 | Noor geeft de bal aan Bo. | Bo (31) | Bo (the dog's name) | bo |  |  |
| 293 | Bo pakt de bal. | Bo (32) | Bo (the dog's name) | bo |  |  |
| 294 | Bo pakt de bal. | pakt (4) | takes | pakken | form of: pakken, to take / grab |  |
| 295 | Bo pakt de bal. | de (40) | the | de |  |  |
| 296 | Bo pakt de bal. | bal (30) | ball | bal |  |  |
| 297 | We spelen samen in de tuin. | We (8) | we | we |  |  |
| 298 | We spelen samen in de tuin. | spelen (1) | to play | spelen |  |  |
| 299 | We spelen samen in de tuin. | samen (2) | together | samen |  |  |
| 300 | We spelen samen in de tuin. | in (13) | in | in |  |  |
| 301 | We spelen samen in de tuin. | de (41) | the | de |  |  |
| 302 | We spelen samen in de tuin. | tuin (12) | garden | tuin |  |  |
| 303 | De tuin is groot. | De (42) | the | de |  |  |
| 304 | De tuin is groot. | tuin (13) | garden | tuin |  |  |
| 305 | De tuin is groot. | is (29) | is | zijn | form of: zijn, to be |  |
| 306 | De tuin is groot. | groot (4) | big / large | groot |  |  |
| 307 | We spelen met de bal. | We (9) | we | we |  |  |
| 308 | We spelen met de bal. | spelen (2) | to play | spelen |  |  |
| 309 | We spelen met de bal. | met (4) | with | met |  |  |
| 310 | We spelen met de bal. | de (43) | the | de |  |  |
| 311 | We spelen met de bal. | bal (31) | ball | bal |  |  |
| 312 | De bal is rood. | De (44) | the | de |  |  |
| 313 | De bal is rood. | bal (32) | ball | bal |  |  |
| 314 | De bal is rood. | is (30) | is | zijn | form of: zijn, to be |  |
| 315 | De bal is rood. | rood (2) | red | rood |  |  |
| 316 | De hond is groot. | De (45) | the | de |  |  |
| 317 | De hond is groot. | hond (2) | dog | hond |  |  |
| 318 | De hond is groot. | is (31) | is | zijn | form of: zijn, to be |  |
| 319 | De hond is groot. | groot (5) | big / large | groot |  |  |
| 320 | De tuin is groot. | De (46) | the | de |  |  |
| 321 | De tuin is groot. | tuin (14) | garden | tuin |  |  |
| 322 | De tuin is groot. | is (32) | is | zijn | form of: zijn, to be |  |
| 323 | De tuin is groot. | groot (6) | big / large | groot |  |  |
| 324 | Het huis is klein. | Het (8) | the | het |  |  |
| 325 | Het huis is klein. | huis (8) | house / home | huis |  |  |
| 326 | Het huis is klein. | is (33) | is | zijn | form of: zijn, to be |  |
| 327 | Het huis is klein. | klein (4) | small / little | klein |  |  |
| 328 | We zijn blij. | We (10) | we | we |  |  |
| 329 | We zijn blij. | zijn (1) | are | zijn |  |  |
| 330 | We zijn blij. | blij (9) | happy / glad | blij |  |  |
| 331 | Bo loopt met de bal naar het huis. | Bo (33) | Bo (the dog's name) | bo |  |  |
| 332 | Bo loopt met de bal naar het huis. | loopt (3) | walks | lopen | form of: lopen, to walk |  |
| 333 | Bo loopt met de bal naar het huis. | met (5) | with | met |  |  |
| 334 | Bo loopt met de bal naar het huis. | de (47) | the | de |  |  |
| 335 | Bo loopt met de bal naar het huis. | bal (33) | ball | bal |  |  |
| 336 | Bo loopt met de bal naar het huis. | naar (7) | to | naar |  |  |
| 337 | Bo loopt met de bal naar het huis. | het (9) | the | het |  |  |
| 338 | Bo loopt met de bal naar het huis. | huis (9) | house / home | huis |  |  |
| 339 | Bo wil de bal in het huis. | Bo (34) | Bo (the dog's name) | bo |  |  |
| 340 | Bo wil de bal in het huis. | wil (6) | wants | willen | form of: willen, to want |  |
| 341 | Bo wil de bal in het huis. | de (48) | the | de |  |  |
| 342 | Bo wil de bal in het huis. | bal (34) | ball | bal |  |  |
| 343 | Bo wil de bal in het huis. | in (14) | in | in |  |  |
| 344 | Bo wil de bal in het huis. | het (10) | the | het |  |  |
| 345 | Bo wil de bal in het huis. | huis (10) | house / home | huis |  |  |
| 346 | Maar de bal is niet in het huis. | Maar (1) | but | maar |  |  |
| 347 | Maar de bal is niet in het huis. | de (49) | the | de |  |  |
| 348 | Maar de bal is niet in het huis. | bal (35) | ball | bal |  |  |
| 349 | Maar de bal is niet in het huis. | is (34) | is | zijn | form of: zijn, to be |  |
| 350 | Maar de bal is niet in het huis. | niet (16) | not | niet |  |  |
| 351 | Maar de bal is niet in het huis. | in (15) | in | in |  |  |
| 352 | Maar de bal is niet in het huis. | het (11) | the | het |  |  |
| 353 | Maar de bal is niet in het huis. | huis (11) | house / home | huis |  |  |
| 354 | De bal is in de tuin. | De (50) | the | de |  |  |
| 355 | De bal is in de tuin. | bal (36) | ball | bal |  |  |
| 356 | De bal is in de tuin. | is (35) | is | zijn | form of: zijn, to be |  |
| 357 | De bal is in de tuin. | in (16) | in | in |  |  |
| 358 | De bal is in de tuin. | de (51) | the | de |  |  |
| 359 | De bal is in de tuin. | tuin (15) | garden | tuin |  |  |
| 360 | Bo gaat naar de tuin. | Bo (35) | Bo (the dog's name) | bo |  |  |
| 361 | Bo gaat naar de tuin. | gaat (3) | goes | gaan | form of: gaan, to go |  |
| 362 | Bo gaat naar de tuin. | naar (8) | to | naar |  |  |
| 363 | Bo gaat naar de tuin. | de (52) | the | de |  |  |
| 364 | Bo gaat naar de tuin. | tuin (16) | garden | tuin |  |  |
| 365 | Sam ziet Bo. | Sam (14) | Sam (a name) | sam |  |  |
| 366 | Sam ziet Bo. | ziet (4) | sees | zien | form of: zien, to see |  |
| 367 | Sam ziet Bo. | Bo (36) | Bo (the dog's name) | bo |  |  |
| 368 | Sam pakt de bal. | Sam (15) | Sam (a name) | sam |  |  |
| 369 | Sam pakt de bal. | pakt (5) | takes | pakken | form of: pakken, to take / grab |  |
| 370 | Sam pakt de bal. | de (53) | the | de |  |  |
| 371 | Sam pakt de bal. | bal (37) | ball | bal |  |  |
| 372 | Sam geeft de bal aan Bo. | Sam (16) | Sam (a name) | sam |  |  |
| 373 | Sam geeft de bal aan Bo. | geeft (5) | gives | geven | form of: geven, to give |  |
| 374 | Sam geeft de bal aan Bo. | de (54) | the | de |  |  |
| 375 | Sam geeft de bal aan Bo. | bal (38) | ball | bal |  |  |
| 376 | Sam geeft de bal aan Bo. | aan (4) | to | aan |  |  |
| 377 | Sam geeft de bal aan Bo. | Bo (37) | Bo (the dog's name) | bo |  |  |
| 378 | Bo is blij. | Bo (38) | Bo (the dog's name) | bo |  |  |
| 379 | Bo is blij. | is (36) | is | zijn | form of: zijn, to be |  |
| 380 | Bo is blij. | blij (10) | happy / glad | blij |  |  |
| 381 | Noor is blij. | Noor (14) | Noor (a name) | noor |  |  |
| 382 | Noor is blij. | is (37) | is | zijn | form of: zijn, to be |  |
| 383 | Noor is blij. | blij (11) | happy / glad | blij |  |  |
| 384 | We zijn blij. | We (11) | we | we |  |  |
| 385 | We zijn blij. | zijn (2) | are | zijn |  |  |
| 386 | We zijn blij. | blij (12) | happy / glad | blij |  |  |
| 387 | We zijn samen. | We (12) | we | we |  |  |
| 388 | We zijn samen. | zijn (3) | are | zijn |  |  |
| 389 | We zijn samen. | samen (3) | together | samen |  |  |
| 390 | Hallo! | Hallo (2) | hello / hi | hallo |  |  |
| 391 | We gaan naar de tuin. | We (13) | we | we |  |  |
| 392 | We gaan naar de tuin. | gaan (2) | to go | gaan |  |  |
| 393 | We gaan naar de tuin. | naar (9) | to | naar |  |  |
| 394 | We gaan naar de tuin. | de (55) | the | de |  |  |
| 395 | We gaan naar de tuin. | tuin (17) | garden | tuin |  |  |
| 396 | We spelen samen. | We (14) | we | we |  |  |
| 397 | We spelen samen. | spelen (3) | to play | spelen |  |  |
| 398 | We spelen samen. | samen (4) | together | samen |  |  |
| 399 | Bo heeft de bal. | Bo (39) | Bo (the dog's name) | bo |  |  |
| 400 | Bo heeft de bal. | heeft (5) | has | hebben | form of: hebben, to have |  |
| 401 | Bo heeft de bal. | de (56) | the | de |  |  |
| 402 | Bo heeft de bal. | bal (39) | ball | bal |  |  |
| 403 | De bal is rood. | De (57) | the | de |  |  |
| 404 | De bal is rood. | bal (40) | ball | bal |  |  |
| 405 | De bal is rood. | is (38) | is | zijn | form of: zijn, to be |  |
| 406 | De bal is rood. | rood (3) | red | rood |  |  |
| 407 | De hond is blij. | De (58) | the | de |  |  |
| 408 | De hond is blij. | hond (3) | dog | hond |  |  |
| 409 | De hond is blij. | is (39) | is | zijn | form of: zijn, to be |  |
| 410 | De hond is blij. | blij (13) | happy / glad | blij |  |  |
| 411 | We zijn altijd samen in de tuin. | We (15) | we | we |  |  |
| 412 | We zijn altijd samen in de tuin. | zijn (4) | are | zijn |  |  |
| 413 | We zijn altijd samen in de tuin. | altijd (3) | always | altijd |  |  |
| 414 | We zijn altijd samen in de tuin. | samen (5) | together | samen |  |  |
| 415 | We zijn altijd samen in de tuin. | in (17) | in | in |  |  |
| 416 | We zijn altijd samen in de tuin. | de (59) | the | de |  |  |
| 417 | We zijn altijd samen in de tuin. | tuin (18) | garden | tuin |  |  |
| 418 | Bo is niet in het huis. | Bo (40) | Bo (the dog's name) | bo |  |  |
| 419 | Bo is niet in het huis. | is (40) | is | zijn | form of: zijn, to be |  |
| 420 | Bo is niet in het huis. | niet (17) | not | niet |  |  |
| 421 | Bo is niet in het huis. | in (18) | in | in |  |  |
| 422 | Bo is niet in het huis. | het (12) | the | het |  |  |
| 423 | Bo is niet in het huis. | huis (12) | house / home | huis |  |  |
| 424 | Bo is altijd in de tuin. | Bo (41) | Bo (the dog's name) | bo |  |  |
| 425 | Bo is altijd in de tuin. | is (41) | is | zijn | form of: zijn, to be |  |
| 426 | Bo is altijd in de tuin. | altijd (4) | always | altijd |  |  |
| 427 | Bo is altijd in de tuin. | in (19) | in | in |  |  |
| 428 | Bo is altijd in de tuin. | de (60) | the | de |  |  |
| 429 | Bo is altijd in de tuin. | tuin (19) | garden | tuin |  |  |
| 430 | De tuin is groot. | De (61) | the | de |  |  |
| 431 | De tuin is groot. | tuin (20) | garden | tuin |  |  |
| 432 | De tuin is groot. | is (42) | is | zijn | form of: zijn, to be |  |
| 433 | De tuin is groot. | groot (7) | big / large | groot |  |  |
| 434 | De hond is groot. | De (62) | the | de |  |  |
| 435 | De hond is groot. | hond (4) | dog | hond |  |  |
| 436 | De hond is groot. | is (43) | is | zijn | form of: zijn, to be |  |
| 437 | De hond is groot. | groot (8) | big / large | groot |  |  |
| 438 | De bal is klein. | De (63) | the | de |  |  |
| 439 | De bal is klein. | bal (41) | ball | bal |  |  |
| 440 | De bal is klein. | is (44) | is | zijn | form of: zijn, to be |  |
| 441 | De bal is klein. | klein (5) | small / little | klein |  |  |
| 442 | De bal is rood. | De (64) | the | de |  |  |
| 443 | De bal is rood. | bal (42) | ball | bal |  |  |
| 444 | De bal is rood. | is (45) | is | zijn | form of: zijn, to be |  |
| 445 | De bal is rood. | rood (4) | red | rood |  |  |
| 446 | Bo is blij. | Bo (42) | Bo (the dog's name) | bo |  |  |
| 447 | Bo is blij. | is (46) | is | zijn | form of: zijn, to be |  |
| 448 | Bo is blij. | blij (14) | happy / glad | blij |  |  |
| 449 | Sam en Noor gaan naar het huis. | Sam (17) | Sam (a name) | sam |  |  |
| 450 | Sam en Noor gaan naar het huis. | en (3) | and | en |  |  |
| 451 | Sam en Noor gaan naar het huis. | Noor (15) | Noor (a name) | noor |  |  |
| 452 | Sam en Noor gaan naar het huis. | gaan (3) | to go | gaan |  |  |
| 453 | Sam en Noor gaan naar het huis. | naar (10) | to | naar |  |  |
| 454 | Sam en Noor gaan naar het huis. | het (13) | the | het |  |  |
| 455 | Sam en Noor gaan naar het huis. | huis (13) | house / home | huis |  |  |
| 456 | Bo gaat naar de tuin. | Bo (43) | Bo (the dog's name) | bo |  |  |
| 457 | Bo gaat naar de tuin. | gaat (4) | goes | gaan | form of: gaan, to go |  |
| 458 | Bo gaat naar de tuin. | naar (11) | to | naar |  |  |
| 459 | Bo gaat naar de tuin. | de (65) | the | de |  |  |
| 460 | Bo gaat naar de tuin. | tuin (21) | garden | tuin |  |  |
| 461 | Bo heeft de bal. | Bo (44) | Bo (the dog's name) | bo |  |  |
| 462 | Bo heeft de bal. | heeft (6) | has | hebben | form of: hebben, to have |  |
| 463 | Bo heeft de bal. | de (66) | the | de |  |  |
| 464 | Bo heeft de bal. | bal (43) | ball | bal |  |  |
| 465 | De bal is rood. | De (67) | the | de |  |  |
| 466 | De bal is rood. | bal (44) | ball | bal |  |  |
| 467 | De bal is rood. | is (47) | is | zijn | form of: zijn, to be |  |
| 468 | De bal is rood. | rood (5) | red | rood |  |  |
| 469 | Bo is blij. | Bo (45) | Bo (the dog's name) | bo |  |  |
| 470 | Bo is blij. | is (48) | is | zijn | form of: zijn, to be |  |
| 471 | Bo is blij. | blij (15) | happy / glad | blij |  |  |
| 472 | We zijn blij. | We (16) | we | we |  |  |
| 473 | We zijn blij. | zijn (5) | are | zijn |  |  |
| 474 | We zijn blij. | blij (16) | happy / glad | blij |  |  |
| 475 | We kijken niet. | We (17) | we | we |  |  |
| 476 | We kijken niet. | kijken (6) | to look | kijken |  |  |
| 477 | We kijken niet. | niet (18) | not | niet |  |  |
| 478 | We zien de bal. | We (18) | we | we |  |  |
| 479 | We zien de bal. | zien (3) | to see | zien |  |  |
| 480 | We zien de bal. | de (68) | the | de |  |  |
| 481 | We zien de bal. | bal (45) | ball | bal |  |  |
| 482 | De bal is hier. | De (69) | the | de |  |  |
| 483 | De bal is hier. | bal (46) | ball | bal |  |  |
| 484 | De bal is hier. | is (49) | is | zijn | form of: zijn, to be |  |
| 485 | De bal is hier. | hier (6) | here | hier |  |  |
| 486 | De bal is in de tuin. | De (70) | the | de |  |  |
| 487 | De bal is in de tuin. | bal (47) | ball | bal |  |  |
| 488 | De bal is in de tuin. | is (50) | is | zijn | form of: zijn, to be |  |
| 489 | De bal is in de tuin. | in (20) | in | in |  |  |
| 490 | De bal is in de tuin. | de (71) | the | de |  |  |
| 491 | De bal is in de tuin. | tuin (22) | garden | tuin |  |  |
| 492 | Bo geeft de bal aan Sam. | Bo (46) | Bo (the dog's name) | bo |  |  |
| 493 | Bo geeft de bal aan Sam. | geeft (6) | gives | geven | form of: geven, to give |  |
| 494 | Bo geeft de bal aan Sam. | de (72) | the | de |  |  |
| 495 | Bo geeft de bal aan Sam. | bal (48) | ball | bal |  |  |
| 496 | Bo geeft de bal aan Sam. | aan (5) | to | aan |  |  |
| 497 | Bo geeft de bal aan Sam. | Sam (18) | Sam (a name) | sam |  |  |
| 498 | Sam geeft de bal aan Noor. | Sam (19) | Sam (a name) | sam |  |  |
| 499 | Sam geeft de bal aan Noor. | geeft (7) | gives | geven | form of: geven, to give |  |
| 500 | Sam geeft de bal aan Noor. | de (73) | the | de |  |  |
| 501 | Sam geeft de bal aan Noor. | bal (49) | ball | bal |  |  |
| 502 | Sam geeft de bal aan Noor. | aan (6) | to | aan |  |  |
| 503 | Sam geeft de bal aan Noor. | Noor (16) | Noor (a name) | noor |  |  |
| 504 | Noor geeft de bal aan Bo. | Noor (17) | Noor (a name) | noor |  |  |
| 505 | Noor geeft de bal aan Bo. | geeft (8) | gives | geven | form of: geven, to give |  |
| 506 | Noor geeft de bal aan Bo. | de (74) | the | de |  |  |
| 507 | Noor geeft de bal aan Bo. | bal (50) | ball | bal |  |  |
| 508 | Noor geeft de bal aan Bo. | aan (7) | to | aan |  |  |
| 509 | Noor geeft de bal aan Bo. | Bo (47) | Bo (the dog's name) | bo |  |  |
| 510 | We spelen samen. | We (19) | we | we |  |  |
| 511 | We spelen samen. | spelen (4) | to play | spelen |  |  |
| 512 | We spelen samen. | samen (6) | together | samen |  |  |
| 513 | We zijn samen. | We (20) | we | we |  |  |
| 514 | We zijn samen. | zijn (6) | are | zijn |  |  |
| 515 | We zijn samen. | samen (7) | together | samen |  |  |
| 516 | De hond is blij. | De (75) | the | de |  |  |
| 517 | De hond is blij. | hond (5) | dog | hond |  |  |
| 518 | De hond is blij. | is (51) | is | zijn | form of: zijn, to be |  |
| 519 | De hond is blij. | blij (17) | happy / glad | blij |  |  |
| 520 | De bal is rood. | De (76) | the | de |  |  |
| 521 | De bal is rood. | bal (51) | ball | bal |  |  |
| 522 | De bal is rood. | is (52) | is | zijn | form of: zijn, to be |  |
| 523 | De bal is rood. | rood (6) | red | rood |  |  |
| 524 | De tuin is groot. | De (77) | the | de |  |  |
| 525 | De tuin is groot. | tuin (23) | garden | tuin |  |  |
| 526 | De tuin is groot. | is (53) | is | zijn | form of: zijn, to be |  |
| 527 | De tuin is groot. | groot (9) | big / large | groot |  |  |
| 528 | Het huis is klein. | Het (14) | the | het |  |  |
| 529 | Het huis is klein. | huis (14) | house / home | huis |  |  |
| 530 | Het huis is klein. | is (54) | is | zijn | form of: zijn, to be |  |
| 531 | Het huis is klein. | klein (6) | small / little | klein |  |  |
| 532 | Hallo, hond! | Hallo (3) | hello / hi | hallo |  |  |
| 533 | Hallo, hond! | hond (6) | dog | hond |  |  |
| 534 | Hallo, bal! | Hallo (4) | hello / hi | hallo |  |  |
| 535 | Hallo, bal! | bal (52) | ball | bal |  |  |
| 536 | Hallo, Sam! | Hallo (5) | hello / hi | hallo |  |  |
| 537 | Hallo, Sam! | Sam (20) | Sam (a name) | sam |  |  |
| 538 | Hallo, Noor! | Hallo (6) | hello / hi | hallo |  |  |
| 539 | Hallo, Noor! | Noor (18) | Noor (a name) | noor |  |  |
| 540 | Bo wil de bal altijd. | Bo (48) | Bo (the dog's name) | bo |  |  |
| 541 | Bo wil de bal altijd. | wil (7) | wants | willen | form of: willen, to want |  |
| 542 | Bo wil de bal altijd. | de (78) | the | de |  |  |
| 543 | Bo wil de bal altijd. | bal (53) | ball | bal |  |  |
| 544 | Bo wil de bal altijd. | altijd (5) | always | altijd |  |  |
| 545 | Bo is altijd blij met de bal. | Bo (49) | Bo (the dog's name) | bo |  |  |
| 546 | Bo is altijd blij met de bal. | is (55) | is | zijn | form of: zijn, to be |  |
| 547 | Bo is altijd blij met de bal. | altijd (6) | always | altijd |  |  |
| 548 | Bo is altijd blij met de bal. | blij (18) | happy / glad | blij |  |  |
| 549 | Bo is altijd blij met de bal. | met (6) | with | met |  |  |
| 550 | Bo is altijd blij met de bal. | de (79) | the | de |  |  |
| 551 | Bo is altijd blij met de bal. | bal (54) | ball | bal |  |  |
| 552 | Sam wil de bal niet altijd. | Sam (21) | Sam (a name) | sam |  |  |
| 553 | Sam wil de bal niet altijd. | wil (8) | wants | willen | form of: willen, to want |  |
| 554 | Sam wil de bal niet altijd. | de (80) | the | de |  |  |
| 555 | Sam wil de bal niet altijd. | bal (55) | ball | bal |  |  |
| 556 | Sam wil de bal niet altijd. | niet (19) | not | niet |  |  |
| 557 | Sam wil de bal niet altijd. | altijd (7) | always | altijd |  |  |
| 558 | Noor wil de bal niet altijd. | Noor (19) | Noor (a name) | noor |  |  |
| 559 | Noor wil de bal niet altijd. | wil (9) | wants | willen | form of: willen, to want |  |
| 560 | Noor wil de bal niet altijd. | de (81) | the | de |  |  |
| 561 | Noor wil de bal niet altijd. | bal (56) | ball | bal |  |  |
| 562 | Noor wil de bal niet altijd. | niet (20) | not | niet |  |  |
| 563 | Noor wil de bal niet altijd. | altijd (8) | always | altijd |  |  |
| 564 | We willen samen spelen. | We (21) | we | we |  |  |
| 565 | We willen samen spelen. | willen (1) | to want | willen |  |  |
| 566 | We willen samen spelen. | samen (8) | together | samen |  |  |
| 567 | We willen samen spelen. | spelen (5) | to play | spelen |  |  |
| 568 | We willen samen blij zijn. | We (22) | we | we |  |  |
| 569 | We willen samen blij zijn. | willen (2) | to want | willen |  |  |
| 570 | We willen samen blij zijn. | samen (9) | together | samen |  |  |
| 571 | We willen samen blij zijn. | blij (19) | happy / glad | blij |  |  |
| 572 | We willen samen blij zijn. | zijn (7) | to be | zijn |  |  |
| 573 | Bo pakt de bal. | Bo (50) | Bo (the dog's name) | bo |  |  |
| 574 | Bo pakt de bal. | pakt (6) | takes | pakken | form of: pakken, to take / grab |  |
| 575 | Bo pakt de bal. | de (82) | the | de |  |  |
| 576 | Bo pakt de bal. | bal (57) | ball | bal |  |  |
| 577 | Bo loopt naar het huis. | Bo (51) | Bo (the dog's name) | bo |  |  |
| 578 | Bo loopt naar het huis. | loopt (4) | walks | lopen | form of: lopen, to walk |  |
| 579 | Bo loopt naar het huis. | naar (12) | to | naar |  |  |
| 580 | Bo loopt naar het huis. | het (15) | the | het |  |  |
| 581 | Bo loopt naar het huis. | huis (15) | house / home | huis |  |  |
| 582 | Bo geeft de bal aan Sam. | Bo (52) | Bo (the dog's name) | bo |  |  |
| 583 | Bo geeft de bal aan Sam. | geeft (9) | gives | geven | form of: geven, to give |  |
| 584 | Bo geeft de bal aan Sam. | de (83) | the | de |  |  |
| 585 | Bo geeft de bal aan Sam. | bal (58) | ball | bal |  |  |
| 586 | Bo geeft de bal aan Sam. | aan (8) | to | aan |  |  |
| 587 | Bo geeft de bal aan Sam. | Sam (22) | Sam (a name) | sam |  |  |
| 588 | Sam is blij. | Sam (23) | Sam (a name) | sam |  |  |
| 589 | Sam is blij. | is (56) | is | zijn | form of: zijn, to be |  |
| 590 | Sam is blij. | blij (20) | happy / glad | blij |  |  |
| 591 | Noor is blij. | Noor (20) | Noor (a name) | noor |  |  |
| 592 | Noor is blij. | is (57) | is | zijn | form of: zijn, to be |  |
| 593 | Noor is blij. | blij (21) | happy / glad | blij |  |  |
| 594 | Bo is blij. | Bo (53) | Bo (the dog's name) | bo |  |  |
| 595 | Bo is blij. | is (58) | is | zijn | form of: zijn, to be |  |
| 596 | Bo is blij. | blij (22) | happy / glad | blij |  |  |
| 597 | We zijn samen in het huis. | We (23) | we | we |  |  |
| 598 | We zijn samen in het huis. | zijn (8) | are | zijn |  |  |
| 599 | We zijn samen in het huis. | samen (10) | together | samen |  |  |
| 600 | We zijn samen in het huis. | in (21) | in | in |  |  |
| 601 | We zijn samen in het huis. | het (16) | the | het |  |  |
| 602 | We zijn samen in het huis. | huis (16) | house / home | huis |  |  |
| 603 | De bal is in het huis. | De (84) | the | de |  |  |
| 604 | De bal is in het huis. | bal (59) | ball | bal |  |  |
| 605 | De bal is in het huis. | is (59) | is | zijn | form of: zijn, to be |  |
| 606 | De bal is in het huis. | in (22) | in | in |  |  |
| 607 | De bal is in het huis. | het (17) | the | het |  |  |
| 608 | De bal is in het huis. | huis (17) | house / home | huis |  |  |
| 609 | De hond is in het huis. | De (85) | the | de |  |  |
| 610 | De hond is in het huis. | hond (7) | dog | hond |  |  |
| 611 | De hond is in het huis. | is (60) | is | zijn | form of: zijn, to be |  |
| 612 | De hond is in het huis. | in (23) | in | in |  |  |
| 613 | De hond is in het huis. | het (18) | the | het |  |  |
| 614 | De hond is in het huis. | huis (18) | house / home | huis |  |  |
| 615 | De tuin is niet in het huis. | De (86) | the | de |  |  |
| 616 | De tuin is niet in het huis. | tuin (24) | garden | tuin |  |  |
| 617 | De tuin is niet in het huis. | is (61) | is | zijn | form of: zijn, to be |  |
| 618 | De tuin is niet in het huis. | niet (21) | not | niet |  |  |
| 619 | De tuin is niet in het huis. | in (24) | in | in |  |  |
| 620 | De tuin is niet in het huis. | het (19) | the | het |  |  |
| 621 | De tuin is niet in het huis. | huis (19) | house / home | huis |  |  |
| 622 | De tuin is daar. | De (87) | the | de |  |  |
| 623 | De tuin is daar. | tuin (25) | garden | tuin |  |  |
| 624 | De tuin is daar. | is (62) | is | zijn | form of: zijn, to be |  |
| 625 | De tuin is daar. | daar (3) | there | daar |  |  |
| 626 | Het huis is hier. | Het (20) | the | het |  |  |
| 627 | Het huis is hier. | huis (20) | house / home | huis |  |  |
| 628 | Het huis is hier. | is (63) | is | zijn | form of: zijn, to be |  |
| 629 | Het huis is hier. | hier (7) | here | hier |  |  |
| 630 | We zijn hier samen. | We (24) | we | we |  |  |
| 631 | We zijn hier samen. | zijn (9) | are | zijn |  |  |
| 632 | We zijn hier samen. | hier (8) | here | hier |  |  |
| 633 | We zijn hier samen. | samen (11) | together | samen |  |  |
| 634 | Bo loopt naar de tuin. | Bo (54) | Bo (the dog's name) | bo |  |  |
| 635 | Bo loopt naar de tuin. | loopt (5) | walks | lopen | form of: lopen, to walk |  |
| 636 | Bo loopt naar de tuin. | naar (13) | to | naar |  |  |
| 637 | Bo loopt naar de tuin. | de (88) | the | de |  |  |
| 638 | Bo loopt naar de tuin. | tuin (26) | garden | tuin |  |  |
| 639 | Bo heeft de bal. | Bo (55) | Bo (the dog's name) | bo |  |  |
| 640 | Bo heeft de bal. | heeft (7) | has | hebben | form of: hebben, to have |  |
| 641 | Bo heeft de bal. | de (89) | the | de |  |  |
| 642 | Bo heeft de bal. | bal (60) | ball | bal |  |  |
| 643 | De bal is rood. | De (90) | the | de |  |  |
| 644 | De bal is rood. | bal (61) | ball | bal |  |  |
| 645 | De bal is rood. | is (64) | is | zijn | form of: zijn, to be |  |
| 646 | De bal is rood. | rood (7) | red | rood |  |  |
| 647 | De tuin is groot. | De (91) | the | de |  |  |
| 648 | De tuin is groot. | tuin (27) | garden | tuin |  |  |
| 649 | De tuin is groot. | is (65) | is | zijn | form of: zijn, to be |  |
| 650 | De tuin is groot. | groot (10) | big / large | groot |  |  |
| 651 | Bo is blij. | Bo (56) | Bo (the dog's name) | bo |  |  |
| 652 | Bo is blij. | is (66) | is | zijn | form of: zijn, to be |  |
| 653 | Bo is blij. | blij (23) | happy / glad | blij |  |  |
| 654 | We gaan naar de tuin. | We (25) | we | we |  |  |
| 655 | We gaan naar de tuin. | gaan (4) | to go | gaan |  |  |
| 656 | We gaan naar de tuin. | naar (14) | to | naar |  |  |
| 657 | We gaan naar de tuin. | de (92) | the | de |  |  |
| 658 | We gaan naar de tuin. | tuin (28) | garden | tuin |  |  |
| 659 | We spelen samen met de bal. | We (26) | we | we |  |  |
| 660 | We spelen samen met de bal. | spelen (6) | to play | spelen |  |  |
| 661 | We spelen samen met de bal. | samen (12) | together | samen |  |  |
| 662 | We spelen samen met de bal. | met (7) | with | met |  |  |
| 663 | We spelen samen met de bal. | de (93) | the | de |  |  |
| 664 | We spelen samen met de bal. | bal (62) | ball | bal |  |  |
| 665 | Hallo, Bo! | Hallo (7) | hello / hi | hallo |  |  |
| 666 | Hallo, Bo! | Bo (57) | Bo (the dog's name) | bo |  |  |
| 667 | Hallo, Sam! | Hallo (8) | hello / hi | hallo |  |  |
| 668 | Hallo, Sam! | Sam (24) | Sam (a name) | sam |  |  |
| 669 | Hallo, Noor! | Hallo (9) | hello / hi | hallo |  |  |
| 670 | Hallo, Noor! | Noor (21) | Noor (a name) | noor |  |  |
| 671 | We zijn blij. | We (27) | we | we |  |  |
| 672 | We zijn blij. | zijn (10) | are | zijn |  |  |
| 673 | We zijn blij. | blij (24) | happy / glad | blij |  |  |
| 674 | De hond is groot. | De (94) | the | de |  |  |
| 675 | De hond is groot. | hond (8) | dog | hond |  |  |
| 676 | De hond is groot. | is (67) | is | zijn | form of: zijn, to be |  |
| 677 | De hond is groot. | groot (11) | big / large | groot |  |  |
| 678 | De bal is klein. | De (95) | the | de |  |  |
| 679 | De bal is klein. | bal (63) | ball | bal |  |  |
| 680 | De bal is klein. | is (68) | is | zijn | form of: zijn, to be |  |
| 681 | De bal is klein. | klein (7) | small / little | klein |  |  |
| 682 | De bal is rood. | De (96) | the | de |  |  |
| 683 | De bal is rood. | bal (64) | ball | bal |  |  |
| 684 | De bal is rood. | is (69) | is | zijn | form of: zijn, to be |  |
| 685 | De bal is rood. | rood (8) | red | rood |  |  |
| 686 | De tuin is groot. | De (97) | the | de |  |  |
| 687 | De tuin is groot. | tuin (29) | garden | tuin |  |  |
| 688 | De tuin is groot. | is (70) | is | zijn | form of: zijn, to be |  |
| 689 | De tuin is groot. | groot (12) | big / large | groot |  |  |
| 690 | Het huis is klein. | Het (21) | the | het |  |  |
| 691 | Het huis is klein. | huis (21) | house / home | huis |  |  |
| 692 | Het huis is klein. | is (71) | is | zijn | form of: zijn, to be |  |
| 693 | Het huis is klein. | klein (8) | small / little | klein |  |  |
| 694 | We zijn altijd samen. | We (28) | we | we |  |  |
| 695 | We zijn altijd samen. | zijn (11) | are | zijn |  |  |
| 696 | We zijn altijd samen. | altijd (9) | always | altijd |  |  |
| 697 | We zijn altijd samen. | samen (13) | together | samen |  |  |

### A2 · De sleutel is weg (`a2-01`)

| # | Sentence | Tapped (occ) | Shown meaning | Lemma | Base line | Flag |
| ---: | --- | --- | --- | --- | --- | --- |
| 1 | Het is maandagochtend. | Het (1) | it | het |  |  |
| 2 | Het is maandagochtend. | is (1) | is | zijn | form of: zijn, to be |  |
| 3 | Het is maandagochtend. | maandagochtend (1) | Monday morning | maandagochtend |  |  |
| 4 | De man wil naar de winkel, maar hij is zijn sleutel kwijt. | De (1) | the | de |  |  |
| 5 | De man wil naar de winkel, maar hij is zijn sleutel kwijt. | man (1) | man | man |  |  |
| 6 | De man wil naar de winkel, maar hij is zijn sleutel kwijt. | wil (1) | wants | willen | form of: willen, to want |  |
| 7 | De man wil naar de winkel, maar hij is zijn sleutel kwijt. | naar (1) | to | naar |  |  |
| 8 | De man wil naar de winkel, maar hij is zijn sleutel kwijt. | de (2) | the | de |  |  |
| 9 | De man wil naar de winkel, maar hij is zijn sleutel kwijt. | winkel (1) | shop | winkel |  |  |
| 10 | De man wil naar de winkel, maar hij is zijn sleutel kwijt. | maar (1) | but | maar |  |  |
| 11 | De man wil naar de winkel, maar hij is zijn sleutel kwijt. | hij (1) | he | hij |  |  |
| 12 | De man wil naar de winkel, maar hij is zijn sleutel kwijt. | is (2) | is | zijn | form of: zijn, to be |  |
| 13 | De man wil naar de winkel, maar hij is zijn sleutel kwijt. | zijn (1) | his | zijn |  |  |
| 14 | De man wil naar de winkel, maar hij is zijn sleutel kwijt. | sleutel (1) | key | sleutel |  |  |
| 15 | De man wil naar de winkel, maar hij is zijn sleutel kwijt. | kwijt (1) | lost / missing | kwijt |  |  |
| 16 | Hij zoekt in zijn tas. | Hij (2) | he | hij |  |  |
| 17 | Hij zoekt in zijn tas. | zoekt (1) | looks for | zoeken | form of: zoeken, to look for |  |
| 18 | Hij zoekt in zijn tas. | in (1) | in | in |  |  |
| 19 | Hij zoekt in zijn tas. | zijn (2) | his | zijn |  |  |
| 20 | Hij zoekt in zijn tas. | tas (1) | bag | tas |  |  |
| 21 | Hij zoekt in zijn broek. | Hij (3) | he | hij |  |  |
| 22 | Hij zoekt in zijn broek. | zoekt (2) | looks for | zoeken | form of: zoeken, to look for |  |
| 23 | Hij zoekt in zijn broek. | in (2) | in | in |  |  |
| 24 | Hij zoekt in zijn broek. | zijn (3) | his | zijn |  |  |
| 25 | Hij zoekt in zijn broek. | broek (1) | trousers | broek |  |  |
| 26 | De sleutel is nergens. | De (3) | the | de |  |  |
| 27 | De sleutel is nergens. | sleutel (2) | key | sleutel |  |  |
| 28 | De sleutel is nergens. | is (3) | is | zijn | form of: zijn, to be |  |
| 29 | De sleutel is nergens. | nergens (1) | nowhere | nergens |  |  |
| 30 | “Waar is mijn sleutel?” vraagt hij aan de hond. | Waar (1) | where | waar |  |  |
| 31 | “Waar is mijn sleutel?” vraagt hij aan de hond. | is (4) | is | zijn | form of: zijn, to be |  |
| 32 | “Waar is mijn sleutel?” vraagt hij aan de hond. | mijn (1) | my | mijn |  |  |
| 33 | “Waar is mijn sleutel?” vraagt hij aan de hond. | sleutel (3) | key | sleutel |  |  |
| 34 | “Waar is mijn sleutel?” vraagt hij aan de hond. | vraagt (1) | asks | vragen | form of: vragen, to ask |  |
| 35 | “Waar is mijn sleutel?” vraagt hij aan de hond. | hij (4) | he | hij |  |  |
| 36 | “Waar is mijn sleutel?” vraagt hij aan de hond. | aan (1) | to | aan |  |  |
| 37 | “Waar is mijn sleutel?” vraagt hij aan de hond. | de (4) | the | de |  |  |
| 38 | “Waar is mijn sleutel?” vraagt hij aan de hond. | hond (1) | dog | hond |  |  |
| 39 | De hond zit op de bank en kijkt naar de man. | De (5) | the | de |  |  |
| 40 | De hond zit op de bank en kijkt naar de man. | hond (2) | dog | hond |  |  |
| 41 | De hond zit op de bank en kijkt naar de man. | zit (1) | sits | zitten | form of: zitten, to sit |  |
| 42 | De hond zit op de bank en kijkt naar de man. | op (1) | on | op |  |  |
| 43 | De hond zit op de bank en kijkt naar de man. | de (6) | the | de |  |  |
| 44 | De hond zit op de bank en kijkt naar de man. | bank (1) | sofa / bench | bank |  |  |
| 45 | De hond zit op de bank en kijkt naar de man. | en (1) | and | en |  |  |
| 46 | De hond zit op de bank en kijkt naar de man. | kijkt (1) | looks | kijken | form of: kijken, to look |  |
| 47 | De hond zit op de bank en kijkt naar de man. | naar (2) | to | naar |  |  |
| 48 | De hond zit op de bank en kijkt naar de man. | de (7) | the | de |  |  |
| 49 | De hond zit op de bank en kijkt naar de man. | man (2) | man | man |  |  |
| 50 | De hond lacht. | De (8) | the | de |  |  |
| 51 | De hond lacht. | hond (3) | dog | hond |  |  |
| 52 | De hond lacht. | lacht (1) | laughs | lachen | form of: lachen, to laugh |  |
| 53 | De man lacht niet. | De (9) | the | de |  |  |
| 54 | De man lacht niet. | man (3) | man | man |  |  |
| 55 | De man lacht niet. | lacht (2) | laughs | lachen | form of: lachen, to laugh |  |
| 56 | De man lacht niet. | niet (1) | not | niet |  |  |
| 57 | Hij zoekt en zoekt, maar hij vindt de sleutel niet. | Hij (5) | he | hij |  |  |
| 58 | Hij zoekt en zoekt, maar hij vindt de sleutel niet. | zoekt (3) | looks for | zoeken | form of: zoeken, to look for |  |
| 59 | Hij zoekt en zoekt, maar hij vindt de sleutel niet. | en (2) | and | en |  |  |
| 60 | Hij zoekt en zoekt, maar hij vindt de sleutel niet. | zoekt (4) | looks for | zoeken | form of: zoeken, to look for |  |
| 61 | Hij zoekt en zoekt, maar hij vindt de sleutel niet. | maar (2) | but | maar |  |  |
| 62 | Hij zoekt en zoekt, maar hij vindt de sleutel niet. | hij (6) | he | hij |  |  |
| 63 | Hij zoekt en zoekt, maar hij vindt de sleutel niet. | vindt (1) | finds | vinden | form of: vinden, to find |  |
| 64 | Hij zoekt en zoekt, maar hij vindt de sleutel niet. | de (10) | the | de |  |  |
| 65 | Hij zoekt en zoekt, maar hij vindt de sleutel niet. | sleutel (4) | key | sleutel |  |  |
| 66 | Hij zoekt en zoekt, maar hij vindt de sleutel niet. | niet (2) | not | niet |  |  |
| 67 | De man gaat naar de keuken. | De (11) | the | de |  |  |
| 68 | De man gaat naar de keuken. | man (4) | man | man |  |  |
| 69 | De man gaat naar de keuken. | gaat (1) | goes | gaan | form of: gaan, go |  |
| 70 | De man gaat naar de keuken. | naar (3) | to | naar |  |  |
| 71 | De man gaat naar de keuken. | de (12) | the | de |  |  |
| 72 | De man gaat naar de keuken. | keuken (1) | kitchen | keuken |  |  |
| 73 | Hij doet de kast open. | Hij (7) | he | hij |  |  |
| 74 | Hij doet de kast open. | doet (1) | opens | opendoen | form of: opendoen, to open |  |
| 75 | Hij doet de kast open. | de (13) | the | de |  |  |
| 76 | Hij doet de kast open. | kast (1) | cupboard | kast |  |  |
| 77 | Hij doet de kast open. | open (1) | open | opendoen | form of: opendoen, to open |  |
| 78 | De kast is leeg. | De (14) | the | de |  |  |
| 79 | De kast is leeg. | kast (2) | cupboard | kast |  |  |
| 80 | De kast is leeg. | is (5) | is | zijn | form of: zijn, to be |  |
| 81 | De kast is leeg. | leeg (1) | empty | leeg |  |  |
| 82 | Hij zoekt op de vloer. | Hij (8) | he | hij |  |  |
| 83 | Hij zoekt op de vloer. | zoekt (5) | looks for | zoeken | form of: zoeken, to look for |  |
| 84 | Hij zoekt op de vloer. | op (2) | on | op |  |  |
| 85 | Hij zoekt op de vloer. | de (15) | the | de |  |  |
| 86 | Hij zoekt op de vloer. | vloer (1) | floor | vloer |  |  |
| 87 | De vloer is koud. | De (16) | the | de |  |  |
| 88 | De vloer is koud. | vloer (2) | floor | vloer |  |  |
| 89 | De vloer is koud. | is (6) | is | zijn | form of: zijn, to be |  |
| 90 | De vloer is koud. | koud (1) | cold | koud |  |  |
| 91 | “Ik heb de sleutel hier gelegd,” zegt de man. | Ik (1) | I | ik |  |  |
| 92 | “Ik heb de sleutel hier gelegd,” zegt de man. | heb (1) | have | hebben | form of: hebben, to have |  |
| 93 | “Ik heb de sleutel hier gelegd,” zegt de man. | de (17) | the | de |  |  |
| 94 | “Ik heb de sleutel hier gelegd,” zegt de man. | sleutel (5) | key | sleutel |  |  |
| 95 | “Ik heb de sleutel hier gelegd,” zegt de man. | hier (1) | here | hier |  |  |
| 96 | “Ik heb de sleutel hier gelegd,” zegt de man. | gelegd (1) | put / laid | leggen | form of: leggen, to put / lay |  |
| 97 | “Ik heb de sleutel hier gelegd,” zegt de man. | zegt (1) | says | zeggen | form of: zeggen, to say |  |
| 98 | “Ik heb de sleutel hier gelegd,” zegt de man. | de (18) | the | de |  |  |
| 99 | “Ik heb de sleutel hier gelegd,” zegt de man. | man (5) | man | man |  |  |
| 100 | Maar de sleutel is niet op de vloer. | Maar (3) | but | maar |  |  |
| 101 | Maar de sleutel is niet op de vloer. | de (19) | the | de |  |  |
| 102 | Maar de sleutel is niet op de vloer. | sleutel (6) | key | sleutel |  |  |
| 103 | Maar de sleutel is niet op de vloer. | is (7) | is | zijn | form of: zijn, to be |  |
| 104 | Maar de sleutel is niet op de vloer. | niet (3) | not | niet |  |  |
| 105 | Maar de sleutel is niet op de vloer. | op (3) | on | op |  |  |
| 106 | Maar de sleutel is niet op de vloer. | de (20) | the | de |  |  |
| 107 | Maar de sleutel is niet op de vloer. | vloer (3) | floor | vloer |  |  |
| 108 | De man is vergeten waar hij de sleutel heeft gelegd. | De (21) | the | de |  |  |
| 109 | De man is vergeten waar hij de sleutel heeft gelegd. | man (6) | man | man |  |  |
| 110 | De man is vergeten waar hij de sleutel heeft gelegd. | is (8) | is | zijn | form of: zijn, to be |  |
| 111 | De man is vergeten waar hij de sleutel heeft gelegd. | vergeten (1) | forgotten | vergeten |  |  |
| 112 | De man is vergeten waar hij de sleutel heeft gelegd. | waar (2) | where | waar |  |  |
| 113 | De man is vergeten waar hij de sleutel heeft gelegd. | hij (9) | he | hij |  |  |
| 114 | De man is vergeten waar hij de sleutel heeft gelegd. | de (22) | the | de |  |  |
| 115 | De man is vergeten waar hij de sleutel heeft gelegd. | sleutel (7) | key | sleutel |  |  |
| 116 | De man is vergeten waar hij de sleutel heeft gelegd. | heeft (1) | has | hebben | form of: hebben, to have |  |
| 117 | De man is vergeten waar hij de sleutel heeft gelegd. | gelegd (2) | put / laid | leggen | form of: leggen, to put / lay |  |
| 118 | De hond komt ook naar de keuken. | De (23) | the | de |  |  |
| 119 | De hond komt ook naar de keuken. | hond (4) | dog | hond |  |  |
| 120 | De hond komt ook naar de keuken. | komt (1) | comes | komen | form of: komen, to come |  |
| 121 | De hond komt ook naar de keuken. | ook (1) | also | ook |  |  |
| 122 | De hond komt ook naar de keuken. | naar (4) | to | naar |  |  |
| 123 | De hond komt ook naar de keuken. | de (24) | the | de |  |  |
| 124 | De hond komt ook naar de keuken. | keuken (2) | kitchen | keuken |  |  |
| 125 | Hij kijkt in de kast. | Hij (10) | he | hij |  |  |
| 126 | Hij kijkt in de kast. | kijkt (2) | looks | kijken | form of: kijken, to look |  |
| 127 | Hij kijkt in de kast. | in (3) | in | in |  |  |
| 128 | Hij kijkt in de kast. | de (25) | the | de |  |  |
| 129 | Hij kijkt in de kast. | kast (3) | cupboard | kast |  |  |
| 130 | De man zucht. | De (26) | the | de |  |  |
| 131 | De man zucht. | man (7) | man | man |  |  |
| 132 | De man zucht. | zucht (1) | sighs | zuchten | form of: zuchten, to sigh |  |
| 133 | “De hond helpt niet,” zegt hij. | De (27) | the | de |  |  |
| 134 | “De hond helpt niet,” zegt hij. | hond (5) | dog | hond |  |  |
| 135 | “De hond helpt niet,” zegt hij. | helpt (1) | helps | helpen | form of: helpen, to help |  |
| 136 | “De hond helpt niet,” zegt hij. | niet (4) | not | niet |  |  |
| 137 | “De hond helpt niet,” zegt hij. | zegt (2) | says | zeggen | form of: zeggen, to say |  |
| 138 | “De hond helpt niet,” zegt hij. | hij (11) | he | hij |  |  |
| 139 | De man gaat naar de woonkamer. | De (28) | the | de |  |  |
| 140 | De man gaat naar de woonkamer. | man (8) | man | man |  |  |
| 141 | De man gaat naar de woonkamer. | gaat (2) | goes | gaan | form of: gaan, go |  |
| 142 | De man gaat naar de woonkamer. | naar (5) | to | naar |  |  |
| 143 | De man gaat naar de woonkamer. | de (29) | the | de |  |  |
| 144 | De man gaat naar de woonkamer. | woonkamer (1) | living room | woonkamer |  |  |
| 145 | Hij zoekt onder de bank. | Hij (12) | he | hij |  |  |
| 146 | Hij zoekt onder de bank. | zoekt (6) | looks for | zoeken | form of: zoeken, to look for |  |
| 147 | Hij zoekt onder de bank. | onder (1) | under | onder |  |  |
| 148 | Hij zoekt onder de bank. | de (30) | the | de |  |  |
| 149 | Hij zoekt onder de bank. | bank (2) | sofa / bench | bank |  |  |
| 150 | De bank is zacht. | De (31) | the | de |  |  |
| 151 | De bank is zacht. | bank (3) | sofa / bench | bank |  |  |
| 152 | De bank is zacht. | is (9) | is | zijn | form of: zijn, to be |  |
| 153 | De bank is zacht. | zacht (1) | soft / quiet | zacht |  |  |
| 154 | De man tilt de bank op. | De (32) | the | de |  |  |
| 155 | De man tilt de bank op. | man (9) | man | man |  |  |
| 156 | De man tilt de bank op. | tilt (1) | lifts | tillen | form of: tillen, to lift |  |
| 157 | De man tilt de bank op. | de (33) | the | de |  |  |
| 158 | De man tilt de bank op. | bank (4) | sofa / bench | bank |  |  |
| 159 | De man tilt de bank op. | op (4) | on | op |  |  |
| 160 | “Aha!” zegt hij. | Aha (1) | aha | aha |  |  |
| 161 | “Aha!” zegt hij. | zegt (3) | says | zeggen | form of: zeggen, to say |  |
| 162 | “Aha!” zegt hij. | hij (13) | he | hij |  |  |
| 163 | Maar er ligt geen sleutel. | Maar (4) | but | maar |  |  |
| 164 | Maar er ligt geen sleutel. | er (1) | there | er |  |  |
| 165 | Maar er ligt geen sleutel. | ligt (1) | lies | liggen | form of: liggen, to lie |  |
| 166 | Maar er ligt geen sleutel. | geen (1) | no / not a | geen |  |  |
| 167 | Maar er ligt geen sleutel. | sleutel (8) | key | sleutel |  |  |
| 168 | Er ligt een bot. | Er (2) | there | er |  |  |
| 169 | Er ligt een bot. | ligt (2) | lies | liggen | form of: liggen, to lie |  |
| 170 | Er ligt een bot. | een (1) | a / an | een |  |  |
| 171 | Er ligt een bot. | bot (1) | bone | bot |  |  |
| 172 | De hond pakt het bot en loopt weg. | De (34) | the | de |  |  |
| 173 | De hond pakt het bot en loopt weg. | hond (6) | dog | hond |  |  |
| 174 | De hond pakt het bot en loopt weg. | pakt (1) | takes | pakken | form of: pakken, to take / grab |  |
| 175 | De hond pakt het bot en loopt weg. | het (2) | the | het |  |  |
| 176 | De hond pakt het bot en loopt weg. | bot (2) | bone | bot |  |  |
| 177 | De hond pakt het bot en loopt weg. | en (3) | and | en |  |  |
| 178 | De hond pakt het bot en loopt weg. | loopt (1) | walks | lopen | form of: lopen, to walk |  |
| 179 | De hond pakt het bot en loopt weg. | weg (1) | away | weg |  |  |
| 180 | De man legt de bank terug. | De (35) | the | de |  |  |
| 181 | De man legt de bank terug. | man (10) | man | man |  |  |
| 182 | De man legt de bank terug. | legt (1) | puts / lays | leggen | form of: leggen, to put / lay |  |
| 183 | De man legt de bank terug. | de (36) | the | de |  |  |
| 184 | De man legt de bank terug. | bank (5) | sofa / bench | bank |  |  |
| 185 | De man legt de bank terug. | terug (1) | back | terug |  |  |
| 186 | Hij zucht weer. | Hij (14) | he | hij |  |  |
| 187 | Hij zucht weer. | zucht (2) | sighs | zuchten | form of: zuchten, to sigh |  |
| 188 | Hij zucht weer. | weer (1) | again | weer |  |  |
| 189 | De hond kijkt naar de bank en kwispelt met zijn staart. | De (37) | the | de |  |  |
| 190 | De hond kijkt naar de bank en kwispelt met zijn staart. | hond (7) | dog | hond |  |  |
| 191 | De hond kijkt naar de bank en kwispelt met zijn staart. | kijkt (3) | looks | kijken | form of: kijken, to look |  |
| 192 | De hond kijkt naar de bank en kwispelt met zijn staart. | naar (6) | to | naar |  |  |
| 193 | De hond kijkt naar de bank en kwispelt met zijn staart. | de (38) | the | de |  |  |
| 194 | De hond kijkt naar de bank en kwispelt met zijn staart. | bank (6) | sofa / bench | bank |  |  |
| 195 | De hond kijkt naar de bank en kwispelt met zijn staart. | en (4) | and | en |  |  |
| 196 | De hond kijkt naar de bank en kwispelt met zijn staart. | kwispelt (1) | wags | kwispelen | form of: kwispelen, to wag |  |
| 197 | De hond kijkt naar de bank en kwispelt met zijn staart. | met (1) | with | met |  |  |
| 198 | De hond kijkt naar de bank en kwispelt met zijn staart. | zijn (4) | his | zijn |  |  |
| 199 | De hond kijkt naar de bank en kwispelt met zijn staart. | staart (1) | tail | staart |  |  |
| 200 | Plotseling hoort de man een geluid. | Plotseling (1) | suddenly | plotseling |  |  |
| 201 | Plotseling hoort de man een geluid. | hoort (1) | hears | horen | form of: horen, to hear |  |
| 202 | Plotseling hoort de man een geluid. | de (39) | the | de |  |  |
| 203 | Plotseling hoort de man een geluid. | man (11) | man | man |  |  |
| 204 | Plotseling hoort de man een geluid. | een (2) | a / an | een |  |  |
| 205 | Plotseling hoort de man een geluid. | geluid (1) | sound | geluid |  |  |
| 206 | Het geluid komt uit de keuken. | Het (3) | the | het |  |  |
| 207 | Het geluid komt uit de keuken. | geluid (2) | sound | geluid |  |  |
| 208 | Het geluid komt uit de keuken. | komt (2) | comes | komen | form of: komen, to come |  |
| 209 | Het geluid komt uit de keuken. | uit (1) | out of | uit |  |  |
| 210 | Het geluid komt uit de keuken. | de (40) | the | de |  |  |
| 211 | Het geluid komt uit de keuken. | keuken (3) | kitchen | keuken |  |  |
| 212 | De hond staat bij de kast. | De (41) | the | de |  |  |
| 213 | De hond staat bij de kast. | hond (8) | dog | hond |  |  |
| 214 | De hond staat bij de kast. | staat (1) | stands | staan | form of: staan, to stand |  |
| 215 | De hond staat bij de kast. | bij (1) | at | bij |  |  |
| 216 | De hond staat bij de kast. | de (42) | the | de |  |  |
| 217 | De hond staat bij de kast. | kast (4) | cupboard | kast |  |  |
| 218 | Hij maakt een zacht geluid. | Hij (15) | he | hij |  |  |
| 219 | Hij maakt een zacht geluid. | maakt (1) | makes | maken | form of: maken, to make |  |
| 220 | Hij maakt een zacht geluid. | een (3) | a / an | een |  |  |
| 221 | Hij maakt een zacht geluid. | zacht (2) | soft / quiet | zacht |  |  |
| 222 | Hij maakt een zacht geluid. | geluid (3) | sound | geluid |  |  |
| 223 | “Wat heb je?” vraagt de man. | Wat (1) | what | wat |  |  |
| 224 | “Wat heb je?” vraagt de man. | heb (2) | have | hebben | form of: hebben, to have |  |
| 225 | “Wat heb je?” vraagt de man. | je (1) | you / your | je |  |  |
| 226 | “Wat heb je?” vraagt de man. | vraagt (2) | asks | vragen | form of: vragen, to ask |  |
| 227 | “Wat heb je?” vraagt de man. | de (43) | the | de |  |  |
| 228 | “Wat heb je?” vraagt de man. | man (12) | man | man |  |  |
| 229 | De hond geeft geen antwoord. | De (44) | the | de |  |  |
| 230 | De hond geeft geen antwoord. | hond (9) | dog | hond |  |  |
| 231 | De hond geeft geen antwoord. | geeft (1) | gives | geven | form of: geven, to give |  |
| 232 | De hond geeft geen antwoord. | geen (2) | no / not a | geen |  |  |
| 233 | De hond geeft geen antwoord. | antwoord (1) | answer | antwoord |  |  |
| 234 | De hond kijkt naar de kast. | De (45) | the | de |  |  |
| 235 | De hond kijkt naar de kast. | hond (10) | dog | hond |  |  |
| 236 | De hond kijkt naar de kast. | kijkt (4) | looks | kijken | form of: kijken, to look |  |
| 237 | De hond kijkt naar de kast. | naar (7) | to | naar |  |  |
| 238 | De hond kijkt naar de kast. | de (46) | the | de |  |  |
| 239 | De hond kijkt naar de kast. | kast (5) | cupboard | kast |  |  |
| 240 | De man doet de kast open. | De (47) | the | de |  |  |
| 241 | De man doet de kast open. | man (13) | man | man |  |  |
| 242 | De man doet de kast open. | doet (2) | opens | opendoen | form of: opendoen, to open |  |
| 243 | De man doet de kast open. | de (48) | the | de |  |  |
| 244 | De man doet de kast open. | kast (6) | cupboard | kast |  |  |
| 245 | De man doet de kast open. | open (2) | open | opendoen | form of: opendoen, to open |  |
| 246 | De kast is nog steeds leeg. | De (49) | the | de |  |  |
| 247 | De kast is nog steeds leeg. | kast (7) | cupboard | kast |  |  |
| 248 | De kast is nog steeds leeg. | is (10) | is | zijn | form of: zijn, to be |  |
| 249 | De kast is nog steeds leeg. | nog (1) | still / yet | nog |  |  |
| 250 | De kast is nog steeds leeg. | steeds (1) | continuously | steeds |  |  |
| 251 | De kast is nog steeds leeg. | leeg (2) | empty | leeg |  |  |
| 252 | De man zucht weer. | De (50) | the | de |  |  |
| 253 | De man zucht weer. | man (14) | man | man |  |  |
| 254 | De man zucht weer. | zucht (3) | sighs | zuchten | form of: zuchten, to sigh |  |
| 255 | De man zucht weer. | weer (2) | again | weer |  |  |
| 256 | “Ik word gek,” zegt hij. | Ik (2) | I | ik |  |  |
| 257 | “Ik word gek,” zegt hij. | word (1) | become | worden | form of: worden, to become |  |
| 258 | “Ik word gek,” zegt hij. | gek (1) | crazy | gek |  |  |
| 259 | “Ik word gek,” zegt hij. | zegt (4) | says | zeggen | form of: zeggen, to say |  |
| 260 | “Ik word gek,” zegt hij. | hij (16) | he | hij |  |  |
| 261 | De man gaat terug naar de bank. | De (51) | the | de |  |  |
| 262 | De man gaat terug naar de bank. | man (15) | man | man |  |  |
| 263 | De man gaat terug naar de bank. | gaat (3) | goes | gaan | form of: gaan, go |  |
| 264 | De man gaat terug naar de bank. | terug (2) | back | terug |  |  |
| 265 | De man gaat terug naar de bank. | naar (8) | to | naar |  |  |
| 266 | De man gaat terug naar de bank. | de (52) | the | de |  |  |
| 267 | De man gaat terug naar de bank. | bank (7) | sofa / bench | bank |  |  |
| 268 | Hij zoekt nog een keer in zijn tas. | Hij (17) | he | hij |  |  |
| 269 | Hij zoekt nog een keer in zijn tas. | zoekt (7) | looks for | zoeken | form of: zoeken, to look for |  |
| 270 | Hij zoekt nog een keer in zijn tas. | nog (2) | still / yet | nog |  |  |
| 271 | Hij zoekt nog een keer in zijn tas. | een (4) | a / an | een |  |  |
| 272 | Hij zoekt nog een keer in zijn tas. | keer (1) | time / turn | keer |  |  |
| 273 | Hij zoekt nog een keer in zijn tas. | in (4) | in | in |  |  |
| 274 | Hij zoekt nog een keer in zijn tas. | zijn (5) | his | zijn |  |  |
| 275 | Hij zoekt nog een keer in zijn tas. | tas (2) | bag | tas |  |  |
| 276 | Hij zoekt in zijn broek. | Hij (18) | he | hij |  |  |
| 277 | Hij zoekt in zijn broek. | zoekt (8) | looks for | zoeken | form of: zoeken, to look for |  |
| 278 | Hij zoekt in zijn broek. | in (5) | in | in |  |  |
| 279 | Hij zoekt in zijn broek. | zijn (6) | his | zijn |  |  |
| 280 | Hij zoekt in zijn broek. | broek (2) | trousers | broek |  |  |
| 281 | Hij zoekt onder de bank. | Hij (19) | he | hij |  |  |
| 282 | Hij zoekt onder de bank. | zoekt (9) | looks for | zoeken | form of: zoeken, to look for |  |
| 283 | Hij zoekt onder de bank. | onder (2) | under | onder |  |  |
| 284 | Hij zoekt onder de bank. | de (53) | the | de |  |  |
| 285 | Hij zoekt onder de bank. | bank (8) | sofa / bench | bank |  |  |
| 286 | De bank is zacht. | De (54) | the | de |  |  |
| 287 | De bank is zacht. | bank (9) | sofa / bench | bank |  |  |
| 288 | De bank is zacht. | is (11) | is | zijn | form of: zijn, to be |  |
| 289 | De bank is zacht. | zacht (3) | soft / quiet | zacht |  |  |
| 290 | De man tilt de bank op. | De (55) | the | de |  |  |
| 291 | De man tilt de bank op. | man (16) | man | man |  |  |
| 292 | De man tilt de bank op. | tilt (2) | lifts | tillen | form of: tillen, to lift |  |
| 293 | De man tilt de bank op. | de (56) | the | de |  |  |
| 294 | De man tilt de bank op. | bank (10) | sofa / bench | bank |  |  |
| 295 | De man tilt de bank op. | op (5) | on | op |  |  |
| 296 | Onder de bank ligt een rode bal. | Onder (3) | under | onder |  |  |
| 297 | Onder de bank ligt een rode bal. | de (57) | the | de |  |  |
| 298 | Onder de bank ligt een rode bal. | bank (11) | sofa / bench | bank |  |  |
| 299 | Onder de bank ligt een rode bal. | ligt (3) | lies | liggen | form of: liggen, to lie |  |
| 300 | Onder de bank ligt een rode bal. | een (5) | a / an | een |  |  |
| 301 | Onder de bank ligt een rode bal. | rode (1) | red (inflected) | rood | form of: rood, red |  |
| 302 | Onder de bank ligt een rode bal. | bal (1) | ball | bal |  |  |
| 303 | De hond heeft de bal. | De (58) | the | de |  |  |
| 304 | De hond heeft de bal. | hond (11) | dog | hond |  |  |
| 305 | De hond heeft de bal. | heeft (2) | has | hebben | form of: hebben, to have |  |
| 306 | De hond heeft de bal. | de (59) | the | de |  |  |
| 307 | De hond heeft de bal. | bal (2) | ball | bal |  |  |
| 308 | De hond pakt de bal en loopt naar de tuin. | De (60) | the | de |  |  |
| 309 | De hond pakt de bal en loopt naar de tuin. | hond (12) | dog | hond |  |  |
| 310 | De hond pakt de bal en loopt naar de tuin. | pakt (2) | takes | pakken | form of: pakken, to take / grab |  |
| 311 | De hond pakt de bal en loopt naar de tuin. | de (61) | the | de |  |  |
| 312 | De hond pakt de bal en loopt naar de tuin. | bal (3) | ball | bal |  |  |
| 313 | De hond pakt de bal en loopt naar de tuin. | en (5) | and | en |  |  |
| 314 | De hond pakt de bal en loopt naar de tuin. | loopt (2) | walks | lopen | form of: lopen, to walk |  |
| 315 | De hond pakt de bal en loopt naar de tuin. | naar (9) | to | naar |  |  |
| 316 | De hond pakt de bal en loopt naar de tuin. | de (62) | the | de |  |  |
| 317 | De hond pakt de bal en loopt naar de tuin. | tuin (1) | garden | tuin |  |  |
| 318 | De man zucht. | De (63) | the | de |  |  |
| 319 | De man zucht. | man (17) | man | man |  |  |
| 320 | De man zucht. | zucht (4) | sighs | zuchten | form of: zuchten, to sigh |  |
| 321 | Hij zoekt in de keuken. | Hij (20) | he | hij |  |  |
| 322 | Hij zoekt in de keuken. | zoekt (10) | looks for | zoeken | form of: zoeken, to look for |  |
| 323 | Hij zoekt in de keuken. | in (6) | in | in |  |  |
| 324 | Hij zoekt in de keuken. | de (64) | the | de |  |  |
| 325 | Hij zoekt in de keuken. | keuken (4) | kitchen | keuken |  |  |
| 326 | Hij zoekt in de kast. | Hij (21) | he | hij |  |  |
| 327 | Hij zoekt in de kast. | zoekt (11) | looks for | zoeken | form of: zoeken, to look for |  |
| 328 | Hij zoekt in de kast. | in (7) | in | in |  |  |
| 329 | Hij zoekt in de kast. | de (65) | the | de |  |  |
| 330 | Hij zoekt in de kast. | kast (8) | cupboard | kast |  |  |
| 331 | Hij zoekt op de vloer. | Hij (22) | he | hij |  |  |
| 332 | Hij zoekt op de vloer. | zoekt (12) | looks for | zoeken | form of: zoeken, to look for |  |
| 333 | Hij zoekt op de vloer. | op (6) | on | op |  |  |
| 334 | Hij zoekt op de vloer. | de (66) | the | de |  |  |
| 335 | Hij zoekt op de vloer. | vloer (4) | floor | vloer |  |  |
| 336 | De sleutel is niet in de keuken. | De (67) | the | de |  |  |
| 337 | De sleutel is niet in de keuken. | sleutel (9) | key | sleutel |  |  |
| 338 | De sleutel is niet in de keuken. | is (12) | is | zijn | form of: zijn, to be |  |
| 339 | De sleutel is niet in de keuken. | niet (5) | not | niet |  |  |
| 340 | De sleutel is niet in de keuken. | in (8) | in | in |  |  |
| 341 | De sleutel is niet in de keuken. | de (68) | the | de |  |  |
| 342 | De sleutel is niet in de keuken. | keuken (5) | kitchen | keuken |  |  |
| 343 | De sleutel is niet in de kast. | De (69) | the | de |  |  |
| 344 | De sleutel is niet in de kast. | sleutel (10) | key | sleutel |  |  |
| 345 | De sleutel is niet in de kast. | is (13) | is | zijn | form of: zijn, to be |  |
| 346 | De sleutel is niet in de kast. | niet (6) | not | niet |  |  |
| 347 | De sleutel is niet in de kast. | in (9) | in | in |  |  |
| 348 | De sleutel is niet in de kast. | de (70) | the | de |  |  |
| 349 | De sleutel is niet in de kast. | kast (9) | cupboard | kast |  |  |
| 350 | De sleutel is niet op de vloer. | De (71) | the | de |  |  |
| 351 | De sleutel is niet op de vloer. | sleutel (11) | key | sleutel |  |  |
| 352 | De sleutel is niet op de vloer. | is (14) | is | zijn | form of: zijn, to be |  |
| 353 | De sleutel is niet op de vloer. | niet (7) | not | niet |  |  |
| 354 | De sleutel is niet op de vloer. | op (7) | on | op |  |  |
| 355 | De sleutel is niet op de vloer. | de (72) | the | de |  |  |
| 356 | De sleutel is niet op de vloer. | vloer (5) | floor | vloer |  |  |
| 357 | De hond komt terug. | De (73) | the | de |  |  |
| 358 | De hond komt terug. | hond (13) | dog | hond |  |  |
| 359 | De hond komt terug. | komt (3) | comes | komen | form of: komen, to come |  |
| 360 | De hond komt terug. | terug (3) | back | terug |  |  |
| 361 | Hij heeft de bal niet. | Hij (23) | he | hij |  |  |
| 362 | Hij heeft de bal niet. | heeft (3) | has | hebben | form of: hebben, to have |  |
| 363 | Hij heeft de bal niet. | de (74) | the | de |  |  |
| 364 | Hij heeft de bal niet. | bal (4) | ball | bal |  |  |
| 365 | Hij heeft de bal niet. | niet (8) | not | niet |  |  |
| 366 | Hij kijkt naar de man. | Hij (24) | he | hij |  |  |
| 367 | Hij kijkt naar de man. | kijkt (5) | looks | kijken | form of: kijken, to look |  |
| 368 | Hij kijkt naar de man. | naar (10) | to | naar |  |  |
| 369 | Hij kijkt naar de man. | de (75) | the | de |  |  |
| 370 | Hij kijkt naar de man. | man (18) | man | man |  |  |
| 371 | De man kijkt naar de hond. | De (76) | the | de |  |  |
| 372 | De man kijkt naar de hond. | man (19) | man | man |  |  |
| 373 | De man kijkt naar de hond. | kijkt (6) | looks | kijken | form of: kijken, to look |  |
| 374 | De man kijkt naar de hond. | naar (11) | to | naar |  |  |
| 375 | De man kijkt naar de hond. | de (77) | the | de |  |  |
| 376 | De man kijkt naar de hond. | hond (14) | dog | hond |  |  |
| 377 | “Waar is de sleutel?” vraagt de man. | Waar (3) | where | waar |  |  |
| 378 | “Waar is de sleutel?” vraagt de man. | is (15) | is | zijn | form of: zijn, to be |  |
| 379 | “Waar is de sleutel?” vraagt de man. | de (78) | the | de |  |  |
| 380 | “Waar is de sleutel?” vraagt de man. | sleutel (12) | key | sleutel |  |  |
| 381 | “Waar is de sleutel?” vraagt de man. | vraagt (3) | asks | vragen | form of: vragen, to ask |  |
| 382 | “Waar is de sleutel?” vraagt de man. | de (79) | the | de |  |  |
| 383 | “Waar is de sleutel?” vraagt de man. | man (20) | man | man |  |  |
| 384 | De hond geeft geen antwoord. | De (80) | the | de |  |  |
| 385 | De hond geeft geen antwoord. | hond (15) | dog | hond |  |  |
| 386 | De hond geeft geen antwoord. | geeft (2) | gives | geven | form of: geven, to give |  |
| 387 | De hond geeft geen antwoord. | geen (3) | no / not a | geen |  |  |
| 388 | De hond geeft geen antwoord. | antwoord (2) | answer | antwoord |  |  |
| 389 | De hond loopt naar de bank. | De (81) | the | de |  |  |
| 390 | De hond loopt naar de bank. | hond (16) | dog | hond |  |  |
| 391 | De hond loopt naar de bank. | loopt (3) | walks | lopen | form of: lopen, to walk |  |
| 392 | De hond loopt naar de bank. | naar (12) | to | naar |  |  |
| 393 | De hond loopt naar de bank. | de (82) | the | de |  |  |
| 394 | De hond loopt naar de bank. | bank (12) | sofa / bench | bank |  |  |
| 395 | De man loopt achter de hond aan. | De (83) | the | de |  |  |
| 396 | De man loopt achter de hond aan. | man (21) | man | man |  |  |
| 397 | De man loopt achter de hond aan. | loopt (4) | walks | lopen | form of: lopen, to walk |  |
| 398 | De man loopt achter de hond aan. | achter (1) | behind | achter |  |  |
| 399 | De man loopt achter de hond aan. | de (84) | the | de |  |  |
| 400 | De man loopt achter de hond aan. | hond (17) | dog | hond |  |  |
| 401 | De man loopt achter de hond aan. | aan (2) | after | aan |  |  |
| 402 | De hond staat bij de bank. | De (85) | the | de |  |  |
| 403 | De hond staat bij de bank. | hond (18) | dog | hond |  |  |
| 404 | De hond staat bij de bank. | staat (2) | stands | staan | form of: staan, to stand |  |
| 405 | De hond staat bij de bank. | bij (2) | at | bij |  |  |
| 406 | De hond staat bij de bank. | de (86) | the | de |  |  |
| 407 | De hond staat bij de bank. | bank (13) | sofa / bench | bank |  |  |
| 408 | Hij kwispelt met zijn staart. | Hij (25) | he | hij |  |  |
| 409 | Hij kwispelt met zijn staart. | kwispelt (2) | wags | kwispelen | form of: kwispelen, to wag |  |
| 410 | Hij kwispelt met zijn staart. | met (2) | with | met |  |  |
| 411 | Hij kwispelt met zijn staart. | zijn (7) | his | zijn |  |  |
| 412 | Hij kwispelt met zijn staart. | staart (2) | tail | staart |  |  |
| 413 | De man tilt de bank weer op. | De (87) | the | de |  |  |
| 414 | De man tilt de bank weer op. | man (22) | man | man |  |  |
| 415 | De man tilt de bank weer op. | tilt (3) | lifts | tillen | form of: tillen, to lift |  |
| 416 | De man tilt de bank weer op. | de (88) | the | de |  |  |
| 417 | De man tilt de bank weer op. | bank (14) | sofa / bench | bank |  |  |
| 418 | De man tilt de bank weer op. | weer (3) | again | weer |  |  |
| 419 | De man tilt de bank weer op. | op (8) | on | op |  |  |
| 420 | Dit keer ziet hij iets. | Dit (1) | this | dit |  |  |
| 421 | Dit keer ziet hij iets. | keer (2) | time / turn | keer |  |  |
| 422 | Dit keer ziet hij iets. | ziet (1) | sees | zien | form of: zien, to see |  |
| 423 | Dit keer ziet hij iets. | hij (26) | he | hij |  |  |
| 424 | Dit keer ziet hij iets. | iets (1) | something | iets |  |  |
| 425 | Eindelijk! | Eindelijk (1) | finally | eindelijk |  |  |
| 426 | De sleutel ligt op de vloer. | De (89) | the | de |  |  |
| 427 | De sleutel ligt op de vloer. | sleutel (13) | key | sleutel |  |  |
| 428 | De sleutel ligt op de vloer. | ligt (4) | lies | liggen | form of: liggen, to lie |  |
| 429 | De sleutel ligt op de vloer. | op (9) | on | op |  |  |
| 430 | De sleutel ligt op de vloer. | de (90) | the | de |  |  |
| 431 | De sleutel ligt op de vloer. | vloer (6) | floor | vloer |  |  |
| 432 | Maar de hond is sneller. | Maar (5) | but | maar |  |  |
| 433 | Maar de hond is sneller. | de (91) | the | de |  |  |
| 434 | Maar de hond is sneller. | hond (19) | dog | hond |  |  |
| 435 | Maar de hond is sneller. | is (16) | is | zijn | form of: zijn, to be |  |
| 436 | Maar de hond is sneller. | sneller (1) | faster | snel | form of: snel, fast |  |
| 437 | De hond pakt de sleutel en loopt naar de deur. | De (92) | the | de |  |  |
| 438 | De hond pakt de sleutel en loopt naar de deur. | hond (20) | dog | hond |  |  |
| 439 | De hond pakt de sleutel en loopt naar de deur. | pakt (3) | takes | pakken | form of: pakken, to take / grab |  |
| 440 | De hond pakt de sleutel en loopt naar de deur. | de (93) | the | de |  |  |
| 441 | De hond pakt de sleutel en loopt naar de deur. | sleutel (14) | key | sleutel |  |  |
| 442 | De hond pakt de sleutel en loopt naar de deur. | en (6) | and | en |  |  |
| 443 | De hond pakt de sleutel en loopt naar de deur. | loopt (5) | walks | lopen | form of: lopen, to walk |  |
| 444 | De hond pakt de sleutel en loopt naar de deur. | naar (13) | to | naar |  |  |
| 445 | De hond pakt de sleutel en loopt naar de deur. | de (94) | the | de |  |  |
| 446 | De hond pakt de sleutel en loopt naar de deur. | deur (1) | door | deur |  |  |
| 447 | “Nee, nee!” roept de man. | Nee (1) | no | nee |  |  |
| 448 | “Nee, nee!” roept de man. | nee (2) | no | nee |  |  |
| 449 | “Nee, nee!” roept de man. | roept (1) | calls / shouts | roepen | form of: roepen, to call / shout |  |
| 450 | “Nee, nee!” roept de man. | de (95) | the | de |  |  |
| 451 | “Nee, nee!” roept de man. | man (23) | man | man |  |  |
| 452 | Hij loopt achter de hond aan. | Hij (27) | he | hij |  |  |
| 453 | Hij loopt achter de hond aan. | loopt (6) | walks | lopen | form of: lopen, to walk |  |
| 454 | Hij loopt achter de hond aan. | achter (2) | behind | achter |  |  |
| 455 | Hij loopt achter de hond aan. | de (96) | the | de |  |  |
| 456 | Hij loopt achter de hond aan. | hond (21) | dog | hond |  |  |
| 457 | Hij loopt achter de hond aan. | aan (3) | after | aan |  |  |
| 458 | De hond wacht bij de deur en lacht. | De (97) | the | de |  |  |
| 459 | De hond wacht bij de deur en lacht. | hond (22) | dog | hond |  |  |
| 460 | De hond wacht bij de deur en lacht. | wacht (1) | waits | wachten | form of: wachten, to wait |  |
| 461 | De hond wacht bij de deur en lacht. | bij (3) | at | bij |  |  |
| 462 | De hond wacht bij de deur en lacht. | de (98) | the | de |  |  |
| 463 | De hond wacht bij de deur en lacht. | deur (2) | door | deur |  |  |
| 464 | De hond wacht bij de deur en lacht. | en (7) | and | en |  |  |
| 465 | De hond wacht bij de deur en lacht. | lacht (3) | laughs | lachen | form of: lachen, to laugh |  |
| 466 | De hond heeft de sleutel in zijn bek. | De (99) | the | de |  |  |
| 467 | De hond heeft de sleutel in zijn bek. | hond (23) | dog | hond |  |  |
| 468 | De hond heeft de sleutel in zijn bek. | heeft (4) | has | hebben | form of: hebben, to have |  |
| 469 | De hond heeft de sleutel in zijn bek. | de (100) | the | de |  |  |
| 470 | De hond heeft de sleutel in zijn bek. | sleutel (15) | key | sleutel |  |  |
| 471 | De hond heeft de sleutel in zijn bek. | in (10) | in | in |  |  |
| 472 | De hond heeft de sleutel in zijn bek. | zijn (8) | his | zijn |  |  |
| 473 | De hond heeft de sleutel in zijn bek. | bek (1) | mouth / muzzle | bek |  |  |
| 474 | “Geef hier,” zegt de man. | Geef (1) | give | geven | form of: geven, to give |  |
| 475 | “Geef hier,” zegt de man. | hier (2) | here | hier |  |  |
| 476 | “Geef hier,” zegt de man. | zegt (5) | says | zeggen | form of: zeggen, to say |  |
| 477 | “Geef hier,” zegt de man. | de (101) | the | de |  |  |
| 478 | “Geef hier,” zegt de man. | man (24) | man | man |  |  |
| 479 | De hond laat de sleutel niet los. | De (102) | the | de |  |  |
| 480 | De hond laat de sleutel niet los. | hond (24) | dog | hond |  |  |
| 481 | De hond laat de sleutel niet los. | laat (1) | lets | laten | form of: laten, to let |  |
| 482 | De hond laat de sleutel niet los. | de (103) | the | de |  |  |
| 483 | De hond laat de sleutel niet los. | sleutel (16) | key | sleutel |  |  |
| 484 | De hond laat de sleutel niet los. | niet (9) | not | niet |  |  |
| 485 | De hond laat de sleutel niet los. | los (1) | loose | los |  |  |
| 486 | Plotseling lacht de man. | Plotseling (2) | suddenly | plotseling |  |  |
| 487 | Plotseling lacht de man. | lacht (4) | laughs | lachen | form of: lachen, to laugh |  |
| 488 | Plotseling lacht de man. | de (104) | the | de |  |  |
| 489 | Plotseling lacht de man. | man (25) | man | man |  |  |
| 490 | “Oké, jij hebt gewonnen.” De hond geeft geen antwoord. | Oké (1) | okay | oké |  |  |
| 491 | “Oké, jij hebt gewonnen.” De hond geeft geen antwoord. | jij (1) | you | jij |  |  |
| 492 | “Oké, jij hebt gewonnen.” De hond geeft geen antwoord. | hebt (1) | have | hebben | form of: hebben, to have |  |
| 493 | “Oké, jij hebt gewonnen.” De hond geeft geen antwoord. | gewonnen (1) | won | winnen | form of: winnen, to win |  |
| 494 | “Oké, jij hebt gewonnen.” De hond geeft geen antwoord. | De (105) | the | de |  |  |
| 495 | “Oké, jij hebt gewonnen.” De hond geeft geen antwoord. | hond (25) | dog | hond |  |  |
| 496 | “Oké, jij hebt gewonnen.” De hond geeft geen antwoord. | geeft (3) | gives | geven | form of: geven, to give |  |
| 497 | “Oké, jij hebt gewonnen.” De hond geeft geen antwoord. | geen (4) | no / not a | geen |  |  |
| 498 | “Oké, jij hebt gewonnen.” De hond geeft geen antwoord. | antwoord (3) | answer | antwoord |  |  |
| 499 | De man vindt het grappig. | De (106) | the | de |  |  |
| 500 | De man vindt het grappig. | man (26) | man | man |  |  |
| 501 | De man vindt het grappig. | vindt (2) | finds | vinden | form of: vinden, to find |  |
| 502 | De man vindt het grappig. | het (4) | the | het |  |  |
| 503 | De man vindt het grappig. | grappig (1) | funny | grappig |  |  |
| 504 | Hij is niet meer boos. | Hij (28) | he | hij |  |  |
| 505 | Hij is niet meer boos. | is (17) | is | zijn | form of: zijn, to be |  |
| 506 | Hij is niet meer boos. | niet (10) | not | niet |  |  |
| 507 | Hij is niet meer boos. | meer (1) | more | meer |  |  |
| 508 | Hij is niet meer boos. | boos (1) | angry | boos |  |  |
| 509 | “Waarom heb je de sleutel?” vraagt hij. | Waarom (1) | why | waarom |  |  |
| 510 | “Waarom heb je de sleutel?” vraagt hij. | heb (3) | have | hebben | form of: hebben, to have |  |
| 511 | “Waarom heb je de sleutel?” vraagt hij. | je (2) | you / your | je |  |  |
| 512 | “Waarom heb je de sleutel?” vraagt hij. | de (107) | the | de |  |  |
| 513 | “Waarom heb je de sleutel?” vraagt hij. | sleutel (17) | key | sleutel |  |  |
| 514 | “Waarom heb je de sleutel?” vraagt hij. | vraagt (4) | asks | vragen | form of: vragen, to ask |  |
| 515 | “Waarom heb je de sleutel?” vraagt hij. | hij (29) | he | hij |  |  |
| 516 | De hond kwispelt met zijn staart. | De (108) | the | de |  |  |
| 517 | De hond kwispelt met zijn staart. | hond (26) | dog | hond |  |  |
| 518 | De hond kwispelt met zijn staart. | kwispelt (3) | wags | kwispelen | form of: kwispelen, to wag |  |
| 519 | De hond kwispelt met zijn staart. | met (3) | with | met |  |  |
| 520 | De hond kwispelt met zijn staart. | zijn (9) | his | zijn |  |  |
| 521 | De hond kwispelt met zijn staart. | staart (3) | tail | staart |  |  |
| 522 | De man is vergeten dat hij boos was. | De (109) | the | de |  |  |
| 523 | De man is vergeten dat hij boos was. | man (27) | man | man |  |  |
| 524 | De man is vergeten dat hij boos was. | is (18) | is | zijn | form of: zijn, to be |  |
| 525 | De man is vergeten dat hij boos was. | vergeten (2) | forgotten | vergeten |  |  |
| 526 | De man is vergeten dat hij boos was. | dat (1) | that | dat |  |  |
| 527 | De man is vergeten dat hij boos was. | hij (30) | he | hij |  |  |
| 528 | De man is vergeten dat hij boos was. | boos (2) | angry | boos |  |  |
| 529 | De man is vergeten dat hij boos was. | was (1) | was | zijn | form of: zijn, to be |  |
| 530 | Eindelijk geeft de hond de sleutel. | Eindelijk (2) | finally | eindelijk |  |  |
| 531 | Eindelijk geeft de hond de sleutel. | geeft (4) | gives | geven | form of: geven, to give |  |
| 532 | Eindelijk geeft de hond de sleutel. | de (110) | the | de |  |  |
| 533 | Eindelijk geeft de hond de sleutel. | hond (27) | dog | hond |  |  |
| 534 | Eindelijk geeft de hond de sleutel. | de (111) | the | de |  |  |
| 535 | Eindelijk geeft de hond de sleutel. | sleutel (18) | key | sleutel |  |  |
| 536 | De man pakt hem en lacht. | De (112) | the | de |  |  |
| 537 | De man pakt hem en lacht. | man (28) | man | man |  |  |
| 538 | De man pakt hem en lacht. | pakt (4) | takes | pakken | form of: pakken, to take / grab |  |
| 539 | De man pakt hem en lacht. | hem (1) | him | hem |  |  |
| 540 | De man pakt hem en lacht. | en (8) | and | en |  |  |
| 541 | De man pakt hem en lacht. | lacht (5) | laughs | lachen | form of: lachen, to laugh |  |
| 542 | “Dank je,” zegt hij. | Dank (1) | thanks | dank |  |  |
| 543 | “Dank je,” zegt hij. | je (3) | you / your | je |  |  |
| 544 | “Dank je,” zegt hij. | zegt (6) | says | zeggen | form of: zeggen, to say |  |
| 545 | “Dank je,” zegt hij. | hij (31) | he | hij |  |  |
| 546 | De hond blaft. | De (113) | the | de |  |  |
| 547 | De hond blaft. | hond (28) | dog | hond |  |  |
| 548 | De hond blaft. | blaft (1) | barks | blaffen | form of: blaffen, to bark |  |
| 549 | De man doet de deur open. | De (114) | the | de |  |  |
| 550 | De man doet de deur open. | man (29) | man | man |  |  |
| 551 | De man doet de deur open. | doet (3) | opens | opendoen | form of: opendoen, to open |  |
| 552 | De man doet de deur open. | de (115) | the | de |  |  |
| 553 | De man doet de deur open. | deur (3) | door | deur |  |  |
| 554 | De man doet de deur open. | open (3) | open | opendoen | form of: opendoen, to open |  |
| 555 | Samen gaan ze naar de winkel. | Samen (1) | together | samen |  |  |
| 556 | Samen gaan ze naar de winkel. | gaan (1) | go | gaan |  |  |
| 557 | Samen gaan ze naar de winkel. | ze (1) | they | ze |  |  |
| 558 | Samen gaan ze naar de winkel. | naar (14) | to | naar |  |  |
| 559 | Samen gaan ze naar de winkel. | de (116) | the | de |  |  |
| 560 | Samen gaan ze naar de winkel. | winkel (2) | shop | winkel |  |  |
| 561 | De sleutel is weer thuis. | De (117) | the | de |  |  |
| 562 | De sleutel is weer thuis. | sleutel (19) | key | sleutel |  |  |
| 563 | De sleutel is weer thuis. | is (19) | is | zijn | form of: zijn, to be |  |
| 564 | De sleutel is weer thuis. | weer (4) | again | weer |  |  |
| 565 | De sleutel is weer thuis. | thuis (1) | at home | thuis |  |  |
| 566 | De man is zijn sleutel niet meer kwijt. | De (118) | the | de |  |  |
| 567 | De man is zijn sleutel niet meer kwijt. | man (30) | man | man |  |  |
| 568 | De man is zijn sleutel niet meer kwijt. | is (20) | is | zijn | form of: zijn, to be |  |
| 569 | De man is zijn sleutel niet meer kwijt. | zijn (10) | his | zijn |  |  |
| 570 | De man is zijn sleutel niet meer kwijt. | sleutel (20) | key | sleutel |  |  |
| 571 | De man is zijn sleutel niet meer kwijt. | niet (11) | not | niet |  |  |
| 572 | De man is zijn sleutel niet meer kwijt. | meer (2) | more | meer |  |  |
| 573 | De man is zijn sleutel niet meer kwijt. | kwijt (2) | lost / missing | kwijt |  |  |
| 574 | En de hond? | En (9) | and | en |  |  |
| 575 | En de hond? | de (119) | the | de |  |  |
| 576 | En de hond? | hond (29) | dog | hond |  |  |
| 577 | De hond is heel blij. | De (120) | the | de |  |  |
| 578 | De hond is heel blij. | hond (30) | dog | hond |  |  |
| 579 | De hond is heel blij. | is (21) | is | zijn | form of: zijn, to be |  |
| 580 | De hond is heel blij. | heel (1) | whole / very | heel |  |  |
| 581 | De hond is heel blij. | blij (1) | happy | blij |  |  |
| 582 | De man geeft de hond een aai over zijn kop. | De (121) | the | de |  |  |
| 583 | De man geeft de hond een aai over zijn kop. | man (31) | man | man |  |  |
| 584 | De man geeft de hond een aai over zijn kop. | geeft (5) | gives | geven | form of: geven, to give |  |
| 585 | De man geeft de hond een aai over zijn kop. | de (122) | the | de |  |  |
| 586 | De man geeft de hond een aai over zijn kop. | hond (31) | dog | hond |  |  |
| 587 | De man geeft de hond een aai over zijn kop. | een (6) | a / an | een |  |  |
| 588 | De man geeft de hond een aai over zijn kop. | aai (1) | pat / stroke | aai |  |  |
| 589 | De man geeft de hond een aai over zijn kop. | over (1) | over | over |  |  |
| 590 | De man geeft de hond een aai over zijn kop. | zijn (11) | his | zijn |  |  |
| 591 | De man geeft de hond een aai over zijn kop. | kop (1) | head | kop |  |  |
| 592 | De hond loopt trots naast hem. | De (123) | the | de |  |  |
| 593 | De hond loopt trots naast hem. | hond (32) | dog | hond |  |  |
| 594 | De hond loopt trots naast hem. | loopt (7) | walks | lopen | form of: lopen, to walk |  |
| 595 | De hond loopt trots naast hem. | trots (1) | proud | trots |  |  |
| 596 | De hond loopt trots naast hem. | naast (1) | next to | naast |  |  |
| 597 | De hond loopt trots naast hem. | hem (2) | him | hem |  |  |
| 598 | Bij de winkel geeft de man de hond een bot. | Bij (4) | at | bij |  |  |
| 599 | Bij de winkel geeft de man de hond een bot. | de (124) | the | de |  |  |
| 600 | Bij de winkel geeft de man de hond een bot. | winkel (3) | shop | winkel |  |  |
| 601 | Bij de winkel geeft de man de hond een bot. | geeft (6) | gives | geven | form of: geven, to give |  |
| 602 | Bij de winkel geeft de man de hond een bot. | de (125) | the | de |  |  |
| 603 | Bij de winkel geeft de man de hond een bot. | man (32) | man | man |  |  |
| 604 | Bij de winkel geeft de man de hond een bot. | de (126) | the | de |  |  |
| 605 | Bij de winkel geeft de man de hond een bot. | hond (33) | dog | hond |  |  |
| 606 | Bij de winkel geeft de man de hond een bot. | een (7) | a / an | een |  |  |
| 607 | Bij de winkel geeft de man de hond een bot. | bot (3) | bone | bot |  |  |
| 608 | De hond is blij. | De (127) | the | de |  |  |
| 609 | De hond is blij. | hond (34) | dog | hond |  |  |
| 610 | De hond is blij. | is (22) | is | zijn | form of: zijn, to be |  |
| 611 | De hond is blij. | blij (2) | happy | blij |  |  |
| 612 | De man is ook blij. | De (128) | the | de |  |  |
| 613 | De man is ook blij. | man (33) | man | man |  |  |
| 614 | De man is ook blij. | is (23) | is | zijn | form of: zijn, to be |  |
| 615 | De man is ook blij. | ook (2) | also | ook |  |  |
| 616 | De man is ook blij. | blij (3) | happy | blij |  |  |
| 617 | Samen gaan ze naar huis. | Samen (2) | together | samen |  |  |
| 618 | Samen gaan ze naar huis. | gaan (2) | go | gaan |  |  |
| 619 | Samen gaan ze naar huis. | ze (2) | they | ze |  |  |
| 620 | Samen gaan ze naar huis. | naar (15) | to | naar |  |  |
| 621 | Samen gaan ze naar huis. | huis (1) | house / home | huis |  |  |

### B1 · De laatste stroopwafel (`b1-01`)

| # | Sentence | Tapped (occ) | Shown meaning | Lemma | Base line | Flag |
| ---: | --- | --- | --- | --- | --- | --- |
| 1 | Het is zaterdagochtend en de markt in het centrum van de stad is al druk. | Het (1) | the / it | het |  |  |
| 2 | Het is zaterdagochtend en de markt in het centrum van de stad is al druk. | is (1) | is | zijn | form of: zijn, to be |  |
| 3 | Het is zaterdagochtend en de markt in het centrum van de stad is al druk. | zaterdagochtend (1) | Saturday morning | zaterdagochtend |  |  |
| 4 | Het is zaterdagochtend en de markt in het centrum van de stad is al druk. | en (1) | and | en |  |  |
| 5 | Het is zaterdagochtend en de markt in het centrum van de stad is al druk. | de (1) | the | de |  |  |
| 6 | Het is zaterdagochtend en de markt in het centrum van de stad is al druk. | markt (1) | market | markt |  |  |
| 7 | Het is zaterdagochtend en de markt in het centrum van de stad is al druk. | in (1) | in | in |  |  |
| 8 | Het is zaterdagochtend en de markt in het centrum van de stad is al druk. | het (2) | the / it | het |  |  |
| 9 | Het is zaterdagochtend en de markt in het centrum van de stad is al druk. | centrum (1) | centre | centrum |  |  |
| 10 | Het is zaterdagochtend en de markt in het centrum van de stad is al druk. | van (1) | of / from | van |  |  |
| 11 | Het is zaterdagochtend en de markt in het centrum van de stad is al druk. | de (2) | the | de |  |  |
| 12 | Het is zaterdagochtend en de markt in het centrum van de stad is al druk. | stad (1) | city | stad |  |  |
| 13 | Het is zaterdagochtend en de markt in het centrum van de stad is al druk. | is (2) | is | zijn | form of: zijn, to be |  |
| 14 | Het is zaterdagochtend en de markt in het centrum van de stad is al druk. | al (1) | already | al |  |  |
| 15 | Het is zaterdagochtend en de markt in het centrum van de stad is al druk. | druk (1) | busy | druk |  |  |
| 16 | Noor loopt langs de kramen met kaas, vis en bloemen. | Noor (1) | Noor (girl's name) | Noor |  |  |
| 17 | Noor loopt langs de kramen met kaas, vis en bloemen. | loopt (1) | walks | lopen | form of: lopen, to walk |  |
| 18 | Noor loopt langs de kramen met kaas, vis en bloemen. | langs (1) | along / past | langs |  |  |
| 19 | Noor loopt langs de kramen met kaas, vis en bloemen. | de (3) | the | de |  |  |
| 20 | Noor loopt langs de kramen met kaas, vis en bloemen. | kramen (1) | stalls | kraam | form of: kraam, stall |  |
| 21 | Noor loopt langs de kramen met kaas, vis en bloemen. | met (1) | with | met |  |  |
| 22 | Noor loopt langs de kramen met kaas, vis en bloemen. | kaas (1) | cheese | kaas |  |  |
| 23 | Noor loopt langs de kramen met kaas, vis en bloemen. | vis (1) | fish | vis |  |  |
| 24 | Noor loopt langs de kramen met kaas, vis en bloemen. | en (2) | and | en |  |  |
| 25 | Noor loopt langs de kramen met kaas, vis en bloemen. | bloemen (1) | flowers | bloem | form of: bloem, flower |  |
| 26 | Ze ruikt iets heerlijks: warme stroopwafels. | Ze (1) | she | ze |  |  |
| 27 | Ze ruikt iets heerlijks: warme stroopwafels. | ruikt (1) | smells | ruiken | form of: ruiken, to smell |  |
| 28 | Ze ruikt iets heerlijks: warme stroopwafels. | iets (1) | something | iets |  |  |
| 29 | Ze ruikt iets heerlijks: warme stroopwafels. | heerlijks (1) | delicious (something delicious) | heerlijk | form of: heerlijk, delicious |  |
| 30 | Ze ruikt iets heerlijks: warme stroopwafels. | warme (1) | warm (inflected) | warm | form of: warm, warm |  |
| 31 | Ze ruikt iets heerlijks: warme stroopwafels. | stroopwafels (1) | stroopwafels | stroopwafel | form of: stroopwafel, stroopwafel |  |
| 32 | Haar mond begint te wateren, want ze heeft sinds het ontbijt niets gegeten. | Haar (1) | her | haar |  |  |
| 33 | Haar mond begint te wateren, want ze heeft sinds het ontbijt niets gegeten. | mond (1) | mouth | mond |  |  |
| 34 | Haar mond begint te wateren, want ze heeft sinds het ontbijt niets gegeten. | begint (1) | begins | beginnen | form of: beginnen, to begin |  |
| 35 | Haar mond begint te wateren, want ze heeft sinds het ontbijt niets gegeten. | te (1) | to | te |  |  |
| 36 | Haar mond begint te wateren, want ze heeft sinds het ontbijt niets gegeten. | wateren (1) | to water | wateren |  |  |
| 37 | Haar mond begint te wateren, want ze heeft sinds het ontbijt niets gegeten. | want (1) | because | want |  |  |
| 38 | Haar mond begint te wateren, want ze heeft sinds het ontbijt niets gegeten. | ze (2) | she | ze |  |  |
| 39 | Haar mond begint te wateren, want ze heeft sinds het ontbijt niets gegeten. | heeft (1) | has | hebben | form of: hebben, to have |  |
| 40 | Haar mond begint te wateren, want ze heeft sinds het ontbijt niets gegeten. | sinds (1) | since | sinds |  |  |
| 41 | Haar mond begint te wateren, want ze heeft sinds het ontbijt niets gegeten. | het (3) | the / it | het |  |  |
| 42 | Haar mond begint te wateren, want ze heeft sinds het ontbijt niets gegeten. | ontbijt (1) | breakfast | ontbijt |  |  |
| 43 | Haar mond begint te wateren, want ze heeft sinds het ontbijt niets gegeten. | niets (1) | nothing | niets |  |  |
| 44 | Haar mond begint te wateren, want ze heeft sinds het ontbijt niets gegeten. | gegeten (1) | eaten | eten | form of: eten, to eat |  |
| 45 | Ze heeft drie euro in haar zak. | Ze (3) | she | ze |  |  |
| 46 | Ze heeft drie euro in haar zak. | heeft (2) | has | hebben | form of: hebben, to have |  |
| 47 | Ze heeft drie euro in haar zak. | drie (1) | three | drie |  |  |
| 48 | Ze heeft drie euro in haar zak. | euro (1) | euro | euro |  |  |
| 49 | Ze heeft drie euro in haar zak. | in (2) | in | in |  |  |
| 50 | Ze heeft drie euro in haar zak. | haar (2) | her | haar |  |  |
| 51 | Ze heeft drie euro in haar zak. | zak (1) | pocket / bag | zak |  |  |
| 52 | Dat is precies genoeg voor één stroopwafel. | Dat (1) | that | dat |  |  |
| 53 | Dat is precies genoeg voor één stroopwafel. | is (3) | is | zijn | form of: zijn, to be |  |
| 54 | Dat is precies genoeg voor één stroopwafel. | precies (1) | exactly | precies |  |  |
| 55 | Dat is precies genoeg voor één stroopwafel. | genoeg (1) | enough | genoeg |  |  |
| 56 | Dat is precies genoeg voor één stroopwafel. | voor (1) | for / before | voor |  |  |
| 57 | Dat is precies genoeg voor één stroopwafel. | één (1) | one | één |  |  |
| 58 | Dat is precies genoeg voor één stroopwafel. | stroopwafel (1) | stroopwafel | stroopwafel |  |  |
| 59 | “Vandaag neem ik de laatste,” zegt ze tegen zichzelf. | Vandaag (1) | today | vandaag |  |  |
| 60 | “Vandaag neem ik de laatste,” zegt ze tegen zichzelf. | neem (1) | take | nemen | form of: nemen, to take |  |
| 61 | “Vandaag neem ik de laatste,” zegt ze tegen zichzelf. | ik (1) | I | ik |  |  |
| 62 | “Vandaag neem ik de laatste,” zegt ze tegen zichzelf. | de (4) | the | de |  |  |
| 63 | “Vandaag neem ik de laatste,” zegt ze tegen zichzelf. | laatste (1) | last (inflected) | laatst | form of: laatst, last |  |
| 64 | “Vandaag neem ik de laatste,” zegt ze tegen zichzelf. | zegt (1) | says | zeggen | form of: zeggen, to say |  |
| 65 | “Vandaag neem ik de laatste,” zegt ze tegen zichzelf. | ze (4) | she | ze |  |  |
| 66 | “Vandaag neem ik de laatste,” zegt ze tegen zichzelf. | tegen (1) | against / to | tegen |  |  |
| 67 | “Vandaag neem ik de laatste,” zegt ze tegen zichzelf. | zichzelf (1) | himself / herself | zichzelf |  |  |
| 68 | Ze loopt snel naar de kraam van mevrouw De Vries, waar altijd een lange rij staat. | Ze (5) | she | ze |  |  |
| 69 | Ze loopt snel naar de kraam van mevrouw De Vries, waar altijd een lange rij staat. | loopt (2) | walks | lopen | form of: lopen, to walk |  |
| 70 | Ze loopt snel naar de kraam van mevrouw De Vries, waar altijd een lange rij staat. | snel (1) | fast | snel |  |  |
| 71 | Ze loopt snel naar de kraam van mevrouw De Vries, waar altijd een lange rij staat. | naar (1) | to | naar |  |  |
| 72 | Ze loopt snel naar de kraam van mevrouw De Vries, waar altijd een lange rij staat. | de (5) | the | de |  |  |
| 73 | Ze loopt snel naar de kraam van mevrouw De Vries, waar altijd een lange rij staat. | kraam (1) | stall | kraam |  |  |
| 74 | Ze loopt snel naar de kraam van mevrouw De Vries, waar altijd een lange rij staat. | van (2) | of / from | van |  |  |
| 75 | Ze loopt snel naar de kraam van mevrouw De Vries, waar altijd een lange rij staat. | mevrouw (1) | lady / Mrs | mevrouw |  |  |
| 76 | Ze loopt snel naar de kraam van mevrouw De Vries, waar altijd een lange rij staat. | De (6) | the | de |  |  |
| 77 | Ze loopt snel naar de kraam van mevrouw De Vries, waar altijd een lange rij staat. | Vries (1) | De Vries (surname) | Vries |  |  |
| 78 | Ze loopt snel naar de kraam van mevrouw De Vries, waar altijd een lange rij staat. | waar (1) | where | waar |  |  |
| 79 | Ze loopt snel naar de kraam van mevrouw De Vries, waar altijd een lange rij staat. | altijd (1) | always | altijd |  |  |
| 80 | Ze loopt snel naar de kraam van mevrouw De Vries, waar altijd een lange rij staat. | een (1) | a / an | een |  |  |
| 81 | Ze loopt snel naar de kraam van mevrouw De Vries, waar altijd een lange rij staat. | lange (1) | long (inflected) | lang | form of: lang, long |  |
| 82 | Ze loopt snel naar de kraam van mevrouw De Vries, waar altijd een lange rij staat. | rij (1) | queue | rij |  |  |
| 83 | Ze loopt snel naar de kraam van mevrouw De Vries, waar altijd een lange rij staat. | staat (1) | stands | staan | form of: staan, to stand |  |
| 84 | Vandaag is de rij kort. | Vandaag (2) | today | vandaag |  |  |
| 85 | Vandaag is de rij kort. | is (4) | is | zijn | form of: zijn, to be |  |
| 86 | Vandaag is de rij kort. | de (7) | the | de |  |  |
| 87 | Vandaag is de rij kort. | rij (2) | queue | rij |  |  |
| 88 | Vandaag is de rij kort. | kort (1) | short | kort |  |  |
| 89 | Noor ziet de stapel stroopwafels en glimlacht. | Noor (2) | Noor (girl's name) | Noor |  |  |
| 90 | Noor ziet de stapel stroopwafels en glimlacht. | ziet (1) | sees | zien | form of: zien, to see |  |
| 91 | Noor ziet de stapel stroopwafels en glimlacht. | de (8) | the | de |  |  |
| 92 | Noor ziet de stapel stroopwafels en glimlacht. | stapel (1) | stack | stapel |  |  |
| 93 | Noor ziet de stapel stroopwafels en glimlacht. | stroopwafels (2) | stroopwafels | stroopwafel | form of: stroopwafel, stroopwafel |  |
| 94 | Noor ziet de stapel stroopwafels en glimlacht. | en (3) | and | en |  |  |
| 95 | Noor ziet de stapel stroopwafels en glimlacht. | glimlacht (1) | smiles | glimlachen | form of: glimlachen, to smile |  |
| 96 | Er is nog maar één stroopwafel over. | Er (1) | there | er |  |  |
| 97 | Er is nog maar één stroopwafel over. | is (5) | is | zijn | form of: zijn, to be |  |
| 98 | Er is nog maar één stroopwafel over. | nog (1) | still / yet | nog |  |  |
| 99 | Er is nog maar één stroopwafel over. | maar (1) | but | maar |  |  |
| 100 | Er is nog maar één stroopwafel over. | één (2) | one | één |  |  |
| 101 | Er is nog maar één stroopwafel over. | stroopwafel (2) | stroopwafel | stroopwafel |  |  |
| 102 | Er is nog maar één stroopwafel over. | over (1) | over / about | over |  |  |
| 103 | “Die is van mij,” denkt ze. | Die (1) | that one | die |  |  |
| 104 | “Die is van mij,” denkt ze. | is (6) | is | zijn | form of: zijn, to be |  |
| 105 | “Die is van mij,” denkt ze. | van (3) | of / from | van |  |  |
| 106 | “Die is van mij,” denkt ze. | mij (1) | me | mij |  |  |
| 107 | “Die is van mij,” denkt ze. | denkt (1) | thinks | denken | form of: denken, to think |  |
| 108 | “Die is van mij,” denkt ze. | ze (6) | she | ze |  |  |
| 109 | Bij de kraam staat een oude bakker met een witte schort. | Bij (1) | at / with | bij |  |  |
| 110 | Bij de kraam staat een oude bakker met een witte schort. | de (9) | the | de |  |  |
| 111 | Bij de kraam staat een oude bakker met een witte schort. | kraam (2) | stall | kraam |  |  |
| 112 | Bij de kraam staat een oude bakker met een witte schort. | staat (2) | stands | staan | form of: staan, to stand |  |
| 113 | Bij de kraam staat een oude bakker met een witte schort. | een (2) | a / an | een |  |  |
| 114 | Bij de kraam staat een oude bakker met een witte schort. | oude (1) | old (inflected) | oud | form of: oud, old |  |
| 115 | Bij de kraam staat een oude bakker met een witte schort. | bakker (1) | baker | bakker |  |  |
| 116 | Bij de kraam staat een oude bakker met een witte schort. | met (2) | with | met |  |  |
| 117 | Bij de kraam staat een oude bakker met een witte schort. | een (3) | a / an | een |  |  |
| 118 | Bij de kraam staat een oude bakker met een witte schort. | witte (1) | white (inflected) | wit | form of: wit, white |  |
| 119 | Bij de kraam staat een oude bakker met een witte schort. | schort (1) | apron | schort |  |  |
| 120 | “Goedemorgen, Noor,” zegt hij vriendelijk. | Goedemorgen (1) | good morning | goedemorgen |  |  |
| 121 | “Goedemorgen, Noor,” zegt hij vriendelijk. | Noor (3) | Noor (girl's name) | Noor |  |  |
| 122 | “Goedemorgen, Noor,” zegt hij vriendelijk. | zegt (2) | says | zeggen | form of: zeggen, to say |  |
| 123 | “Goedemorgen, Noor,” zegt hij vriendelijk. | hij (1) | he | hij |  |  |
| 124 | “Goedemorgen, Noor,” zegt hij vriendelijk. | vriendelijk (1) | friendly | vriendelijk |  |  |
| 125 | “Je bent de eerste vandaag.” Noor wijst naar de laatste stroopwafel. | Je (1) | you / your | je |  |  |
| 126 | “Je bent de eerste vandaag.” Noor wijst naar de laatste stroopwafel. | bent (1) | are | zijn | form of: zijn, to be |  |
| 127 | “Je bent de eerste vandaag.” Noor wijst naar de laatste stroopwafel. | de (10) | the | de |  |  |
| 128 | “Je bent de eerste vandaag.” Noor wijst naar de laatste stroopwafel. | eerste (1) | first | eerste |  |  |
| 129 | “Je bent de eerste vandaag.” Noor wijst naar de laatste stroopwafel. | vandaag (3) | today | vandaag |  |  |
| 130 | “Je bent de eerste vandaag.” Noor wijst naar de laatste stroopwafel. | Noor (4) | Noor (girl's name) | Noor |  |  |
| 131 | “Je bent de eerste vandaag.” Noor wijst naar de laatste stroopwafel. | wijst (1) | points | wijzen | form of: wijzen, to point |  |
| 132 | “Je bent de eerste vandaag.” Noor wijst naar de laatste stroopwafel. | naar (2) | to | naar |  |  |
| 133 | “Je bent de eerste vandaag.” Noor wijst naar de laatste stroopwafel. | de (11) | the | de |  |  |
| 134 | “Je bent de eerste vandaag.” Noor wijst naar de laatste stroopwafel. | laatste (2) | last (inflected) | laatst | form of: laatst, last |  |
| 135 | “Je bent de eerste vandaag.” Noor wijst naar de laatste stroopwafel. | stroopwafel (3) | stroopwafel | stroopwafel |  |  |
| 136 | “Is die nog vrij?” vraagt ze. | Is (7) | is | zijn | form of: zijn, to be |  |
| 137 | “Is die nog vrij?” vraagt ze. | die (2) | that one | die |  |  |
| 138 | “Is die nog vrij?” vraagt ze. | nog (2) | still / yet | nog |  |  |
| 139 | “Is die nog vrij?” vraagt ze. | vrij (1) | free | vrij |  |  |
| 140 | “Is die nog vrij?” vraagt ze. | vraagt (1) | asks | vragen | form of: vragen, to ask |  |
| 141 | “Is die nog vrij?” vraagt ze. | ze (7) | she | ze |  |  |
| 142 | De bakker knikt. | De (12) | the | de |  |  |
| 143 | De bakker knikt. | bakker (2) | baker | bakker |  |  |
| 144 | De bakker knikt. | knikt (1) | nods | knikken | form of: knikken, to nod |  |
| 145 | “Ja, maar wees snel, want de meeuwen zijn vandaag erg brutaal.” Noor lacht. | Ja (1) | yes | ja |  |  |
| 146 | “Ja, maar wees snel, want de meeuwen zijn vandaag erg brutaal.” Noor lacht. | maar (2) | but | maar |  |  |
| 147 | “Ja, maar wees snel, want de meeuwen zijn vandaag erg brutaal.” Noor lacht. | wees (1) | be | zijn | form of: zijn, to be |  |
| 148 | “Ja, maar wees snel, want de meeuwen zijn vandaag erg brutaal.” Noor lacht. | snel (2) | fast | snel |  |  |
| 149 | “Ja, maar wees snel, want de meeuwen zijn vandaag erg brutaal.” Noor lacht. | want (2) | because | want |  |  |
| 150 | “Ja, maar wees snel, want de meeuwen zijn vandaag erg brutaal.” Noor lacht. | de (13) | the | de |  |  |
| 151 | “Ja, maar wees snel, want de meeuwen zijn vandaag erg brutaal.” Noor lacht. | meeuwen (1) | seagulls | meeuw | form of: meeuw, seagull |  |
| 152 | “Ja, maar wees snel, want de meeuwen zijn vandaag erg brutaal.” Noor lacht. | zijn (1) | are | zijn |  |  |
| 153 | “Ja, maar wees snel, want de meeuwen zijn vandaag erg brutaal.” Noor lacht. | vandaag (4) | today | vandaag |  |  |
| 154 | “Ja, maar wees snel, want de meeuwen zijn vandaag erg brutaal.” Noor lacht. | erg (1) | very | erg |  |  |
| 155 | “Ja, maar wees snel, want de meeuwen zijn vandaag erg brutaal.” Noor lacht. | brutaal (1) | cheeky | brutaal |  |  |
| 156 | “Ja, maar wees snel, want de meeuwen zijn vandaag erg brutaal.” Noor lacht. | Noor (5) | Noor (girl's name) | Noor |  |  |
| 157 | “Ja, maar wees snel, want de meeuwen zijn vandaag erg brutaal.” Noor lacht. | lacht (1) | laughs | lachen | form of: lachen, to laugh |  |
| 158 | Ze geeft hem drie euro en pakt de stroopwafel voorzichtig vast. | Ze (8) | she | ze |  |  |
| 159 | Ze geeft hem drie euro en pakt de stroopwafel voorzichtig vast. | geeft (1) | gives | geven | form of: geven, to give |  |
| 160 | Ze geeft hem drie euro en pakt de stroopwafel voorzichtig vast. | hem (1) | him | hem |  |  |
| 161 | Ze geeft hem drie euro en pakt de stroopwafel voorzichtig vast. | drie (2) | three | drie |  |  |
| 162 | Ze geeft hem drie euro en pakt de stroopwafel voorzichtig vast. | euro (2) | euro | euro |  |  |
| 163 | Ze geeft hem drie euro en pakt de stroopwafel voorzichtig vast. | en (4) | and | en |  |  |
| 164 | Ze geeft hem drie euro en pakt de stroopwafel voorzichtig vast. | pakt (1) | takes | pakken | form of: pakken, to take / grab |  |
| 165 | Ze geeft hem drie euro en pakt de stroopwafel voorzichtig vast. | de (14) | the | de |  |  |
| 166 | Ze geeft hem drie euro en pakt de stroopwafel voorzichtig vast. | stroopwafel (4) | stroopwafel | stroopwafel |  |  |
| 167 | Ze geeft hem drie euro en pakt de stroopwafel voorzichtig vast. | voorzichtig (1) | careful | voorzichtig |  |  |
| 168 | Ze geeft hem drie euro en pakt de stroopwafel voorzichtig vast. | vast (1) | firmly | vast |  |  |
| 169 | De stroopwafel is nog warm en ruikt heerlijk. | De (15) | the | de |  |  |
| 170 | De stroopwafel is nog warm en ruikt heerlijk. | stroopwafel (5) | stroopwafel | stroopwafel |  |  |
| 171 | De stroopwafel is nog warm en ruikt heerlijk. | is (8) | is | zijn | form of: zijn, to be |  |
| 172 | De stroopwafel is nog warm en ruikt heerlijk. | nog (3) | still / yet | nog |  |  |
| 173 | De stroopwafel is nog warm en ruikt heerlijk. | warm (1) | warm | warm |  |  |
| 174 | De stroopwafel is nog warm en ruikt heerlijk. | en (5) | and | en |  |  |
| 175 | De stroopwafel is nog warm en ruikt heerlijk. | ruikt (2) | smells | ruiken | form of: ruiken, to smell |  |
| 176 | De stroopwafel is nog warm en ruikt heerlijk. | heerlijk (1) | delicious | heerlijk |  |  |
| 177 | Ze wil net een hap nemen, als er plotseling een grote meeuw op de kraam landt. | Ze (9) | she | ze |  |  |
| 178 | Ze wil net een hap nemen, als er plotseling een grote meeuw op de kraam landt. | wil (1) | wants | willen | form of: willen, to want |  |
| 179 | Ze wil net een hap nemen, als er plotseling een grote meeuw op de kraam landt. | net (1) | just | net |  |  |
| 180 | Ze wil net een hap nemen, als er plotseling een grote meeuw op de kraam landt. | een (4) | a / an | een |  |  |
| 181 | Ze wil net een hap nemen, als er plotseling een grote meeuw op de kraam landt. | hap (1) | bite | hap |  |  |
| 182 | Ze wil net een hap nemen, als er plotseling een grote meeuw op de kraam landt. | nemen (1) | to take | nemen |  |  |
| 183 | Ze wil net een hap nemen, als er plotseling een grote meeuw op de kraam landt. | als (1) | when | als |  |  |
| 184 | Ze wil net een hap nemen, als er plotseling een grote meeuw op de kraam landt. | er (2) | there | er |  |  |
| 185 | Ze wil net een hap nemen, als er plotseling een grote meeuw op de kraam landt. | plotseling (1) | suddenly | plotseling |  |  |
| 186 | Ze wil net een hap nemen, als er plotseling een grote meeuw op de kraam landt. | een (5) | a / an | een |  |  |
| 187 | Ze wil net een hap nemen, als er plotseling een grote meeuw op de kraam landt. | grote (1) | big (inflected) | groot | form of: groot, big |  |
| 188 | Ze wil net een hap nemen, als er plotseling een grote meeuw op de kraam landt. | meeuw (1) | seagull | meeuw |  |  |
| 189 | Ze wil net een hap nemen, als er plotseling een grote meeuw op de kraam landt. | op (1) | on | op |  |  |
| 190 | Ze wil net een hap nemen, als er plotseling een grote meeuw op de kraam landt. | de (16) | the | de |  |  |
| 191 | Ze wil net een hap nemen, als er plotseling een grote meeuw op de kraam landt. | kraam (3) | stall | kraam |  |  |
| 192 | Ze wil net een hap nemen, als er plotseling een grote meeuw op de kraam landt. | landt (1) | lands | landen | form of: landen, to land |  |
| 193 | De vogel kijkt naar de stroopwafel. | De (17) | the | de |  |  |
| 194 | De vogel kijkt naar de stroopwafel. | vogel (1) | bird | vogel |  |  |
| 195 | De vogel kijkt naar de stroopwafel. | kijkt (1) | looks | kijken | form of: kijken, to look |  |
| 196 | De vogel kijkt naar de stroopwafel. | naar (3) | to | naar |  |  |
| 197 | De vogel kijkt naar de stroopwafel. | de (18) | the | de |  |  |
| 198 | De vogel kijkt naar de stroopwafel. | stroopwafel (6) | stroopwafel | stroopwafel |  |  |
| 199 | “Nee,” zegt Noor streng. | Nee (1) | no | nee |  |  |
| 200 | “Nee,” zegt Noor streng. | zegt (3) | says | zeggen | form of: zeggen, to say |  |
| 201 | “Nee,” zegt Noor streng. | Noor (6) | Noor (girl's name) | Noor |  |  |
| 202 | “Nee,” zegt Noor streng. | streng (1) | strict | streng |  |  |
| 203 | “Dit is mijn ontbijt.” De meeuw kijkt haar aan en grijnst. | Dit (1) | this | dit |  |  |
| 204 | “Dit is mijn ontbijt.” De meeuw kijkt haar aan en grijnst. | is (9) | is | zijn | form of: zijn, to be |  |
| 205 | “Dit is mijn ontbijt.” De meeuw kijkt haar aan en grijnst. | mijn (1) | my | mijn |  |  |
| 206 | “Dit is mijn ontbijt.” De meeuw kijkt haar aan en grijnst. | ontbijt (2) | breakfast | ontbijt |  |  |
| 207 | “Dit is mijn ontbijt.” De meeuw kijkt haar aan en grijnst. | De (19) | the | de |  |  |
| 208 | “Dit is mijn ontbijt.” De meeuw kijkt haar aan en grijnst. | meeuw (2) | seagull | meeuw |  |  |
| 209 | “Dit is mijn ontbijt.” De meeuw kijkt haar aan en grijnst. | kijkt (2) | looks | kijken | form of: kijken, to look |  |
| 210 | “Dit is mijn ontbijt.” De meeuw kijkt haar aan en grijnst. | haar (3) | her | haar |  |  |
| 211 | “Dit is mijn ontbijt.” De meeuw kijkt haar aan en grijnst. | aan (1) | at | aan |  |  |
| 212 | “Dit is mijn ontbijt.” De meeuw kijkt haar aan en grijnst. | en (6) | and | en |  |  |
| 213 | “Dit is mijn ontbijt.” De meeuw kijkt haar aan en grijnst. | grijnst (1) | grins | grijnzen | form of: grijnzen, to grin |  |
| 214 | De meeuw wacht niet. | De (20) | the | de |  |  |
| 215 | De meeuw wacht niet. | meeuw (3) | seagull | meeuw |  |  |
| 216 | De meeuw wacht niet. | wacht (1) | waits | wachten | form of: wachten, to wait |  |
| 217 | De meeuw wacht niet. | niet (1) | not | niet |  |  |
| 218 | Hij steelt de stroopwafel met zijn snavel en vliegt weg. | Hij (2) | he | hij |  |  |
| 219 | Hij steelt de stroopwafel met zijn snavel en vliegt weg. | steelt (1) | steals | stelen | form of: stelen, to steal |  |
| 220 | Hij steelt de stroopwafel met zijn snavel en vliegt weg. | de (21) | the | de |  |  |
| 221 | Hij steelt de stroopwafel met zijn snavel en vliegt weg. | stroopwafel (7) | stroopwafel | stroopwafel |  |  |
| 222 | Hij steelt de stroopwafel met zijn snavel en vliegt weg. | met (3) | with | met |  |  |
| 223 | Hij steelt de stroopwafel met zijn snavel en vliegt weg. | zijn (2) | his | zijn |  |  |
| 224 | Hij steelt de stroopwafel met zijn snavel en vliegt weg. | snavel (1) | beak | snavel |  |  |
| 225 | Hij steelt de stroopwafel met zijn snavel en vliegt weg. | en (7) | and | en |  |  |
| 226 | Hij steelt de stroopwafel met zijn snavel en vliegt weg. | vliegt (1) | flies | vliegen | form of: vliegen, to fly |  |
| 227 | Hij steelt de stroopwafel met zijn snavel en vliegt weg. | weg (1) | away | weg |  |  |
| 228 | “Hé!” roept Noor. | Hé (1) | hey | hé |  |  |
| 229 | “Hé!” roept Noor. | roept (1) | calls / shouts | roepen | form of: roepen, to call / shout |  |
| 230 | “Hé!” roept Noor. | Noor (7) | Noor (girl's name) | Noor |  |  |
| 231 | “Dat is mijn stroopwafel!” Ze rent de meeuw achterna. | Dat (2) | that | dat |  |  |
| 232 | “Dat is mijn stroopwafel!” Ze rent de meeuw achterna. | is (10) | is | zijn | form of: zijn, to be |  |
| 233 | “Dat is mijn stroopwafel!” Ze rent de meeuw achterna. | mijn (2) | my | mijn |  |  |
| 234 | “Dat is mijn stroopwafel!” Ze rent de meeuw achterna. | stroopwafel (8) | stroopwafel | stroopwafel |  |  |
| 235 | “Dat is mijn stroopwafel!” Ze rent de meeuw achterna. | Ze (10) | she | ze |  |  |
| 236 | “Dat is mijn stroopwafel!” Ze rent de meeuw achterna. | rent (1) | runs | rennen | form of: rennen, to run |  |
| 237 | “Dat is mijn stroopwafel!” Ze rent de meeuw achterna. | de (22) | the | de |  |  |
| 238 | “Dat is mijn stroopwafel!” Ze rent de meeuw achterna. | meeuw (4) | seagull | meeuw |  |  |
| 239 | “Dat is mijn stroopwafel!” Ze rent de meeuw achterna. | achterna (1) | after (in 'achterna rennen') | achterna |  |  |
| 240 | De meeuw vliegt laag over de markt, langs de kramen en over de hoofden van de mensen. | De (23) | the | de |  |  |
| 241 | De meeuw vliegt laag over de markt, langs de kramen en over de hoofden van de mensen. | meeuw (5) | seagull | meeuw |  |  |
| 242 | De meeuw vliegt laag over de markt, langs de kramen en over de hoofden van de mensen. | vliegt (2) | flies | vliegen | form of: vliegen, to fly |  |
| 243 | De meeuw vliegt laag over de markt, langs de kramen en over de hoofden van de mensen. | laag (1) | low | laag |  |  |
| 244 | De meeuw vliegt laag over de markt, langs de kramen en over de hoofden van de mensen. | over (2) | over / about | over |  |  |
| 245 | De meeuw vliegt laag over de markt, langs de kramen en over de hoofden van de mensen. | de (24) | the | de |  |  |
| 246 | De meeuw vliegt laag over de markt, langs de kramen en over de hoofden van de mensen. | markt (2) | market | markt |  |  |
| 247 | De meeuw vliegt laag over de markt, langs de kramen en over de hoofden van de mensen. | langs (2) | along / past | langs |  |  |
| 248 | De meeuw vliegt laag over de markt, langs de kramen en over de hoofden van de mensen. | de (25) | the | de |  |  |
| 249 | De meeuw vliegt laag over de markt, langs de kramen en over de hoofden van de mensen. | kramen (2) | stalls | kraam | form of: kraam, stall |  |
| 250 | De meeuw vliegt laag over de markt, langs de kramen en over de hoofden van de mensen. | en (8) | and | en |  |  |
| 251 | De meeuw vliegt laag over de markt, langs de kramen en over de hoofden van de mensen. | over (3) | over / about | over |  |  |
| 252 | De meeuw vliegt laag over de markt, langs de kramen en over de hoofden van de mensen. | de (26) | the | de |  |  |
| 253 | De meeuw vliegt laag over de markt, langs de kramen en over de hoofden van de mensen. | hoofden (1) | heads | hoofd | form of: hoofd, head |  |
| 254 | De meeuw vliegt laag over de markt, langs de kramen en over de hoofden van de mensen. | van (4) | of / from | van |  |  |
| 255 | De meeuw vliegt laag over de markt, langs de kramen en over de hoofden van de mensen. | de (27) | the | de |  |  |
| 256 | De meeuw vliegt laag over de markt, langs de kramen en over de hoofden van de mensen. | mensen (1) | people | mens | form of: mens, person / human |  |
| 257 | Noor rent zo snel ze kan, maar de meeuw is sneller. | Noor (8) | Noor (girl's name) | Noor |  |  |
| 258 | Noor rent zo snel ze kan, maar de meeuw is sneller. | rent (2) | runs | rennen | form of: rennen, to run |  |
| 259 | Noor rent zo snel ze kan, maar de meeuw is sneller. | zo (1) | so | zo |  |  |
| 260 | Noor rent zo snel ze kan, maar de meeuw is sneller. | snel (3) | fast | snel |  |  |
| 261 | Noor rent zo snel ze kan, maar de meeuw is sneller. | ze (11) | she | ze |  |  |
| 262 | Noor rent zo snel ze kan, maar de meeuw is sneller. | kan (1) | can | kunnen | form of: kunnen, to be able to |  |
| 263 | Noor rent zo snel ze kan, maar de meeuw is sneller. | maar (3) | but | maar |  |  |
| 264 | Noor rent zo snel ze kan, maar de meeuw is sneller. | de (28) | the | de |  |  |
| 265 | Noor rent zo snel ze kan, maar de meeuw is sneller. | meeuw (6) | seagull | meeuw |  |  |
| 266 | Noor rent zo snel ze kan, maar de meeuw is sneller. | is (11) | is | zijn | form of: zijn, to be |  |
| 267 | Noor rent zo snel ze kan, maar de meeuw is sneller. | sneller (1) | faster | snel | form of: snel, fast |  |
| 268 | Ze hijgt en haar gezicht is rood. | Ze (12) | she | ze |  |  |
| 269 | Ze hijgt en haar gezicht is rood. | hijgt (1) | pants | hijgen | form of: hijgen, to pant |  |
| 270 | Ze hijgt en haar gezicht is rood. | en (9) | and | en |  |  |
| 271 | Ze hijgt en haar gezicht is rood. | haar (4) | her | haar |  |  |
| 272 | Ze hijgt en haar gezicht is rood. | gezicht (1) | face | gezicht |  |  |
| 273 | Ze hijgt en haar gezicht is rood. | is (12) | is | zijn | form of: zijn, to be |  |
| 274 | Ze hijgt en haar gezicht is rood. | rood (1) | red | rood |  |  |
| 275 | De bakker roept iets, maar ze hoort het niet. | De (29) | the | de |  |  |
| 276 | De bakker roept iets, maar ze hoort het niet. | bakker (3) | baker | bakker |  |  |
| 277 | De bakker roept iets, maar ze hoort het niet. | roept (2) | calls / shouts | roepen | form of: roepen, to call / shout |  |
| 278 | De bakker roept iets, maar ze hoort het niet. | iets (2) | something | iets |  |  |
| 279 | De bakker roept iets, maar ze hoort het niet. | maar (4) | but | maar |  |  |
| 280 | De bakker roept iets, maar ze hoort het niet. | ze (13) | she | ze |  |  |
| 281 | De bakker roept iets, maar ze hoort het niet. | hoort (1) | hears | horen | form of: horen, to hear |  |
| 282 | De bakker roept iets, maar ze hoort het niet. | het (4) | the / it | het |  |  |
| 283 | De bakker roept iets, maar ze hoort het niet. | niet (2) | not | niet |  |  |
| 284 | Ze wil die stroopwafel terug. | Ze (14) | she | ze |  |  |
| 285 | Ze wil die stroopwafel terug. | wil (2) | wants | willen | form of: willen, to want |  |
| 286 | Ze wil die stroopwafel terug. | die (3) | that | die |  |  |
| 287 | Ze wil die stroopwafel terug. | stroopwafel (9) | stroopwafel | stroopwafel |  |  |
| 288 | Ze wil die stroopwafel terug. | terug (1) | back | terug |  |  |
| 289 | De meeuw landt op een bankje bij de gracht. | De (30) | the | de |  |  |
| 290 | De meeuw landt op een bankje bij de gracht. | meeuw (7) | seagull | meeuw |  |  |
| 291 | De meeuw landt op een bankje bij de gracht. | landt (2) | lands | landen | form of: landen, to land |  |
| 292 | De meeuw landt op een bankje bij de gracht. | op (2) | on | op |  |  |
| 293 | De meeuw landt op een bankje bij de gracht. | een (6) | a / an | een |  |  |
| 294 | De meeuw landt op een bankje bij de gracht. | bankje (1) | bench | bankje |  |  |
| 295 | De meeuw landt op een bankje bij de gracht. | bij (2) | at / with | bij |  |  |
| 296 | De meeuw landt op een bankje bij de gracht. | de (31) | the | de |  |  |
| 297 | De meeuw landt op een bankje bij de gracht. | gracht (1) | canal | gracht |  |  |
| 298 | Noor is buiten adem als ze eindelijk bij het bankje komt. | Noor (9) | Noor (girl's name) | Noor |  |  |
| 299 | Noor is buiten adem als ze eindelijk bij het bankje komt. | is (13) | is | zijn | form of: zijn, to be |  |
| 300 | Noor is buiten adem als ze eindelijk bij het bankje komt. | buiten (1) | outside | buiten |  |  |
| 301 | Noor is buiten adem als ze eindelijk bij het bankje komt. | adem (1) | breath | adem |  |  |
| 302 | Noor is buiten adem als ze eindelijk bij het bankje komt. | als (2) | when | als |  |  |
| 303 | Noor is buiten adem als ze eindelijk bij het bankje komt. | ze (15) | she | ze |  |  |
| 304 | Noor is buiten adem als ze eindelijk bij het bankje komt. | eindelijk (1) | finally | eindelijk |  |  |
| 305 | Noor is buiten adem als ze eindelijk bij het bankje komt. | bij (3) | at / with | bij |  |  |
| 306 | Noor is buiten adem als ze eindelijk bij het bankje komt. | het (5) | the / it | het |  |  |
| 307 | Noor is buiten adem als ze eindelijk bij het bankje komt. | bankje (2) | bench | bankje |  |  |
| 308 | Noor is buiten adem als ze eindelijk bij het bankje komt. | komt (1) | comes | komen | form of: komen, to come |  |
| 309 | De meeuw heeft de stroopwafel al opgegeten. | De (32) | the | de |  |  |
| 310 | De meeuw heeft de stroopwafel al opgegeten. | meeuw (8) | seagull | meeuw |  |  |
| 311 | De meeuw heeft de stroopwafel al opgegeten. | heeft (3) | has | hebben | form of: hebben, to have |  |
| 312 | De meeuw heeft de stroopwafel al opgegeten. | de (33) | the | de |  |  |
| 313 | De meeuw heeft de stroopwafel al opgegeten. | stroopwafel (10) | stroopwafel | stroopwafel |  |  |
| 314 | De meeuw heeft de stroopwafel al opgegeten. | al (2) | already | al |  |  |
| 315 | De meeuw heeft de stroopwafel al opgegeten. | opgegeten (1) | ate up | opeten | form of: opeten, to eat up |  |
| 316 | Hij kijkt haar aan met een tevreden blik. | Hij (3) | he | hij |  |  |
| 317 | Hij kijkt haar aan met een tevreden blik. | kijkt (3) | looks | kijken | form of: kijken, to look |  |
| 318 | Hij kijkt haar aan met een tevreden blik. | haar (5) | her | haar |  |  |
| 319 | Hij kijkt haar aan met een tevreden blik. | aan (2) | at | aan |  |  |
| 320 | Hij kijkt haar aan met een tevreden blik. | met (4) | with | met |  |  |
| 321 | Hij kijkt haar aan met een tevreden blik. | een (7) | a / an | een |  |  |
| 322 | Hij kijkt haar aan met een tevreden blik. | tevreden (1) | satisfied | tevreden |  |  |
| 323 | Hij kijkt haar aan met een tevreden blik. | blik (1) | look / glance | blik |  |  |
| 324 | Noor gaat teleurgesteld op het bankje zitten. | Noor (10) | Noor (girl's name) | Noor |  |  |
| 325 | Noor gaat teleurgesteld op het bankje zitten. | gaat (1) | goes | gaan | form of: gaan, to go |  |
| 326 | Noor gaat teleurgesteld op het bankje zitten. | teleurgesteld (1) | disappointed | teleurgesteld |  |  |
| 327 | Noor gaat teleurgesteld op het bankje zitten. | op (3) | on | op |  |  |
| 328 | Noor gaat teleurgesteld op het bankje zitten. | het (6) | the / it | het |  |  |
| 329 | Noor gaat teleurgesteld op het bankje zitten. | bankje (3) | bench | bankje |  |  |
| 330 | Noor gaat teleurgesteld op het bankje zitten. | zitten (1) | to sit | zitten |  |  |
| 331 | Ze kijkt naar de meeuw, die nu rustig zijn veren schoonmaakt. | Ze (16) | she | ze |  |  |
| 332 | Ze kijkt naar de meeuw, die nu rustig zijn veren schoonmaakt. | kijkt (4) | looks | kijken | form of: kijken, to look |  |
| 333 | Ze kijkt naar de meeuw, die nu rustig zijn veren schoonmaakt. | naar (4) | to | naar |  |  |
| 334 | Ze kijkt naar de meeuw, die nu rustig zijn veren schoonmaakt. | de (34) | the | de |  |  |
| 335 | Ze kijkt naar de meeuw, die nu rustig zijn veren schoonmaakt. | meeuw (9) | seagull | meeuw |  |  |
| 336 | Ze kijkt naar de meeuw, die nu rustig zijn veren schoonmaakt. | die (4) | who | die |  |  |
| 337 | Ze kijkt naar de meeuw, die nu rustig zijn veren schoonmaakt. | nu (1) | now | nu |  |  |
| 338 | Ze kijkt naar de meeuw, die nu rustig zijn veren schoonmaakt. | rustig (1) | calm / quiet | rustig |  |  |
| 339 | Ze kijkt naar de meeuw, die nu rustig zijn veren schoonmaakt. | zijn (3) | his | zijn |  |  |
| 340 | Ze kijkt naar de meeuw, die nu rustig zijn veren schoonmaakt. | veren (1) | feathers | veer | form of: veer, feather |  |
| 341 | Ze kijkt naar de meeuw, die nu rustig zijn veren schoonmaakt. | schoonmaakt (1) | cleans | schoonmaken | form of: schoonmaken, to clean |  |
| 342 | “Jij hebt geen schuld,” zegt ze zacht. | Jij (1) | you | jij |  |  |
| 343 | “Jij hebt geen schuld,” zegt ze zacht. | hebt (1) | have | hebben | form of: hebben, to have |  |
| 344 | “Jij hebt geen schuld,” zegt ze zacht. | geen (1) | no / not a | geen |  |  |
| 345 | “Jij hebt geen schuld,” zegt ze zacht. | schuld (1) | blame / debt | schuld |  |  |
| 346 | “Jij hebt geen schuld,” zegt ze zacht. | zegt (4) | says | zeggen | form of: zeggen, to say |  |
| 347 | “Jij hebt geen schuld,” zegt ze zacht. | ze (17) | she | ze |  |  |
| 348 | “Jij hebt geen schuld,” zegt ze zacht. | zacht (1) | soft / quiet | zacht |  |  |
| 349 | “Jij bent gewoon een meeuw.” | Jij (2) | you | jij |  |  |
| 350 | “Jij bent gewoon een meeuw.” | bent (2) | are | zijn | form of: zijn, to be |  |
| 351 | “Jij bent gewoon een meeuw.” | gewoon (1) | just / normal | gewoon |  |  |
| 352 | “Jij bent gewoon een meeuw.” | een (8) | a / an | een |  |  |
| 353 | “Jij bent gewoon een meeuw.” | meeuw (10) | seagull | meeuw |  |  |
| 354 | Een buurman die zijn hond uitlaat, blijft staan. | Een (9) | a / an | een |  |  |
| 355 | Een buurman die zijn hond uitlaat, blijft staan. | buurman (1) | neighbour | buurman |  |  |
| 356 | Een buurman die zijn hond uitlaat, blijft staan. | die (5) | who | die |  |  |
| 357 | Een buurman die zijn hond uitlaat, blijft staan. | zijn (4) | his | zijn |  |  |
| 358 | Een buurman die zijn hond uitlaat, blijft staan. | hond (1) | dog | hond |  |  |
| 359 | Een buurman die zijn hond uitlaat, blijft staan. | uitlaat (1) | walks (a dog) | uitlaten | form of: uitlaten, to walk (a dog) |  |
| 360 | Een buurman die zijn hond uitlaat, blijft staan. | blijft (1) | stays | blijven | form of: blijven, to stay |  |
| 361 | Een buurman die zijn hond uitlaat, blijft staan. | staan (1) | to stand | staan |  |  |
| 362 | “Die meeuw steelt al jaren stroopwafels,” zegt hij. | Die (6) | that | die |  |  |
| 363 | “Die meeuw steelt al jaren stroopwafels,” zegt hij. | meeuw (11) | seagull | meeuw |  |  |
| 364 | “Die meeuw steelt al jaren stroopwafels,” zegt hij. | steelt (2) | steals | stelen | form of: stelen, to steal |  |
| 365 | “Die meeuw steelt al jaren stroopwafels,” zegt hij. | al (3) | already | al |  |  |
| 366 | “Die meeuw steelt al jaren stroopwafels,” zegt hij. | jaren (1) | years | jaar | form of: jaar, year |  |
| 367 | “Die meeuw steelt al jaren stroopwafels,” zegt hij. | stroopwafels (3) | stroopwafels | stroopwafel | form of: stroopwafel, stroopwafel |  |
| 368 | “Die meeuw steelt al jaren stroopwafels,” zegt hij. | zegt (5) | says | zeggen | form of: zeggen, to say |  |
| 369 | “Die meeuw steelt al jaren stroopwafels,” zegt hij. | hij (4) | he | hij |  |  |
| 370 | “Hij is berucht in de hele stad.” Noor moet lachen, ook al is ze nog steeds een beetje boos. | Hij (5) | he | hij |  |  |
| 371 | “Hij is berucht in de hele stad.” Noor moet lachen, ook al is ze nog steeds een beetje boos. | is (14) | is | zijn | form of: zijn, to be |  |
| 372 | “Hij is berucht in de hele stad.” Noor moet lachen, ook al is ze nog steeds een beetje boos. | berucht (1) | notorious | berucht |  |  |
| 373 | “Hij is berucht in de hele stad.” Noor moet lachen, ook al is ze nog steeds een beetje boos. | in (3) | in | in |  |  |
| 374 | “Hij is berucht in de hele stad.” Noor moet lachen, ook al is ze nog steeds een beetje boos. | de (35) | the | de |  |  |
| 375 | “Hij is berucht in de hele stad.” Noor moet lachen, ook al is ze nog steeds een beetje boos. | hele (1) | whole | heel | form of: heel, whole / very |  |
| 376 | “Hij is berucht in de hele stad.” Noor moet lachen, ook al is ze nog steeds een beetje boos. | stad (2) | city | stad |  |  |
| 377 | “Hij is berucht in de hele stad.” Noor moet lachen, ook al is ze nog steeds een beetje boos. | Noor (11) | Noor (girl's name) | Noor |  |  |
| 378 | “Hij is berucht in de hele stad.” Noor moet lachen, ook al is ze nog steeds een beetje boos. | moet (1) | must / has to | moeten | form of: moeten, to have to |  |
| 379 | “Hij is berucht in de hele stad.” Noor moet lachen, ook al is ze nog steeds een beetje boos. | lachen (1) | to laugh | lachen |  |  |
| 380 | “Hij is berucht in de hele stad.” Noor moet lachen, ook al is ze nog steeds een beetje boos. | ook (1) | also | ook |  |  |
| 381 | “Hij is berucht in de hele stad.” Noor moet lachen, ook al is ze nog steeds een beetje boos. | al (4) | already | al |  |  |
| 382 | “Hij is berucht in de hele stad.” Noor moet lachen, ook al is ze nog steeds een beetje boos. | is (15) | is | zijn | form of: zijn, to be |  |
| 383 | “Hij is berucht in de hele stad.” Noor moet lachen, ook al is ze nog steeds een beetje boos. | ze (18) | she | ze |  |  |
| 384 | “Hij is berucht in de hele stad.” Noor moet lachen, ook al is ze nog steeds een beetje boos. | nog (4) | still / yet | nog |  |  |
| 385 | “Hij is berucht in de hele stad.” Noor moet lachen, ook al is ze nog steeds een beetje boos. | steeds (1) | continuously | steeds |  |  |
| 386 | “Hij is berucht in de hele stad.” Noor moet lachen, ook al is ze nog steeds een beetje boos. | een (10) | a / an | een |  |  |
| 387 | “Hij is berucht in de hele stad.” Noor moet lachen, ook al is ze nog steeds een beetje boos. | beetje (1) | a little | beetje |  |  |
| 388 | “Hij is berucht in de hele stad.” Noor moet lachen, ook al is ze nog steeds een beetje boos. | boos (1) | angry | boos |  |  |
| 389 | “Hij heeft mijn ontbijt gestolen,” zegt ze. | Hij (6) | he | hij |  |  |
| 390 | “Hij heeft mijn ontbijt gestolen,” zegt ze. | heeft (4) | has | hebben | form of: hebben, to have |  |
| 391 | “Hij heeft mijn ontbijt gestolen,” zegt ze. | mijn (3) | my | mijn |  |  |
| 392 | “Hij heeft mijn ontbijt gestolen,” zegt ze. | ontbijt (3) | breakfast | ontbijt |  |  |
| 393 | “Hij heeft mijn ontbijt gestolen,” zegt ze. | gestolen (1) | stolen | stelen | form of: stelen, to steal |  |
| 394 | “Hij heeft mijn ontbijt gestolen,” zegt ze. | zegt (6) | says | zeggen | form of: zeggen, to say |  |
| 395 | “Hij heeft mijn ontbijt gestolen,” zegt ze. | ze (19) | she | ze |  |  |
| 396 | “Dat is niet eerlijk.” De buurman knikt. | Dat (3) | that | dat |  |  |
| 397 | “Dat is niet eerlijk.” De buurman knikt. | is (16) | is | zijn | form of: zijn, to be |  |
| 398 | “Dat is niet eerlijk.” De buurman knikt. | niet (3) | not | niet |  |  |
| 399 | “Dat is niet eerlijk.” De buurman knikt. | eerlijk (1) | honest / fair | eerlijk |  |  |
| 400 | “Dat is niet eerlijk.” De buurman knikt. | De (36) | the | de |  |  |
| 401 | “Dat is niet eerlijk.” De buurman knikt. | buurman (2) | neighbour | buurman |  |  |
| 402 | “Dat is niet eerlijk.” De buurman knikt. | knikt (2) | nods | knikken | form of: knikken, to nod |  |
| 403 | “Nee, dat is het niet. | Nee (2) | no | nee |  |  |
| 404 | “Nee, dat is het niet. | dat (4) | that | dat |  |  |
| 405 | “Nee, dat is het niet. | is (17) | is | zijn | form of: zijn, to be |  |
| 406 | “Nee, dat is het niet. | het (7) | the / it | het |  |  |
| 407 | “Nee, dat is het niet. | niet (4) | not | niet |  |  |
| 408 | Maar misschien heeft de bakker nog iets voor je.” Noor kijkt op. | Maar (5) | but | maar |  |  |
| 409 | Maar misschien heeft de bakker nog iets voor je.” Noor kijkt op. | misschien (1) | maybe | misschien |  |  |
| 410 | Maar misschien heeft de bakker nog iets voor je.” Noor kijkt op. | heeft (5) | has | hebben | form of: hebben, to have |  |
| 411 | Maar misschien heeft de bakker nog iets voor je.” Noor kijkt op. | de (37) | the | de |  |  |
| 412 | Maar misschien heeft de bakker nog iets voor je.” Noor kijkt op. | bakker (4) | baker | bakker |  |  |
| 413 | Maar misschien heeft de bakker nog iets voor je.” Noor kijkt op. | nog (5) | still / yet | nog |  |  |
| 414 | Maar misschien heeft de bakker nog iets voor je.” Noor kijkt op. | iets (3) | something | iets |  |  |
| 415 | Maar misschien heeft de bakker nog iets voor je.” Noor kijkt op. | voor (2) | for / before | voor |  |  |
| 416 | Maar misschien heeft de bakker nog iets voor je.” Noor kijkt op. | je (2) | you / your | je |  |  |
| 417 | Maar misschien heeft de bakker nog iets voor je.” Noor kijkt op. | Noor (12) | Noor (girl's name) | Noor |  |  |
| 418 | Maar misschien heeft de bakker nog iets voor je.” Noor kijkt op. | kijkt (5) | looks | kijken | form of: kijken, to look |  |
| 419 | Maar misschien heeft de bakker nog iets voor je.” Noor kijkt op. | op (4) | on | op |  |  |
| 420 | Ze is weer nieuwsgierig. | Ze (20) | she | ze |  |  |
| 421 | Ze is weer nieuwsgierig. | is (18) | is | zijn | form of: zijn, to be |  |
| 422 | Ze is weer nieuwsgierig. | weer (1) | again | weer |  |  |
| 423 | Ze is weer nieuwsgierig. | nieuwsgierig (1) | curious | nieuwsgierig |  |  |
| 424 | Noor loopt terug naar de kraam van de bakker. | Noor (13) | Noor (girl's name) | Noor |  |  |
| 425 | Noor loopt terug naar de kraam van de bakker. | loopt (3) | walks | lopen | form of: lopen, to walk |  |
| 426 | Noor loopt terug naar de kraam van de bakker. | terug (2) | back | terug |  |  |
| 427 | Noor loopt terug naar de kraam van de bakker. | naar (5) | to | naar |  |  |
| 428 | Noor loopt terug naar de kraam van de bakker. | de (38) | the | de |  |  |
| 429 | Noor loopt terug naar de kraam van de bakker. | kraam (4) | stall | kraam |  |  |
| 430 | Noor loopt terug naar de kraam van de bakker. | van (5) | of / from | van |  |  |
| 431 | Noor loopt terug naar de kraam van de bakker. | de (39) | the | de |  |  |
| 432 | Noor loopt terug naar de kraam van de bakker. | bakker (5) | baker | bakker |  |  |
| 433 | De bakker staat al te wachten met een warme stroopwafel in zijn hand. | De (40) | the | de |  |  |
| 434 | De bakker staat al te wachten met een warme stroopwafel in zijn hand. | bakker (6) | baker | bakker |  |  |
| 435 | De bakker staat al te wachten met een warme stroopwafel in zijn hand. | staat (3) | stands | staan | form of: staan, to stand |  |
| 436 | De bakker staat al te wachten met een warme stroopwafel in zijn hand. | al (5) | already | al |  |  |
| 437 | De bakker staat al te wachten met een warme stroopwafel in zijn hand. | te (2) | to | te |  |  |
| 438 | De bakker staat al te wachten met een warme stroopwafel in zijn hand. | wachten (1) | to wait | wachten |  |  |
| 439 | De bakker staat al te wachten met een warme stroopwafel in zijn hand. | met (5) | with | met |  |  |
| 440 | De bakker staat al te wachten met een warme stroopwafel in zijn hand. | een (11) | a / an | een |  |  |
| 441 | De bakker staat al te wachten met een warme stroopwafel in zijn hand. | warme (2) | warm (inflected) | warm | form of: warm, warm |  |
| 442 | De bakker staat al te wachten met een warme stroopwafel in zijn hand. | stroopwafel (11) | stroopwafel | stroopwafel |  |  |
| 443 | De bakker staat al te wachten met een warme stroopwafel in zijn hand. | in (4) | in | in |  |  |
| 444 | De bakker staat al te wachten met een warme stroopwafel in zijn hand. | zijn (5) | his | zijn |  |  |
| 445 | De bakker staat al te wachten met een warme stroopwafel in zijn hand. | hand (1) | hand | hand |  |  |
| 446 | “Deze is voor jou,” zegt hij. | Deze (1) | this | deze |  |  |
| 447 | “Deze is voor jou,” zegt hij. | is (19) | is | zijn | form of: zijn, to be |  |
| 448 | “Deze is voor jou,” zegt hij. | voor (3) | for / before | voor |  |  |
| 449 | “Deze is voor jou,” zegt hij. | jou (1) | you | jou |  |  |
| 450 | “Deze is voor jou,” zegt hij. | zegt (7) | says | zeggen | form of: zeggen, to say |  |
| 451 | “Deze is voor jou,” zegt hij. | hij (7) | he | hij |  |  |
| 452 | “En dit is mijn geheim: ik maak ze met extra kaneel.” | En (10) | and | en |  |  |
| 453 | “En dit is mijn geheim: ik maak ze met extra kaneel.” | dit (2) | this | dit |  |  |
| 454 | “En dit is mijn geheim: ik maak ze met extra kaneel.” | is (20) | is | zijn | form of: zijn, to be |  |
| 455 | “En dit is mijn geheim: ik maak ze met extra kaneel.” | mijn (4) | my | mijn |  |  |
| 456 | “En dit is mijn geheim: ik maak ze met extra kaneel.” | geheim (1) | secret | geheim |  |  |
| 457 | “En dit is mijn geheim: ik maak ze met extra kaneel.” | ik (2) | I | ik |  |  |
| 458 | “En dit is mijn geheim: ik maak ze met extra kaneel.” | maak (1) | make | maken | form of: maken, to make |  |
| 459 | “En dit is mijn geheim: ik maak ze met extra kaneel.” | ze (21) | she | ze |  |  |
| 460 | “En dit is mijn geheim: ik maak ze met extra kaneel.” | met (6) | with | met |  |  |
| 461 | “En dit is mijn geheim: ik maak ze met extra kaneel.” | extra (1) | extra | extra |  |  |
| 462 | “En dit is mijn geheim: ik maak ze met extra kaneel.” | kaneel (1) | cinnamon | kaneel |  |  |
| 463 | Noor proeft de stroopwafel. | Noor (14) | Noor (girl's name) | Noor |  |  |
| 464 | Noor proeft de stroopwafel. | proeft (1) | tastes | proeven | form of: proeven, to taste |  |
| 465 | Noor proeft de stroopwafel. | de (41) | the | de |  |  |
| 466 | Noor proeft de stroopwafel. | stroopwafel (12) | stroopwafel | stroopwafel |  |  |
| 467 | Hij is heerlijk. | Hij (8) | he | hij |  |  |
| 468 | Hij is heerlijk. | is (21) | is | zijn | form of: zijn, to be |  |
| 469 | Hij is heerlijk. | heerlijk (2) | delicious | heerlijk |  |  |
| 470 | “Waarom vertelt u mij dat geheim?” vraagt ze. | Waarom (1) | why | waarom |  |  |
| 471 | “Waarom vertelt u mij dat geheim?” vraagt ze. | vertelt (1) | tells | vertellen | form of: vertellen, to tell |  |
| 472 | “Waarom vertelt u mij dat geheim?” vraagt ze. | u (1) | you (formal) | u |  |  |
| 473 | “Waarom vertelt u mij dat geheim?” vraagt ze. | mij (2) | me | mij |  |  |
| 474 | “Waarom vertelt u mij dat geheim?” vraagt ze. | dat (5) | that | dat |  |  |
| 475 | “Waarom vertelt u mij dat geheim?” vraagt ze. | geheim (2) | secret | geheim |  |  |
| 476 | “Waarom vertelt u mij dat geheim?” vraagt ze. | vraagt (2) | asks | vragen | form of: vragen, to ask |  |
| 477 | “Waarom vertelt u mij dat geheim?” vraagt ze. | ze (22) | she | ze |  |  |
| 478 | De bakker lacht. | De (42) | the | de |  |  |
| 479 | De bakker lacht. | bakker (7) | baker | bakker |  |  |
| 480 | De bakker lacht. | lacht (2) | laughs | lachen | form of: lachen, to laugh |  |
| 481 | “Omdat jij eerlijk bent. | Omdat (1) | because | omdat |  |  |
| 482 | “Omdat jij eerlijk bent. | jij (3) | you | jij |  |  |
| 483 | “Omdat jij eerlijk bent. | eerlijk (2) | honest / fair | eerlijk |  |  |
| 484 | “Omdat jij eerlijk bent. | bent (3) | are | zijn | form of: zijn, to be |  |
| 485 | Je was boos, maar je gaf de meeuw niet de schuld. | Je (3) | you / your | je |  |  |
| 486 | Je was boos, maar je gaf de meeuw niet de schuld. | was (1) | was | zijn | form of: zijn, to be |  |
| 487 | Je was boos, maar je gaf de meeuw niet de schuld. | boos (2) | angry | boos |  |  |
| 488 | Je was boos, maar je gaf de meeuw niet de schuld. | maar (6) | but | maar |  |  |
| 489 | Je was boos, maar je gaf de meeuw niet de schuld. | je (4) | you / your | je |  |  |
| 490 | Je was boos, maar je gaf de meeuw niet de schuld. | gaf (1) | gave | geven | form of: geven, to give |  |
| 491 | Je was boos, maar je gaf de meeuw niet de schuld. | de (43) | the | de |  |  |
| 492 | Je was boos, maar je gaf de meeuw niet de schuld. | meeuw (12) | seagull | meeuw |  |  |
| 493 | Je was boos, maar je gaf de meeuw niet de schuld. | niet (5) | not | niet |  |  |
| 494 | Je was boos, maar je gaf de meeuw niet de schuld. | de (44) | the | de |  |  |
| 495 | Je was boos, maar je gaf de meeuw niet de schuld. | schuld (2) | blame / debt | schuld |  |  |
| 496 | Dat vind ik knap.” Noor bloost. | Dat (6) | that | dat |  |  |
| 497 | Dat vind ik knap.” Noor bloost. | vind (1) | find | vinden | form of: vinden, to find |  |
| 498 | Dat vind ik knap.” Noor bloost. | ik (3) | I | ik |  |  |
| 499 | Dat vind ik knap.” Noor bloost. | knap (1) | clever / good-looking | knap |  |  |
| 500 | Dat vind ik knap.” Noor bloost. | Noor (15) | Noor (girl's name) | Noor |  |  |
| 501 | Dat vind ik knap.” Noor bloost. | bloost (1) | blushes | blozen | form of: blozen, to blush |  |
| 502 | “Maar ik heb niets gedaan,” zegt ze. | Maar (7) | but | maar |  |  |
| 503 | “Maar ik heb niets gedaan,” zegt ze. | ik (4) | I | ik |  |  |
| 504 | “Maar ik heb niets gedaan,” zegt ze. | heb (1) | have | hebben | form of: hebben, to have |  |
| 505 | “Maar ik heb niets gedaan,” zegt ze. | niets (2) | nothing | niets |  |  |
| 506 | “Maar ik heb niets gedaan,” zegt ze. | gedaan (1) | done | doen | form of: doen, to do |  |
| 507 | “Maar ik heb niets gedaan,” zegt ze. | zegt (8) | says | zeggen | form of: zeggen, to say |  |
| 508 | “Maar ik heb niets gedaan,” zegt ze. | ze (23) | she | ze |  |  |
| 509 | “Toch wel,” zegt de bakker. | Toch (1) | yet / still / though | toch |  |  |
| 510 | “Toch wel,” zegt de bakker. | wel (1) | indeed / well | wel |  |  |
| 511 | “Toch wel,” zegt de bakker. | zegt (9) | says | zeggen | form of: zeggen, to say |  |
| 512 | “Toch wel,” zegt de bakker. | de (45) | the | de |  |  |
| 513 | “Toch wel,” zegt de bakker. | bakker (8) | baker | bakker |  |  |
| 514 | “Je bent teruggekomen. | Je (5) | you / your | je |  |  |
| 515 | “Je bent teruggekomen. | bent (4) | are | zijn | form of: zijn, to be |  |
| 516 | “Je bent teruggekomen. | teruggekomen (1) | come back | terugkomen | form of: terugkomen, to come back |  |
| 517 | Dat is ook een keuze.” Hij geeft haar nog een stroopwafel. | Dat (7) | that | dat |  |  |
| 518 | Dat is ook een keuze.” Hij geeft haar nog een stroopwafel. | is (22) | is | zijn | form of: zijn, to be |  |
| 519 | Dat is ook een keuze.” Hij geeft haar nog een stroopwafel. | ook (2) | also | ook |  |  |
| 520 | Dat is ook een keuze.” Hij geeft haar nog een stroopwafel. | een (12) | a / an | een |  |  |
| 521 | Dat is ook een keuze.” Hij geeft haar nog een stroopwafel. | keuze (1) | choice | keuze |  |  |
| 522 | Dat is ook een keuze.” Hij geeft haar nog een stroopwafel. | Hij (9) | he | hij |  |  |
| 523 | Dat is ook een keuze.” Hij geeft haar nog een stroopwafel. | geeft (2) | gives | geven | form of: geven, to give |  |
| 524 | Dat is ook een keuze.” Hij geeft haar nog een stroopwafel. | haar (6) | her | haar |  |  |
| 525 | Dat is ook een keuze.” Hij geeft haar nog een stroopwafel. | nog (6) | still / yet | nog |  |  |
| 526 | Dat is ook een keuze.” Hij geeft haar nog een stroopwafel. | een (13) | a / an | een |  |  |
| 527 | Dat is ook een keuze.” Hij geeft haar nog een stroopwafel. | stroopwafel (13) | stroopwafel | stroopwafel |  |  |
| 528 | “Deze is voor de meeuw. | Deze (2) | this | deze |  |  |
| 529 | “Deze is voor de meeuw. | is (23) | is | zijn | form of: zijn, to be |  |
| 530 | “Deze is voor de meeuw. | voor (4) | for / before | voor |  |  |
| 531 | “Deze is voor de meeuw. | de (46) | the | de |  |  |
| 532 | “Deze is voor de meeuw. | meeuw (13) | seagull | meeuw |  |  |
| 533 | Deel hem maar met je nieuwe vriend.” Noor is niet meer teleurgesteld. | Deel (1) | share | delen | form of: delen, to share |  |
| 534 | Deel hem maar met je nieuwe vriend.” Noor is niet meer teleurgesteld. | hem (2) | him | hem |  |  |
| 535 | Deel hem maar met je nieuwe vriend.” Noor is niet meer teleurgesteld. | maar (8) | but | maar |  |  |
| 536 | Deel hem maar met je nieuwe vriend.” Noor is niet meer teleurgesteld. | met (7) | with | met |  |  |
| 537 | Deel hem maar met je nieuwe vriend.” Noor is niet meer teleurgesteld. | je (6) | you / your | je |  |  |
| 538 | Deel hem maar met je nieuwe vriend.” Noor is niet meer teleurgesteld. | nieuwe (1) | new (inflected) | nieuw | form of: nieuw, new |  |
| 539 | Deel hem maar met je nieuwe vriend.” Noor is niet meer teleurgesteld. | vriend (1) | friend | vriend |  |  |
| 540 | Deel hem maar met je nieuwe vriend.” Noor is niet meer teleurgesteld. | Noor (16) | Noor (girl's name) | Noor |  |  |
| 541 | Deel hem maar met je nieuwe vriend.” Noor is niet meer teleurgesteld. | is (24) | is | zijn | form of: zijn, to be |  |
| 542 | Deel hem maar met je nieuwe vriend.” Noor is niet meer teleurgesteld. | niet (6) | not | niet |  |  |
| 543 | Deel hem maar met je nieuwe vriend.” Noor is niet meer teleurgesteld. | meer (1) | more | meer |  |  |
| 544 | Deel hem maar met je nieuwe vriend.” Noor is niet meer teleurgesteld. | teleurgesteld (2) | disappointed | teleurgesteld |  |  |
| 545 | Noor gaat terug naar de gracht. | Noor (17) | Noor (girl's name) | Noor |  |  |
| 546 | Noor gaat terug naar de gracht. | gaat (2) | goes | gaan | form of: gaan, to go |  |
| 547 | Noor gaat terug naar de gracht. | terug (3) | back | terug |  |  |
| 548 | Noor gaat terug naar de gracht. | naar (6) | to | naar |  |  |
| 549 | Noor gaat terug naar de gracht. | de (47) | the | de |  |  |
| 550 | Noor gaat terug naar de gracht. | gracht (2) | canal | gracht |  |  |
| 551 | De meeuw zit nog steeds op het bankje. | De (48) | the | de |  |  |
| 552 | De meeuw zit nog steeds op het bankje. | meeuw (14) | seagull | meeuw |  |  |
| 553 | De meeuw zit nog steeds op het bankje. | zit (1) | sits | zitten | form of: zitten, to sit |  |
| 554 | De meeuw zit nog steeds op het bankje. | nog (7) | still / yet | nog |  |  |
| 555 | De meeuw zit nog steeds op het bankje. | steeds (2) | continuously | steeds |  |  |
| 556 | De meeuw zit nog steeds op het bankje. | op (5) | on | op |  |  |
| 557 | De meeuw zit nog steeds op het bankje. | het (8) | the / it | het |  |  |
| 558 | De meeuw zit nog steeds op het bankje. | bankje (4) | bench | bankje |  |  |
| 559 | Ze deelt de stroopwafel met de meeuw. | Ze (24) | she | ze |  |  |
| 560 | Ze deelt de stroopwafel met de meeuw. | deelt (1) | shares | delen | form of: delen, to share |  |
| 561 | Ze deelt de stroopwafel met de meeuw. | de (49) | the | de |  |  |
| 562 | Ze deelt de stroopwafel met de meeuw. | stroopwafel (14) | stroopwafel | stroopwafel |  |  |
| 563 | Ze deelt de stroopwafel met de meeuw. | met (8) | with | met |  |  |
| 564 | Ze deelt de stroopwafel met de meeuw. | de (50) | the | de |  |  |
| 565 | Ze deelt de stroopwafel met de meeuw. | meeuw (15) | seagull | meeuw |  |  |
| 566 | Ze legt één stuk naast zich. | Ze (25) | she | ze |  |  |
| 567 | Ze legt één stuk naast zich. | legt (1) | puts / lays | leggen | form of: leggen, to put / lay |  |
| 568 | Ze legt één stuk naast zich. | één (3) | one | één |  |  |
| 569 | Ze legt één stuk naast zich. | stuk (1) | piece | stuk |  |  |
| 570 | Ze legt één stuk naast zich. | naast (1) | next to | naast |  |  |
| 571 | Ze legt één stuk naast zich. | zich (1) | himself / herself | zich |  |  |
| 572 | De meeuw kijkt verbaasd. | De (51) | the | de |  |  |
| 573 | De meeuw kijkt verbaasd. | meeuw (16) | seagull | meeuw |  |  |
| 574 | De meeuw kijkt verbaasd. | kijkt (6) | looks | kijken | form of: kijken, to look |  |
| 575 | De meeuw kijkt verbaasd. | verbaasd (1) | surprised | verbaasd |  |  |
| 576 | “Voor jou,” zegt Noor. | Voor (5) | for / before | voor |  |  |
| 577 | “Voor jou,” zegt Noor. | jou (2) | you | jou |  |  |
| 578 | “Voor jou,” zegt Noor. | zegt (10) | says | zeggen | form of: zeggen, to say |  |
| 579 | “Voor jou,” zegt Noor. | Noor (18) | Noor (girl's name) | Noor |  |  |
| 580 | “Maar dan moet je beloven dat je niet meer steelt.” De meeuw grijnst weer, alsof hij het begrijpt. | Maar (9) | but | maar |  |  |
| 581 | “Maar dan moet je beloven dat je niet meer steelt.” De meeuw grijnst weer, alsof hij het begrijpt. | dan (1) | then / than | dan |  |  |
| 582 | “Maar dan moet je beloven dat je niet meer steelt.” De meeuw grijnst weer, alsof hij het begrijpt. | moet (2) | must / has to | moeten | form of: moeten, to have to |  |
| 583 | “Maar dan moet je beloven dat je niet meer steelt.” De meeuw grijnst weer, alsof hij het begrijpt. | je (7) | you / your | je |  |  |
| 584 | “Maar dan moet je beloven dat je niet meer steelt.” De meeuw grijnst weer, alsof hij het begrijpt. | beloven (1) | to promise | beloven |  |  |
| 585 | “Maar dan moet je beloven dat je niet meer steelt.” De meeuw grijnst weer, alsof hij het begrijpt. | dat (8) | that | dat |  |  |
| 586 | “Maar dan moet je beloven dat je niet meer steelt.” De meeuw grijnst weer, alsof hij het begrijpt. | je (8) | you / your | je |  |  |
| 587 | “Maar dan moet je beloven dat je niet meer steelt.” De meeuw grijnst weer, alsof hij het begrijpt. | niet (7) | not | niet |  |  |
| 588 | “Maar dan moet je beloven dat je niet meer steelt.” De meeuw grijnst weer, alsof hij het begrijpt. | meer (2) | more | meer |  |  |
| 589 | “Maar dan moet je beloven dat je niet meer steelt.” De meeuw grijnst weer, alsof hij het begrijpt. | steelt (3) | steals | stelen | form of: stelen, to steal |  |
| 590 | “Maar dan moet je beloven dat je niet meer steelt.” De meeuw grijnst weer, alsof hij het begrijpt. | De (52) | the | de |  |  |
| 591 | “Maar dan moet je beloven dat je niet meer steelt.” De meeuw grijnst weer, alsof hij het begrijpt. | meeuw (17) | seagull | meeuw |  |  |
| 592 | “Maar dan moet je beloven dat je niet meer steelt.” De meeuw grijnst weer, alsof hij het begrijpt. | grijnst (2) | grins | grijnzen | form of: grijnzen, to grin |  |
| 593 | “Maar dan moet je beloven dat je niet meer steelt.” De meeuw grijnst weer, alsof hij het begrijpt. | weer (2) | again | weer |  |  |
| 594 | “Maar dan moet je beloven dat je niet meer steelt.” De meeuw grijnst weer, alsof hij het begrijpt. | alsof (1) | as if | alsof |  |  |
| 595 | “Maar dan moet je beloven dat je niet meer steelt.” De meeuw grijnst weer, alsof hij het begrijpt. | hij (10) | he | hij |  |  |
| 596 | “Maar dan moet je beloven dat je niet meer steelt.” De meeuw grijnst weer, alsof hij het begrijpt. | het (9) | the / it | het |  |  |
| 597 | “Maar dan moet je beloven dat je niet meer steelt.” De meeuw grijnst weer, alsof hij het begrijpt. | begrijpt (1) | understands | begrijpen | form of: begrijpen, to understand |  |
| 598 | Hij pakt het stuk en eet het snel op. | Hij (11) | he | hij |  |  |
| 599 | Hij pakt het stuk en eet het snel op. | pakt (2) | takes | pakken | form of: pakken, to take / grab |  |
| 600 | Hij pakt het stuk en eet het snel op. | het (10) | the / it | het |  |  |
| 601 | Hij pakt het stuk en eet het snel op. | stuk (2) | piece | stuk |  |  |
| 602 | Hij pakt het stuk en eet het snel op. | en (11) | and | en |  |  |
| 603 | Hij pakt het stuk en eet het snel op. | eet (1) | eats | eten | form of: eten, to eat |  |
| 604 | Hij pakt het stuk en eet het snel op. | het (11) | the / it | het |  |  |
| 605 | Hij pakt het stuk en eet het snel op. | snel (4) | fast | snel |  |  |
| 606 | Hij pakt het stuk en eet het snel op. | op (6) | on | op |  |  |
| 607 | Noor lacht. | Noor (19) | Noor (girl's name) | Noor |  |  |
| 608 | Noor lacht. | lacht (3) | laughs | lachen | form of: lachen, to laugh |  |
| 609 | “Je bent niet eerlijk,” zegt ze. | Je (9) | you / your | je |  |  |
| 610 | “Je bent niet eerlijk,” zegt ze. | bent (5) | are | zijn | form of: zijn, to be |  |
| 611 | “Je bent niet eerlijk,” zegt ze. | niet (8) | not | niet |  |  |
| 612 | “Je bent niet eerlijk,” zegt ze. | eerlijk (3) | honest / fair | eerlijk |  |  |
| 613 | “Je bent niet eerlijk,” zegt ze. | zegt (11) | says | zeggen | form of: zeggen, to say |  |
| 614 | “Je bent niet eerlijk,” zegt ze. | ze (26) | she | ze |  |  |
| 615 | “Maar je bent wel grappig.” De buurman komt weer langs met zijn hond. | Maar (10) | but | maar |  |  |
| 616 | “Maar je bent wel grappig.” De buurman komt weer langs met zijn hond. | je (10) | you / your | je |  |  |
| 617 | “Maar je bent wel grappig.” De buurman komt weer langs met zijn hond. | bent (6) | are | zijn | form of: zijn, to be |  |
| 618 | “Maar je bent wel grappig.” De buurman komt weer langs met zijn hond. | wel (2) | indeed / well | wel |  |  |
| 619 | “Maar je bent wel grappig.” De buurman komt weer langs met zijn hond. | grappig (1) | funny | grappig |  |  |
| 620 | “Maar je bent wel grappig.” De buurman komt weer langs met zijn hond. | De (53) | the | de |  |  |
| 621 | “Maar je bent wel grappig.” De buurman komt weer langs met zijn hond. | buurman (3) | neighbour | buurman |  |  |
| 622 | “Maar je bent wel grappig.” De buurman komt weer langs met zijn hond. | komt (2) | comes | komen | form of: komen, to come |  |
| 623 | “Maar je bent wel grappig.” De buurman komt weer langs met zijn hond. | weer (3) | again | weer |  |  |
| 624 | “Maar je bent wel grappig.” De buurman komt weer langs met zijn hond. | langs (3) | along / past | langs |  |  |
| 625 | “Maar je bent wel grappig.” De buurman komt weer langs met zijn hond. | met (9) | with | met |  |  |
| 626 | “Maar je bent wel grappig.” De buurman komt weer langs met zijn hond. | zijn (6) | his | zijn |  |  |
| 627 | “Maar je bent wel grappig.” De buurman komt weer langs met zijn hond. | hond (2) | dog | hond |  |  |
| 628 | De hond rent de meeuw achterna, maar de meeuw blijft rustig zitten. | De (54) | the | de |  |  |
| 629 | De hond rent de meeuw achterna, maar de meeuw blijft rustig zitten. | hond (3) | dog | hond |  |  |
| 630 | De hond rent de meeuw achterna, maar de meeuw blijft rustig zitten. | rent (3) | runs | rennen | form of: rennen, to run |  |
| 631 | De hond rent de meeuw achterna, maar de meeuw blijft rustig zitten. | de (55) | the | de |  |  |
| 632 | De hond rent de meeuw achterna, maar de meeuw blijft rustig zitten. | meeuw (18) | seagull | meeuw |  |  |
| 633 | De hond rent de meeuw achterna, maar de meeuw blijft rustig zitten. | achterna (2) | after (in 'achterna rennen') | achterna |  |  |
| 634 | De hond rent de meeuw achterna, maar de meeuw blijft rustig zitten. | maar (11) | but | maar |  |  |
| 635 | De hond rent de meeuw achterna, maar de meeuw blijft rustig zitten. | de (56) | the | de |  |  |
| 636 | De hond rent de meeuw achterna, maar de meeuw blijft rustig zitten. | meeuw (19) | seagull | meeuw |  |  |
| 637 | De hond rent de meeuw achterna, maar de meeuw blijft rustig zitten. | blijft (2) | stays | blijven | form of: blijven, to stay |  |
| 638 | De hond rent de meeuw achterna, maar de meeuw blijft rustig zitten. | rustig (2) | calm / quiet | rustig |  |  |
| 639 | De hond rent de meeuw achterna, maar de meeuw blijft rustig zitten. | zitten (2) | to sit | zitten |  |  |
| 640 | “Kijk eens aan,” zegt de buurman. | Kijk (1) | look | kijk |  |  |
| 641 | “Kijk eens aan,” zegt de buurman. | eens (1) | once / agree | eens |  |  |
| 642 | “Kijk eens aan,” zegt de buurman. | aan (3) | there you go | aan |  |  |
| 643 | “Kijk eens aan,” zegt de buurman. | zegt (12) | says | zeggen | form of: zeggen, to say |  |
| 644 | “Kijk eens aan,” zegt de buurman. | de (57) | the | de |  |  |
| 645 | “Kijk eens aan,” zegt de buurman. | buurman (4) | neighbour | buurman |  |  |
| 646 | “Jij hebt een meeuw als vriend.” Noor haalt haar schouders op. | Jij (4) | you | jij |  |  |
| 647 | “Jij hebt een meeuw als vriend.” Noor haalt haar schouders op. | hebt (2) | have | hebben | form of: hebben, to have |  |
| 648 | “Jij hebt een meeuw als vriend.” Noor haalt haar schouders op. | een (14) | a / an | een |  |  |
| 649 | “Jij hebt een meeuw als vriend.” Noor haalt haar schouders op. | meeuw (20) | seagull | meeuw |  |  |
| 650 | “Jij hebt een meeuw als vriend.” Noor haalt haar schouders op. | als (3) | as | als |  |  |
| 651 | “Jij hebt een meeuw als vriend.” Noor haalt haar schouders op. | vriend (2) | friend | vriend |  |  |
| 652 | “Jij hebt een meeuw als vriend.” Noor haalt haar schouders op. | Noor (20) | Noor (girl's name) | Noor |  |  |
| 653 | “Jij hebt een meeuw als vriend.” Noor haalt haar schouders op. | haalt (1) | shrugs | halen | form of: halen, to fetch / shrug |  |
| 654 | “Jij hebt een meeuw als vriend.” Noor haalt haar schouders op. | haar (7) | her | haar |  |  |
| 655 | “Jij hebt een meeuw als vriend.” Noor haalt haar schouders op. | schouders (1) | shoulders | schouder | form of: schouder, shoulder |  |
| 656 | “Jij hebt een meeuw als vriend.” Noor haalt haar schouders op. | op (7) | on | op |  |  |
| 657 | “Hij is geen vriend. | Hij (12) | he | hij |  |  |
| 658 | “Hij is geen vriend. | is (25) | is | zijn | form of: zijn, to be |  |
| 659 | “Hij is geen vriend. | geen (2) | no / not a | geen |  |  |
| 660 | “Hij is geen vriend. | vriend (3) | friend | vriend |  |  |
| 661 | Hij is een dief met geluk.” | Hij (13) | he | hij |  |  |
| 662 | Hij is een dief met geluk.” | is (26) | is | zijn | form of: zijn, to be |  |
| 663 | Hij is een dief met geluk.” | een (15) | a / an | een |  |  |
| 664 | Hij is een dief met geluk.” | dief (1) | thief | dief |  |  |
| 665 | Hij is een dief met geluk.” | met (10) | with | met |  |  |
| 666 | Hij is een dief met geluk.” | geluk (1) | happiness / luck | geluk |  |  |
| 667 | Die avond zit Noor thuis op de bank. | Die (7) | that | die |  |  |
| 668 | Die avond zit Noor thuis op de bank. | avond (1) | evening | avond |  |  |
| 669 | Die avond zit Noor thuis op de bank. | zit (2) | sits | zitten | form of: zitten, to sit |  |
| 670 | Die avond zit Noor thuis op de bank. | Noor (21) | Noor (girl's name) | Noor |  |  |
| 671 | Die avond zit Noor thuis op de bank. | thuis (1) | at home | thuis |  |  |
| 672 | Die avond zit Noor thuis op de bank. | op (8) | on | op |  |  |
| 673 | Die avond zit Noor thuis op de bank. | de (58) | the | de |  |  |
| 674 | Die avond zit Noor thuis op de bank. | bank (1) | sofa / bench | bank |  |  |
| 675 | Ze denkt aan de meeuw, de bakker en de laatste stroopwafel. | Ze (27) | she | ze |  |  |
| 676 | Ze denkt aan de meeuw, de bakker en de laatste stroopwafel. | denkt (2) | thinks | denken | form of: denken, to think |  |
| 677 | Ze denkt aan de meeuw, de bakker en de laatste stroopwafel. | aan (4) | of | aan |  |  |
| 678 | Ze denkt aan de meeuw, de bakker en de laatste stroopwafel. | de (59) | the | de |  |  |
| 679 | Ze denkt aan de meeuw, de bakker en de laatste stroopwafel. | meeuw (21) | seagull | meeuw |  |  |
| 680 | Ze denkt aan de meeuw, de bakker en de laatste stroopwafel. | de (60) | the | de |  |  |
| 681 | Ze denkt aan de meeuw, de bakker en de laatste stroopwafel. | bakker (9) | baker | bakker |  |  |
| 682 | Ze denkt aan de meeuw, de bakker en de laatste stroopwafel. | en (12) | and | en |  |  |
| 683 | Ze denkt aan de meeuw, de bakker en de laatste stroopwafel. | de (61) | the | de |  |  |
| 684 | Ze denkt aan de meeuw, de bakker en de laatste stroopwafel. | laatste (3) | last (inflected) | laatst | form of: laatst, last |  |
| 685 | Ze denkt aan de meeuw, de bakker en de laatste stroopwafel. | stroopwafel (15) | stroopwafel | stroopwafel |  |  |
| 686 | Ze heeft geen stroopwafel meer, maar ze voelt zich toch gelukkig. | Ze (28) | she | ze |  |  |
| 687 | Ze heeft geen stroopwafel meer, maar ze voelt zich toch gelukkig. | heeft (6) | has | hebben | form of: hebben, to have |  |
| 688 | Ze heeft geen stroopwafel meer, maar ze voelt zich toch gelukkig. | geen (3) | no / not a | geen |  |  |
| 689 | Ze heeft geen stroopwafel meer, maar ze voelt zich toch gelukkig. | stroopwafel (16) | stroopwafel | stroopwafel |  |  |
| 690 | Ze heeft geen stroopwafel meer, maar ze voelt zich toch gelukkig. | meer (3) | more | meer |  |  |
| 691 | Ze heeft geen stroopwafel meer, maar ze voelt zich toch gelukkig. | maar (12) | but | maar |  |  |
| 692 | Ze heeft geen stroopwafel meer, maar ze voelt zich toch gelukkig. | ze (29) | she | ze |  |  |
| 693 | Ze heeft geen stroopwafel meer, maar ze voelt zich toch gelukkig. | voelt (1) | feels | voelen | form of: voelen, to feel |  |
| 694 | Ze heeft geen stroopwafel meer, maar ze voelt zich toch gelukkig. | zich (2) | himself / herself | zich |  |  |
| 695 | Ze heeft geen stroopwafel meer, maar ze voelt zich toch gelukkig. | toch (2) | yet / still / though | toch |  |  |
| 696 | Ze heeft geen stroopwafel meer, maar ze voelt zich toch gelukkig. | gelukkig (1) | happy | gelukkig |  |  |
| 697 | “Soms is geluk niet wat je koopt,” zegt ze tegen haar moeder. | Soms (1) | sometimes | soms |  |  |
| 698 | “Soms is geluk niet wat je koopt,” zegt ze tegen haar moeder. | is (27) | is | zijn | form of: zijn, to be |  |
| 699 | “Soms is geluk niet wat je koopt,” zegt ze tegen haar moeder. | geluk (2) | happiness / luck | geluk |  |  |
| 700 | “Soms is geluk niet wat je koopt,” zegt ze tegen haar moeder. | niet (9) | not | niet |  |  |
| 701 | “Soms is geluk niet wat je koopt,” zegt ze tegen haar moeder. | wat (1) | what | wat |  |  |
| 702 | “Soms is geluk niet wat je koopt,” zegt ze tegen haar moeder. | je (11) | you / your | je |  |  |
| 703 | “Soms is geluk niet wat je koopt,” zegt ze tegen haar moeder. | koopt (1) | buys | kopen | form of: kopen, to buy |  |
| 704 | “Soms is geluk niet wat je koopt,” zegt ze tegen haar moeder. | zegt (13) | says | zeggen | form of: zeggen, to say |  |
| 705 | “Soms is geluk niet wat je koopt,” zegt ze tegen haar moeder. | ze (30) | she | ze |  |  |
| 706 | “Soms is geluk niet wat je koopt,” zegt ze tegen haar moeder. | tegen (2) | against / to | tegen |  |  |
| 707 | “Soms is geluk niet wat je koopt,” zegt ze tegen haar moeder. | haar (8) | her | haar |  |  |
| 708 | “Soms is geluk niet wat je koopt,” zegt ze tegen haar moeder. | moeder (1) | mother | moeder |  |  |
| 709 | Haar moeder kijkt op van haar boek. | Haar (9) | her | haar |  |  |
| 710 | Haar moeder kijkt op van haar boek. | moeder (2) | mother | moeder |  |  |
| 711 | Haar moeder kijkt op van haar boek. | kijkt (7) | looks | kijken | form of: kijken, to look |  |
| 712 | Haar moeder kijkt op van haar boek. | op (9) | on | op |  |  |
| 713 | Haar moeder kijkt op van haar boek. | van (6) | of / from | van |  |  |
| 714 | Haar moeder kijkt op van haar boek. | haar (10) | her | haar |  |  |
| 715 | Haar moeder kijkt op van haar boek. | boek (1) | book | boek |  |  |
| 716 | “Wat bedoel je?” vraagt ze. | Wat (2) | what | wat |  |  |
| 717 | “Wat bedoel je?” vraagt ze. | bedoel (1) | mean | bedoelen | form of: bedoelen, to mean |  |
| 718 | “Wat bedoel je?” vraagt ze. | je (12) | you / your | je |  |  |
| 719 | “Wat bedoel je?” vraagt ze. | vraagt (3) | asks | vragen | form of: vragen, to ask |  |
| 720 | “Wat bedoel je?” vraagt ze. | ze (31) | she | ze |  |  |
| 721 | Noor vertelt het hele verhaal. | Noor (22) | Noor (girl's name) | Noor |  |  |
| 722 | Noor vertelt het hele verhaal. | vertelt (2) | tells | vertellen | form of: vertellen, to tell |  |
| 723 | Noor vertelt het hele verhaal. | het (12) | the / it | het |  |  |
| 724 | Noor vertelt het hele verhaal. | hele (2) | whole | heel | form of: heel, whole / very |  |
| 725 | Noor vertelt het hele verhaal. | verhaal (1) | story | verhaal |  |  |
| 726 | Haar moeder lacht. | Haar (11) | her | haar |  |  |
| 727 | Haar moeder lacht. | moeder (3) | mother | moeder |  |  |
| 728 | Haar moeder lacht. | lacht (4) | laughs | lachen | form of: lachen, to laugh |  |
| 729 | “Dus je hebt je ontbijt weggegeven aan een meeuw?” “Niet weggegeven,” zegt Noor. | Dus (1) | so / therefore | dus |  |  |
| 730 | “Dus je hebt je ontbijt weggegeven aan een meeuw?” “Niet weggegeven,” zegt Noor. | je (13) | you / your | je |  |  |
| 731 | “Dus je hebt je ontbijt weggegeven aan een meeuw?” “Niet weggegeven,” zegt Noor. | hebt (3) | have | hebben | form of: hebben, to have |  |
| 732 | “Dus je hebt je ontbijt weggegeven aan een meeuw?” “Niet weggegeven,” zegt Noor. | je (14) | you / your | je |  |  |
| 733 | “Dus je hebt je ontbijt weggegeven aan een meeuw?” “Niet weggegeven,” zegt Noor. | ontbijt (4) | breakfast | ontbijt |  |  |
| 734 | “Dus je hebt je ontbijt weggegeven aan een meeuw?” “Niet weggegeven,” zegt Noor. | weggegeven (1) | given away | weggeven | form of: weggeven, to give away |  |
| 735 | “Dus je hebt je ontbijt weggegeven aan een meeuw?” “Niet weggegeven,” zegt Noor. | aan (5) | to | aan |  |  |
| 736 | “Dus je hebt je ontbijt weggegeven aan een meeuw?” “Niet weggegeven,” zegt Noor. | een (16) | a / an | een |  |  |
| 737 | “Dus je hebt je ontbijt weggegeven aan een meeuw?” “Niet weggegeven,” zegt Noor. | meeuw (22) | seagull | meeuw |  |  |
| 738 | “Dus je hebt je ontbijt weggegeven aan een meeuw?” “Niet weggegeven,” zegt Noor. | Niet (10) | not | niet |  |  |
| 739 | “Dus je hebt je ontbijt weggegeven aan een meeuw?” “Niet weggegeven,” zegt Noor. | weggegeven (2) | given away | weggeven | form of: weggeven, to give away |  |
| 740 | “Dus je hebt je ontbijt weggegeven aan een meeuw?” “Niet weggegeven,” zegt Noor. | zegt (14) | says | zeggen | form of: zeggen, to say |  |
| 741 | “Dus je hebt je ontbijt weggegeven aan een meeuw?” “Niet weggegeven,” zegt Noor. | Noor (23) | Noor (girl's name) | Noor |  |  |
| 742 | “Gedeeld.” | Gedeeld (1) | shared | delen | form of: delen, to share |  |
| 743 | De volgende zaterdag gaat ze weer naar de markt. | De (62) | the | de |  |  |
| 744 | De volgende zaterdag gaat ze weer naar de markt. | volgende (1) | next (inflected) | volgend | form of: volgend, next |  |
| 745 | De volgende zaterdag gaat ze weer naar de markt. | zaterdag (1) | Saturday | zaterdag |  |  |
| 746 | De volgende zaterdag gaat ze weer naar de markt. | gaat (3) | goes | gaan | form of: gaan, to go |  |
| 747 | De volgende zaterdag gaat ze weer naar de markt. | ze (32) | she | ze |  |  |
| 748 | De volgende zaterdag gaat ze weer naar de markt. | weer (4) | again | weer |  |  |
| 749 | De volgende zaterdag gaat ze weer naar de markt. | naar (7) | to | naar |  |  |
| 750 | De volgende zaterdag gaat ze weer naar de markt. | de (63) | the | de |  |  |
| 751 | De volgende zaterdag gaat ze weer naar de markt. | markt (3) | market | markt |  |  |
| 752 | Ze hijgt nog een beetje van het lopen. | Ze (33) | she | ze |  |  |
| 753 | Ze hijgt nog een beetje van het lopen. | hijgt (2) | pants | hijgen | form of: hijgen, to pant |  |
| 754 | Ze hijgt nog een beetje van het lopen. | nog (8) | still / yet | nog |  |  |
| 755 | Ze hijgt nog een beetje van het lopen. | een (17) | a / an | een |  |  |
| 756 | Ze hijgt nog een beetje van het lopen. | beetje (2) | a little | beetje |  |  |
| 757 | Ze hijgt nog een beetje van het lopen. | van (7) | of / from | van |  |  |
| 758 | Ze hijgt nog een beetje van het lopen. | het (13) | the / it | het |  |  |
| 759 | Ze hijgt nog een beetje van het lopen. | lopen (1) | to walk | lopen |  |  |
| 760 | De bakker zwaait al van verre. | De (64) | the | de |  |  |
| 761 | De bakker zwaait al van verre. | bakker (10) | baker | bakker |  |  |
| 762 | De bakker zwaait al van verre. | zwaait (1) | waves | zwaaien | form of: zwaaien, to wave |  |
| 763 | De bakker zwaait al van verre. | al (6) | already | al |  |  |
| 764 | De bakker zwaait al van verre. | van (8) | of / from | van |  |  |
| 765 | De bakker zwaait al van verre. | verre (1) | far (inflected) | ver | form of: ver, far |  |
| 766 | Noor lacht en proeft de stroopwafel die ze net heeft gekocht. | Noor (24) | Noor (girl's name) | Noor |  |  |
| 767 | Noor lacht en proeft de stroopwafel die ze net heeft gekocht. | lacht (5) | laughs | lachen | form of: lachen, to laugh |  |
| 768 | Noor lacht en proeft de stroopwafel die ze net heeft gekocht. | en (13) | and | en |  |  |
| 769 | Noor lacht en proeft de stroopwafel die ze net heeft gekocht. | proeft (2) | tastes | proeven | form of: proeven, to taste |  |
| 770 | Noor lacht en proeft de stroopwafel die ze net heeft gekocht. | de (65) | the | de |  |  |
| 771 | Noor lacht en proeft de stroopwafel die ze net heeft gekocht. | stroopwafel (17) | stroopwafel | stroopwafel |  |  |
| 772 | Noor lacht en proeft de stroopwafel die ze net heeft gekocht. | die (8) | which | die |  |  |
| 773 | Noor lacht en proeft de stroopwafel die ze net heeft gekocht. | ze (34) | she | ze |  |  |
| 774 | Noor lacht en proeft de stroopwafel die ze net heeft gekocht. | net (2) | just | net |  |  |
| 775 | Noor lacht en proeft de stroopwafel die ze net heeft gekocht. | heeft (7) | has | hebben | form of: hebben, to have |  |
| 776 | Noor lacht en proeft de stroopwafel die ze net heeft gekocht. | gekocht (1) | bought | kopen | form of: kopen, to buy |  |
| 777 | “Ik heb je recept geprobeerd,” zegt Noor. | Ik (5) | I | ik |  |  |
| 778 | “Ik heb je recept geprobeerd,” zegt Noor. | heb (2) | have | hebben | form of: hebben, to have |  |
| 779 | “Ik heb je recept geprobeerd,” zegt Noor. | je (15) | you / your | je |  |  |
| 780 | “Ik heb je recept geprobeerd,” zegt Noor. | recept (1) | recipe | recept |  |  |
| 781 | “Ik heb je recept geprobeerd,” zegt Noor. | geprobeerd (1) | tried | proberen | form of: proberen, to try |  |
| 782 | “Ik heb je recept geprobeerd,” zegt Noor. | zegt (15) | says | zeggen | form of: zeggen, to say |  |
| 783 | “Ik heb je recept geprobeerd,” zegt Noor. | Noor (25) | Noor (girl's name) | Noor |  |  |
| 784 | “Het is niet gelukt, maar ik heb het wel geprobeerd.” De bakker lacht. | Het (14) | the / it | het |  |  |
| 785 | “Het is niet gelukt, maar ik heb het wel geprobeerd.” De bakker lacht. | is (28) | is | zijn | form of: zijn, to be |  |
| 786 | “Het is niet gelukt, maar ik heb het wel geprobeerd.” De bakker lacht. | niet (11) | not | niet |  |  |
| 787 | “Het is niet gelukt, maar ik heb het wel geprobeerd.” De bakker lacht. | gelukt (1) | succeeded | lukken | form of: lukken, to succeed |  |
| 788 | “Het is niet gelukt, maar ik heb het wel geprobeerd.” De bakker lacht. | maar (13) | but | maar |  |  |
| 789 | “Het is niet gelukt, maar ik heb het wel geprobeerd.” De bakker lacht. | ik (6) | I | ik |  |  |
| 790 | “Het is niet gelukt, maar ik heb het wel geprobeerd.” De bakker lacht. | heb (3) | have | hebben | form of: hebben, to have |  |
| 791 | “Het is niet gelukt, maar ik heb het wel geprobeerd.” De bakker lacht. | het (15) | the / it | het |  |  |
| 792 | “Het is niet gelukt, maar ik heb het wel geprobeerd.” De bakker lacht. | wel (3) | indeed / well | wel |  |  |
| 793 | “Het is niet gelukt, maar ik heb het wel geprobeerd.” De bakker lacht. | geprobeerd (2) | tried | proberen | form of: proberen, to try |  |
| 794 | “Het is niet gelukt, maar ik heb het wel geprobeerd.” De bakker lacht. | De (66) | the | de |  |  |
| 795 | “Het is niet gelukt, maar ik heb het wel geprobeerd.” De bakker lacht. | bakker (11) | baker | bakker |  |  |
| 796 | “Het is niet gelukt, maar ik heb het wel geprobeerd.” De bakker lacht. | lacht (6) | laughs | lachen | form of: lachen, to laugh |  |
| 797 | “Het geheim is niet het recept. | Het (16) | the / it | het |  |  |
| 798 | “Het geheim is niet het recept. | geheim (3) | secret | geheim |  |  |
| 799 | “Het geheim is niet het recept. | is (29) | is | zijn | form of: zijn, to be |  |
| 800 | “Het geheim is niet het recept. | niet (12) | not | niet |  |  |
| 801 | “Het geheim is niet het recept. | het (17) | the / it | het |  |  |
| 802 | “Het geheim is niet het recept. | recept (2) | recipe | recept |  |  |
| 803 | Het geheim is delen.” En op het bankje bij de gracht zit een meeuw te wachten. | Het (18) | the / it | het |  |  |
| 804 | Het geheim is delen.” En op het bankje bij de gracht zit een meeuw te wachten. | geheim (4) | secret | geheim |  |  |
| 805 | Het geheim is delen.” En op het bankje bij de gracht zit een meeuw te wachten. | is (30) | is | zijn | form of: zijn, to be |  |
| 806 | Het geheim is delen.” En op het bankje bij de gracht zit een meeuw te wachten. | delen (1) | to share | delen |  |  |
| 807 | Het geheim is delen.” En op het bankje bij de gracht zit een meeuw te wachten. | En (14) | and | en |  |  |
| 808 | Het geheim is delen.” En op het bankje bij de gracht zit een meeuw te wachten. | op (10) | on | op |  |  |
| 809 | Het geheim is delen.” En op het bankje bij de gracht zit een meeuw te wachten. | het (19) | the / it | het |  |  |
| 810 | Het geheim is delen.” En op het bankje bij de gracht zit een meeuw te wachten. | bankje (5) | bench | bankje |  |  |
| 811 | Het geheim is delen.” En op het bankje bij de gracht zit een meeuw te wachten. | bij (4) | at / with | bij |  |  |
| 812 | Het geheim is delen.” En op het bankje bij de gracht zit een meeuw te wachten. | de (67) | the | de |  |  |
| 813 | Het geheim is delen.” En op het bankje bij de gracht zit een meeuw te wachten. | gracht (3) | canal | gracht |  |  |
| 814 | Het geheim is delen.” En op het bankje bij de gracht zit een meeuw te wachten. | zit (3) | sits | zitten | form of: zitten, to sit |  |
| 815 | Het geheim is delen.” En op het bankje bij de gracht zit een meeuw te wachten. | een (18) | a / an | een |  |  |
| 816 | Het geheim is delen.” En op het bankje bij de gracht zit een meeuw te wachten. | meeuw (23) | seagull | meeuw |  |  |
| 817 | Het geheim is delen.” En op het bankje bij de gracht zit een meeuw te wachten. | te (3) | to | te |  |  |
| 818 | Het geheim is delen.” En op het bankje bij de gracht zit een meeuw te wachten. | wachten (2) | to wait | wachten |  |  |
| 819 | De hond van de buurman rent de meeuw achterna, maar de meeuw blijft rustig zitten. | De (68) | the | de |  |  |
| 820 | De hond van de buurman rent de meeuw achterna, maar de meeuw blijft rustig zitten. | hond (4) | dog | hond |  |  |
| 821 | De hond van de buurman rent de meeuw achterna, maar de meeuw blijft rustig zitten. | van (9) | of / from | van |  |  |
| 822 | De hond van de buurman rent de meeuw achterna, maar de meeuw blijft rustig zitten. | de (69) | the | de |  |  |
| 823 | De hond van de buurman rent de meeuw achterna, maar de meeuw blijft rustig zitten. | buurman (5) | neighbour | buurman |  |  |
| 824 | De hond van de buurman rent de meeuw achterna, maar de meeuw blijft rustig zitten. | rent (4) | runs | rennen | form of: rennen, to run |  |
| 825 | De hond van de buurman rent de meeuw achterna, maar de meeuw blijft rustig zitten. | de (70) | the | de |  |  |
| 826 | De hond van de buurman rent de meeuw achterna, maar de meeuw blijft rustig zitten. | meeuw (24) | seagull | meeuw |  |  |
| 827 | De hond van de buurman rent de meeuw achterna, maar de meeuw blijft rustig zitten. | achterna (3) | after (in 'achterna rennen') | achterna |  |  |
| 828 | De hond van de buurman rent de meeuw achterna, maar de meeuw blijft rustig zitten. | maar (14) | but | maar |  |  |
| 829 | De hond van de buurman rent de meeuw achterna, maar de meeuw blijft rustig zitten. | de (71) | the | de |  |  |
| 830 | De hond van de buurman rent de meeuw achterna, maar de meeuw blijft rustig zitten. | meeuw (25) | seagull | meeuw |  |  |
| 831 | De hond van de buurman rent de meeuw achterna, maar de meeuw blijft rustig zitten. | blijft (3) | stays | blijven | form of: blijven, to stay |  |
| 832 | De hond van de buurman rent de meeuw achterna, maar de meeuw blijft rustig zitten. | rustig (3) | calm / quiet | rustig |  |  |
| 833 | De hond van de buurman rent de meeuw achterna, maar de meeuw blijft rustig zitten. | zitten (3) | to sit | zitten |  |  |
| 834 | Noor heeft twee stroopwafels bij zich. | Noor (26) | Noor (girl's name) | Noor |  |  |
| 835 | Noor heeft twee stroopwafels bij zich. | heeft (8) | has | hebben | form of: hebben, to have |  |
| 836 | Noor heeft twee stroopwafels bij zich. | twee (1) | two | twee |  |  |
| 837 | Noor heeft twee stroopwafels bij zich. | stroopwafels (4) | stroopwafels | stroopwafel | form of: stroopwafel, stroopwafel |  |
| 838 | Noor heeft twee stroopwafels bij zich. | bij (5) | at / with | bij |  |  |
| 839 | Noor heeft twee stroopwafels bij zich. | zich (3) | himself / herself | zich |  |  |
| 840 | Eén voor haar, één voor haar vriend. | Eén (1) | one (at sentence start) | één | form of: één, one |  |
| 841 | Eén voor haar, één voor haar vriend. | voor (6) | for / before | voor |  |  |
| 842 | Eén voor haar, één voor haar vriend. | haar (12) | her | haar |  |  |
| 843 | Eén voor haar, één voor haar vriend. | één (4) | one | één |  |  |
| 844 | Eén voor haar, één voor haar vriend. | voor (7) | for / before | voor |  |  |
| 845 | Eén voor haar, één voor haar vriend. | haar (13) | her | haar |  |  |
| 846 | Eén voor haar, één voor haar vriend. | vriend (4) | friend | vriend |  |  |

### B2 · Het sollicitatiegesprek (`b2-01`)

| # | Sentence | Tapped (occ) | Shown meaning | Lemma | Base line | Flag |
| ---: | --- | --- | --- | --- | --- | --- |
| 1 | Om negen uur 's ochtends stond Daan voor het kantoor van Van Dijk Advies. | Om (1) | at | om |  |  |
| 2 | Om negen uur 's ochtends stond Daan voor het kantoor van Van Dijk Advies. | negen (1) | nine | negen |  |  |
| 3 | Om negen uur 's ochtends stond Daan voor het kantoor van Van Dijk Advies. | uur (1) | hour | uur |  |  |
| 4 | Om negen uur 's ochtends stond Daan voor het kantoor van Van Dijk Advies. | 's (1) | in the morning | 's ochtends |  |  |
| 5 | Om negen uur 's ochtends stond Daan voor het kantoor van Van Dijk Advies. | ochtends (1) | in the morning | 's ochtends |  |  |
| 6 | Om negen uur 's ochtends stond Daan voor het kantoor van Van Dijk Advies. | stond (1) | stood | staan | form of: staan, to stand |  |
| 7 | Om negen uur 's ochtends stond Daan voor het kantoor van Van Dijk Advies. | Daan (1) | Daan (the applicant's name) | Daan |  |  |
| 8 | Om negen uur 's ochtends stond Daan voor het kantoor van Van Dijk Advies. | voor (1) | for / before | voor |  |  |
| 9 | Om negen uur 's ochtends stond Daan voor het kantoor van Van Dijk Advies. | het (1) | the / it | het |  |  |
| 10 | Om negen uur 's ochtends stond Daan voor het kantoor van Van Dijk Advies. | kantoor (1) | office | kantoor |  |  |
| 11 | Om negen uur 's ochtends stond Daan voor het kantoor van Van Dijk Advies. | van (1) | of / from | van |  |  |
| 12 | Om negen uur 's ochtends stond Daan voor het kantoor van Van Dijk Advies. | Van (2) | of / from | van |  |  |
| 13 | Om negen uur 's ochtends stond Daan voor het kantoor van Van Dijk Advies. | Dijk (1) | Dijk (surname) | Dijk |  |  |
| 14 | Om negen uur 's ochtends stond Daan voor het kantoor van Van Dijk Advies. | Advies (1) | Advies (part of a company name) | advies |  |  |
| 15 | Het was zijn eerste sollicitatie ooit, en hij was zenuwachtig. | Het (2) | the / it | het |  |  |
| 16 | Het was zijn eerste sollicitatie ooit, en hij was zenuwachtig. | was (1) | was | zijn | form of: zijn, to be |  |
| 17 | Het was zijn eerste sollicitatie ooit, en hij was zenuwachtig. | zijn (1) | his | zijn |  |  |
| 18 | Het was zijn eerste sollicitatie ooit, en hij was zenuwachtig. | eerste (1) | first | eerste |  |  |
| 19 | Het was zijn eerste sollicitatie ooit, en hij was zenuwachtig. | sollicitatie (1) | job application / interview | sollicitatie |  |  |
| 20 | Het was zijn eerste sollicitatie ooit, en hij was zenuwachtig. | ooit (1) | ever | ooit |  |  |
| 21 | Het was zijn eerste sollicitatie ooit, en hij was zenuwachtig. | en (1) | and | en |  |  |
| 22 | Het was zijn eerste sollicitatie ooit, en hij was zenuwachtig. | hij (1) | he | hij |  |  |
| 23 | Het was zijn eerste sollicitatie ooit, en hij was zenuwachtig. | was (2) | was | zijn | form of: zijn, to be |  |
| 24 | Het was zijn eerste sollicitatie ooit, en hij was zenuwachtig. | zenuwachtig (1) | nervous | zenuwachtig |  |  |
| 25 | Hij had de avond ervoor drie keer zijn cv gelezen en zijn beste overhemd gestreken. | Hij (2) | he | hij |  |  |
| 26 | Hij had de avond ervoor drie keer zijn cv gelezen en zijn beste overhemd gestreken. | had (1) | had | hebben | form of: hebben, to have |  |
| 27 | Hij had de avond ervoor drie keer zijn cv gelezen en zijn beste overhemd gestreken. | de (1) | the | de |  |  |
| 28 | Hij had de avond ervoor drie keer zijn cv gelezen en zijn beste overhemd gestreken. | avond (1) | evening | avond |  |  |
| 29 | Hij had de avond ervoor drie keer zijn cv gelezen en zijn beste overhemd gestreken. | ervoor (1) | before | ervoor |  |  |
| 30 | Hij had de avond ervoor drie keer zijn cv gelezen en zijn beste overhemd gestreken. | drie (1) | three | drie |  |  |
| 31 | Hij had de avond ervoor drie keer zijn cv gelezen en zijn beste overhemd gestreken. | keer (1) | time / turn | keer |  |  |
| 32 | Hij had de avond ervoor drie keer zijn cv gelezen en zijn beste overhemd gestreken. | zijn (2) | his | zijn |  |  |
| 33 | Hij had de avond ervoor drie keer zijn cv gelezen en zijn beste overhemd gestreken. | cv (1) | CV | cv |  |  |
| 34 | Hij had de avond ervoor drie keer zijn cv gelezen en zijn beste overhemd gestreken. | gelezen (1) | read | lezen | form of: lezen, to read |  |
| 35 | Hij had de avond ervoor drie keer zijn cv gelezen en zijn beste overhemd gestreken. | en (2) | and | en |  |  |
| 36 | Hij had de avond ervoor drie keer zijn cv gelezen en zijn beste overhemd gestreken. | zijn (3) | his | zijn |  |  |
| 37 | Hij had de avond ervoor drie keer zijn cv gelezen en zijn beste overhemd gestreken. | beste (1) | best | beter | form of: beter, better |  |
| 38 | Hij had de avond ervoor drie keer zijn cv gelezen en zijn beste overhemd gestreken. | overhemd (1) | shirt | overhemd |  |  |
| 39 | Hij had de avond ervoor drie keer zijn cv gelezen en zijn beste overhemd gestreken. | gestreken (1) | ironed | strijken | form of: strijken, to iron |  |
| 40 | Op de deur stond een klein bordje: “Vergadering om negen uur. | Op (1) | on | op |  |  |
| 41 | Op de deur stond een klein bordje: “Vergadering om negen uur. | de (2) | the | de |  |  |
| 42 | Op de deur stond een klein bordje: “Vergadering om negen uur. | deur (1) | door | deur |  |  |
| 43 | Op de deur stond een klein bordje: “Vergadering om negen uur. | stond (2) | stood | staan | form of: staan, to stand |  |
| 44 | Op de deur stond een klein bordje: “Vergadering om negen uur. | een (1) | a / an | een |  |  |
| 45 | Op de deur stond een klein bordje: “Vergadering om negen uur. | klein (1) | small | klein |  |  |
| 46 | Op de deur stond een klein bordje: “Vergadering om negen uur. | bordje (1) | sign | bord | form of: bord, sign / plate |  |
| 47 | Op de deur stond een klein bordje: “Vergadering om negen uur. | Vergadering (1) | meeting | vergadering |  |  |
| 48 | Op de deur stond een klein bordje: “Vergadering om negen uur. | om (2) | at | om |  |  |
| 49 | Op de deur stond een klein bordje: “Vergadering om negen uur. | negen (2) | nine | negen |  |  |
| 50 | Op de deur stond een klein bordje: “Vergadering om negen uur. | uur (2) | hour | uur |  |  |
| 51 | Niet storen.” | Niet (1) | not | niet |  |  |
| 52 | Niet storen.” | storen (1) | to disturb | storen |  |  |
| 53 | Daan twijfelde. | Daan (2) | Daan (the applicant's name) | Daan |  |  |
| 54 | Daan twijfelde. | twijfelde (1) | doubted | twijfelen | form of: twijfelen, to doubt |  |
| 55 | Was dit wel het juiste adres? | Was (3) | was | zijn | form of: zijn, to be |  |
| 56 | Was dit wel het juiste adres? | dit (1) | this | dit |  |  |
| 57 | Was dit wel het juiste adres? | wel (1) | indeed / well | wel |  |  |
| 58 | Was dit wel het juiste adres? | het (3) | the / it | het |  |  |
| 59 | Was dit wel het juiste adres? | juiste (1) | right (inflected) | juist | form of: juist, right |  |
| 60 | Was dit wel het juiste adres? | adres (1) | address | adres |  |  |
| 61 | Hij had een uitnodiging voor een gesprek ontvangen, maar het bericht was vaag geweest. | Hij (3) | he | hij |  |  |
| 62 | Hij had een uitnodiging voor een gesprek ontvangen, maar het bericht was vaag geweest. | had (2) | had | hebben | form of: hebben, to have |  |
| 63 | Hij had een uitnodiging voor een gesprek ontvangen, maar het bericht was vaag geweest. | een (2) | a / an | een |  |  |
| 64 | Hij had een uitnodiging voor een gesprek ontvangen, maar het bericht was vaag geweest. | uitnodiging (1) | invitation | uitnodiging |  |  |
| 65 | Hij had een uitnodiging voor een gesprek ontvangen, maar het bericht was vaag geweest. | voor (2) | for / before | voor |  |  |
| 66 | Hij had een uitnodiging voor een gesprek ontvangen, maar het bericht was vaag geweest. | een (3) | a / an | een |  |  |
| 67 | Hij had een uitnodiging voor een gesprek ontvangen, maar het bericht was vaag geweest. | gesprek (1) | conversation / interview | gesprek |  |  |
| 68 | Hij had een uitnodiging voor een gesprek ontvangen, maar het bericht was vaag geweest. | ontvangen (1) | to receive | ontvangen |  |  |
| 69 | Hij had een uitnodiging voor een gesprek ontvangen, maar het bericht was vaag geweest. | maar (1) | but | maar |  |  |
| 70 | Hij had een uitnodiging voor een gesprek ontvangen, maar het bericht was vaag geweest. | het (4) | the / it | het |  |  |
| 71 | Hij had een uitnodiging voor een gesprek ontvangen, maar het bericht was vaag geweest. | bericht (1) | message | bericht |  |  |
| 72 | Hij had een uitnodiging voor een gesprek ontvangen, maar het bericht was vaag geweest. | was (4) | was | zijn | form of: zijn, to be |  |
| 73 | Hij had een uitnodiging voor een gesprek ontvangen, maar het bericht was vaag geweest. | vaag (1) | vague | vaag |  |  |
| 74 | Hij had een uitnodiging voor een gesprek ontvangen, maar het bericht was vaag geweest. | geweest (1) | been | zijn | form of: zijn, to be |  |
| 75 | Er stond alleen: “U spreekt met de baas.” Daan had aangenomen dat hij met mevrouw Van Dijk zou praten. | Er (1) | there | er |  |  |
| 76 | Er stond alleen: “U spreekt met de baas.” Daan had aangenomen dat hij met mevrouw Van Dijk zou praten. | stond (3) | stood | staan | form of: staan, to stand |  |
| 77 | Er stond alleen: “U spreekt met de baas.” Daan had aangenomen dat hij met mevrouw Van Dijk zou praten. | alleen (1) | only | alleen |  |  |
| 78 | Er stond alleen: “U spreekt met de baas.” Daan had aangenomen dat hij met mevrouw Van Dijk zou praten. | U (1) | you (formal) | u |  |  |
| 79 | Er stond alleen: “U spreekt met de baas.” Daan had aangenomen dat hij met mevrouw Van Dijk zou praten. | spreekt (1) | speaks | spreken | form of: spreken, to speak |  |
| 80 | Er stond alleen: “U spreekt met de baas.” Daan had aangenomen dat hij met mevrouw Van Dijk zou praten. | met (1) | with | met |  |  |
| 81 | Er stond alleen: “U spreekt met de baas.” Daan had aangenomen dat hij met mevrouw Van Dijk zou praten. | de (3) | the | de |  |  |
| 82 | Er stond alleen: “U spreekt met de baas.” Daan had aangenomen dat hij met mevrouw Van Dijk zou praten. | baas (1) | boss | baas |  |  |
| 83 | Er stond alleen: “U spreekt met de baas.” Daan had aangenomen dat hij met mevrouw Van Dijk zou praten. | Daan (3) | Daan (the applicant's name) | Daan |  |  |
| 84 | Er stond alleen: “U spreekt met de baas.” Daan had aangenomen dat hij met mevrouw Van Dijk zou praten. | had (3) | had | hebben | form of: hebben, to have |  |
| 85 | Er stond alleen: “U spreekt met de baas.” Daan had aangenomen dat hij met mevrouw Van Dijk zou praten. | aangenomen (1) | assumed | aannemen | form of: aannemen, to assume |  |
| 86 | Er stond alleen: “U spreekt met de baas.” Daan had aangenomen dat hij met mevrouw Van Dijk zou praten. | dat (1) | that | dat |  |  |
| 87 | Er stond alleen: “U spreekt met de baas.” Daan had aangenomen dat hij met mevrouw Van Dijk zou praten. | hij (4) | he | hij |  |  |
| 88 | Er stond alleen: “U spreekt met de baas.” Daan had aangenomen dat hij met mevrouw Van Dijk zou praten. | met (2) | with | met |  |  |
| 89 | Er stond alleen: “U spreekt met de baas.” Daan had aangenomen dat hij met mevrouw Van Dijk zou praten. | mevrouw (1) | lady / Mrs | mevrouw |  |  |
| 90 | Er stond alleen: “U spreekt met de baas.” Daan had aangenomen dat hij met mevrouw Van Dijk zou praten. | Van (3) | of / from | van |  |  |
| 91 | Er stond alleen: “U spreekt met de baas.” Daan had aangenomen dat hij met mevrouw Van Dijk zou praten. | Dijk (2) | Dijk (surname) | Dijk |  |  |
| 92 | Er stond alleen: “U spreekt met de baas.” Daan had aangenomen dat hij met mevrouw Van Dijk zou praten. | zou (1) | would | zullen | form of: zullen, will |  |
| 93 | Er stond alleen: “U spreekt met de baas.” Daan had aangenomen dat hij met mevrouw Van Dijk zou praten. | praten (1) | to talk | praten |  |  |
| 94 | Toen de deur openging, zag hij een lege kamer. | Toen (1) | when | toen |  |  |
| 95 | Toen de deur openging, zag hij een lege kamer. | de (4) | the | de |  |  |
| 96 | Toen de deur openging, zag hij een lege kamer. | deur (2) | door | deur |  |  |
| 97 | Toen de deur openging, zag hij een lege kamer. | openging (1) | opened | opengaan | form of: opengaan, to open |  |
| 98 | Toen de deur openging, zag hij een lege kamer. | zag (1) | saw | zien | form of: zien, to see |  |
| 99 | Toen de deur openging, zag hij een lege kamer. | hij (5) | he | hij |  |  |
| 100 | Toen de deur openging, zag hij een lege kamer. | een (4) | a / an | een |  |  |
| 101 | Toen de deur openging, zag hij een lege kamer. | lege (1) | empty (inflected) | leeg | form of: leeg, empty |  |
| 102 | Toen de deur openging, zag hij een lege kamer. | kamer (1) | room | kamer |  |  |
| 103 | Aan het hoofd van de tafel zat geen mens, maar een grote bruine hond. | Aan (1) | at | aan |  |  |
| 104 | Aan het hoofd van de tafel zat geen mens, maar een grote bruine hond. | het (5) | the / it | het |  |  |
| 105 | Aan het hoofd van de tafel zat geen mens, maar een grote bruine hond. | hoofd (1) | head | hoofd |  |  |
| 106 | Aan het hoofd van de tafel zat geen mens, maar een grote bruine hond. | van (4) | of / from | van |  |  |
| 107 | Aan het hoofd van de tafel zat geen mens, maar een grote bruine hond. | de (5) | the | de |  |  |
| 108 | Aan het hoofd van de tafel zat geen mens, maar een grote bruine hond. | tafel (1) | table | tafel |  |  |
| 109 | Aan het hoofd van de tafel zat geen mens, maar een grote bruine hond. | zat (1) | sat | zitten | form of: zitten, to sit |  |
| 110 | Aan het hoofd van de tafel zat geen mens, maar een grote bruine hond. | geen (1) | no / not a | geen |  |  |
| 111 | Aan het hoofd van de tafel zat geen mens, maar een grote bruine hond. | mens (1) | person / human | mens |  |  |
| 112 | Aan het hoofd van de tafel zat geen mens, maar een grote bruine hond. | maar (2) | but | maar |  |  |
| 113 | Aan het hoofd van de tafel zat geen mens, maar een grote bruine hond. | een (5) | a / an | een |  |  |
| 114 | Aan het hoofd van de tafel zat geen mens, maar een grote bruine hond. | grote (1) | big (inflected) | groot | form of: groot, big |  |
| 115 | Aan het hoofd van de tafel zat geen mens, maar een grote bruine hond. | bruine (1) | brown (inflected) | bruin | form of: bruin, brown |  |
| 116 | Aan het hoofd van de tafel zat geen mens, maar een grote bruine hond. | hond (1) | dog | hond |  |  |
| 117 | De hond keek hem rustig aan. | De (6) | the | de |  |  |
| 118 | De hond keek hem rustig aan. | hond (2) | dog | hond |  |  |
| 119 | De hond keek hem rustig aan. | keek (1) | looked | kijken | form of: kijken, to look |  |
| 120 | De hond keek hem rustig aan. | hem (1) | him | hem |  |  |
| 121 | De hond keek hem rustig aan. | rustig (1) | calm / quiet | rustig |  |  |
| 122 | De hond keek hem rustig aan. | aan (2) | at | aan |  |  |
| 123 | “Kom binnen,” zei een stem achter hem. | Kom (1) | come | komen | form of: komen, to come |  |
| 124 | “Kom binnen,” zei een stem achter hem. | binnen (1) | inside | binnen |  |  |
| 125 | “Kom binnen,” zei een stem achter hem. | zei (1) | said | zeggen | form of: zeggen, to say |  |
| 126 | “Kom binnen,” zei een stem achter hem. | een (6) | a / an | een |  |  |
| 127 | “Kom binnen,” zei een stem achter hem. | stem (1) | voice | stem |  |  |
| 128 | “Kom binnen,” zei een stem achter hem. | achter (1) | behind | achter |  |  |
| 129 | “Kom binnen,” zei een stem achter hem. | hem (2) | him | hem |  |  |
| 130 | “U bent vast Daan. | U (2) | you (formal) | u |  |  |
| 131 | “U bent vast Daan. | bent (1) | are | zijn | form of: zijn, to be |  |
| 132 | “U bent vast Daan. | vast (1) | probably | vast |  |  |
| 133 | “U bent vast Daan. | Daan (4) | Daan (the applicant's name) | Daan |  |  |
| 134 | Ik ben mevrouw Van Dijk. | Ik (1) | I | ik |  |  |
| 135 | Ik ben mevrouw Van Dijk. | ben (1) | am | zijn | form of: zijn, to be |  |
| 136 | Ik ben mevrouw Van Dijk. | mevrouw (2) | lady / Mrs | mevrouw |  |  |
| 137 | Ik ben mevrouw Van Dijk. | Van (5) | of / from | van |  |  |
| 138 | Ik ben mevrouw Van Dijk. | Dijk (3) | Dijk (surname) | Dijk |  |  |
| 139 | En dit is Boris, onze directeur.” | En (3) | and | en |  |  |
| 140 | En dit is Boris, onze directeur.” | dit (2) | this | dit |  |  |
| 141 | En dit is Boris, onze directeur.” | is (1) | is | zijn | form of: zijn, to be |  |
| 142 | En dit is Boris, onze directeur.” | Boris (1) | Boris (the dog's name) | Boris |  |  |
| 143 | En dit is Boris, onze directeur.” | onze (1) | our | onze |  |  |
| 144 | En dit is Boris, onze directeur.” | directeur (1) | director | directeur |  |  |
| 145 | Mevrouw Van Dijk ging zitten en opende een map. | Mevrouw (3) | lady / Mrs | mevrouw |  |  |
| 146 | Mevrouw Van Dijk ging zitten en opende een map. | Van (6) | of / from | van |  |  |
| 147 | Mevrouw Van Dijk ging zitten en opende een map. | Dijk (4) | Dijk (surname) | Dijk |  |  |
| 148 | Mevrouw Van Dijk ging zitten en opende een map. | ging (1) | went | gaan | form of: gaan, to go |  |
| 149 | Mevrouw Van Dijk ging zitten en opende een map. | zitten (1) | to sit | zitten |  |  |
| 150 | Mevrouw Van Dijk ging zitten en opende een map. | en (4) | and | en |  |  |
| 151 | Mevrouw Van Dijk ging zitten en opende een map. | opende (1) | opened | openen | form of: openen, to open |  |
| 152 | Mevrouw Van Dijk ging zitten en opende een map. | een (7) | a / an | een |  |  |
| 153 | Mevrouw Van Dijk ging zitten en opende een map. | map (1) | folder | map |  |  |
| 154 | “Laten we beginnen,” zei ze. | Laten (1) | to let | laten |  |  |
| 155 | “Laten we beginnen,” zei ze. | we (1) | we | we |  |  |
| 156 | “Laten we beginnen,” zei ze. | beginnen (1) | to begin | beginnen |  |  |
| 157 | “Laten we beginnen,” zei ze. | zei (2) | said | zeggen | form of: zeggen, to say |  |
| 158 | “Laten we beginnen,” zei ze. | ze (1) | she | ze |  |  |
| 159 | “Deze vergadering duurt precies twintig minuten. | Deze (1) | this | deze |  |  |
| 160 | “Deze vergadering duurt precies twintig minuten. | vergadering (2) | meeting | vergadering |  |  |
| 161 | “Deze vergadering duurt precies twintig minuten. | duurt (1) | lasts | duren | form of: duren, to last |  |
| 162 | “Deze vergadering duurt precies twintig minuten. | precies (1) | exactly | precies |  |  |
| 163 | “Deze vergadering duurt precies twintig minuten. | twintig (1) | twenty | twintig |  |  |
| 164 | “Deze vergadering duurt precies twintig minuten. | minuten (1) | minutes | minuut | form of: minuut, minute |  |
| 165 | Ik stel u een paar vragen, en u geeft antwoord. | Ik (2) | I | ik |  |  |
| 166 | Ik stel u een paar vragen, en u geeft antwoord. | stel (1) | ask | stellen | form of: stellen |  |
| 167 | Ik stel u een paar vragen, en u geeft antwoord. | u (3) | you (formal) | u |  |  |
| 168 | Ik stel u een paar vragen, en u geeft antwoord. | een (8) | a / an | een |  |  |
| 169 | Ik stel u een paar vragen, en u geeft antwoord. | paar (1) | a few | paar |  |  |
| 170 | Ik stel u een paar vragen, en u geeft antwoord. | vragen (1) | to ask | vragen |  |  |
| 171 | Ik stel u een paar vragen, en u geeft antwoord. | en (5) | and | en |  |  |
| 172 | Ik stel u een paar vragen, en u geeft antwoord. | u (4) | you (formal) | u |  |  |
| 173 | Ik stel u een paar vragen, en u geeft antwoord. | geeft (1) | gives | geven | form of: geven, to give |  |
| 174 | Ik stel u een paar vragen, en u geeft antwoord. | antwoord (1) | answer | antwoord |  |  |
| 175 | Boris let op.” | Boris (2) | Boris (the dog's name) | Boris |  |  |
| 176 | Boris let op.” | let (1) | pay attention | letten | form of: letten, to pay attention |  |
| 177 | Boris let op.” | op (2) | on | op |  |  |
| 178 | Daan keek naar de hond. | Daan (5) | Daan (the applicant's name) | Daan |  |  |
| 179 | Daan keek naar de hond. | keek (2) | looked | kijken | form of: kijken, to look |  |
| 180 | Daan keek naar de hond. | naar (1) | to | naar |  |  |
| 181 | Daan keek naar de hond. | de (7) | the | de |  |  |
| 182 | Daan keek naar de hond. | hond (3) | dog | hond |  |  |
| 183 | Boris had inmiddels zijn ogen gesloten en leek te slapen. | Boris (3) | Boris (the dog's name) | Boris |  |  |
| 184 | Boris had inmiddels zijn ogen gesloten en leek te slapen. | had (4) | had | hebben | form of: hebben, to have |  |
| 185 | Boris had inmiddels zijn ogen gesloten en leek te slapen. | inmiddels (1) | meanwhile | inmiddels |  |  |
| 186 | Boris had inmiddels zijn ogen gesloten en leek te slapen. | zijn (4) | his | zijn |  |  |
| 187 | Boris had inmiddels zijn ogen gesloten en leek te slapen. | ogen (1) | eyes | oog | form of: oog, eye |  |
| 188 | Boris had inmiddels zijn ogen gesloten en leek te slapen. | gesloten (1) | closed | sluiten | form of: sluiten, to close |  |
| 189 | Boris had inmiddels zijn ogen gesloten en leek te slapen. | en (6) | and | en |  |  |
| 190 | Boris had inmiddels zijn ogen gesloten en leek te slapen. | leek (1) | seemed | lijken | form of: lijken, to seem |  |
| 191 | Boris had inmiddels zijn ogen gesloten en leek te slapen. | te (1) | to | te |  |  |
| 192 | Boris had inmiddels zijn ogen gesloten en leek te slapen. | slapen (1) | to sleep | slapen |  |  |
| 193 | “Waarom wilt u bij ons werken?” vroeg mevrouw Van Dijk. | Waarom (1) | why | waarom |  |  |
| 194 | “Waarom wilt u bij ons werken?” vroeg mevrouw Van Dijk. | wilt (1) | want | willen | form of: willen, to want |  |
| 195 | “Waarom wilt u bij ons werken?” vroeg mevrouw Van Dijk. | u (5) | you (formal) | u |  |  |
| 196 | “Waarom wilt u bij ons werken?” vroeg mevrouw Van Dijk. | bij (1) | at / with | bij |  |  |
| 197 | “Waarom wilt u bij ons werken?” vroeg mevrouw Van Dijk. | ons (1) | us | ons |  |  |
| 198 | “Waarom wilt u bij ons werken?” vroeg mevrouw Van Dijk. | werken (1) | to work | werken |  |  |
| 199 | “Waarom wilt u bij ons werken?” vroeg mevrouw Van Dijk. | vroeg (1) | asked | vragen | form of: vragen, to ask |  |
| 200 | “Waarom wilt u bij ons werken?” vroeg mevrouw Van Dijk. | mevrouw (4) | lady / Mrs | mevrouw |  |  |
| 201 | “Waarom wilt u bij ons werken?” vroeg mevrouw Van Dijk. | Van (7) | of / from | van |  |  |
| 202 | “Waarom wilt u bij ons werken?” vroeg mevrouw Van Dijk. | Dijk (5) | Dijk (surname) | Dijk |  |  |
| 203 | Daan had dit antwoord geoefend. | Daan (6) | Daan (the applicant's name) | Daan |  |  |
| 204 | Daan had dit antwoord geoefend. | had (5) | had | hebben | form of: hebben, to have |  |
| 205 | Daan had dit antwoord geoefend. | dit (3) | this | dit |  |  |
| 206 | Daan had dit antwoord geoefend. | antwoord (2) | answer | antwoord |  |  |
| 207 | Daan had dit antwoord geoefend. | geoefend (1) | practised | oefenen | form of: oefenen, to practise |  |
| 208 | Hij sprak over zijn ambitie, over zijn verantwoordelijkheid en over het belang van goede samenwerking. | Hij (6) | he | hij |  |  |
| 209 | Hij sprak over zijn ambitie, over zijn verantwoordelijkheid en over het belang van goede samenwerking. | sprak (1) | spoke | spreken | form of: spreken, to speak |  |
| 210 | Hij sprak over zijn ambitie, over zijn verantwoordelijkheid en over het belang van goede samenwerking. | over (1) | over / about | over |  |  |
| 211 | Hij sprak over zijn ambitie, over zijn verantwoordelijkheid en over het belang van goede samenwerking. | zijn (5) | his | zijn |  |  |
| 212 | Hij sprak over zijn ambitie, over zijn verantwoordelijkheid en over het belang van goede samenwerking. | ambitie (1) | ambition | ambitie |  |  |
| 213 | Hij sprak over zijn ambitie, over zijn verantwoordelijkheid en over het belang van goede samenwerking. | over (2) | over / about | over |  |  |
| 214 | Hij sprak over zijn ambitie, over zijn verantwoordelijkheid en over het belang van goede samenwerking. | zijn (6) | his | zijn |  |  |
| 215 | Hij sprak over zijn ambitie, over zijn verantwoordelijkheid en over het belang van goede samenwerking. | verantwoordelijkheid (1) | responsibility | verantwoordelijkheid |  |  |
| 216 | Hij sprak over zijn ambitie, over zijn verantwoordelijkheid en over het belang van goede samenwerking. | en (7) | and | en |  |  |
| 217 | Hij sprak over zijn ambitie, over zijn verantwoordelijkheid en over het belang van goede samenwerking. | over (3) | over / about | over |  |  |
| 218 | Hij sprak over zijn ambitie, over zijn verantwoordelijkheid en over het belang van goede samenwerking. | het (6) | the / it | het |  |  |
| 219 | Hij sprak over zijn ambitie, over zijn verantwoordelijkheid en over het belang van goede samenwerking. | belang (1) | importance | belang |  |  |
| 220 | Hij sprak over zijn ambitie, over zijn verantwoordelijkheid en over het belang van goede samenwerking. | van (8) | of / from | van |  |  |
| 221 | Hij sprak over zijn ambitie, over zijn verantwoordelijkheid en over het belang van goede samenwerking. | goede (1) | good (inflected) | goed | form of: goed, good |  |
| 222 | Hij sprak over zijn ambitie, over zijn verantwoordelijkheid en over het belang van goede samenwerking. | samenwerking (1) | cooperation | samenwerking |  |  |
| 223 | Hij wilde een goede indruk maken. | Hij (7) | he | hij |  |  |
| 224 | Hij wilde een goede indruk maken. | wilde (1) | wanted | willen | form of: willen, to want |  |
| 225 | Hij wilde een goede indruk maken. | een (9) | a / an | een |  |  |
| 226 | Hij wilde een goede indruk maken. | goede (2) | good (inflected) | goed | form of: goed, good |  |
| 227 | Hij wilde een goede indruk maken. | indruk (1) | impression | indruk |  |  |
| 228 | Hij wilde een goede indruk maken. | maken (1) | to make | maken |  |  |
| 229 | Toen hij klaar was, opende Boris één oog en geeuwde. | Toen (2) | when | toen |  |  |
| 230 | Toen hij klaar was, opende Boris één oog en geeuwde. | hij (8) | he | hij |  |  |
| 231 | Toen hij klaar was, opende Boris één oog en geeuwde. | klaar (1) | ready | klaar |  |  |
| 232 | Toen hij klaar was, opende Boris één oog en geeuwde. | was (5) | was | zijn | form of: zijn, to be |  |
| 233 | Toen hij klaar was, opende Boris één oog en geeuwde. | opende (2) | opened | openen | form of: openen, to open |  |
| 234 | Toen hij klaar was, opende Boris één oog en geeuwde. | Boris (4) | Boris (the dog's name) | Boris |  |  |
| 235 | Toen hij klaar was, opende Boris één oog en geeuwde. | één (1) | one | één |  |  |
| 236 | Toen hij klaar was, opende Boris één oog en geeuwde. | oog (1) | eye | oog |  |  |
| 237 | Toen hij klaar was, opende Boris één oog en geeuwde. | en (8) | and | en |  |  |
| 238 | Toen hij klaar was, opende Boris één oog en geeuwde. | geeuwde (1) | yawned | geeuwen | form of: geeuwen, to yawn |  |
| 239 | “Boris is het niet eens met uw antwoord,” zei mevrouw Van Dijk. | Boris (5) | Boris (the dog's name) | Boris |  |  |
| 240 | “Boris is het niet eens met uw antwoord,” zei mevrouw Van Dijk. | is (2) | is | zijn | form of: zijn, to be |  |
| 241 | “Boris is het niet eens met uw antwoord,” zei mevrouw Van Dijk. | het (7) | the / it | het |  |  |
| 242 | “Boris is het niet eens met uw antwoord,” zei mevrouw Van Dijk. | niet (2) | not | niet |  |  |
| 243 | “Boris is het niet eens met uw antwoord,” zei mevrouw Van Dijk. | eens (1) | once / agree | eens |  |  |
| 244 | “Boris is het niet eens met uw antwoord,” zei mevrouw Van Dijk. | met (3) | with | met |  |  |
| 245 | “Boris is het niet eens met uw antwoord,” zei mevrouw Van Dijk. | uw (1) | your (formal) | uw |  |  |
| 246 | “Boris is het niet eens met uw antwoord,” zei mevrouw Van Dijk. | antwoord (3) | answer | antwoord |  |  |
| 247 | “Boris is het niet eens met uw antwoord,” zei mevrouw Van Dijk. | zei (3) | said | zeggen | form of: zeggen, to say |  |
| 248 | “Boris is het niet eens met uw antwoord,” zei mevrouw Van Dijk. | mevrouw (5) | lady / Mrs | mevrouw |  |  |
| 249 | “Boris is het niet eens met uw antwoord,” zei mevrouw Van Dijk. | Van (9) | of / from | van |  |  |
| 250 | “Boris is het niet eens met uw antwoord,” zei mevrouw Van Dijk. | Dijk (6) | Dijk (surname) | Dijk |  |  |
| 251 | “Hij vindt het te lang.” Daan slikte. | Hij (9) | he | hij |  |  |
| 252 | “Hij vindt het te lang.” Daan slikte. | vindt (1) | finds | vinden | form of: vinden, to find |  |
| 253 | “Hij vindt het te lang.” Daan slikte. | het (8) | the / it | het |  |  |
| 254 | “Hij vindt het te lang.” Daan slikte. | te (2) | too | te |  |  |
| 255 | “Hij vindt het te lang.” Daan slikte. | lang (1) | long | lang |  |  |
| 256 | “Hij vindt het te lang.” Daan slikte. | Daan (7) | Daan (the applicant's name) | Daan |  |  |
| 257 | “Hij vindt het te lang.” Daan slikte. | slikte (1) | swallowed | slikken | form of: slikken, to swallow |  |
| 258 | “Mag ik het opnieuw proberen?” vroeg hij. | Mag (1) | may | mogen | form of: mogen, to be allowed |  |
| 259 | “Mag ik het opnieuw proberen?” vroeg hij. | ik (3) | I | ik |  |  |
| 260 | “Mag ik het opnieuw proberen?” vroeg hij. | het (9) | the / it | het |  |  |
| 261 | “Mag ik het opnieuw proberen?” vroeg hij. | opnieuw (1) | again | opnieuw |  |  |
| 262 | “Mag ik het opnieuw proberen?” vroeg hij. | proberen (1) | to try | proberen |  |  |
| 263 | “Mag ik het opnieuw proberen?” vroeg hij. | vroeg (2) | asked | vragen | form of: vragen, to ask |  |
| 264 | “Mag ik het opnieuw proberen?” vroeg hij. | hij (10) | he | hij |  |  |
| 265 | “Natuurlijk,” zei ze. | Natuurlijk (1) | of course | natuurlijk |  |  |
| 266 | “Natuurlijk,” zei ze. | zei (4) | said | zeggen | form of: zeggen, to say |  |
| 267 | “Natuurlijk,” zei ze. | ze (2) | she | ze |  |  |
| 268 | “Maar deze keer in één zin.” | Maar (3) | but | maar |  |  |
| 269 | “Maar deze keer in één zin.” | deze (2) | this | deze |  |  |
| 270 | “Maar deze keer in één zin.” | keer (2) | time / turn | keer |  |  |
| 271 | “Maar deze keer in één zin.” | in (1) | in | in |  |  |
| 272 | “Maar deze keer in één zin.” | één (2) | one | één |  |  |
| 273 | “Maar deze keer in één zin.” | zin (1) | sentence | zin |  |  |
| 274 | Daan dacht na. | Daan (8) | Daan (the applicant's name) | Daan |  |  |
| 275 | Daan dacht na. | dacht (1) | thought | denken | form of: denken, to think |  |
| 276 | Daan dacht na. | na (1) | after | na |  |  |
| 277 | “Ik wil graag leren, en ik denk dat ik hier veel kan leren,” zei hij. | Ik (4) | I | ik |  |  |
| 278 | “Ik wil graag leren, en ik denk dat ik hier veel kan leren,” zei hij. | wil (1) | wants | willen | form of: willen, to want |  |
| 279 | “Ik wil graag leren, en ik denk dat ik hier veel kan leren,” zei hij. | graag (1) | gladly | graag |  |  |
| 280 | “Ik wil graag leren, en ik denk dat ik hier veel kan leren,” zei hij. | leren (1) | to learn | leren |  |  |
| 281 | “Ik wil graag leren, en ik denk dat ik hier veel kan leren,” zei hij. | en (9) | and | en |  |  |
| 282 | “Ik wil graag leren, en ik denk dat ik hier veel kan leren,” zei hij. | ik (5) | I | ik |  |  |
| 283 | “Ik wil graag leren, en ik denk dat ik hier veel kan leren,” zei hij. | denk (1) | think | denken | form of: denken, to think |  |
| 284 | “Ik wil graag leren, en ik denk dat ik hier veel kan leren,” zei hij. | dat (2) | that | dat |  |  |
| 285 | “Ik wil graag leren, en ik denk dat ik hier veel kan leren,” zei hij. | ik (6) | I | ik |  |  |
| 286 | “Ik wil graag leren, en ik denk dat ik hier veel kan leren,” zei hij. | hier (1) | here | hier |  |  |
| 287 | “Ik wil graag leren, en ik denk dat ik hier veel kan leren,” zei hij. | veel (1) | much / a lot | veel |  |  |
| 288 | “Ik wil graag leren, en ik denk dat ik hier veel kan leren,” zei hij. | kan (1) | can | kunnen | form of: kunnen, to be able to |  |
| 289 | “Ik wil graag leren, en ik denk dat ik hier veel kan leren,” zei hij. | leren (2) | to learn | leren |  |  |
| 290 | “Ik wil graag leren, en ik denk dat ik hier veel kan leren,” zei hij. | zei (5) | said | zeggen | form of: zeggen, to say |  |
| 291 | “Ik wil graag leren, en ik denk dat ik hier veel kan leren,” zei hij. | hij (11) | he | hij |  |  |
| 292 | Boris kwispelde met zijn staart. | Boris (6) | Boris (the dog's name) | Boris |  |  |
| 293 | Boris kwispelde met zijn staart. | kwispelde (1) | wagged | kwispelen | form of: kwispelen, to wag |  |
| 294 | Boris kwispelde met zijn staart. | met (4) | with | met |  |  |
| 295 | Boris kwispelde met zijn staart. | zijn (7) | his | zijn |  |  |
| 296 | Boris kwispelde met zijn staart. | staart (1) | tail | staart |  |  |
| 297 | “Beter,” zei mevrouw Van Dijk. | Beter (1) | better | beter |  |  |
| 298 | “Beter,” zei mevrouw Van Dijk. | zei (6) | said | zeggen | form of: zeggen, to say |  |
| 299 | “Beter,” zei mevrouw Van Dijk. | mevrouw (6) | lady / Mrs | mevrouw |  |  |
| 300 | “Beter,” zei mevrouw Van Dijk. | Van (10) | of / from | van |  |  |
| 301 | “Beter,” zei mevrouw Van Dijk. | Dijk (7) | Dijk (surname) | Dijk |  |  |
| 302 | “Nu een lastige vraag. | Nu (1) | now | nu |  |  |
| 303 | “Nu een lastige vraag. | een (10) | a / an | een |  |  |
| 304 | “Nu een lastige vraag. | lastige (1) | difficult (inflected) | lastig | form of: lastig, difficult |  |
| 305 | “Nu een lastige vraag. | vraag (1) | question | vraag |  |  |
| 306 | Hoe gaat u om met een deadline die niet haalbaar is?” | Hoe (1) | how | hoe |  |  |
| 307 | Hoe gaat u om met een deadline die niet haalbaar is?” | gaat (1) | goes | gaan | form of: gaan, to go |  |
| 308 | Hoe gaat u om met een deadline die niet haalbaar is?” | u (6) | you (formal) | u |  |  |
| 309 | Hoe gaat u om met een deadline die niet haalbaar is?” | om (3) | at | om |  |  |
| 310 | Hoe gaat u om met een deadline die niet haalbaar is?” | met (5) | with | met |  |  |
| 311 | Hoe gaat u om met een deadline die niet haalbaar is?” | een (11) | a / an | een |  |  |
| 312 | Hoe gaat u om met een deadline die niet haalbaar is?” | deadline (1) | deadline | deadline |  |  |
| 313 | Hoe gaat u om met een deadline die niet haalbaar is?” | die (1) | which | die |  |  |
| 314 | Hoe gaat u om met een deadline die niet haalbaar is?” | niet (3) | not | niet |  |  |
| 315 | Hoe gaat u om met een deadline die niet haalbaar is?” | haalbaar (1) | achievable | haalbaar |  |  |
| 316 | Hoe gaat u om met een deadline die niet haalbaar is?” | is (3) | is | zijn | form of: zijn, to be |  |
| 317 | Daan voelde zijn hart sneller kloppen. | Daan (9) | Daan (the applicant's name) | Daan |  |  |
| 318 | Daan voelde zijn hart sneller kloppen. | voelde (1) | felt | voelen | form of: voelen, to feel |  |
| 319 | Daan voelde zijn hart sneller kloppen. | zijn (8) | his | zijn |  |  |
| 320 | Daan voelde zijn hart sneller kloppen. | hart (1) | heart | hart |  |  |
| 321 | Daan voelde zijn hart sneller kloppen. | sneller (1) | faster | snel | form of: snel, fast |  |
| 322 | Daan voelde zijn hart sneller kloppen. | kloppen (1) | to beat | kloppen |  |  |
| 323 | Hij wilde eerlijk zijn, maar hij wilde ook niet zwak overkomen. | Hij (12) | he | hij |  |  |
| 324 | Hij wilde eerlijk zijn, maar hij wilde ook niet zwak overkomen. | wilde (2) | wanted | willen | form of: willen, to want |  |
| 325 | Hij wilde eerlijk zijn, maar hij wilde ook niet zwak overkomen. | eerlijk (1) | honest / fair | eerlijk |  |  |
| 326 | Hij wilde eerlijk zijn, maar hij wilde ook niet zwak overkomen. | zijn (9) | to be | zijn |  |  |
| 327 | Hij wilde eerlijk zijn, maar hij wilde ook niet zwak overkomen. | maar (4) | but | maar |  |  |
| 328 | Hij wilde eerlijk zijn, maar hij wilde ook niet zwak overkomen. | hij (13) | he | hij |  |  |
| 329 | Hij wilde eerlijk zijn, maar hij wilde ook niet zwak overkomen. | wilde (3) | wanted | willen | form of: willen, to want |  |
| 330 | Hij wilde eerlijk zijn, maar hij wilde ook niet zwak overkomen. | ook (1) | also | ook |  |  |
| 331 | Hij wilde eerlijk zijn, maar hij wilde ook niet zwak overkomen. | niet (4) | not | niet |  |  |
| 332 | Hij wilde eerlijk zijn, maar hij wilde ook niet zwak overkomen. | zwak (1) | weak | zwak |  |  |
| 333 | Hij wilde eerlijk zijn, maar hij wilde ook niet zwak overkomen. | overkomen (1) | to seem | overkomen |  |  |
| 334 | “Ik zou eerst met mijn collega's praten,” zei hij. | Ik (7) | I | ik |  |  |
| 335 | “Ik zou eerst met mijn collega's praten,” zei hij. | zou (2) | would | zullen | form of: zullen, will |  |
| 336 | “Ik zou eerst met mijn collega's praten,” zei hij. | eerst (1) | first | eerst |  |  |
| 337 | “Ik zou eerst met mijn collega's praten,” zei hij. | met (6) | with | met |  |  |
| 338 | “Ik zou eerst met mijn collega's praten,” zei hij. | mijn (1) | my | mijn |  |  |
| 339 | “Ik zou eerst met mijn collega's praten,” zei hij. | collega's (1) | colleagues | collega | form of: collega, colleague |  |
| 340 | “Ik zou eerst met mijn collega's praten,” zei hij. | praten (2) | to talk | praten |  |  |
| 341 | “Ik zou eerst met mijn collega's praten,” zei hij. | zei (7) | said | zeggen | form of: zeggen, to say |  |
| 342 | “Ik zou eerst met mijn collega's praten,” zei hij. | hij (14) | he | hij |  |  |
| 343 | “Misschien kunnen we het werk verdelen. | Misschien (1) | maybe | misschien |  |  |
| 344 | “Misschien kunnen we het werk verdelen. | kunnen (1) | to be able to | kunnen |  |  |
| 345 | “Misschien kunnen we het werk verdelen. | we (2) | we | we |  |  |
| 346 | “Misschien kunnen we het werk verdelen. | het (10) | the / it | het |  |  |
| 347 | “Misschien kunnen we het werk verdelen. | werk (1) | work | werk |  |  |
| 348 | “Misschien kunnen we het werk verdelen. | verdelen (1) | to divide | verdelen |  |  |
| 349 | Als dat niet lukt, zou ik de deadline willen uitstellen. | Als (1) | if | als |  |  |
| 350 | Als dat niet lukt, zou ik de deadline willen uitstellen. | dat (3) | that | dat |  |  |
| 351 | Als dat niet lukt, zou ik de deadline willen uitstellen. | niet (5) | not | niet |  |  |
| 352 | Als dat niet lukt, zou ik de deadline willen uitstellen. | lukt (1) | succeeds | lukken | form of: lukken, to succeed |  |
| 353 | Als dat niet lukt, zou ik de deadline willen uitstellen. | zou (3) | would | zullen | form of: zullen, will |  |
| 354 | Als dat niet lukt, zou ik de deadline willen uitstellen. | ik (8) | I | ik |  |  |
| 355 | Als dat niet lukt, zou ik de deadline willen uitstellen. | de (8) | the | de |  |  |
| 356 | Als dat niet lukt, zou ik de deadline willen uitstellen. | deadline (2) | deadline | deadline |  |  |
| 357 | Als dat niet lukt, zou ik de deadline willen uitstellen. | willen (1) | to want | willen |  |  |
| 358 | Als dat niet lukt, zou ik de deadline willen uitstellen. | uitstellen (1) | to postpone | uitstellen |  |  |
| 359 | Maar ik zou nooit doen alsof alles onder controle is.” Mevrouw Van Dijk schreef iets op. | Maar (5) | but | maar |  |  |
| 360 | Maar ik zou nooit doen alsof alles onder controle is.” Mevrouw Van Dijk schreef iets op. | ik (9) | I | ik |  |  |
| 361 | Maar ik zou nooit doen alsof alles onder controle is.” Mevrouw Van Dijk schreef iets op. | zou (4) | would | zullen | form of: zullen, will |  |
| 362 | Maar ik zou nooit doen alsof alles onder controle is.” Mevrouw Van Dijk schreef iets op. | nooit (1) | never | nooit |  |  |
| 363 | Maar ik zou nooit doen alsof alles onder controle is.” Mevrouw Van Dijk schreef iets op. | doen (1) | to do | doen |  |  |
| 364 | Maar ik zou nooit doen alsof alles onder controle is.” Mevrouw Van Dijk schreef iets op. | alsof (1) | as if | alsof |  |  |
| 365 | Maar ik zou nooit doen alsof alles onder controle is.” Mevrouw Van Dijk schreef iets op. | alles (1) | everything | alles |  |  |
| 366 | Maar ik zou nooit doen alsof alles onder controle is.” Mevrouw Van Dijk schreef iets op. | onder (1) | under | onder |  |  |
| 367 | Maar ik zou nooit doen alsof alles onder controle is.” Mevrouw Van Dijk schreef iets op. | controle (1) | control | controle |  |  |
| 368 | Maar ik zou nooit doen alsof alles onder controle is.” Mevrouw Van Dijk schreef iets op. | is (4) | is | zijn | form of: zijn, to be |  |
| 369 | Maar ik zou nooit doen alsof alles onder controle is.” Mevrouw Van Dijk schreef iets op. | Mevrouw (7) | lady / Mrs | mevrouw |  |  |
| 370 | Maar ik zou nooit doen alsof alles onder controle is.” Mevrouw Van Dijk schreef iets op. | Van (11) | of / from | van |  |  |
| 371 | Maar ik zou nooit doen alsof alles onder controle is.” Mevrouw Van Dijk schreef iets op. | Dijk (8) | Dijk (surname) | Dijk |  |  |
| 372 | Maar ik zou nooit doen alsof alles onder controle is.” Mevrouw Van Dijk schreef iets op. | schreef (1) | wrote | schrijven | form of: schrijven, to write |  |
| 373 | Maar ik zou nooit doen alsof alles onder controle is.” Mevrouw Van Dijk schreef iets op. | iets (1) | something | iets |  |  |
| 374 | Maar ik zou nooit doen alsof alles onder controle is.” Mevrouw Van Dijk schreef iets op. | op (3) | on | op |  |  |
| 375 | Boris blafte één keer. | Boris (7) | Boris (the dog's name) | Boris |  |  |
| 376 | Boris blafte één keer. | blafte (1) | barked | blaffen | form of: blaffen, to bark |  |
| 377 | Boris blafte één keer. | één (3) | one | één |  |  |
| 378 | Boris blafte één keer. | keer (3) | time / turn | keer |  |  |
| 379 | “Boris vindt dat een goed voorstel,” zei ze. | Boris (8) | Boris (the dog's name) | Boris |  |  |
| 380 | “Boris vindt dat een goed voorstel,” zei ze. | vindt (2) | finds | vinden | form of: vinden, to find |  |
| 381 | “Boris vindt dat een goed voorstel,” zei ze. | dat (4) | that | dat |  |  |
| 382 | “Boris vindt dat een goed voorstel,” zei ze. | een (12) | a / an | een |  |  |
| 383 | “Boris vindt dat een goed voorstel,” zei ze. | goed (1) | good | goed |  |  |
| 384 | “Boris vindt dat een goed voorstel,” zei ze. | voorstel (1) | proposal | voorstel |  |  |
| 385 | “Boris vindt dat een goed voorstel,” zei ze. | zei (8) | said | zeggen | form of: zeggen, to say |  |
| 386 | “Boris vindt dat een goed voorstel,” zei ze. | ze (3) | she | ze |  |  |
| 387 | “Hij houdt van eerlijke mensen.” Daan durfde bijna niet te vragen wat Boris precies deed, maar hij deed het toch. | Hij (15) | he | hij |  |  |
| 388 | “Hij houdt van eerlijke mensen.” Daan durfde bijna niet te vragen wat Boris precies deed, maar hij deed het toch. | houdt (1) | likes | houden | form of: houden, to like / hold |  |
| 389 | “Hij houdt van eerlijke mensen.” Daan durfde bijna niet te vragen wat Boris precies deed, maar hij deed het toch. | van (12) | of / from | van |  |  |
| 390 | “Hij houdt van eerlijke mensen.” Daan durfde bijna niet te vragen wat Boris precies deed, maar hij deed het toch. | eerlijke (1) | honest | eerlijk | form of: eerlijk, honest / fair |  |
| 391 | “Hij houdt van eerlijke mensen.” Daan durfde bijna niet te vragen wat Boris precies deed, maar hij deed het toch. | mensen (1) | people | mens | form of: mens, person / human |  |
| 392 | “Hij houdt van eerlijke mensen.” Daan durfde bijna niet te vragen wat Boris precies deed, maar hij deed het toch. | Daan (10) | Daan (the applicant's name) | Daan |  |  |
| 393 | “Hij houdt van eerlijke mensen.” Daan durfde bijna niet te vragen wat Boris precies deed, maar hij deed het toch. | durfde (1) | dared | durven | form of: durven, to dare |  |
| 394 | “Hij houdt van eerlijke mensen.” Daan durfde bijna niet te vragen wat Boris precies deed, maar hij deed het toch. | bijna (1) | almost | bijna |  |  |
| 395 | “Hij houdt van eerlijke mensen.” Daan durfde bijna niet te vragen wat Boris precies deed, maar hij deed het toch. | niet (6) | not | niet |  |  |
| 396 | “Hij houdt van eerlijke mensen.” Daan durfde bijna niet te vragen wat Boris precies deed, maar hij deed het toch. | te (3) | to | te |  |  |
| 397 | “Hij houdt van eerlijke mensen.” Daan durfde bijna niet te vragen wat Boris precies deed, maar hij deed het toch. | vragen (2) | to ask | vragen |  |  |
| 398 | “Hij houdt van eerlijke mensen.” Daan durfde bijna niet te vragen wat Boris precies deed, maar hij deed het toch. | wat (1) | what | wat |  |  |
| 399 | “Hij houdt van eerlijke mensen.” Daan durfde bijna niet te vragen wat Boris precies deed, maar hij deed het toch. | Boris (9) | Boris (the dog's name) | Boris |  |  |
| 400 | “Hij houdt van eerlijke mensen.” Daan durfde bijna niet te vragen wat Boris precies deed, maar hij deed het toch. | precies (2) | exactly | precies |  |  |
| 401 | “Hij houdt van eerlijke mensen.” Daan durfde bijna niet te vragen wat Boris precies deed, maar hij deed het toch. | deed (1) | did | doen | form of: doen, to do |  |
| 402 | “Hij houdt van eerlijke mensen.” Daan durfde bijna niet te vragen wat Boris precies deed, maar hij deed het toch. | maar (6) | but | maar |  |  |
| 403 | “Hij houdt van eerlijke mensen.” Daan durfde bijna niet te vragen wat Boris precies deed, maar hij deed het toch. | hij (16) | he | hij |  |  |
| 404 | “Hij houdt van eerlijke mensen.” Daan durfde bijna niet te vragen wat Boris precies deed, maar hij deed het toch. | deed (2) | did | doen | form of: doen, to do |  |
| 405 | “Hij houdt van eerlijke mensen.” Daan durfde bijna niet te vragen wat Boris precies deed, maar hij deed het toch. | het (11) | the / it | het |  |  |
| 406 | “Hij houdt van eerlijke mensen.” Daan durfde bijna niet te vragen wat Boris precies deed, maar hij deed het toch. | toch (1) | yet / still / though | toch |  |  |
| 407 | “Wat is de rol van Boris in dit bedrijf?” Mevrouw Van Dijk glimlachte. | Wat (2) | what | wat |  |  |
| 408 | “Wat is de rol van Boris in dit bedrijf?” Mevrouw Van Dijk glimlachte. | is (5) | is | zijn | form of: zijn, to be |  |
| 409 | “Wat is de rol van Boris in dit bedrijf?” Mevrouw Van Dijk glimlachte. | de (9) | the | de |  |  |
| 410 | “Wat is de rol van Boris in dit bedrijf?” Mevrouw Van Dijk glimlachte. | rol (1) | role | rol |  |  |
| 411 | “Wat is de rol van Boris in dit bedrijf?” Mevrouw Van Dijk glimlachte. | van (13) | of / from | van |  |  |
| 412 | “Wat is de rol van Boris in dit bedrijf?” Mevrouw Van Dijk glimlachte. | Boris (10) | Boris (the dog's name) | Boris |  |  |
| 413 | “Wat is de rol van Boris in dit bedrijf?” Mevrouw Van Dijk glimlachte. | in (2) | in | in |  |  |
| 414 | “Wat is de rol van Boris in dit bedrijf?” Mevrouw Van Dijk glimlachte. | dit (4) | this | dit |  |  |
| 415 | “Wat is de rol van Boris in dit bedrijf?” Mevrouw Van Dijk glimlachte. | bedrijf (1) | company | bedrijf |  |  |
| 416 | “Wat is de rol van Boris in dit bedrijf?” Mevrouw Van Dijk glimlachte. | Mevrouw (8) | lady / Mrs | mevrouw |  |  |
| 417 | “Wat is de rol van Boris in dit bedrijf?” Mevrouw Van Dijk glimlachte. | Van (14) | of / from | van |  |  |
| 418 | “Wat is de rol van Boris in dit bedrijf?” Mevrouw Van Dijk glimlachte. | Dijk (9) | Dijk (surname) | Dijk |  |  |
| 419 | “Wat is de rol van Boris in dit bedrijf?” Mevrouw Van Dijk glimlachte. | glimlachte (1) | smiled | glimlachen | form of: glimlachen, to smile |  |
| 420 | “Dat is een lange discussie.” | Dat (5) | that | dat |  |  |
| 421 | “Dat is een lange discussie.” | is (6) | is | zijn | form of: zijn, to be |  |
| 422 | “Dat is een lange discussie.” | een (13) | a / an | een |  |  |
| 423 | “Dat is een lange discussie.” | lange (1) | long (inflected) | lang | form of: lang, long |  |
| 424 | “Dat is een lange discussie.” | discussie (1) | discussion | discussie |  |  |
| 425 | Op dat moment ging de telefoon. | Op (4) | on | op |  |  |
| 426 | Op dat moment ging de telefoon. | dat (6) | that | dat |  |  |
| 427 | Op dat moment ging de telefoon. | moment (1) | moment | moment |  |  |
| 428 | Op dat moment ging de telefoon. | ging (2) | went | gaan | form of: gaan, to go |  |
| 429 | Op dat moment ging de telefoon. | de (10) | the | de |  |  |
| 430 | Op dat moment ging de telefoon. | telefoon (1) | telephone | telefoon |  |  |
| 431 | Mevrouw Van Dijk keek op haar scherm en zuchtte. | Mevrouw (9) | lady / Mrs | mevrouw |  |  |
| 432 | Mevrouw Van Dijk keek op haar scherm en zuchtte. | Van (15) | of / from | van |  |  |
| 433 | Mevrouw Van Dijk keek op haar scherm en zuchtte. | Dijk (10) | Dijk (surname) | Dijk |  |  |
| 434 | Mevrouw Van Dijk keek op haar scherm en zuchtte. | keek (3) | looked | kijken | form of: kijken, to look |  |
| 435 | Mevrouw Van Dijk keek op haar scherm en zuchtte. | op (5) | on | op |  |  |
| 436 | Mevrouw Van Dijk keek op haar scherm en zuchtte. | haar (1) | her | haar |  |  |
| 437 | Mevrouw Van Dijk keek op haar scherm en zuchtte. | scherm (1) | screen | scherm |  |  |
| 438 | Mevrouw Van Dijk keek op haar scherm en zuchtte. | en (10) | and | en |  |  |
| 439 | Mevrouw Van Dijk keek op haar scherm en zuchtte. | zuchtte (1) | sighed | zuchten | form of: zuchten, to sigh |  |
| 440 | “Excuses,” zei ze. | Excuses (1) | apologies | excuus | form of: excuus, apology |  |
| 441 | “Excuses,” zei ze. | zei (9) | said | zeggen | form of: zeggen, to say |  |
| 442 | “Excuses,” zei ze. | ze (4) | she | ze |  |  |
| 443 | “Er is een spoedvergadering. | Er (2) | there | er |  |  |
| 444 | “Er is een spoedvergadering. | is (7) | is | zijn | form of: zijn, to be |  |
| 445 | “Er is een spoedvergadering. | een (14) | a / an | een |  |  |
| 446 | “Er is een spoedvergadering. | spoedvergadering (1) | emergency meeting | spoedvergadering |  |  |
| 447 | Ik moet even weg. | Ik (10) | I | ik |  |  |
| 448 | Ik moet even weg. | moet (1) | must / has to | moeten | form of: moeten, to have to |  |
| 449 | Ik moet even weg. | even (1) | briefly | even |  |  |
| 450 | Ik moet even weg. | weg (1) | away | weg |  |  |
| 451 | Boris blijft bij u.” Ze stond op en liep de kamer uit. | Boris (11) | Boris (the dog's name) | Boris |  |  |
| 452 | Boris blijft bij u.” Ze stond op en liep de kamer uit. | blijft (1) | stays | blijven | form of: blijven, to stay |  |
| 453 | Boris blijft bij u.” Ze stond op en liep de kamer uit. | bij (2) | at / with | bij |  |  |
| 454 | Boris blijft bij u.” Ze stond op en liep de kamer uit. | u (7) | you (formal) | u |  |  |
| 455 | Boris blijft bij u.” Ze stond op en liep de kamer uit. | Ze (5) | she | ze |  |  |
| 456 | Boris blijft bij u.” Ze stond op en liep de kamer uit. | stond (4) | stood | staan | form of: staan, to stand |  |
| 457 | Boris blijft bij u.” Ze stond op en liep de kamer uit. | op (6) | on | op |  |  |
| 458 | Boris blijft bij u.” Ze stond op en liep de kamer uit. | en (11) | and | en |  |  |
| 459 | Boris blijft bij u.” Ze stond op en liep de kamer uit. | liep (1) | walked | lopen | form of: lopen, to walk |  |
| 460 | Boris blijft bij u.” Ze stond op en liep de kamer uit. | de (11) | the | de |  |  |
| 461 | Boris blijft bij u.” Ze stond op en liep de kamer uit. | kamer (2) | room | kamer |  |  |
| 462 | Boris blijft bij u.” Ze stond op en liep de kamer uit. | uit (1) | out of | uit |  |  |
| 463 | Daan bleef alleen achter met de hond. | Daan (11) | Daan (the applicant's name) | Daan |  |  |
| 464 | Daan bleef alleen achter met de hond. | bleef (1) | stayed | blijven | form of: blijven, to stay |  |
| 465 | Daan bleef alleen achter met de hond. | alleen (2) | alone | alleen |  |  |
| 466 | Daan bleef alleen achter met de hond. | achter (2) | behind | achter |  |  |
| 467 | Daan bleef alleen achter met de hond. | met (7) | with | met |  |  |
| 468 | Daan bleef alleen achter met de hond. | de (12) | the | de |  |  |
| 469 | Daan bleef alleen achter met de hond. | hond (4) | dog | hond |  |  |
| 470 | Boris keek hem aan en legde zijn kop op tafel. | Boris (12) | Boris (the dog's name) | Boris |  |  |
| 471 | Boris keek hem aan en legde zijn kop op tafel. | keek (4) | looked | kijken | form of: kijken, to look |  |
| 472 | Boris keek hem aan en legde zijn kop op tafel. | hem (3) | him | hem |  |  |
| 473 | Boris keek hem aan en legde zijn kop op tafel. | aan (3) | at | aan |  |  |
| 474 | Boris keek hem aan en legde zijn kop op tafel. | en (12) | and | en |  |  |
| 475 | Boris keek hem aan en legde zijn kop op tafel. | legde (1) | put / laid | leggen | form of: leggen, to put / lay |  |
| 476 | Boris keek hem aan en legde zijn kop op tafel. | zijn (10) | his | zijn |  |  |
| 477 | Boris keek hem aan en legde zijn kop op tafel. | kop (1) | head | kop |  |  |
| 478 | Boris keek hem aan en legde zijn kop op tafel. | op (7) | on | op |  |  |
| 479 | Boris keek hem aan en legde zijn kop op tafel. | tafel (2) | table | tafel |  |  |
| 480 | Daan wist niet wat hij moest doen. | Daan (12) | Daan (the applicant's name) | Daan |  |  |
| 481 | Daan wist niet wat hij moest doen. | wist (1) | knew | weten | form of: weten, to know |  |
| 482 | Daan wist niet wat hij moest doen. | niet (7) | not | niet |  |  |
| 483 | Daan wist niet wat hij moest doen. | wat (3) | what | wat |  |  |
| 484 | Daan wist niet wat hij moest doen. | hij (17) | he | hij |  |  |
| 485 | Daan wist niet wat hij moest doen. | moest (1) | had to | moeten | form of: moeten, to have to |  |
| 486 | Daan wist niet wat hij moest doen. | doen (2) | to do | doen |  |  |
| 487 | Moest hij blijven zitten? | Moest (2) | had to | moeten | form of: moeten, to have to |  |
| 488 | Moest hij blijven zitten? | hij (18) | he | hij |  |  |
| 489 | Moest hij blijven zitten? | blijven (1) | to stay | blijven |  |  |
| 490 | Moest hij blijven zitten? | zitten (2) | to sit | zitten |  |  |
| 491 | Moest hij weggaan? | Moest (3) | had to | moeten | form of: moeten, to have to |  |
| 492 | Moest hij weggaan? | hij (19) | he | hij |  |  |
| 493 | Moest hij weggaan? | weggaan (1) | to leave / go away | weggaan |  |  |
| 494 | Hij twijfelde opnieuw. | Hij (20) | he | hij |  |  |
| 495 | Hij twijfelde opnieuw. | twijfelde (2) | doubted | twijfelen | form of: twijfelen, to doubt |  |
| 496 | Hij twijfelde opnieuw. | opnieuw (2) | again | opnieuw |  |  |
| 497 | Na een minuut stond hij op. | Na (2) | after | na |  |  |
| 498 | Na een minuut stond hij op. | een (15) | a / an | een |  |  |
| 499 | Na een minuut stond hij op. | minuut (1) | minute | minuut |  |  |
| 500 | Na een minuut stond hij op. | stond (5) | stood | staan | form of: staan, to stand |  |
| 501 | Na een minuut stond hij op. | hij (21) | he | hij |  |  |
| 502 | Na een minuut stond hij op. | op (8) | on | op |  |  |
| 503 | “Ik weet niet of u mij begrijpt,” zei hij tegen Boris, “maar ik heb echt een baan nodig.” Boris geeuwde weer. | Ik (11) | I | ik |  |  |
| 504 | “Ik weet niet of u mij begrijpt,” zei hij tegen Boris, “maar ik heb echt een baan nodig.” Boris geeuwde weer. | weet (1) | know | weten | form of: weten, to know |  |
| 505 | “Ik weet niet of u mij begrijpt,” zei hij tegen Boris, “maar ik heb echt een baan nodig.” Boris geeuwde weer. | niet (8) | not | niet |  |  |
| 506 | “Ik weet niet of u mij begrijpt,” zei hij tegen Boris, “maar ik heb echt een baan nodig.” Boris geeuwde weer. | of (1) | whether | of |  |  |
| 507 | “Ik weet niet of u mij begrijpt,” zei hij tegen Boris, “maar ik heb echt een baan nodig.” Boris geeuwde weer. | u (8) | you (formal) | u |  |  |
| 508 | “Ik weet niet of u mij begrijpt,” zei hij tegen Boris, “maar ik heb echt een baan nodig.” Boris geeuwde weer. | mij (1) | me | mij |  |  |
| 509 | “Ik weet niet of u mij begrijpt,” zei hij tegen Boris, “maar ik heb echt een baan nodig.” Boris geeuwde weer. | begrijpt (1) | understands | begrijpen | form of: begrijpen, to understand |  |
| 510 | “Ik weet niet of u mij begrijpt,” zei hij tegen Boris, “maar ik heb echt een baan nodig.” Boris geeuwde weer. | zei (10) | said | zeggen | form of: zeggen, to say |  |
| 511 | “Ik weet niet of u mij begrijpt,” zei hij tegen Boris, “maar ik heb echt een baan nodig.” Boris geeuwde weer. | hij (22) | he | hij |  |  |
| 512 | “Ik weet niet of u mij begrijpt,” zei hij tegen Boris, “maar ik heb echt een baan nodig.” Boris geeuwde weer. | tegen (1) | against / to | tegen |  |  |
| 513 | “Ik weet niet of u mij begrijpt,” zei hij tegen Boris, “maar ik heb echt een baan nodig.” Boris geeuwde weer. | Boris (13) | Boris (the dog's name) | Boris |  |  |
| 514 | “Ik weet niet of u mij begrijpt,” zei hij tegen Boris, “maar ik heb echt een baan nodig.” Boris geeuwde weer. | maar (7) | but | maar |  |  |
| 515 | “Ik weet niet of u mij begrijpt,” zei hij tegen Boris, “maar ik heb echt een baan nodig.” Boris geeuwde weer. | ik (12) | I | ik |  |  |
| 516 | “Ik weet niet of u mij begrijpt,” zei hij tegen Boris, “maar ik heb echt een baan nodig.” Boris geeuwde weer. | heb (1) | have | hebben | form of: hebben, to have |  |
| 517 | “Ik weet niet of u mij begrijpt,” zei hij tegen Boris, “maar ik heb echt een baan nodig.” Boris geeuwde weer. | echt (1) | really | echt |  |  |
| 518 | “Ik weet niet of u mij begrijpt,” zei hij tegen Boris, “maar ik heb echt een baan nodig.” Boris geeuwde weer. | een (16) | a / an | een |  |  |
| 519 | “Ik weet niet of u mij begrijpt,” zei hij tegen Boris, “maar ik heb echt een baan nodig.” Boris geeuwde weer. | baan (1) | job | baan |  |  |
| 520 | “Ik weet niet of u mij begrijpt,” zei hij tegen Boris, “maar ik heb echt een baan nodig.” Boris geeuwde weer. | nodig (1) | necessary | nodig |  |  |
| 521 | “Ik weet niet of u mij begrijpt,” zei hij tegen Boris, “maar ik heb echt een baan nodig.” Boris geeuwde weer. | Boris (14) | Boris (the dog's name) | Boris |  |  |
| 522 | “Ik weet niet of u mij begrijpt,” zei hij tegen Boris, “maar ik heb echt een baan nodig.” Boris geeuwde weer. | geeuwde (2) | yawned | geeuwen | form of: geeuwen, to yawn |  |
| 523 | “Ik weet niet of u mij begrijpt,” zei hij tegen Boris, “maar ik heb echt een baan nodig.” Boris geeuwde weer. | weer (1) | again | weer |  |  |
| 524 | Daan zuchtte. | Daan (13) | Daan (the applicant's name) | Daan |  |  |
| 525 | Daan zuchtte. | zuchtte (2) | sighed | zuchten | form of: zuchten, to sigh |  |
| 526 | “Mijn salaris hoeft niet hoog te zijn. | Mijn (2) | my | mijn |  |  |
| 527 | “Mijn salaris hoeft niet hoog te zijn. | salaris (1) | salary | salaris |  |  |
| 528 | “Mijn salaris hoeft niet hoog te zijn. | hoeft (1) | needs to | hoeven | form of: hoeven, to need to |  |
| 529 | “Mijn salaris hoeft niet hoog te zijn. | niet (9) | not | niet |  |  |
| 530 | “Mijn salaris hoeft niet hoog te zijn. | hoog (1) | high | hoog |  |  |
| 531 | “Mijn salaris hoeft niet hoog te zijn. | te (4) | to | te |  |  |
| 532 | “Mijn salaris hoeft niet hoog te zijn. | zijn (11) | to be | zijn |  |  |
| 533 | Ik wil gewoon een contract en een eerlijke kans.” Boris sloeg met zijn staart op de stoel. | Ik (13) | I | ik |  |  |
| 534 | Ik wil gewoon een contract en een eerlijke kans.” Boris sloeg met zijn staart op de stoel. | wil (2) | wants | willen | form of: willen, to want |  |
| 535 | Ik wil gewoon een contract en een eerlijke kans.” Boris sloeg met zijn staart op de stoel. | gewoon (1) | just / normal | gewoon |  |  |
| 536 | Ik wil gewoon een contract en een eerlijke kans.” Boris sloeg met zijn staart op de stoel. | een (17) | a / an | een |  |  |
| 537 | Ik wil gewoon een contract en een eerlijke kans.” Boris sloeg met zijn staart op de stoel. | contract (1) | contract | contract |  |  |
| 538 | Ik wil gewoon een contract en een eerlijke kans.” Boris sloeg met zijn staart op de stoel. | en (13) | and | en |  |  |
| 539 | Ik wil gewoon een contract en een eerlijke kans.” Boris sloeg met zijn staart op de stoel. | een (18) | a / an | een |  |  |
| 540 | Ik wil gewoon een contract en een eerlijke kans.” Boris sloeg met zijn staart op de stoel. | eerlijke (2) | honest | eerlijk | form of: eerlijk, honest / fair |  |
| 541 | Ik wil gewoon een contract en een eerlijke kans.” Boris sloeg met zijn staart op de stoel. | kans (1) | chance | kans |  |  |
| 542 | Ik wil gewoon een contract en een eerlijke kans.” Boris sloeg met zijn staart op de stoel. | Boris (15) | Boris (the dog's name) | Boris |  |  |
| 543 | Ik wil gewoon een contract en een eerlijke kans.” Boris sloeg met zijn staart op de stoel. | sloeg (1) | hit | slaan | form of: slaan, to hit |  |
| 544 | Ik wil gewoon een contract en een eerlijke kans.” Boris sloeg met zijn staart op de stoel. | met (8) | with | met |  |  |
| 545 | Ik wil gewoon een contract en een eerlijke kans.” Boris sloeg met zijn staart op de stoel. | zijn (12) | his | zijn |  |  |
| 546 | Ik wil gewoon een contract en een eerlijke kans.” Boris sloeg met zijn staart op de stoel. | staart (2) | tail | staart |  |  |
| 547 | Ik wil gewoon een contract en een eerlijke kans.” Boris sloeg met zijn staart op de stoel. | op (9) | on | op |  |  |
| 548 | Ik wil gewoon een contract en een eerlijke kans.” Boris sloeg met zijn staart op de stoel. | de (13) | the | de |  |  |
| 549 | Ik wil gewoon een contract en een eerlijke kans.” Boris sloeg met zijn staart op de stoel. | stoel (1) | chair | stoel |  |  |
| 550 | Daan glimlachte ondanks zichzelf. | Daan (14) | Daan (the applicant's name) | Daan |  |  |
| 551 | Daan glimlachte ondanks zichzelf. | glimlachte (2) | smiled | glimlachen | form of: glimlachen, to smile |  |
| 552 | Daan glimlachte ondanks zichzelf. | ondanks (1) | despite | ondanks |  |  |
| 553 | Daan glimlachte ondanks zichzelf. | zichzelf (1) | himself / herself | zichzelf |  |  |
| 554 | “U bent een rare baas,” zei hij zacht. | U (9) | you (formal) | u |  |  |
| 555 | “U bent een rare baas,” zei hij zacht. | bent (2) | are | zijn | form of: zijn, to be |  |
| 556 | “U bent een rare baas,” zei hij zacht. | een (19) | a / an | een |  |  |
| 557 | “U bent een rare baas,” zei hij zacht. | rare (1) | strange (inflected) | raar | form of: raar, strange |  |
| 558 | “U bent een rare baas,” zei hij zacht. | baas (2) | boss | baas |  |  |
| 559 | “U bent een rare baas,” zei hij zacht. | zei (11) | said | zeggen | form of: zeggen, to say |  |
| 560 | “U bent een rare baas,” zei hij zacht. | hij (23) | he | hij |  |  |
| 561 | “U bent een rare baas,” zei hij zacht. | zacht (1) | soft / quiet | zacht |  |  |
| 562 | Na tien minuten kwam mevrouw Van Dijk terug. | Na (3) | after | na |  |  |
| 563 | Na tien minuten kwam mevrouw Van Dijk terug. | tien (1) | ten | tien |  |  |
| 564 | Na tien minuten kwam mevrouw Van Dijk terug. | minuten (2) | minutes | minuut | form of: minuut, minute |  |
| 565 | Na tien minuten kwam mevrouw Van Dijk terug. | kwam (1) | came | komen | form of: komen, to come |  |
| 566 | Na tien minuten kwam mevrouw Van Dijk terug. | mevrouw (10) | lady / Mrs | mevrouw |  |  |
| 567 | Na tien minuten kwam mevrouw Van Dijk terug. | Van (16) | of / from | van |  |  |
| 568 | Na tien minuten kwam mevrouw Van Dijk terug. | Dijk (11) | Dijk (surname) | Dijk |  |  |
| 569 | Na tien minuten kwam mevrouw Van Dijk terug. | terug (1) | back | terug |  |  |
| 570 | Ze zag er vermoeid uit. | Ze (6) | she | ze |  |  |
| 571 | Ze zag er vermoeid uit. | zag (2) | saw | zien | form of: zien, to see |  |
| 572 | Ze zag er vermoeid uit. | er (3) | there | er |  |  |
| 573 | Ze zag er vermoeid uit. | vermoeid (1) | tired | vermoeid |  |  |
| 574 | Ze zag er vermoeid uit. | uit (2) | looked (in 'eruitzien') | uit |  |  |
| 575 | “Sorry,” zei ze. | Sorry (1) | sorry | sorry |  |  |
| 576 | “Sorry,” zei ze. | zei (12) | said | zeggen | form of: zeggen, to say |  |
| 577 | “Sorry,” zei ze. | ze (7) | she | ze |  |  |
| 578 | “De vergadering liep uit. | De (14) | the | de |  |  |
| 579 | “De vergadering liep uit. | vergadering (3) | meeting | vergadering |  |  |
| 580 | “De vergadering liep uit. | liep (2) | walked | lopen | form of: lopen, to walk |  |
| 581 | “De vergadering liep uit. | uit (3) | ran over (in 'uitlopen') | uit |  |  |
| 582 | Dat gebeurt hier vaak.” Ze ging weer zitten en keek Daan onderzoekend aan. | Dat (7) | that | dat |  |  |
| 583 | Dat gebeurt hier vaak.” Ze ging weer zitten en keek Daan onderzoekend aan. | gebeurt (1) | happens | gebeuren | form of: gebeuren, to happen |  |
| 584 | Dat gebeurt hier vaak.” Ze ging weer zitten en keek Daan onderzoekend aan. | hier (2) | here | hier |  |  |
| 585 | Dat gebeurt hier vaak.” Ze ging weer zitten en keek Daan onderzoekend aan. | vaak (1) | often | vaak |  |  |
| 586 | Dat gebeurt hier vaak.” Ze ging weer zitten en keek Daan onderzoekend aan. | Ze (8) | she | ze |  |  |
| 587 | Dat gebeurt hier vaak.” Ze ging weer zitten en keek Daan onderzoekend aan. | ging (3) | went | gaan | form of: gaan, to go |  |
| 588 | Dat gebeurt hier vaak.” Ze ging weer zitten en keek Daan onderzoekend aan. | weer (2) | again | weer |  |  |
| 589 | Dat gebeurt hier vaak.” Ze ging weer zitten en keek Daan onderzoekend aan. | zitten (3) | to sit | zitten |  |  |
| 590 | Dat gebeurt hier vaak.” Ze ging weer zitten en keek Daan onderzoekend aan. | en (14) | and | en |  |  |
| 591 | Dat gebeurt hier vaak.” Ze ging weer zitten en keek Daan onderzoekend aan. | keek (5) | looked | kijken | form of: kijken, to look |  |
| 592 | Dat gebeurt hier vaak.” Ze ging weer zitten en keek Daan onderzoekend aan. | Daan (15) | Daan (the applicant's name) | Daan |  |  |
| 593 | Dat gebeurt hier vaak.” Ze ging weer zitten en keek Daan onderzoekend aan. | onderzoekend (1) | searching / inquiring | onderzoekend |  |  |
| 594 | Dat gebeurt hier vaak.” Ze ging weer zitten en keek Daan onderzoekend aan. | aan (4) | at | aan |  |  |
| 595 | “Ik moet u iets uitleggen. | Ik (14) | I | ik |  |  |
| 596 | “Ik moet u iets uitleggen. | moet (2) | must / has to | moeten | form of: moeten, to have to |  |
| 597 | “Ik moet u iets uitleggen. | u (10) | you (formal) | u |  |  |
| 598 | “Ik moet u iets uitleggen. | iets (2) | something | iets |  |  |
| 599 | “Ik moet u iets uitleggen. | uitleggen (1) | to explain | uitleggen |  |  |
| 600 | Dit bedrijf is niet zoals andere bedrijven. | Dit (5) | this | dit |  |  |
| 601 | Dit bedrijf is niet zoals andere bedrijven. | bedrijf (2) | company | bedrijf |  |  |
| 602 | Dit bedrijf is niet zoals andere bedrijven. | is (8) | is | zijn | form of: zijn, to be |  |
| 603 | Dit bedrijf is niet zoals andere bedrijven. | niet (10) | not | niet |  |  |
| 604 | Dit bedrijf is niet zoals andere bedrijven. | zoals (1) | like | zoals |  |  |
| 605 | Dit bedrijf is niet zoals andere bedrijven. | andere (1) | other (inflected) | ander | form of: ander, other |  |
| 606 | Dit bedrijf is niet zoals andere bedrijven. | bedrijven (1) | companies | bedrijf | form of: bedrijf, company |  |
| 607 | Boris is niet mijn hond. | Boris (16) | Boris (the dog's name) | Boris |  |  |
| 608 | Boris is niet mijn hond. | is (9) | is | zijn | form of: zijn, to be |  |
| 609 | Boris is niet mijn hond. | niet (11) | not | niet |  |  |
| 610 | Boris is niet mijn hond. | mijn (3) | my | mijn |  |  |
| 611 | Boris is niet mijn hond. | hond (5) | dog | hond |  |  |
| 612 | Hij is de baas. | Hij (24) | he | hij |  |  |
| 613 | Hij is de baas. | is (10) | is | zijn | form of: zijn, to be |  |
| 614 | Hij is de baas. | de (15) | the | de |  |  |
| 615 | Hij is de baas. | baas (3) | boss | baas |  |  |
| 616 | Ik ben zijn assistent.” Daan wist niet of ze een grap maakte. | Ik (15) | I | ik |  |  |
| 617 | Ik ben zijn assistent.” Daan wist niet of ze een grap maakte. | ben (2) | am | zijn | form of: zijn, to be |  |
| 618 | Ik ben zijn assistent.” Daan wist niet of ze een grap maakte. | zijn (13) | his | zijn |  |  |
| 619 | Ik ben zijn assistent.” Daan wist niet of ze een grap maakte. | assistent (1) | assistant | assistent |  |  |
| 620 | Ik ben zijn assistent.” Daan wist niet of ze een grap maakte. | Daan (16) | Daan (the applicant's name) | Daan |  |  |
| 621 | Ik ben zijn assistent.” Daan wist niet of ze een grap maakte. | wist (2) | knew | weten | form of: weten, to know |  |
| 622 | Ik ben zijn assistent.” Daan wist niet of ze een grap maakte. | niet (12) | not | niet |  |  |
| 623 | Ik ben zijn assistent.” Daan wist niet of ze een grap maakte. | of (2) | whether | of |  |  |
| 624 | Ik ben zijn assistent.” Daan wist niet of ze een grap maakte. | ze (9) | she | ze |  |  |
| 625 | Ik ben zijn assistent.” Daan wist niet of ze een grap maakte. | een (20) | a / an | een |  |  |
| 626 | Ik ben zijn assistent.” Daan wist niet of ze een grap maakte. | grap (1) | joke | grap |  |  |
| 627 | Ik ben zijn assistent.” Daan wist niet of ze een grap maakte. | maakte (1) | made | maken | form of: maken, to make |  |
| 628 | “Bedoelt u dat de hond de beslissingen neemt?” vroeg hij. | Bedoelt (1) | means | bedoelen | form of: bedoelen, to mean |  |
| 629 | “Bedoelt u dat de hond de beslissingen neemt?” vroeg hij. | u (11) | you (formal) | u |  |  |
| 630 | “Bedoelt u dat de hond de beslissingen neemt?” vroeg hij. | dat (8) | that | dat |  |  |
| 631 | “Bedoelt u dat de hond de beslissingen neemt?” vroeg hij. | de (16) | the | de |  |  |
| 632 | “Bedoelt u dat de hond de beslissingen neemt?” vroeg hij. | hond (6) | dog | hond |  |  |
| 633 | “Bedoelt u dat de hond de beslissingen neemt?” vroeg hij. | de (17) | the | de |  |  |
| 634 | “Bedoelt u dat de hond de beslissingen neemt?” vroeg hij. | beslissingen (1) | decisions | beslissing | form of: beslissing, decision |  |
| 635 | “Bedoelt u dat de hond de beslissingen neemt?” vroeg hij. | neemt (1) | takes | nemen | form of: nemen, to take |  |
| 636 | “Bedoelt u dat de hond de beslissingen neemt?” vroeg hij. | vroeg (3) | asked | vragen | form of: vragen, to ask |  |
| 637 | “Bedoelt u dat de hond de beslissingen neemt?” vroeg hij. | hij (25) | he | hij |  |  |
| 638 | “Niet alle beslissingen,” zei ze. | Niet (13) | not | niet |  |  |
| 639 | “Niet alle beslissingen,” zei ze. | alle (1) | all | alle |  |  |
| 640 | “Niet alle beslissingen,” zei ze. | beslissingen (2) | decisions | beslissing | form of: beslissing, decision |  |
| 641 | “Niet alle beslissingen,” zei ze. | zei (13) | said | zeggen | form of: zeggen, to say |  |
| 642 | “Niet alle beslissingen,” zei ze. | ze (10) | she | ze |  |  |
| 643 | “Maar de belangrijke wel. | Maar (8) | but | maar |  |  |
| 644 | “Maar de belangrijke wel. | de (18) | the | de |  |  |
| 645 | “Maar de belangrijke wel. | belangrijke (1) | important (inflected) | belangrijk | form of: belangrijk, important |  |
| 646 | “Maar de belangrijke wel. | wel (2) | indeed / well | wel |  |  |
| 647 | Hij heeft een goede neus voor mensen. | Hij (26) | he | hij |  |  |
| 648 | Hij heeft een goede neus voor mensen. | heeft (1) | has | hebben | form of: hebben, to have |  |
| 649 | Hij heeft een goede neus voor mensen. | een (21) | a / an | een |  |  |
| 650 | Hij heeft een goede neus voor mensen. | goede (3) | good (inflected) | goed | form of: goed, good |  |
| 651 | Hij heeft een goede neus voor mensen. | neus (1) | nose | neus |  |  |
| 652 | Hij heeft een goede neus voor mensen. | voor (3) | for / before | voor |  |  |
| 653 | Hij heeft een goede neus voor mensen. | mensen (2) | people | mens | form of: mens, person / human |  |
| 654 | Daarom zit hij bij elk gesprek. | Daarom (1) | therefore | daarom |  |  |
| 655 | Daarom zit hij bij elk gesprek. | zit (1) | sits | zitten | form of: zitten, to sit |  |
| 656 | Daarom zit hij bij elk gesprek. | hij (27) | he | hij |  |  |
| 657 | Daarom zit hij bij elk gesprek. | bij (3) | at / with | bij |  |  |
| 658 | Daarom zit hij bij elk gesprek. | elk (1) | each | elk |  |  |
| 659 | Daarom zit hij bij elk gesprek. | gesprek (2) | conversation / interview | gesprek |  |  |
| 660 | Als hij u niet mag, krijgt u de baan niet.” Daan keek naar Boris. | Als (2) | if | als |  |  |
| 661 | Als hij u niet mag, krijgt u de baan niet.” Daan keek naar Boris. | hij (28) | he | hij |  |  |
| 662 | Als hij u niet mag, krijgt u de baan niet.” Daan keek naar Boris. | u (12) | you (formal) | u |  |  |
| 663 | Als hij u niet mag, krijgt u de baan niet.” Daan keek naar Boris. | niet (14) | not | niet |  |  |
| 664 | Als hij u niet mag, krijgt u de baan niet.” Daan keek naar Boris. | mag (2) | may | mogen | form of: mogen, to be allowed |  |
| 665 | Als hij u niet mag, krijgt u de baan niet.” Daan keek naar Boris. | krijgt (1) | gets | krijgen | form of: krijgen, to get |  |
| 666 | Als hij u niet mag, krijgt u de baan niet.” Daan keek naar Boris. | u (13) | you (formal) | u |  |  |
| 667 | Als hij u niet mag, krijgt u de baan niet.” Daan keek naar Boris. | de (19) | the | de |  |  |
| 668 | Als hij u niet mag, krijgt u de baan niet.” Daan keek naar Boris. | baan (2) | job | baan |  |  |
| 669 | Als hij u niet mag, krijgt u de baan niet.” Daan keek naar Boris. | niet (15) | not | niet |  |  |
| 670 | Als hij u niet mag, krijgt u de baan niet.” Daan keek naar Boris. | Daan (17) | Daan (the applicant's name) | Daan |  |  |
| 671 | Als hij u niet mag, krijgt u de baan niet.” Daan keek naar Boris. | keek (6) | looked | kijken | form of: kijken, to look |  |
| 672 | Als hij u niet mag, krijgt u de baan niet.” Daan keek naar Boris. | naar (2) | to | naar |  |  |
| 673 | Als hij u niet mag, krijgt u de baan niet.” Daan keek naar Boris. | Boris (17) | Boris (the dog's name) | Boris |  |  |
| 674 | Boris keek terug. | Boris (18) | Boris (the dog's name) | Boris |  |  |
| 675 | Boris keek terug. | keek (7) | looked | kijken | form of: kijken, to look |  |
| 676 | Boris keek terug. | terug (2) | back | terug |  |  |
| 677 | “En als ik hem wel mag?” vroeg Daan. | En (15) | and | en |  |  |
| 678 | “En als ik hem wel mag?” vroeg Daan. | als (3) | if | als |  |  |
| 679 | “En als ik hem wel mag?” vroeg Daan. | ik (16) | I | ik |  |  |
| 680 | “En als ik hem wel mag?” vroeg Daan. | hem (4) | him | hem |  |  |
| 681 | “En als ik hem wel mag?” vroeg Daan. | wel (3) | indeed / well | wel |  |  |
| 682 | “En als ik hem wel mag?” vroeg Daan. | mag (3) | may | mogen | form of: mogen, to be allowed |  |
| 683 | “En als ik hem wel mag?” vroeg Daan. | vroeg (4) | asked | vragen | form of: vragen, to ask |  |
| 684 | “En als ik hem wel mag?” vroeg Daan. | Daan (18) | Daan (the applicant's name) | Daan |  |  |
| 685 | “Dan moet u hem nog overtuigen,” zei mevrouw Van Dijk. | Dan (1) | then / than | dan |  |  |
| 686 | “Dan moet u hem nog overtuigen,” zei mevrouw Van Dijk. | moet (3) | must / has to | moeten | form of: moeten, to have to |  |
| 687 | “Dan moet u hem nog overtuigen,” zei mevrouw Van Dijk. | u (14) | you (formal) | u |  |  |
| 688 | “Dan moet u hem nog overtuigen,” zei mevrouw Van Dijk. | hem (5) | him | hem |  |  |
| 689 | “Dan moet u hem nog overtuigen,” zei mevrouw Van Dijk. | nog (1) | still / yet | nog |  |  |
| 690 | “Dan moet u hem nog overtuigen,” zei mevrouw Van Dijk. | overtuigen (1) | to convince | overtuigen |  |  |
| 691 | “Dan moet u hem nog overtuigen,” zei mevrouw Van Dijk. | zei (14) | said | zeggen | form of: zeggen, to say |  |
| 692 | “Dan moet u hem nog overtuigen,” zei mevrouw Van Dijk. | mevrouw (11) | lady / Mrs | mevrouw |  |  |
| 693 | “Dan moet u hem nog overtuigen,” zei mevrouw Van Dijk. | Van (17) | of / from | van |  |  |
| 694 | “Dan moet u hem nog overtuigen,” zei mevrouw Van Dijk. | Dijk (12) | Dijk (surname) | Dijk |  |  |
| 695 | “Hij houdt niet van mensen die alleen maar praten. | Hij (29) | he | hij |  |  |
| 696 | “Hij houdt niet van mensen die alleen maar praten. | houdt (2) | likes | houden | form of: houden, to like / hold |  |
| 697 | “Hij houdt niet van mensen die alleen maar praten. | niet (16) | not | niet |  |  |
| 698 | “Hij houdt niet van mensen die alleen maar praten. | van (18) | of / from | van |  |  |
| 699 | “Hij houdt niet van mensen die alleen maar praten. | mensen (3) | people | mens | form of: mens, person / human |  |
| 700 | “Hij houdt niet van mensen die alleen maar praten. | die (2) | who | die |  |  |
| 701 | “Hij houdt niet van mensen die alleen maar praten. | alleen (3) | only | alleen |  |  |
| 702 | “Hij houdt niet van mensen die alleen maar praten. | maar (9) | but | maar |  |  |
| 703 | “Hij houdt niet van mensen die alleen maar praten. | praten (3) | to talk | praten |  |  |
| 704 | Hij wil bewijs zien.” | Hij (30) | he | hij |  |  |
| 705 | Hij wil bewijs zien.” | wil (3) | wants | willen | form of: willen, to want |  |
| 706 | Hij wil bewijs zien.” | bewijs (1) | proof | bewijs |  |  |
| 707 | Hij wil bewijs zien.” | zien (1) | to see | zien |  |  |
| 708 | Daan dacht koortsachtig na. | Daan (19) | Daan (the applicant's name) | Daan |  |  |
| 709 | Daan dacht koortsachtig na. | dacht (2) | thought | denken | form of: denken, to think |  |
| 710 | Daan dacht koortsachtig na. | koortsachtig (1) | feverish | koortsachtig |  |  |
| 711 | Daan dacht koortsachtig na. | na (4) | after | na |  |  |
| 712 | Wat kon hij doen om Boris te overtuigen? | Wat (4) | what | wat |  |  |
| 713 | Wat kon hij doen om Boris te overtuigen? | kon (1) | could | kunnen | form of: kunnen, to be able to |  |
| 714 | Wat kon hij doen om Boris te overtuigen? | hij (31) | he | hij |  |  |
| 715 | Wat kon hij doen om Boris te overtuigen? | doen (3) | to do | doen |  |  |
| 716 | Wat kon hij doen om Boris te overtuigen? | om (4) | at | om |  |  |
| 717 | Wat kon hij doen om Boris te overtuigen? | Boris (19) | Boris (the dog's name) | Boris |  |  |
| 718 | Wat kon hij doen om Boris te overtuigen? | te (5) | to | te |  |  |
| 719 | Wat kon hij doen om Boris te overtuigen? | overtuigen (2) | to convince | overtuigen |  |  |
| 720 | Hij had geen cv dat indruk maakte, geen diploma's van beroemde universiteiten en geen grote ambities. | Hij (32) | he | hij |  |  |
| 721 | Hij had geen cv dat indruk maakte, geen diploma's van beroemde universiteiten en geen grote ambities. | had (6) | had | hebben | form of: hebben, to have |  |
| 722 | Hij had geen cv dat indruk maakte, geen diploma's van beroemde universiteiten en geen grote ambities. | geen (2) | no / not a | geen |  |  |
| 723 | Hij had geen cv dat indruk maakte, geen diploma's van beroemde universiteiten en geen grote ambities. | cv (2) | CV | cv |  |  |
| 724 | Hij had geen cv dat indruk maakte, geen diploma's van beroemde universiteiten en geen grote ambities. | dat (9) | that | dat |  |  |
| 725 | Hij had geen cv dat indruk maakte, geen diploma's van beroemde universiteiten en geen grote ambities. | indruk (2) | impression | indruk |  |  |
| 726 | Hij had geen cv dat indruk maakte, geen diploma's van beroemde universiteiten en geen grote ambities. | maakte (2) | made | maken | form of: maken, to make |  |
| 727 | Hij had geen cv dat indruk maakte, geen diploma's van beroemde universiteiten en geen grote ambities. | geen (3) | no / not a | geen |  |  |
| 728 | Hij had geen cv dat indruk maakte, geen diploma's van beroemde universiteiten en geen grote ambities. | diploma's (1) | diplomas | diploma | form of: diploma, diploma |  |
| 729 | Hij had geen cv dat indruk maakte, geen diploma's van beroemde universiteiten en geen grote ambities. | van (19) | of / from | van |  |  |
| 730 | Hij had geen cv dat indruk maakte, geen diploma's van beroemde universiteiten en geen grote ambities. | beroemde (1) | famous (inflected) | beroemd | form of: beroemd, famous |  |
| 731 | Hij had geen cv dat indruk maakte, geen diploma's van beroemde universiteiten en geen grote ambities. | universiteiten (1) | universities | universiteit | form of: universiteit, university |  |
| 732 | Hij had geen cv dat indruk maakte, geen diploma's van beroemde universiteiten en geen grote ambities. | en (16) | and | en |  |  |
| 733 | Hij had geen cv dat indruk maakte, geen diploma's van beroemde universiteiten en geen grote ambities. | geen (4) | no / not a | geen |  |  |
| 734 | Hij had geen cv dat indruk maakte, geen diploma's van beroemde universiteiten en geen grote ambities. | grote (2) | big (inflected) | groot | form of: groot, big |  |
| 735 | Hij had geen cv dat indruk maakte, geen diploma's van beroemde universiteiten en geen grote ambities. | ambities (1) | ambitions | ambitie | form of: ambitie, ambition |  |
| 736 | Maar hij had wel een idee. | Maar (10) | but | maar |  |  |
| 737 | Maar hij had wel een idee. | hij (33) | he | hij |  |  |
| 738 | Maar hij had wel een idee. | had (7) | had | hebben | form of: hebben, to have |  |
| 739 | Maar hij had wel een idee. | wel (4) | indeed / well | wel |  |  |
| 740 | Maar hij had wel een idee. | een (22) | a / an | een |  |  |
| 741 | Maar hij had wel een idee. | idee (1) | idea | idee |  |  |
| 742 | “Mijn voorstel is dat we de vergaderingen korter maken,” zei hij. | Mijn (4) | my | mijn |  |  |
| 743 | “Mijn voorstel is dat we de vergaderingen korter maken,” zei hij. | voorstel (2) | proposal | voorstel |  |  |
| 744 | “Mijn voorstel is dat we de vergaderingen korter maken,” zei hij. | is (11) | is | zijn | form of: zijn, to be |  |
| 745 | “Mijn voorstel is dat we de vergaderingen korter maken,” zei hij. | dat (10) | that | dat |  |  |
| 746 | “Mijn voorstel is dat we de vergaderingen korter maken,” zei hij. | we (3) | we | we |  |  |
| 747 | “Mijn voorstel is dat we de vergaderingen korter maken,” zei hij. | de (20) | the | de |  |  |
| 748 | “Mijn voorstel is dat we de vergaderingen korter maken,” zei hij. | vergaderingen (1) | meetings | vergadering | form of: vergadering, meeting |  |
| 749 | “Mijn voorstel is dat we de vergaderingen korter maken,” zei hij. | korter (1) | shorter | kort | form of: kort, short |  |
| 750 | “Mijn voorstel is dat we de vergaderingen korter maken,” zei hij. | maken (2) | to make | maken |  |  |
| 751 | “Mijn voorstel is dat we de vergaderingen korter maken,” zei hij. | zei (15) | said | zeggen | form of: zeggen, to say |  |
| 752 | “Mijn voorstel is dat we de vergaderingen korter maken,” zei hij. | hij (34) | he | hij |  |  |
| 753 | “En dat we meer pauzes nemen. | En (17) | and | en |  |  |
| 754 | “En dat we meer pauzes nemen. | dat (11) | that | dat |  |  |
| 755 | “En dat we meer pauzes nemen. | we (4) | we | we |  |  |
| 756 | “En dat we meer pauzes nemen. | meer (1) | more | meer |  |  |
| 757 | “En dat we meer pauzes nemen. | pauzes (1) | breaks | pauze | form of: pauze, break |  |
| 758 | “En dat we meer pauzes nemen. | nemen (1) | to take | nemen |  |  |
| 759 | Een hond heeft beweging nodig. | Een (23) | a / an | een |  |  |
| 760 | Een hond heeft beweging nodig. | hond (7) | dog | hond |  |  |
| 761 | Een hond heeft beweging nodig. | heeft (2) | has | hebben | form of: hebben, to have |  |
| 762 | Een hond heeft beweging nodig. | beweging (1) | movement / exercise | beweging |  |  |
| 763 | Een hond heeft beweging nodig. | nodig (2) | necessary | nodig |  |  |
| 764 | Mensen ook.” Mevrouw Van Dijk keek verrast. | Mensen (4) | people | mens | form of: mens, person / human |  |
| 765 | Mensen ook.” Mevrouw Van Dijk keek verrast. | ook (2) | also | ook |  |  |
| 766 | Mensen ook.” Mevrouw Van Dijk keek verrast. | Mevrouw (12) | lady / Mrs | mevrouw |  |  |
| 767 | Mensen ook.” Mevrouw Van Dijk keek verrast. | Van (20) | of / from | van |  |  |
| 768 | Mensen ook.” Mevrouw Van Dijk keek verrast. | Dijk (13) | Dijk (surname) | Dijk |  |  |
| 769 | Mensen ook.” Mevrouw Van Dijk keek verrast. | keek (8) | looked | kijken | form of: kijken, to look |  |
| 770 | Mensen ook.” Mevrouw Van Dijk keek verrast. | verrast (1) | surprised | verrast |  |  |
| 771 | Boris hief zijn kop op. | Boris (20) | Boris (the dog's name) | Boris |  |  |
| 772 | Boris hief zijn kop op. | hief (1) | lifted | heffen | form of: heffen, to lift |  |
| 773 | Boris hief zijn kop op. | zijn (14) | his | zijn |  |  |
| 774 | Boris hief zijn kop op. | kop (2) | head | kop |  |  |
| 775 | Boris hief zijn kop op. | op (10) | on | op |  |  |
| 776 | “Als we elke vergadering met een korte pauze en een wandeling beginnen, worden we allemaal rustiger,” ging Daan verder. | Als (4) | if | als |  |  |
| 777 | “Als we elke vergadering met een korte pauze en een wandeling beginnen, worden we allemaal rustiger,” ging Daan verder. | we (5) | we | we |  |  |
| 778 | “Als we elke vergadering met een korte pauze en een wandeling beginnen, worden we allemaal rustiger,” ging Daan verder. | elke (1) | each (inflected) | elk | form of: elk, each |  |
| 779 | “Als we elke vergadering met een korte pauze en een wandeling beginnen, worden we allemaal rustiger,” ging Daan verder. | vergadering (4) | meeting | vergadering |  |  |
| 780 | “Als we elke vergadering met een korte pauze en een wandeling beginnen, worden we allemaal rustiger,” ging Daan verder. | met (9) | with | met |  |  |
| 781 | “Als we elke vergadering met een korte pauze en een wandeling beginnen, worden we allemaal rustiger,” ging Daan verder. | een (24) | a / an | een |  |  |
| 782 | “Als we elke vergadering met een korte pauze en een wandeling beginnen, worden we allemaal rustiger,” ging Daan verder. | korte (1) | short (inflected) | kort | form of: kort, short |  |
| 783 | “Als we elke vergadering met een korte pauze en een wandeling beginnen, worden we allemaal rustiger,” ging Daan verder. | pauze (1) | break | pauze |  |  |
| 784 | “Als we elke vergadering met een korte pauze en een wandeling beginnen, worden we allemaal rustiger,” ging Daan verder. | en (18) | and | en |  |  |
| 785 | “Als we elke vergadering met een korte pauze en een wandeling beginnen, worden we allemaal rustiger,” ging Daan verder. | een (25) | a / an | een |  |  |
| 786 | “Als we elke vergadering met een korte pauze en een wandeling beginnen, worden we allemaal rustiger,” ging Daan verder. | wandeling (1) | walk | wandeling |  |  |
| 787 | “Als we elke vergadering met een korte pauze en een wandeling beginnen, worden we allemaal rustiger,” ging Daan verder. | beginnen (2) | to begin | beginnen |  |  |
| 788 | “Als we elke vergadering met een korte pauze en een wandeling beginnen, worden we allemaal rustiger,” ging Daan verder. | worden (1) | to become | worden |  |  |
| 789 | “Als we elke vergadering met een korte pauze en een wandeling beginnen, worden we allemaal rustiger,” ging Daan verder. | we (6) | we | we |  |  |
| 790 | “Als we elke vergadering met een korte pauze en een wandeling beginnen, worden we allemaal rustiger,” ging Daan verder. | allemaal (1) | all | allemaal |  |  |
| 791 | “Als we elke vergadering met een korte pauze en een wandeling beginnen, worden we allemaal rustiger,” ging Daan verder. | rustiger (1) | calmer | rustig | form of: rustig, calm / quiet |  |
| 792 | “Als we elke vergadering met een korte pauze en een wandeling beginnen, worden we allemaal rustiger,” ging Daan verder. | ging (4) | went | gaan | form of: gaan, to go |  |
| 793 | “Als we elke vergadering met een korte pauze en een wandeling beginnen, worden we allemaal rustiger,” ging Daan verder. | Daan (20) | Daan (the applicant's name) | Daan |  |  |
| 794 | “Als we elke vergadering met een korte pauze en een wandeling beginnen, worden we allemaal rustiger,” ging Daan verder. | verder (1) | on | verder |  |  |
| 795 | “Dan kunnen we betere beslissingen nemen. | Dan (2) | then / than | dan |  |  |
| 796 | “Dan kunnen we betere beslissingen nemen. | kunnen (2) | to be able to | kunnen |  |  |
| 797 | “Dan kunnen we betere beslissingen nemen. | we (7) | we | we |  |  |
| 798 | “Dan kunnen we betere beslissingen nemen. | betere (1) | better (inflected) | beter | form of: beter, better |  |
| 799 | “Dan kunnen we betere beslissingen nemen. | beslissingen (3) | decisions | beslissing | form of: beslissing, decision |  |
| 800 | “Dan kunnen we betere beslissingen nemen. | nemen (2) | to take | nemen |  |  |
| 801 | En we hoeven de deadline niet uit te stellen, omdat we efficiënter werken.” Boris stond op en liep naar Daan. | En (19) | and | en |  |  |
| 802 | En we hoeven de deadline niet uit te stellen, omdat we efficiënter werken.” Boris stond op en liep naar Daan. | we (8) | we | we |  |  |
| 803 | En we hoeven de deadline niet uit te stellen, omdat we efficiënter werken.” Boris stond op en liep naar Daan. | hoeven (1) | to need to | hoeven |  |  |
| 804 | En we hoeven de deadline niet uit te stellen, omdat we efficiënter werken.” Boris stond op en liep naar Daan. | de (21) | the | de |  |  |
| 805 | En we hoeven de deadline niet uit te stellen, omdat we efficiënter werken.” Boris stond op en liep naar Daan. | deadline (3) | deadline | deadline |  |  |
| 806 | En we hoeven de deadline niet uit te stellen, omdat we efficiënter werken.” Boris stond op en liep naar Daan. | niet (17) | not | niet |  |  |
| 807 | En we hoeven de deadline niet uit te stellen, omdat we efficiënter werken.” Boris stond op en liep naar Daan. | uit (4) | postpone (in 'uitstellen') | uit |  |  |
| 808 | En we hoeven de deadline niet uit te stellen, omdat we efficiënter werken.” Boris stond op en liep naar Daan. | te (6) | to | te |  |  |
| 809 | En we hoeven de deadline niet uit te stellen, omdat we efficiënter werken.” Boris stond op en liep naar Daan. | stellen (1) | to postpone (separable verb) | uitstellen | form of: uitstellen, to postpone |  |
| 810 | En we hoeven de deadline niet uit te stellen, omdat we efficiënter werken.” Boris stond op en liep naar Daan. | omdat (1) | because | omdat |  |  |
| 811 | En we hoeven de deadline niet uit te stellen, omdat we efficiënter werken.” Boris stond op en liep naar Daan. | we (9) | we | we |  |  |
| 812 | En we hoeven de deadline niet uit te stellen, omdat we efficiënter werken.” Boris stond op en liep naar Daan. | efficiënter (1) | more efficient | efficiënt | form of: efficiënt, efficient |  |
| 813 | En we hoeven de deadline niet uit te stellen, omdat we efficiënter werken.” Boris stond op en liep naar Daan. | werken (2) | to work | werken |  |  |
| 814 | En we hoeven de deadline niet uit te stellen, omdat we efficiënter werken.” Boris stond op en liep naar Daan. | Boris (21) | Boris (the dog's name) | Boris |  |  |
| 815 | En we hoeven de deadline niet uit te stellen, omdat we efficiënter werken.” Boris stond op en liep naar Daan. | stond (6) | stood | staan | form of: staan, to stand |  |
| 816 | En we hoeven de deadline niet uit te stellen, omdat we efficiënter werken.” Boris stond op en liep naar Daan. | op (11) | on | op |  |  |
| 817 | En we hoeven de deadline niet uit te stellen, omdat we efficiënter werken.” Boris stond op en liep naar Daan. | en (20) | and | en |  |  |
| 818 | En we hoeven de deadline niet uit te stellen, omdat we efficiënter werken.” Boris stond op en liep naar Daan. | liep (3) | walked | lopen | form of: lopen, to walk |  |
| 819 | En we hoeven de deadline niet uit te stellen, omdat we efficiënter werken.” Boris stond op en liep naar Daan. | naar (3) | to | naar |  |  |
| 820 | En we hoeven de deadline niet uit te stellen, omdat we efficiënter werken.” Boris stond op en liep naar Daan. | Daan (21) | Daan (the applicant's name) | Daan |  |  |
| 821 | Hij snuffelde aan zijn hand en likte hem. | Hij (35) | he | hij |  |  |
| 822 | Hij snuffelde aan zijn hand en likte hem. | snuffelde (1) | sniffed | snuffelen | form of: snuffelen, to sniff |  |
| 823 | Hij snuffelde aan zijn hand en likte hem. | aan (5) | at | aan |  |  |
| 824 | Hij snuffelde aan zijn hand en likte hem. | zijn (15) | his | zijn |  |  |
| 825 | Hij snuffelde aan zijn hand en likte hem. | hand (1) | hand | hand |  |  |
| 826 | Hij snuffelde aan zijn hand en likte hem. | en (21) | and | en |  |  |
| 827 | Hij snuffelde aan zijn hand en likte hem. | likte (1) | licked | likken | form of: likken, to lick |  |
| 828 | Hij snuffelde aan zijn hand en likte hem. | hem (6) | him | hem |  |  |
| 829 | Mevrouw Van Dijk lachte. | Mevrouw (13) | lady / Mrs | mevrouw |  |  |
| 830 | Mevrouw Van Dijk lachte. | Van (21) | of / from | van |  |  |
| 831 | Mevrouw Van Dijk lachte. | Dijk (14) | Dijk (surname) | Dijk |  |  |
| 832 | Mevrouw Van Dijk lachte. | lachte (1) | laughed | lachen | form of: lachen, to laugh |  |
| 833 | “Ik geloof dat u een indruk hebt gemaakt,” zei ze. | Ik (17) | I | ik |  |  |
| 834 | “Ik geloof dat u een indruk hebt gemaakt,” zei ze. | geloof (1) | believe | geloven | form of: geloven, to believe |  |
| 835 | “Ik geloof dat u een indruk hebt gemaakt,” zei ze. | dat (12) | that | dat |  |  |
| 836 | “Ik geloof dat u een indruk hebt gemaakt,” zei ze. | u (15) | you (formal) | u |  |  |
| 837 | “Ik geloof dat u een indruk hebt gemaakt,” zei ze. | een (26) | a / an | een |  |  |
| 838 | “Ik geloof dat u een indruk hebt gemaakt,” zei ze. | indruk (3) | impression | indruk |  |  |
| 839 | “Ik geloof dat u een indruk hebt gemaakt,” zei ze. | hebt (1) | have | hebben | form of: hebben, to have |  |
| 840 | “Ik geloof dat u een indruk hebt gemaakt,” zei ze. | gemaakt (1) | made | maken | form of: maken, to make |  |
| 841 | “Ik geloof dat u een indruk hebt gemaakt,” zei ze. | zei (16) | said | zeggen | form of: zeggen, to say |  |
| 842 | “Ik geloof dat u een indruk hebt gemaakt,” zei ze. | ze (11) | she | ze |  |  |
| 843 | “En u hebt bewezen dat u niet alleen praat.” | En (22) | and | en |  |  |
| 844 | “En u hebt bewezen dat u niet alleen praat.” | u (16) | you (formal) | u |  |  |
| 845 | “En u hebt bewezen dat u niet alleen praat.” | hebt (2) | have | hebben | form of: hebben, to have |  |
| 846 | “En u hebt bewezen dat u niet alleen praat.” | bewezen (1) | proved | bewijzen | form of: bewijzen, to prove |  |
| 847 | “En u hebt bewezen dat u niet alleen praat.” | dat (13) | that | dat |  |  |
| 848 | “En u hebt bewezen dat u niet alleen praat.” | u (17) | you (formal) | u |  |  |
| 849 | “En u hebt bewezen dat u niet alleen praat.” | niet (18) | not | niet |  |  |
| 850 | “En u hebt bewezen dat u niet alleen praat.” | alleen (4) | only | alleen |  |  |
| 851 | “En u hebt bewezen dat u niet alleen praat.” | praat (1) | talk | praten | form of: praten, to talk |  |
| 852 | Twee weken later zat Daan op zijn eerste werkdag achter een bureau. | Twee (1) | two | twee |  |  |
| 853 | Twee weken later zat Daan op zijn eerste werkdag achter een bureau. | weken (1) | weeks | week | form of: week, week |  |
| 854 | Twee weken later zat Daan op zijn eerste werkdag achter een bureau. | later (1) | later | later |  |  |
| 855 | Twee weken later zat Daan op zijn eerste werkdag achter een bureau. | zat (2) | sat | zitten | form of: zitten, to sit |  |
| 856 | Twee weken later zat Daan op zijn eerste werkdag achter een bureau. | Daan (22) | Daan (the applicant's name) | Daan |  |  |
| 857 | Twee weken later zat Daan op zijn eerste werkdag achter een bureau. | op (12) | on | op |  |  |
| 858 | Twee weken later zat Daan op zijn eerste werkdag achter een bureau. | zijn (16) | his | zijn |  |  |
| 859 | Twee weken later zat Daan op zijn eerste werkdag achter een bureau. | eerste (2) | first | eerste |  |  |
| 860 | Twee weken later zat Daan op zijn eerste werkdag achter een bureau. | werkdag (1) | workday | werkdag |  |  |
| 861 | Twee weken later zat Daan op zijn eerste werkdag achter een bureau. | achter (3) | behind | achter |  |  |
| 862 | Twee weken later zat Daan op zijn eerste werkdag achter een bureau. | een (27) | a / an | een |  |  |
| 863 | Twee weken later zat Daan op zijn eerste werkdag achter een bureau. | bureau (1) | desk | bureau |  |  |
| 864 | Zijn sollicitatie was de vreemdste ooit geweest, maar hij had de baan gekregen. | Zijn (17) | his | zijn |  |  |
| 865 | Zijn sollicitatie was de vreemdste ooit geweest, maar hij had de baan gekregen. | sollicitatie (2) | job application / interview | sollicitatie |  |  |
| 866 | Zijn sollicitatie was de vreemdste ooit geweest, maar hij had de baan gekregen. | was (6) | was | zijn | form of: zijn, to be |  |
| 867 | Zijn sollicitatie was de vreemdste ooit geweest, maar hij had de baan gekregen. | de (22) | the | de |  |  |
| 868 | Zijn sollicitatie was de vreemdste ooit geweest, maar hij had de baan gekregen. | vreemdste (1) | strangest | vreemd | form of: vreemd, strange |  |
| 869 | Zijn sollicitatie was de vreemdste ooit geweest, maar hij had de baan gekregen. | ooit (2) | ever | ooit |  |  |
| 870 | Zijn sollicitatie was de vreemdste ooit geweest, maar hij had de baan gekregen. | geweest (2) | been | zijn | form of: zijn, to be |  |
| 871 | Zijn sollicitatie was de vreemdste ooit geweest, maar hij had de baan gekregen. | maar (11) | but | maar |  |  |
| 872 | Zijn sollicitatie was de vreemdste ooit geweest, maar hij had de baan gekregen. | hij (36) | he | hij |  |  |
| 873 | Zijn sollicitatie was de vreemdste ooit geweest, maar hij had de baan gekregen. | had (8) | had | hebben | form of: hebben, to have |  |
| 874 | Zijn sollicitatie was de vreemdste ooit geweest, maar hij had de baan gekregen. | de (23) | the | de |  |  |
| 875 | Zijn sollicitatie was de vreemdste ooit geweest, maar hij had de baan gekregen. | baan (3) | job | baan |  |  |
| 876 | Zijn sollicitatie was de vreemdste ooit geweest, maar hij had de baan gekregen. | gekregen (1) | received | krijgen | form of: krijgen, to get |  |
| 877 | Op de deur hing een nieuw bordje: “Vergadering om negen uur. | Op (13) | on | op |  |  |
| 878 | Op de deur hing een nieuw bordje: “Vergadering om negen uur. | de (24) | the | de |  |  |
| 879 | Op de deur hing een nieuw bordje: “Vergadering om negen uur. | deur (3) | door | deur |  |  |
| 880 | Op de deur hing een nieuw bordje: “Vergadering om negen uur. | hing (1) | hung | hangen | form of: hangen, to hang |  |
| 881 | Op de deur hing een nieuw bordje: “Vergadering om negen uur. | een (28) | a / an | een |  |  |
| 882 | Op de deur hing een nieuw bordje: “Vergadering om negen uur. | nieuw (1) | new | nieuw |  |  |
| 883 | Op de deur hing een nieuw bordje: “Vergadering om negen uur. | bordje (2) | sign | bord | form of: bord, sign / plate |  |
| 884 | Op de deur hing een nieuw bordje: “Vergadering om negen uur. | Vergadering (5) | meeting | vergadering |  |  |
| 885 | Op de deur hing een nieuw bordje: “Vergadering om negen uur. | om (5) | at | om |  |  |
| 886 | Op de deur hing een nieuw bordje: “Vergadering om negen uur. | negen (3) | nine | negen |  |  |
| 887 | Op de deur hing een nieuw bordje: “Vergadering om negen uur. | uur (3) | hour | uur |  |  |
| 888 | Wandeling eerst.” | Wandeling (2) | walk | wandeling |  |  |
| 889 | Wandeling eerst.” | eerst (2) | first | eerst |  |  |
| 890 | Boris lag onder het bureau te slapen. | Boris (22) | Boris (the dog's name) | Boris |  |  |
| 891 | Boris lag onder het bureau te slapen. | lag (1) | lay | liggen | form of: liggen, to lie |  |
| 892 | Boris lag onder het bureau te slapen. | onder (2) | under | onder |  |  |
| 893 | Boris lag onder het bureau te slapen. | het (12) | the / it | het |  |  |
| 894 | Boris lag onder het bureau te slapen. | bureau (2) | desk | bureau |  |  |
| 895 | Boris lag onder het bureau te slapen. | te (7) | to | te |  |  |
| 896 | Boris lag onder het bureau te slapen. | slapen (2) | to sleep | slapen |  |  |
| 897 | Mevrouw Van Dijk bracht een contract en een kop koffie. | Mevrouw (14) | lady / Mrs | mevrouw |  |  |
| 898 | Mevrouw Van Dijk bracht een contract en een kop koffie. | Van (22) | of / from | van |  |  |
| 899 | Mevrouw Van Dijk bracht een contract en een kop koffie. | Dijk (15) | Dijk (surname) | Dijk |  |  |
| 900 | Mevrouw Van Dijk bracht een contract en een kop koffie. | bracht (1) | brought | brengen | form of: brengen, to bring |  |
| 901 | Mevrouw Van Dijk bracht een contract en een kop koffie. | een (29) | a / an | een |  |  |
| 902 | Mevrouw Van Dijk bracht een contract en een kop koffie. | contract (2) | contract | contract |  |  |
| 903 | Mevrouw Van Dijk bracht een contract en een kop koffie. | en (23) | and | en |  |  |
| 904 | Mevrouw Van Dijk bracht een contract en een kop koffie. | een (30) | a / an | een |  |  |
| 905 | Mevrouw Van Dijk bracht een contract en een kop koffie. | kop (3) | head | kop |  |  |
| 906 | Mevrouw Van Dijk bracht een contract en een kop koffie. | koffie (1) | coffee | koffie |  |  |
| 907 | “Welkom bij het team,” zei ze. | Welkom (1) | welcome | welkom |  |  |
| 908 | “Welkom bij het team,” zei ze. | bij (4) | at / with | bij |  |  |
| 909 | “Welkom bij het team,” zei ze. | het (13) | the / it | het |  |  |
| 910 | “Welkom bij het team,” zei ze. | team (1) | team | team |  |  |
| 911 | “Welkom bij het team,” zei ze. | zei (17) | said | zeggen | form of: zeggen, to say |  |
| 912 | “Welkom bij het team,” zei ze. | ze (12) | she | ze |  |  |
| 913 | “Uw salaris is niet hoog, maar de pauzes zijn lang.” Daan was niet meer zenuwachtig. | Uw (2) | your (formal) | uw |  |  |
| 914 | “Uw salaris is niet hoog, maar de pauzes zijn lang.” Daan was niet meer zenuwachtig. | salaris (2) | salary | salaris |  |  |
| 915 | “Uw salaris is niet hoog, maar de pauzes zijn lang.” Daan was niet meer zenuwachtig. | is (12) | is | zijn | form of: zijn, to be |  |
| 916 | “Uw salaris is niet hoog, maar de pauzes zijn lang.” Daan was niet meer zenuwachtig. | niet (19) | not | niet |  |  |
| 917 | “Uw salaris is niet hoog, maar de pauzes zijn lang.” Daan was niet meer zenuwachtig. | hoog (2) | high | hoog |  |  |
| 918 | “Uw salaris is niet hoog, maar de pauzes zijn lang.” Daan was niet meer zenuwachtig. | maar (12) | but | maar |  |  |
| 919 | “Uw salaris is niet hoog, maar de pauzes zijn lang.” Daan was niet meer zenuwachtig. | de (25) | the | de |  |  |
| 920 | “Uw salaris is niet hoog, maar de pauzes zijn lang.” Daan was niet meer zenuwachtig. | pauzes (2) | breaks | pauze | form of: pauze, break |  |
| 921 | “Uw salaris is niet hoog, maar de pauzes zijn lang.” Daan was niet meer zenuwachtig. | zijn (18) | are | zijn |  |  |
| 922 | “Uw salaris is niet hoog, maar de pauzes zijn lang.” Daan was niet meer zenuwachtig. | lang (2) | long | lang |  |  |
| 923 | “Uw salaris is niet hoog, maar de pauzes zijn lang.” Daan was niet meer zenuwachtig. | Daan (23) | Daan (the applicant's name) | Daan |  |  |
| 924 | “Uw salaris is niet hoog, maar de pauzes zijn lang.” Daan was niet meer zenuwachtig. | was (7) | was | zijn | form of: zijn, to be |  |
| 925 | “Uw salaris is niet hoog, maar de pauzes zijn lang.” Daan was niet meer zenuwachtig. | niet (20) | not | niet |  |  |
| 926 | “Uw salaris is niet hoog, maar de pauzes zijn lang.” Daan was niet meer zenuwachtig. | meer (2) | more | meer |  |  |
| 927 | “Uw salaris is niet hoog, maar de pauzes zijn lang.” Daan was niet meer zenuwachtig. | zenuwachtig (2) | nervous | zenuwachtig |  |  |
| 928 | Hij voelde zich hier al bijna thuis. | Hij (37) | he | hij |  |  |
| 929 | Hij voelde zich hier al bijna thuis. | voelde (2) | felt | voelen | form of: voelen, to feel |  |
| 930 | Hij voelde zich hier al bijna thuis. | zich (1) | himself / herself | zich |  |  |
| 931 | Hij voelde zich hier al bijna thuis. | hier (3) | here | hier |  |  |
| 932 | Hij voelde zich hier al bijna thuis. | al (1) | already | al |  |  |
| 933 | Hij voelde zich hier al bijna thuis. | bijna (2) | almost | bijna |  |  |
| 934 | Hij voelde zich hier al bijna thuis. | thuis (1) | at home | thuis |  |  |
| 935 | Zijn nieuwe collega's waren aardig, ook al vonden ze hem een beetje vreemd. | Zijn (19) | his | zijn |  |  |
| 936 | Zijn nieuwe collega's waren aardig, ook al vonden ze hem een beetje vreemd. | nieuwe (1) | new (inflected) | nieuw | form of: nieuw, new |  |
| 937 | Zijn nieuwe collega's waren aardig, ook al vonden ze hem een beetje vreemd. | collega's (2) | colleagues | collega | form of: collega, colleague |  |
| 938 | Zijn nieuwe collega's waren aardig, ook al vonden ze hem een beetje vreemd. | waren (1) | were | zijn | form of: zijn, to be |  |
| 939 | Zijn nieuwe collega's waren aardig, ook al vonden ze hem een beetje vreemd. | aardig (1) | nice | aardig |  |  |
| 940 | Zijn nieuwe collega's waren aardig, ook al vonden ze hem een beetje vreemd. | ook (3) | also | ook |  |  |
| 941 | Zijn nieuwe collega's waren aardig, ook al vonden ze hem een beetje vreemd. | al (2) | already | al |  |  |
| 942 | Zijn nieuwe collega's waren aardig, ook al vonden ze hem een beetje vreemd. | vonden (1) | found | vinden | form of: vinden, to find |  |
| 943 | Zijn nieuwe collega's waren aardig, ook al vonden ze hem een beetje vreemd. | ze (13) | they | ze |  |  |
| 944 | Zijn nieuwe collega's waren aardig, ook al vonden ze hem een beetje vreemd. | hem (7) | him | hem |  |  |
| 945 | Zijn nieuwe collega's waren aardig, ook al vonden ze hem een beetje vreemd. | een (31) | a / an | een |  |  |
| 946 | Zijn nieuwe collega's waren aardig, ook al vonden ze hem een beetje vreemd. | beetje (1) | a little | beetje |  |  |
| 947 | Zijn nieuwe collega's waren aardig, ook al vonden ze hem een beetje vreemd. | vreemd (1) | strange | vreemd |  |  |
| 948 | Mevrouw Van Dijk gaf hem veel verantwoordelijkheid, en hij nam die serieus. | Mevrouw (15) | lady / Mrs | mevrouw |  |  |
| 949 | Mevrouw Van Dijk gaf hem veel verantwoordelijkheid, en hij nam die serieus. | Van (23) | of / from | van |  |  |
| 950 | Mevrouw Van Dijk gaf hem veel verantwoordelijkheid, en hij nam die serieus. | Dijk (16) | Dijk (surname) | Dijk |  |  |
| 951 | Mevrouw Van Dijk gaf hem veel verantwoordelijkheid, en hij nam die serieus. | gaf (1) | gave | geven | form of: geven, to give |  |
| 952 | Mevrouw Van Dijk gaf hem veel verantwoordelijkheid, en hij nam die serieus. | hem (8) | him | hem |  |  |
| 953 | Mevrouw Van Dijk gaf hem veel verantwoordelijkheid, en hij nam die serieus. | veel (2) | much / a lot | veel |  |  |
| 954 | Mevrouw Van Dijk gaf hem veel verantwoordelijkheid, en hij nam die serieus. | verantwoordelijkheid (2) | responsibility | verantwoordelijkheid |  |  |
| 955 | Mevrouw Van Dijk gaf hem veel verantwoordelijkheid, en hij nam die serieus. | en (24) | and | en |  |  |
| 956 | Mevrouw Van Dijk gaf hem veel verantwoordelijkheid, en hij nam die serieus. | hij (38) | he | hij |  |  |
| 957 | Mevrouw Van Dijk gaf hem veel verantwoordelijkheid, en hij nam die serieus. | nam (1) | took | nemen | form of: nemen, to take |  |
| 958 | Mevrouw Van Dijk gaf hem veel verantwoordelijkheid, en hij nam die serieus. | die (3) | it | die |  |  |
| 959 | Mevrouw Van Dijk gaf hem veel verantwoordelijkheid, en hij nam die serieus. | serieus (1) | serious | serieus |  |  |
| 960 | Hij had de baan gekregen, niet omdat hij de beste cv had, maar omdat hij de baas had overtuigd met een simpel voorstel. | Hij (39) | he | hij |  |  |
| 961 | Hij had de baan gekregen, niet omdat hij de beste cv had, maar omdat hij de baas had overtuigd met een simpel voorstel. | had (9) | had | hebben | form of: hebben, to have |  |
| 962 | Hij had de baan gekregen, niet omdat hij de beste cv had, maar omdat hij de baas had overtuigd met een simpel voorstel. | de (26) | the | de |  |  |
| 963 | Hij had de baan gekregen, niet omdat hij de beste cv had, maar omdat hij de baas had overtuigd met een simpel voorstel. | baan (4) | job | baan |  |  |
| 964 | Hij had de baan gekregen, niet omdat hij de beste cv had, maar omdat hij de baas had overtuigd met een simpel voorstel. | gekregen (2) | received | krijgen | form of: krijgen, to get |  |
| 965 | Hij had de baan gekregen, niet omdat hij de beste cv had, maar omdat hij de baas had overtuigd met een simpel voorstel. | niet (21) | not | niet |  |  |
| 966 | Hij had de baan gekregen, niet omdat hij de beste cv had, maar omdat hij de baas had overtuigd met een simpel voorstel. | omdat (2) | because | omdat |  |  |
| 967 | Hij had de baan gekregen, niet omdat hij de beste cv had, maar omdat hij de baas had overtuigd met een simpel voorstel. | hij (40) | he | hij |  |  |
| 968 | Hij had de baan gekregen, niet omdat hij de beste cv had, maar omdat hij de baas had overtuigd met een simpel voorstel. | de (27) | the | de |  |  |
| 969 | Hij had de baan gekregen, niet omdat hij de beste cv had, maar omdat hij de baas had overtuigd met een simpel voorstel. | beste (2) | best | beter | form of: beter, better |  |
| 970 | Hij had de baan gekregen, niet omdat hij de beste cv had, maar omdat hij de baas had overtuigd met een simpel voorstel. | cv (3) | CV | cv |  |  |
| 971 | Hij had de baan gekregen, niet omdat hij de beste cv had, maar omdat hij de baas had overtuigd met een simpel voorstel. | had (10) | had | hebben | form of: hebben, to have |  |
| 972 | Hij had de baan gekregen, niet omdat hij de beste cv had, maar omdat hij de baas had overtuigd met een simpel voorstel. | maar (13) | but | maar |  |  |
| 973 | Hij had de baan gekregen, niet omdat hij de beste cv had, maar omdat hij de baas had overtuigd met een simpel voorstel. | omdat (3) | because | omdat |  |  |
| 974 | Hij had de baan gekregen, niet omdat hij de beste cv had, maar omdat hij de baas had overtuigd met een simpel voorstel. | hij (41) | he | hij |  |  |
| 975 | Hij had de baan gekregen, niet omdat hij de beste cv had, maar omdat hij de baas had overtuigd met een simpel voorstel. | de (28) | the | de |  |  |
| 976 | Hij had de baan gekregen, niet omdat hij de beste cv had, maar omdat hij de baas had overtuigd met een simpel voorstel. | baas (4) | boss | baas |  |  |
| 977 | Hij had de baan gekregen, niet omdat hij de beste cv had, maar omdat hij de baas had overtuigd met een simpel voorstel. | had (11) | had | hebben | form of: hebben, to have |  |
| 978 | Hij had de baan gekregen, niet omdat hij de beste cv had, maar omdat hij de baas had overtuigd met een simpel voorstel. | overtuigd (1) | convinced | overtuigen | form of: overtuigen, to convince |  |
| 979 | Hij had de baan gekregen, niet omdat hij de beste cv had, maar omdat hij de baas had overtuigd met een simpel voorstel. | met (10) | with | met |  |  |
| 980 | Hij had de baan gekregen, niet omdat hij de beste cv had, maar omdat hij de baas had overtuigd met een simpel voorstel. | een (32) | a / an | een |  |  |
| 981 | Hij had de baan gekregen, niet omdat hij de beste cv had, maar omdat hij de baas had overtuigd met een simpel voorstel. | simpel (1) | simple | simpel |  |  |
| 982 | Hij had de baan gekregen, niet omdat hij de beste cv had, maar omdat hij de baas had overtuigd met een simpel voorstel. | voorstel (3) | proposal | voorstel |  |  |
| 983 | Af en toe twijfelde hij nog of dit allemaal echt was. | Af (1) | now and then | af en toe |  |  |
| 984 | Af en toe twijfelde hij nog of dit allemaal echt was. | en (25) | and | en |  |  |
| 985 | Af en toe twijfelde hij nog of dit allemaal echt was. | toe (1) | now and then | af en toe |  |  |
| 986 | Af en toe twijfelde hij nog of dit allemaal echt was. | twijfelde (3) | doubted | twijfelen | form of: twijfelen, to doubt |  |
| 987 | Af en toe twijfelde hij nog of dit allemaal echt was. | hij (42) | he | hij |  |  |
| 988 | Af en toe twijfelde hij nog of dit allemaal echt was. | nog (2) | still / yet | nog |  |  |
| 989 | Af en toe twijfelde hij nog of dit allemaal echt was. | of (3) | whether | of |  |  |
| 990 | Af en toe twijfelde hij nog of dit allemaal echt was. | dit (6) | this | dit |  |  |
| 991 | Af en toe twijfelde hij nog of dit allemaal echt was. | allemaal (2) | all | allemaal |  |  |
| 992 | Af en toe twijfelde hij nog of dit allemaal echt was. | echt (2) | really | echt |  |  |
| 993 | Af en toe twijfelde hij nog of dit allemaal echt was. | was (8) | was | zijn | form of: zijn, to be |  |
| 994 | Maar als Boris zijn kop op zijn knie legde, wist hij het zeker. | Maar (14) | but | maar |  |  |
| 995 | Maar als Boris zijn kop op zijn knie legde, wist hij het zeker. | als (5) | when | als |  |  |
| 996 | Maar als Boris zijn kop op zijn knie legde, wist hij het zeker. | Boris (23) | Boris (the dog's name) | Boris |  |  |
| 997 | Maar als Boris zijn kop op zijn knie legde, wist hij het zeker. | zijn (20) | his | zijn |  |  |
| 998 | Maar als Boris zijn kop op zijn knie legde, wist hij het zeker. | kop (4) | head | kop |  |  |
| 999 | Maar als Boris zijn kop op zijn knie legde, wist hij het zeker. | op (14) | on | op |  |  |
| 1000 | Maar als Boris zijn kop op zijn knie legde, wist hij het zeker. | zijn (21) | his | zijn |  |  |
| 1001 | Maar als Boris zijn kop op zijn knie legde, wist hij het zeker. | knie (1) | knee | knie |  |  |
| 1002 | Maar als Boris zijn kop op zijn knie legde, wist hij het zeker. | legde (2) | put / laid | leggen | form of: leggen, to put / lay |  |
| 1003 | Maar als Boris zijn kop op zijn knie legde, wist hij het zeker. | wist (3) | knew | weten | form of: weten, to know |  |
| 1004 | Maar als Boris zijn kop op zijn knie legde, wist hij het zeker. | hij (43) | he | hij |  |  |
| 1005 | Maar als Boris zijn kop op zijn knie legde, wist hij het zeker. | het (14) | the / it | het |  |  |
| 1006 | Maar als Boris zijn kop op zijn knie legde, wist hij het zeker. | zeker (1) | certain / sure | zeker |  |  |
| 1007 | Dit was een vreemd bedrijf, maar het was wel zijn bedrijf. | Dit (7) | this | dit |  |  |
| 1008 | Dit was een vreemd bedrijf, maar het was wel zijn bedrijf. | was (9) | was | zijn | form of: zijn, to be |  |
| 1009 | Dit was een vreemd bedrijf, maar het was wel zijn bedrijf. | een (33) | a / an | een |  |  |
| 1010 | Dit was een vreemd bedrijf, maar het was wel zijn bedrijf. | vreemd (2) | strange | vreemd |  |  |
| 1011 | Dit was een vreemd bedrijf, maar het was wel zijn bedrijf. | bedrijf (3) | company | bedrijf |  |  |
| 1012 | Dit was een vreemd bedrijf, maar het was wel zijn bedrijf. | maar (15) | but | maar |  |  |
| 1013 | Dit was een vreemd bedrijf, maar het was wel zijn bedrijf. | het (15) | the / it | het |  |  |
| 1014 | Dit was een vreemd bedrijf, maar het was wel zijn bedrijf. | was (10) | was | zijn | form of: zijn, to be |  |
| 1015 | Dit was een vreemd bedrijf, maar het was wel zijn bedrijf. | wel (5) | indeed / well | wel |  |  |
| 1016 | Dit was een vreemd bedrijf, maar het was wel zijn bedrijf. | zijn (22) | his | zijn |  |  |
| 1017 | Dit was een vreemd bedrijf, maar het was wel zijn bedrijf. | bedrijf (4) | company | bedrijf |  |  |
| 1018 | En de eerste vergadering van de dag? | En (26) | and | en |  |  |
| 1019 | En de eerste vergadering van de dag? | de (29) | the | de |  |  |
| 1020 | En de eerste vergadering van de dag? | eerste (3) | first | eerste |  |  |
| 1021 | En de eerste vergadering van de dag? | vergadering (6) | meeting | vergadering |  |  |
| 1022 | En de eerste vergadering van de dag? | van (24) | of / from | van |  |  |
| 1023 | En de eerste vergadering van de dag? | de (30) | the | de |  |  |
| 1024 | En de eerste vergadering van de dag? | dag (1) | day | dag |  |  |
| 1025 | Die werd, zoals altijd, uitgesteld. | Die (4) | that one | die |  |  |
| 1026 | Die werd, zoals altijd, uitgesteld. | werd (1) | was | worden | form of: worden, to become |  |
| 1027 | Die werd, zoals altijd, uitgesteld. | zoals (2) | as | zoals |  |  |
| 1028 | Die werd, zoals altijd, uitgesteld. | altijd (1) | always | altijd |  |  |
| 1029 | Die werd, zoals altijd, uitgesteld. | uitgesteld (1) | postponed | uitstellen | form of: uitstellen, to postpone |  |
| 1030 | Boris had namelijk nog een dutje nodig. | Boris (24) | Boris (the dog's name) | Boris |  |  |
| 1031 | Boris had namelijk nog een dutje nodig. | had (12) | had | hebben | form of: hebben, to have |  |
| 1032 | Boris had namelijk nog een dutje nodig. | namelijk (1) | namely | namelijk |  |  |
| 1033 | Boris had namelijk nog een dutje nodig. | nog (3) | still / yet | nog |  |  |
| 1034 | Boris had namelijk nog een dutje nodig. | een (34) | a / an | een |  |  |
| 1035 | Boris had namelijk nog een dutje nodig. | dutje (1) | a little nap | dut | form of: dut, nap |  |
| 1036 | Boris had namelijk nog een dutje nodig. | nodig (3) | necessary | nodig |  |  |
| 1037 | Over één ding was geen discussie: de hond bepaalde het tempo. | Over (4) | over / about | over |  |  |
| 1038 | Over één ding was geen discussie: de hond bepaalde het tempo. | één (4) | one | één |  |  |
| 1039 | Over één ding was geen discussie: de hond bepaalde het tempo. | ding (1) | thing | ding |  |  |
| 1040 | Over één ding was geen discussie: de hond bepaalde het tempo. | was (11) | was | zijn | form of: zijn, to be |  |
| 1041 | Over één ding was geen discussie: de hond bepaalde het tempo. | geen (5) | no / not a | geen |  |  |
| 1042 | Over één ding was geen discussie: de hond bepaalde het tempo. | discussie (2) | discussion | discussie |  |  |
| 1043 | Over één ding was geen discussie: de hond bepaalde het tempo. | de (31) | the | de |  |  |
| 1044 | Over één ding was geen discussie: de hond bepaalde het tempo. | hond (8) | dog | hond |  |  |
| 1045 | Over één ding was geen discussie: de hond bepaalde het tempo. | bepaalde (1) | determined | bepalen | form of: bepalen, to determine |  |
| 1046 | Over één ding was geen discussie: de hond bepaalde het tempo. | het (16) | the / it | het |  |  |
| 1047 | Over één ding was geen discussie: de hond bepaalde het tempo. | tempo (1) | pace | tempo |  |  |
| 1048 | Daan hoefde niets meer te bewijzen. | Daan (24) | Daan (the applicant's name) | Daan |  |  |
| 1049 | Daan hoefde niets meer te bewijzen. | hoefde (1) | needed to | hoeven | form of: hoeven, to need to |  |
| 1050 | Daan hoefde niets meer te bewijzen. | niets (1) | nothing | niets |  |  |
| 1051 | Daan hoefde niets meer te bewijzen. | meer (3) | more | meer |  |  |
| 1052 | Daan hoefde niets meer te bewijzen. | te (8) | to | te |  |  |
| 1053 | Daan hoefde niets meer te bewijzen. | bewijzen (1) | to prove | bewijzen |  |  |

## Changed entries

Every entry that differs from the pre-audit glossary. `meaning` changes are shown as `before → after`; added and removed entries are listed in full.

| Story | Form | Occ | POS | Change | Before | After | Why |
| --- | --- | ---: | --- | --- | --- | --- | --- |
| a1-01 | ga | None | verb | removed | to go / goes | — | not a surface form in the story |
| a1-01 | loop | None | verb | removed | to walk / walks | — | not a surface form in the story |
| a1-01 | kijk | None | verb | removed | to look / looks | — | not a surface form in the story |
| a1-01 | kom | None | verb | removed | to come / comes | — | not a surface form in the story |
| a1-01 | pak | None | verb | removed | to take / grab / grabs | — | not a surface form in the story |
| a1-01 | geef | None | verb | removed | to give / gives | — | not a surface form in the story |
| a1-01 | zie | None | verb | removed | to see / sees | — | not a surface form in the story |
| a1-01 | zijn | 1 | verb | added | — | are | form or lemma was missing |
| a1-01 | zijn | 2 | verb | added | — | are | form or lemma was missing |
| a1-01 | zijn | 3 | verb | added | — | are | form or lemma was missing |
| a1-01 | zijn | 4 | verb | added | — | are | form or lemma was missing |
| a1-01 | zijn | 5 | verb | added | — | are | form or lemma was missing |
| a1-01 | zijn | 6 | verb | added | — | are | form or lemma was missing |
| a1-01 | zijn | 8 | verb | added | — | are | form or lemma was missing |
| a1-01 | zijn | 9 | verb | added | — | are | form or lemma was missing |
| a1-01 | zijn | 10 | verb | added | — | are | form or lemma was missing |
| a1-01 | zijn | 11 | verb | added | — | are | form or lemma was missing |
| a1-01 | lopen | None | verb | added | — | to walk | form or lemma was missing |
| a1-01 | komen | None | verb | added | — | to come | form or lemma was missing |
| a1-01 | pakken | None | verb | added | — | to take / grab | form or lemma was missing |
| a1-01 | geven | None | verb | added | — | to give | form or lemma was missing |
| a1-01 | aan | None | preposition | added | — | to | form or lemma was missing |
| a1-01 | gaat | None | verb | changed | to go / goes | goes | meaning now matches this surface form / context |
| a1-01 | gaan | None | verb | changed | to go / goes | to go | meaning now matches this surface form / context |
| a1-01 | loopt | None | verb | changed | to walk / walks | walks | meaning now matches this surface form / context |
| a1-01 | kijkt | None | verb | changed | to look / looks | looks | meaning now matches this surface form / context |
| a1-01 | kijken | None | verb | changed | to look / looks | to look | meaning now matches this surface form / context |
| a1-01 | komt | None | verb | changed | to come / comes | comes | meaning now matches this surface form / context |
| a1-01 | pakt | None | verb | changed | to take / grab / grabs | takes | meaning now matches this surface form / context |
| a1-01 | geeft | None | verb | changed | to give / gives | gives | meaning now matches this surface form / context |
| a1-01 | wil | None | verb | changed | to want / wants | wants | meaning now matches this surface form / context |
| a1-01 | willen | None | verb | changed | to want / wants | to want | meaning now matches this surface form / context |
| a1-01 | ziet | None | verb | changed | to see / sees | sees | meaning now matches this surface form / context |
| a1-01 | zien | None | verb | changed | to see / sees | to see | meaning now matches this surface form / context |
| a1-01 | spelen | None | verb | changed | to play / plays | to play | meaning now matches this surface form / context |
| a2-01 | Dan | None | adverb | removed | then / than | — | not a surface form in the story |
| a2-01 | Het | 1 | article | added | — | it | form or lemma was missing |
| a2-01 | aan | 2 | preposition | added | — | after | form or lemma was missing |
| a2-01 | aan | 3 | preposition | added | — | after | form or lemma was missing |
| a2-01 | willen | None | verb | added | — | to want | form or lemma was missing |
| a2-01 | zoeken | None | verb | added | — | to look for | form or lemma was missing |
| a2-01 | vragen | None | verb | added | — | to ask | form or lemma was missing |
| a2-01 | zitten | None | verb | added | — | to sit | form or lemma was missing |
| a2-01 | kijken | None | verb | added | — | to look | form or lemma was missing |
| a2-01 | lachen | None | verb | added | — | to laugh | form or lemma was missing |
| a2-01 | vinden | None | verb | added | — | to find | form or lemma was missing |
| a2-01 | opendoen | None | verb | added | — | to open | form or lemma was missing |
| a2-01 | leggen | None | verb | added | — | to put / lay | form or lemma was missing |
| a2-01 | zeggen | None | verb | added | — | to say | form or lemma was missing |
| a2-01 | komen | None | verb | added | — | to come | form or lemma was missing |
| a2-01 | zuchten | None | verb | added | — | to sigh | form or lemma was missing |
| a2-01 | helpen | None | verb | added | — | to help | form or lemma was missing |
| a2-01 | tillen | None | verb | added | — | to lift | form or lemma was missing |
| a2-01 | liggen | None | verb | added | — | to lie | form or lemma was missing |
| a2-01 | pakken | None | verb | added | — | to take / grab | form or lemma was missing |
| a2-01 | lopen | None | verb | added | — | to walk | form or lemma was missing |
| a2-01 | kwispelen | None | verb | added | — | to wag | form or lemma was missing |
| a2-01 | horen | None | verb | added | — | to hear | form or lemma was missing |
| a2-01 | staan | None | verb | added | — | to stand | form or lemma was missing |
| a2-01 | maken | None | verb | added | — | to make | form or lemma was missing |
| a2-01 | geven | None | verb | added | — | to give | form or lemma was missing |
| a2-01 | worden | None | verb | added | — | to become | form or lemma was missing |
| a2-01 | zien | None | verb | added | — | to see | form or lemma was missing |
| a2-01 | snel | None | adjective | added | — | fast | form or lemma was missing |
| a2-01 | roepen | None | verb | added | — | to call / shout | form or lemma was missing |
| a2-01 | wachten | None | verb | added | — | to wait | form or lemma was missing |
| a2-01 | laten | None | verb | added | — | to let | form or lemma was missing |
| a2-01 | winnen | None | verb | added | — | to win | form or lemma was missing |
| a2-01 | blaffen | None | verb | added | — | to bark | form or lemma was missing |
| a2-01 | rood | None | adjective | added | — | red | form or lemma was missing |
| a2-01 | Het | None | article | changed | the / it | the | meaning now matches this surface form / context |
| a2-01 | is | None | verb | changed | to be / is / are / was / were / be | is | meaning now matches this surface form / context |
| a2-01 | wil | None | verb | changed | to want / wants / wanted | wants | meaning now matches this surface form / context |
| a2-01 | zijn | None | determiner | changed | his / are (to be) | his | meaning now matches this surface form / context |
| a2-01 | zoekt | None | verb | changed | to look for / searches | looks for | meaning now matches this surface form / context |
| a2-01 | vraagt | None | verb | changed | to ask / asks / asked | asks | meaning now matches this surface form / context |
| a2-01 | aan | None | preposition | changed | on / at / to | to | meaning now matches this surface form / context |
| a2-01 | zit | None | verb | changed | to sit / sits / sat | sits | meaning now matches this surface form / context |
| a2-01 | kijkt | None | verb | changed | to look / looks / looked | looks | meaning now matches this surface form / context |
| a2-01 | lacht | None | verb | changed | to laugh / laughs / laughed | laughs | meaning now matches this surface form / context |
| a2-01 | vindt | None | verb | changed | to find / finds / found | finds | meaning now matches this surface form / context |
| a2-01 | gaat | None | verb | changed | to go / goes / went | goes | meaning now matches this surface form / context |
| a2-01 | heb | None | verb | changed | to have / has / had | have | meaning now matches this surface form / context |
| a2-01 | gelegd | None | verb | changed | to put / lay / lays / laid | put / laid | meaning now matches this surface form / context |
| a2-01 | zegt | None | verb | changed | to say / says / said | says | meaning now matches this surface form / context |
| a2-01 | vergeten | None | verb | changed | to forget / forgot / forgotten | forgotten | meaning now matches this surface form / context |
| a2-01 | heeft | None | verb | changed | to have / has / had | has | meaning now matches this surface form / context |
| a2-01 | komt | None | verb | changed | to come / comes / came | comes | meaning now matches this surface form / context |
| a2-01 | zucht | None | verb | changed | to sigh / sighs / sighed | sighs | meaning now matches this surface form / context |
| a2-01 | helpt | None | verb | changed | to help / helps / helped | helps | meaning now matches this surface form / context |
| a2-01 | tilt | None | verb | changed | to lift / lifts / lifted | lifts | meaning now matches this surface form / context |
| a2-01 | ligt | None | verb | changed | to lie / lies / lay | lies | meaning now matches this surface form / context |
| a2-01 | pakt | None | verb | changed | to take / grab / takes / grabbed | takes | meaning now matches this surface form / context |
| a2-01 | loopt | None | verb | changed | to walk / walks / walked | walks | meaning now matches this surface form / context |
| a2-01 | legt | None | verb | changed | to put / lay / lays / laid | puts / lays | meaning now matches this surface form / context |
| a2-01 | kwispelt | None | verb | changed | to wag / wags / wagged | wags | meaning now matches this surface form / context |
| a2-01 | hoort | None | verb | changed | to hear / hears / heard | hears | meaning now matches this surface form / context |
| a2-01 | uit | None | adverb | changed | out | out of | meaning now matches this surface form / context |
| a2-01 | staat | None | verb | changed | to stand / stands / stood | stands | meaning now matches this surface form / context |
| a2-01 | bij | None | preposition | changed | at / with | at | meaning now matches this surface form / context |
| a2-01 | maakt | None | verb | changed | to make / makes / made | makes | meaning now matches this surface form / context |
| a2-01 | geeft | None | verb | changed | to give / gives / gave / given | gives | meaning now matches this surface form / context |
| a2-01 | word | None | verb | changed | to become / becomes / became | become | meaning now matches this surface form / context |
| a2-01 | ziet | None | verb | changed | to see / sees / saw | sees | meaning now matches this surface form / context |
| a2-01 | sneller | None | adjective | changed | fast | faster | meaning now matches this surface form / context |
| a2-01 | roept | None | verb | changed | to call / shout / shouts | calls / shouts | meaning now matches this surface form / context |
| a2-01 | wacht | None | verb | changed | to wait / waits / waited | waits | meaning now matches this surface form / context |
| a2-01 | Geef | None | verb | changed | to give / gives / gave / given | give | meaning now matches this surface form / context |
| a2-01 | laat | None | verb | changed | to let / leave / lets | lets | meaning now matches this surface form / context |
| a2-01 | hebt | None | verb | changed | to have / has / had | have | meaning now matches this surface form / context |
| a2-01 | gewonnen | None | verb | changed | to win / won | won | meaning now matches this surface form / context |
| a2-01 | was | None | verb | changed | to be / is / are / was / were / be | was | meaning now matches this surface form / context |
| a2-01 | blaft | None | verb | changed | to bark / barks / barked | barks | meaning now matches this surface form / context |
| a2-01 | gaan | None | verb | changed | to go / goes / went | go | meaning now matches this surface form / context |
| a2-01 | ze | None | pronoun | changed | she / they | they | meaning now matches this surface form / context |
| a2-01 | over | None | preposition | changed | over / about | over | meaning now matches this surface form / context |
| a2-01 | rode | None | adjective | changed | red | red (inflected) | meaning now matches this surface form / context |
| b1-01 | zijn | 1 | verb | added | — | are | form or lemma was missing |
| b1-01 | als | 3 | conjunction | added | — | as | form or lemma was missing |
| b1-01 | die | 1 | pronoun | added | — | that one | form or lemma was missing |
| b1-01 | die | 2 | pronoun | added | — | that one | form or lemma was missing |
| b1-01 | die | 4 | pronoun | added | — | who | form or lemma was missing |
| b1-01 | die | 5 | pronoun | added | — | who | form or lemma was missing |
| b1-01 | die | 8 | pronoun | added | — | which | form or lemma was missing |
| b1-01 | aan | 3 | preposition | added | — | there you go | form or lemma was missing |
| b1-01 | aan | 4 | preposition | added | — | of | form or lemma was missing |
| b1-01 | aan | 5 | preposition | added | — | to | form or lemma was missing |
| b1-01 | bloem | None | noun | added | — | flower | form or lemma was missing |
| b1-01 | ruiken | None | verb | added | — | to smell | form or lemma was missing |
| b1-01 | beginnen | None | verb | added | — | to begin | form or lemma was missing |
| b1-01 | eten | None | verb | added | — | to eat | form or lemma was missing |
| b1-01 | laatst | None | adjective | added | — | last | form or lemma was missing |
| b1-01 | zeggen | None | verb | added | — | to say | form or lemma was missing |
| b1-01 | lang | None | adjective | added | — | long | form or lemma was missing |
| b1-01 | zien | None | verb | added | — | to see | form or lemma was missing |
| b1-01 | glimlachen | None | verb | added | — | to smile | form or lemma was missing |
| b1-01 | denken | None | verb | added | — | to think | form or lemma was missing |
| b1-01 | oud | None | adjective | added | — | old | form or lemma was missing |
| b1-01 | wit | None | adjective | added | — | white | form or lemma was missing |
| b1-01 | wijzen | None | verb | added | — | to point | form or lemma was missing |
| b1-01 | vragen | None | verb | added | — | to ask | form or lemma was missing |
| b1-01 | knikken | None | verb | added | — | to nod | form or lemma was missing |
| b1-01 | geven | None | verb | added | — | to give | form or lemma was missing |
| b1-01 | pakken | None | verb | added | — | to take / grab | form or lemma was missing |
| b1-01 | willen | None | verb | added | — | to want | form or lemma was missing |
| b1-01 | groot | None | adjective | added | — | big | form or lemma was missing |
| b1-01 | landen | None | verb | added | — | to land | form or lemma was missing |
| b1-01 | kijken | None | verb | added | — | to look | form or lemma was missing |
| b1-01 | grijnzen | None | verb | added | — | to grin | form or lemma was missing |
| b1-01 | stelen | None | verb | added | — | to steal | form or lemma was missing |
| b1-01 | vliegen | None | verb | added | — | to fly | form or lemma was missing |
| b1-01 | roepen | None | verb | added | — | to call / shout | form or lemma was missing |
| b1-01 | rennen | None | verb | added | — | to run | form or lemma was missing |
| b1-01 | hoofd | None | noun | added | — | head | form or lemma was missing |
| b1-01 | mens | None | noun | added | — | person / human | form or lemma was missing |
| b1-01 | kunnen | None | verb | added | — | to be able to | form or lemma was missing |
| b1-01 | hijgen | None | verb | added | — | to pant | form or lemma was missing |
| b1-01 | horen | None | verb | added | — | to hear | form or lemma was missing |
| b1-01 | komen | None | verb | added | — | to come | form or lemma was missing |
| b1-01 | opeten | None | verb | added | — | to eat up | form or lemma was missing |
| b1-01 | gaan | None | verb | added | — | to go | form or lemma was missing |
| b1-01 | veer | None | noun | added | — | feather | form or lemma was missing |
| b1-01 | schoonmaken | None | verb | added | — | to clean | form or lemma was missing |
| b1-01 | uitlaten | None | verb | added | — | to walk (a dog) | form or lemma was missing |
| b1-01 | blijven | None | verb | added | — | to stay | form or lemma was missing |
| b1-01 | jaar | None | noun | added | — | year | form or lemma was missing |
| b1-01 | heel | None | adjective | added | — | whole / very | form or lemma was missing |
| b1-01 | moeten | None | verb | added | — | to have to | form or lemma was missing |
| b1-01 | maken | None | verb | added | — | to make | form or lemma was missing |
| b1-01 | proeven | None | verb | added | — | to taste | form or lemma was missing |
| b1-01 | vertellen | None | verb | added | — | to tell | form or lemma was missing |
| b1-01 | vinden | None | verb | added | — | to find | form or lemma was missing |
| b1-01 | blozen | None | verb | added | — | to blush | form or lemma was missing |
| b1-01 | doen | None | verb | added | — | to do | form or lemma was missing |
| b1-01 | terugkomen | None | verb | added | — | to come back | form or lemma was missing |
| b1-01 | nieuw | None | adjective | added | — | new | form or lemma was missing |
| b1-01 | leggen | None | verb | added | — | to put / lay | form or lemma was missing |
| b1-01 | begrijpen | None | verb | added | — | to understand | form or lemma was missing |
| b1-01 | halen | None | verb | added | — | to fetch / shrug | form or lemma was missing |
| b1-01 | schouder | None | noun | added | — | shoulder | form or lemma was missing |
| b1-01 | voelen | None | verb | added | — | to feel | form or lemma was missing |
| b1-01 | kopen | None | verb | added | — | to buy | form or lemma was missing |
| b1-01 | bedoelen | None | verb | added | — | to mean | form or lemma was missing |
| b1-01 | volgend | None | adjective | added | — | next | form or lemma was missing |
| b1-01 | zwaaien | None | verb | added | — | to wave | form or lemma was missing |
| b1-01 | ver | None | adjective | added | — | far | form or lemma was missing |
| b1-01 | proberen | None | verb | added | — | to try | form or lemma was missing |
| b1-01 | lukken | None | verb | added | — | to succeed | form or lemma was missing |
| b1-01 | weggeven | None | verb | added | — | to give away | form or lemma was missing |
| b1-01 | is | None | verb | changed | to be / is / are / was / were / be | is | meaning now matches this surface form / context |
| b1-01 | loopt | None | verb | changed | to walk / walks / walked | walks | meaning now matches this surface form / context |
| b1-01 | kramen | None | noun | changed | stall | stalls | meaning now matches this surface form / context |
| b1-01 | bloemen | None | noun | changed | flower | flowers | meaning now matches this surface form / context |
| b1-01 | Ze | None | pronoun | changed | she / they | she | meaning now matches this surface form / context |
| b1-01 | ruikt | None | verb | changed | to smell / smells / smelled | smells | meaning now matches this surface form / context |
| b1-01 | heerlijks | None | adjective | changed | delicious | delicious (something delicious) | meaning now matches this surface form / context |
| b1-01 | warme | None | adjective | changed | warm | warm (inflected) | meaning now matches this surface form / context |
| b1-01 | stroopwafels | None | noun | changed | stroopwafel | stroopwafels | meaning now matches this surface form / context |
| b1-01 | begint | None | verb | changed | to begin / begins / began | begins | meaning now matches this surface form / context |
| b1-01 | te | None | adverb | changed | too | to | meaning now matches this surface form / context |
| b1-01 | heeft | None | verb | changed | to have / has / had | has | meaning now matches this surface form / context |
| b1-01 | gegeten | None | verb | changed | to eat / eats / ate / eaten | eaten | meaning now matches this surface form / context |
| b1-01 | neem | None | verb | changed | to take | take | meaning now matches this surface form / context |
| b1-01 | laatste | None | adjective | changed | last | last (inflected) | meaning now matches this surface form / context |
| b1-01 | zegt | None | verb | changed | to say / says / said | says | meaning now matches this surface form / context |
| b1-01 | lange | None | adjective | changed | long | long (inflected) | meaning now matches this surface form / context |
| b1-01 | staat | None | verb | changed | to stand / stands / stood | stands | meaning now matches this surface form / context |
| b1-01 | ziet | None | verb | changed | to see / sees / saw | sees | meaning now matches this surface form / context |
| b1-01 | glimlacht | None | verb | changed | to smile / smiles / smiled | smiles | meaning now matches this surface form / context |
| b1-01 | Die | None | pronoun | changed | that / who | that | meaning now matches this surface form / context |
| b1-01 | denkt | None | verb | changed | to think / thinks / thought | thinks | meaning now matches this surface form / context |
| b1-01 | oude | None | adjective | changed | old | old (inflected) | meaning now matches this surface form / context |
| b1-01 | witte | None | adjective | changed | white | white (inflected) | meaning now matches this surface form / context |
| b1-01 | bent | None | verb | changed | to be / is / are / was / were / be | are | meaning now matches this surface form / context |
| b1-01 | wijst | None | verb | changed | to point / points / pointed | points | meaning now matches this surface form / context |
| b1-01 | vraagt | None | verb | changed | to ask / asks / asked | asks | meaning now matches this surface form / context |
| b1-01 | knikt | None | verb | changed | to nod / nods / nodded | nods | meaning now matches this surface form / context |
| b1-01 | wees | None | verb | changed | to be / is / are / was / were / be | be | meaning now matches this surface form / context |
| b1-01 | meeuwen | None | noun | changed | seagull | seagulls | meaning now matches this surface form / context |
| b1-01 | zijn | None | determiner | changed | his / are (to be) | his | meaning now matches this surface form / context |
| b1-01 | lacht | None | verb | changed | to laugh / laughs / laughed | laughs | meaning now matches this surface form / context |
| b1-01 | geeft | None | verb | changed | to give / gives / gave / given | gives | meaning now matches this surface form / context |
| b1-01 | pakt | None | verb | changed | to take / grab / takes / grabbed | takes | meaning now matches this surface form / context |
| b1-01 | vast | None | adverb | changed | certainly | firmly | meaning now matches this surface form / context |
| b1-01 | wil | None | verb | changed | to want / wants / wanted | wants | meaning now matches this surface form / context |
| b1-01 | nemen | None | verb | changed | to take / takes / took | to take | meaning now matches this surface form / context |
| b1-01 | als | None | conjunction | changed | if / when / as | when | meaning now matches this surface form / context |
| b1-01 | grote | None | adjective | changed | big | big (inflected) | meaning now matches this surface form / context |
| b1-01 | landt | None | verb | changed | to land / lands / landed | lands | meaning now matches this surface form / context |
| b1-01 | kijkt | None | verb | changed | to look / looks / looked | looks | meaning now matches this surface form / context |
| b1-01 | aan | None | preposition | changed | on / at / to | at | meaning now matches this surface form / context |
| b1-01 | grijnst | None | verb | changed | to grin / grins / grinned | grins | meaning now matches this surface form / context |
| b1-01 | wacht | None | verb | changed | to wait / waits / waited | waits | meaning now matches this surface form / context |
| b1-01 | steelt | None | verb | changed | to steal / steals / stole / stolen | steals | meaning now matches this surface form / context |
| b1-01 | vliegt | None | verb | changed | to fly / flies / flew | flies | meaning now matches this surface form / context |
| b1-01 | roept | None | verb | changed | to call / shout / shouts | calls / shouts | meaning now matches this surface form / context |
| b1-01 | rent | None | verb | changed | to run / runs / ran | runs | meaning now matches this surface form / context |
| b1-01 | hoofden | None | noun | changed | head | heads | meaning now matches this surface form / context |
| b1-01 | mensen | None | noun | changed | person / human | people | meaning now matches this surface form / context |
| b1-01 | kan | None | verb | changed | to can / be able / could | can | meaning now matches this surface form / context |
| b1-01 | sneller | None | adjective | changed | fast | faster | meaning now matches this surface form / context |
| b1-01 | hijgt | None | verb | changed | to pant / pants / panted | pants | meaning now matches this surface form / context |
| b1-01 | hoort | None | verb | changed | to hear / hears / heard | hears | meaning now matches this surface form / context |
| b1-01 | komt | None | verb | changed | to come / comes / came | comes | meaning now matches this surface form / context |
| b1-01 | opgegeten | None | verb | changed | to eat up / ate up | ate up | meaning now matches this surface form / context |
| b1-01 | gaat | None | verb | changed | to go / goes / went | goes | meaning now matches this surface form / context |
| b1-01 | zitten | None | verb | changed | to sit / sits / sat | to sit | meaning now matches this surface form / context |
| b1-01 | veren | None | noun | changed | feather | feathers | meaning now matches this surface form / context |
| b1-01 | schoonmaakt | None | verb | changed | to clean | cleans | meaning now matches this surface form / context |
| b1-01 | hebt | None | verb | changed | to have / has / had | have | meaning now matches this surface form / context |
| b1-01 | uitlaat | None | verb | changed | to walk (a dog) | walks (a dog) | meaning now matches this surface form / context |
| b1-01 | blijft | None | verb | changed | to stay / stays / stayed | stays | meaning now matches this surface form / context |
| b1-01 | staan | None | verb | changed | to stand / stands / stood | to stand | meaning now matches this surface form / context |
| b1-01 | jaren | None | noun | changed | year | years | meaning now matches this surface form / context |
| b1-01 | hele | None | adjective | changed | whole / very | whole | meaning now matches this surface form / context |
| b1-01 | moet | None | verb | changed | to must / have to / had to | must / has to | meaning now matches this surface form / context |
| b1-01 | lachen | None | verb | changed | to laugh / laughs / laughed | to laugh | meaning now matches this surface form / context |
| b1-01 | gestolen | None | verb | changed | to steal / steals / stole / stolen | stolen | meaning now matches this surface form / context |
| b1-01 | wachten | None | verb | changed | to wait / waits / waited | to wait | meaning now matches this surface form / context |
| b1-01 | maak | None | verb | changed | to make | make | meaning now matches this surface form / context |
| b1-01 | proeft | None | verb | changed | to taste / tastes / tasted | tastes | meaning now matches this surface form / context |
| b1-01 | vertelt | None | verb | changed | to tell / tells / told | tells | meaning now matches this surface form / context |
| b1-01 | was | None | verb | changed | to be / is / are / was / were / be | was | meaning now matches this surface form / context |
| b1-01 | gaf | None | verb | changed | to give / gives / gave / given | gave | meaning now matches this surface form / context |
| b1-01 | vind | None | verb | changed | to find | find | meaning now matches this surface form / context |
| b1-01 | bloost | None | verb | changed | to blush / blushes / blushed | blushes | meaning now matches this surface form / context |
| b1-01 | heb | None | verb | changed | to have / has / had | have | meaning now matches this surface form / context |
| b1-01 | gedaan | None | verb | changed | to do / does / did / done | done | meaning now matches this surface form / context |
| b1-01 | teruggekomen | None | verb | changed | to come back / came back | come back | meaning now matches this surface form / context |
| b1-01 | Deel | None | verb | changed | to share / shares / shared | share | meaning now matches this surface form / context |
| b1-01 | nieuwe | None | adjective | changed | new | new (inflected) | meaning now matches this surface form / context |
| b1-01 | zit | None | verb | changed | to sit / sits / sat | sits | meaning now matches this surface form / context |
| b1-01 | deelt | None | verb | changed | to share / shares / shared | shares | meaning now matches this surface form / context |
| b1-01 | legt | None | verb | changed | to put / lay / lays / laid | puts / lays | meaning now matches this surface form / context |
| b1-01 | beloven | None | verb | changed | to promise / promises / promised | to promise | meaning now matches this surface form / context |
| b1-01 | begrijpt | None | verb | changed | to understand / understands / understood | understands | meaning now matches this surface form / context |
| b1-01 | eet | None | verb | changed | to eat / eats / ate / eaten | eats | meaning now matches this surface form / context |
| b1-01 | haalt | None | verb | changed | to fetch / shrug | shrugs | meaning now matches this surface form / context |
| b1-01 | schouders | None | noun | changed | shoulder | shoulders | meaning now matches this surface form / context |
| b1-01 | voelt | None | verb | changed | to feel / feels / felt | feels | meaning now matches this surface form / context |
| b1-01 | koopt | None | verb | changed | to buy / buys / bought | buys | meaning now matches this surface form / context |
| b1-01 | bedoel | None | verb | changed | to mean | mean | meaning now matches this surface form / context |
| b1-01 | weggegeven | None | verb | changed | to give away / gave away | given away | meaning now matches this surface form / context |
| b1-01 | Gedeeld | None | verb | changed | to share / shares / shared | shared | meaning now matches this surface form / context |
| b1-01 | volgende | None | adjective | changed | next | next (inflected) | meaning now matches this surface form / context |
| b1-01 | lopen | None | verb | changed | to walk / walks / walked | to walk | meaning now matches this surface form / context |
| b1-01 | zwaait | None | verb | changed | to wave / waves / waved | waves | meaning now matches this surface form / context |
| b1-01 | verre | None | adjective | changed | far | far (inflected) | meaning now matches this surface form / context |
| b1-01 | gekocht | None | verb | changed | to buy / buys / bought | bought | meaning now matches this surface form / context |
| b1-01 | geprobeerd | None | verb | changed | to try / tries / tried | tried | meaning now matches this surface form / context |
| b1-01 | gelukt | None | verb | changed | to succeed / succeeded | succeeded | meaning now matches this surface form / context |
| b1-01 | delen | None | verb | changed | to share / shares / shared | to share | meaning now matches this surface form / context |
| b1-01 | Eén | None | numeral | changed | one | one (at sentence start) | meaning now matches this surface form / context |
| b2-01 | 's | None | other | removed | in the morning (part of 's ochtends) | — | not a surface form in the story |
| b2-01 | ochtends | None | noun | removed | morning | — | not a surface form in the story |
| b2-01 | Af | None | adverb | removed | off / finished | — | not a surface form in the story |
| b2-01 | toe | None | adverb | removed | to / closed | — | not a surface form in the story |
| b2-01 | 's ochtends | None | phrase | added | — | in the morning | form or lemma was missing |
| b2-01 | af en toe | None | phrase | added | — | now and then | form or lemma was missing |
| b2-01 | zijn | 9 | verb | added | — | to be | form or lemma was missing |
| b2-01 | zijn | 11 | verb | added | — | to be | form or lemma was missing |
| b2-01 | zijn | 18 | verb | added | — | are | form or lemma was missing |
| b2-01 | te | 2 | adverb | added | — | too | form or lemma was missing |
| b2-01 | als | 5 | conjunction | added | — | when | form or lemma was missing |
| b2-01 | die | 1 | pronoun | added | — | which | form or lemma was missing |
| b2-01 | die | 2 | pronoun | added | — | who | form or lemma was missing |
| b2-01 | die | 3 | pronoun | added | — | it | form or lemma was missing |
| b2-01 | die | 4 | pronoun | added | — | that one | form or lemma was missing |
| b2-01 | ze | 13 | pronoun | added | — | they | form or lemma was missing |
| b2-01 | alleen | 2 | adverb | added | — | alone | form or lemma was missing |
| b2-01 | zoals | 2 | conjunction | added | — | as | form or lemma was missing |
| b2-01 | uit | 1 | adverb | added | — | out of | form or lemma was missing |
| b2-01 | uit | 2 | adverb | added | — | looked (in 'eruitzien') | form or lemma was missing |
| b2-01 | uit | 3 | adverb | added | — | ran over (in 'uitlopen') | form or lemma was missing |
| b2-01 | uit | 4 | adverb | added | — | postpone (in 'uitstellen') | form or lemma was missing |
| b2-01 | staan | None | verb | added | — | to stand | form or lemma was missing |
| b2-01 | lezen | None | verb | added | — | to read | form or lemma was missing |
| b2-01 | strijken | None | verb | added | — | to iron | form or lemma was missing |
| b2-01 | bord | None | noun | added | — | sign / plate | form or lemma was missing |
| b2-01 | twijfelen | None | verb | added | — | to doubt | form or lemma was missing |
| b2-01 | juist | None | adjective | added | — | right | form or lemma was missing |
| b2-01 | spreken | None | verb | added | — | to speak | form or lemma was missing |
| b2-01 | aannemen | None | verb | added | — | to assume | form or lemma was missing |
| b2-01 | zullen | None | verb | added | — | will | form or lemma was missing |
| b2-01 | opengaan | None | verb | added | — | to open | form or lemma was missing |
| b2-01 | leeg | None | adjective | added | — | empty | form or lemma was missing |
| b2-01 | groot | None | adjective | added | — | big | form or lemma was missing |
| b2-01 | bruin | None | adjective | added | — | brown | form or lemma was missing |
| b2-01 | kijken | None | verb | added | — | to look | form or lemma was missing |
| b2-01 | komen | None | verb | added | — | to come | form or lemma was missing |
| b2-01 | zeggen | None | verb | added | — | to say | form or lemma was missing |
| b2-01 | gaan | None | verb | added | — | to go | form or lemma was missing |
| b2-01 | openen | None | verb | added | — | to open | form or lemma was missing |
| b2-01 | duren | None | verb | added | — | to last | form or lemma was missing |
| b2-01 | geven | None | verb | added | — | to give | form or lemma was missing |
| b2-01 | letten | None | verb | added | — | to pay attention | form or lemma was missing |
| b2-01 | sluiten | None | verb | added | — | to close | form or lemma was missing |
| b2-01 | lijken | None | verb | added | — | to seem | form or lemma was missing |
| b2-01 | oefenen | None | verb | added | — | to practise | form or lemma was missing |
| b2-01 | geeuwen | None | verb | added | — | to yawn | form or lemma was missing |
| b2-01 | vinden | None | verb | added | — | to find | form or lemma was missing |
| b2-01 | slikken | None | verb | added | — | to swallow | form or lemma was missing |
| b2-01 | mogen | None | verb | added | — | to be allowed | form or lemma was missing |
| b2-01 | denken | None | verb | added | — | to think | form or lemma was missing |
| b2-01 | kwispelen | None | verb | added | — | to wag | form or lemma was missing |
| b2-01 | lastig | None | adjective | added | — | difficult | form or lemma was missing |
| b2-01 | voelen | None | verb | added | — | to feel | form or lemma was missing |
| b2-01 | snel | None | adjective | added | — | fast | form or lemma was missing |
| b2-01 | collega | None | noun | added | — | colleague | form or lemma was missing |
| b2-01 | lukken | None | verb | added | — | to succeed | form or lemma was missing |
| b2-01 | schrijven | None | verb | added | — | to write | form or lemma was missing |
| b2-01 | blaffen | None | verb | added | — | to bark | form or lemma was missing |
| b2-01 | houden | None | verb | added | — | to like / hold | form or lemma was missing |
| b2-01 | durven | None | verb | added | — | to dare | form or lemma was missing |
| b2-01 | glimlachen | None | verb | added | — | to smile | form or lemma was missing |
| b2-01 | zuchten | None | verb | added | — | to sigh | form or lemma was missing |
| b2-01 | excuus | None | noun | added | — | apology | form or lemma was missing |
| b2-01 | moeten | None | verb | added | — | to have to | form or lemma was missing |
| b2-01 | lopen | None | verb | added | — | to walk | form or lemma was missing |
| b2-01 | leggen | None | verb | added | — | to put / lay | form or lemma was missing |
| b2-01 | weten | None | verb | added | — | to know | form or lemma was missing |
| b2-01 | begrijpen | None | verb | added | — | to understand | form or lemma was missing |
| b2-01 | slaan | None | verb | added | — | to hit | form or lemma was missing |
| b2-01 | raar | None | adjective | added | — | strange | form or lemma was missing |
| b2-01 | gebeuren | None | verb | added | — | to happen | form or lemma was missing |
| b2-01 | ander | None | adjective | added | — | other | form or lemma was missing |
| b2-01 | bedoelen | None | verb | added | — | to mean | form or lemma was missing |
| b2-01 | beslissing | None | noun | added | — | decision | form or lemma was missing |
| b2-01 | belangrijk | None | adjective | added | — | important | form or lemma was missing |
| b2-01 | krijgen | None | verb | added | — | to get | form or lemma was missing |
| b2-01 | diploma | None | noun | added | — | diploma | form or lemma was missing |
| b2-01 | beroemd | None | adjective | added | — | famous | form or lemma was missing |
| b2-01 | universiteit | None | noun | added | — | university | form or lemma was missing |
| b2-01 | kort | None | adjective | added | — | short | form or lemma was missing |
| b2-01 | heffen | None | verb | added | — | to lift | form or lemma was missing |
| b2-01 | efficiënt | None | adjective | added | — | efficient | form or lemma was missing |
| b2-01 | snuffelen | None | verb | added | — | to sniff | form or lemma was missing |
| b2-01 | likken | None | verb | added | — | to lick | form or lemma was missing |
| b2-01 | lachen | None | verb | added | — | to laugh | form or lemma was missing |
| b2-01 | geloven | None | verb | added | — | to believe | form or lemma was missing |
| b2-01 | week | None | noun | added | — | week | form or lemma was missing |
| b2-01 | hangen | None | verb | added | — | to hang | form or lemma was missing |
| b2-01 | liggen | None | verb | added | — | to lie | form or lemma was missing |
| b2-01 | brengen | None | verb | added | — | to bring | form or lemma was missing |
| b2-01 | dut | None | noun | added | — | nap | form or lemma was missing |
| b2-01 | bepalen | None | verb | added | — | to determine | form or lemma was missing |
| b2-01 | Om | None | preposition | changed | at / around | at | meaning now matches this surface form / context |
| b2-01 | stond | None | verb | changed | to stand / stands / stood | stood | meaning now matches this surface form / context |
| b2-01 | was | None | verb | changed | to be / is / are / was / were / be | was | meaning now matches this surface form / context |
| b2-01 | zijn | None | determiner | changed | his / are (to be) | his | meaning now matches this surface form / context |
| b2-01 | had | None | verb | changed | to have / has / had | had | meaning now matches this surface form / context |
| b2-01 | gelezen | None | verb | changed | to read / reads / read | read | meaning now matches this surface form / context |
| b2-01 | beste | None | adjective | changed | better | best | meaning now matches this surface form / context |
| b2-01 | gestreken | None | verb | changed | to iron / ironed | ironed | meaning now matches this surface form / context |
| b2-01 | bordje | None | noun | changed | sign / plate | sign | meaning now matches this surface form / context |
| b2-01 | twijfelde | None | verb | changed | to doubt / doubted | doubted | meaning now matches this surface form / context |
| b2-01 | juiste | None | adjective | changed | correct | right (inflected) | meaning now matches this surface form / context |
| b2-01 | geweest | None | verb | changed | to be / is / are / was / were / be | been | meaning now matches this surface form / context |
| b2-01 | alleen | None | adverb | changed | alone / only | only | meaning now matches this surface form / context |
| b2-01 | spreekt | None | verb | changed | to speak / speaks / spoke | speaks | meaning now matches this surface form / context |
| b2-01 | aangenomen | None | verb | changed | to assume / assumed | assumed | meaning now matches this surface form / context |
| b2-01 | zou | None | verb | changed | will / would | would | meaning now matches this surface form / context |
| b2-01 | Toen | None | conjunction | changed | when / then | when | meaning now matches this surface form / context |
| b2-01 | openging | None | verb | changed | to open / opened | opened | meaning now matches this surface form / context |
| b2-01 | zag | None | verb | changed | to see / sees / saw | saw | meaning now matches this surface form / context |
| b2-01 | lege | None | adjective | changed | empty | empty (inflected) | meaning now matches this surface form / context |
| b2-01 | Aan | None | preposition | changed | on / at / to | at | meaning now matches this surface form / context |
| b2-01 | zat | None | verb | changed | to sit / sits / sat | sat | meaning now matches this surface form / context |
| b2-01 | grote | None | adjective | changed | big | big (inflected) | meaning now matches this surface form / context |
| b2-01 | bruine | None | adjective | changed | brown | brown (inflected) | meaning now matches this surface form / context |
| b2-01 | keek | None | verb | changed | to look / looks / looked | looked | meaning now matches this surface form / context |
| b2-01 | Kom | None | verb | changed | to come | come | meaning now matches this surface form / context |
| b2-01 | zei | None | verb | changed | to say / says / said | said | meaning now matches this surface form / context |
| b2-01 | bent | None | verb | changed | to be / is / are / was / were / be | are | meaning now matches this surface form / context |
| b2-01 | vast | None | adverb | changed | certainly | probably | meaning now matches this surface form / context |
| b2-01 | ben | None | verb | changed | to be / is / are / was / were / be | am | meaning now matches this surface form / context |
| b2-01 | is | None | verb | changed | to be / is / are / was / were / be | is | meaning now matches this surface form / context |
| b2-01 | ging | None | verb | changed | to go / goes / went | went | meaning now matches this surface form / context |
| b2-01 | zitten | None | verb | changed | to sit / sits / sat | to sit | meaning now matches this surface form / context |
| b2-01 | opende | None | verb | changed | to open / opened | opened | meaning now matches this surface form / context |
| b2-01 | Laten | None | verb | changed | to let / leave / lets | to let | meaning now matches this surface form / context |
| b2-01 | beginnen | None | verb | changed | to begin / begins / began | to begin | meaning now matches this surface form / context |
| b2-01 | ze | None | pronoun | changed | she / they | she | meaning now matches this surface form / context |
| b2-01 | duurt | None | verb | changed | to last / lasts / lasted | lasts | meaning now matches this surface form / context |
| b2-01 | minuten | None | noun | changed | minute | minutes | meaning now matches this surface form / context |
| b2-01 | stel | None | verb | changed | to put / ask | ask | meaning now matches this surface form / context |
| b2-01 | vragen | None | verb | changed | to ask / asks / asked | to ask | meaning now matches this surface form / context |
| b2-01 | geeft | None | verb | changed | to give / gives / gave / given | gives | meaning now matches this surface form / context |
| b2-01 | let | None | verb | changed | to pay attention (let op) | pay attention | meaning now matches this surface form / context |
| b2-01 | ogen | None | noun | changed | eye | eyes | meaning now matches this surface form / context |
| b2-01 | gesloten | None | verb | changed | to close / closed | closed | meaning now matches this surface form / context |
| b2-01 | leek | None | verb | changed | to seem / seemed | seemed | meaning now matches this surface form / context |
| b2-01 | te | None | adverb | changed | too | to | meaning now matches this surface form / context |
| b2-01 | slapen | None | verb | changed | to sleep / sleeps / slept | to sleep | meaning now matches this surface form / context |
| b2-01 | wilt | None | verb | changed | to want / wants / wanted | want | meaning now matches this surface form / context |
| b2-01 | ons | None | pronoun | changed | us / our | us | meaning now matches this surface form / context |
| b2-01 | vroeg | None | verb | changed | to ask / asks / asked | asked | meaning now matches this surface form / context |
| b2-01 | geoefend | None | verb | changed | to practise / practised | practised | meaning now matches this surface form / context |
| b2-01 | sprak | None | verb | changed | to speak / speaks / spoke | spoke | meaning now matches this surface form / context |
| b2-01 | goede | None | adjective | changed | good | good (inflected) | meaning now matches this surface form / context |
| b2-01 | wilde | None | verb | changed | to want / wants / wanted | wanted | meaning now matches this surface form / context |
| b2-01 | maken | None | verb | changed | to make / makes / made | to make | meaning now matches this surface form / context |
| b2-01 | geeuwde | None | verb | changed | to yawn / yawned | yawned | meaning now matches this surface form / context |
| b2-01 | vindt | None | verb | changed | to find / finds / found | finds | meaning now matches this surface form / context |
| b2-01 | slikte | None | verb | changed | to swallow / swallowed | swallowed | meaning now matches this surface form / context |
| b2-01 | Mag | None | verb | changed | to may / be allowed | may | meaning now matches this surface form / context |
| b2-01 | proberen | None | verb | changed | to try / tries / tried | to try | meaning now matches this surface form / context |
| b2-01 | dacht | None | verb | changed | to think / thinks / thought | thought | meaning now matches this surface form / context |
| b2-01 | wil | None | verb | changed | to want / wants / wanted | wants | meaning now matches this surface form / context |
| b2-01 | denk | None | verb | changed | to think | think | meaning now matches this surface form / context |
| b2-01 | kan | None | verb | changed | to can / be able / could | can | meaning now matches this surface form / context |
| b2-01 | kwispelde | None | verb | changed | to wag / wags / wagged | wagged | meaning now matches this surface form / context |
| b2-01 | lastige | None | adjective | changed | difficult | difficult (inflected) | meaning now matches this surface form / context |
| b2-01 | gaat | None | verb | changed | to go / goes / went | goes | meaning now matches this surface form / context |
| b2-01 | die | None | pronoun | changed | that / who | that | meaning now matches this surface form / context |
| b2-01 | voelde | None | verb | changed | to feel / feels / felt | felt | meaning now matches this surface form / context |
| b2-01 | sneller | None | adjective | changed | fast | faster | meaning now matches this surface form / context |
| b2-01 | kloppen | None | verb | changed | to beat / knock | to beat | meaning now matches this surface form / context |
| b2-01 | overkomen | None | verb | changed | to come across / seem | to seem | meaning now matches this surface form / context |
| b2-01 | collega's | None | noun | changed | colleague | colleagues | meaning now matches this surface form / context |
| b2-01 | kunnen | None | verb | changed | to can / be able / could | to be able to | meaning now matches this surface form / context |
| b2-01 | verdelen | None | verb | changed | to divide / share out | to divide | meaning now matches this surface form / context |
| b2-01 | Als | None | conjunction | changed | if / when / as | if | meaning now matches this surface form / context |
| b2-01 | lukt | None | verb | changed | to succeed / succeeded | succeeds | meaning now matches this surface form / context |
| b2-01 | willen | None | verb | changed | to want / wants / wanted | to want | meaning now matches this surface form / context |
| b2-01 | uitstellen | None | verb | changed | to postpone / postponed | to postpone | meaning now matches this surface form / context |
| b2-01 | doen | None | verb | changed | to do / does / did / done | to do | meaning now matches this surface form / context |
| b2-01 | schreef | None | verb | changed | to write / writes / wrote | wrote | meaning now matches this surface form / context |
| b2-01 | blafte | None | verb | changed | to bark / barks / barked | barked | meaning now matches this surface form / context |
| b2-01 | houdt | None | verb | changed | to like / hold | likes | meaning now matches this surface form / context |
| b2-01 | eerlijke | None | adjective | changed | honest / fair | honest | meaning now matches this surface form / context |
| b2-01 | mensen | None | noun | changed | person / human | people | meaning now matches this surface form / context |
| b2-01 | durfde | None | verb | changed | to dare / dared | dared | meaning now matches this surface form / context |
| b2-01 | deed | None | verb | changed | to do / does / did / done | did | meaning now matches this surface form / context |
| b2-01 | glimlachte | None | verb | changed | to smile / smiles / smiled | smiled | meaning now matches this surface form / context |
| b2-01 | lange | None | adjective | changed | long | long (inflected) | meaning now matches this surface form / context |
| b2-01 | zuchtte | None | verb | changed | to sigh / sighs / sighed | sighed | meaning now matches this surface form / context |
| b2-01 | Excuses | None | noun | changed | apology | apologies | meaning now matches this surface form / context |
| b2-01 | moet | None | verb | changed | to must / have to / had to | must / has to | meaning now matches this surface form / context |
| b2-01 | blijft | None | verb | changed | to stay / stays / stayed | stays | meaning now matches this surface form / context |
| b2-01 | liep | None | verb | changed | to walk / walks / walked | walked | meaning now matches this surface form / context |
| b2-01 | bleef | None | verb | changed | to stay / stays / stayed | stayed | meaning now matches this surface form / context |
| b2-01 | legde | None | verb | changed | to put / lay / lays / laid | put / laid | meaning now matches this surface form / context |
| b2-01 | wist | None | verb | changed | to know / knows / knew | knew | meaning now matches this surface form / context |
| b2-01 | moest | None | verb | changed | to must / have to / had to | had to | meaning now matches this surface form / context |
| b2-01 | blijven | None | verb | changed | to stay / stays / stayed | to stay | meaning now matches this surface form / context |
| b2-01 | weet | None | verb | changed | to know / knows / knew | know | meaning now matches this surface form / context |
| b2-01 | of | None | conjunction | changed | or | whether | meaning now matches this surface form / context |
| b2-01 | begrijpt | None | verb | changed | to understand / understands / understood | understands | meaning now matches this surface form / context |
| b2-01 | heb | None | verb | changed | to have / has / had | have | meaning now matches this surface form / context |
| b2-01 | hoeft | None | verb | changed | to need / have to | needs to | meaning now matches this surface form / context |
| b2-01 | sloeg | None | verb | changed | to hit / beat | hit | meaning now matches this surface form / context |
| b2-01 | rare | None | adjective | changed | strange | strange (inflected) | meaning now matches this surface form / context |
| b2-01 | kwam | None | verb | changed | to come / comes / came | came | meaning now matches this surface form / context |
| b2-01 | gebeurt | None | verb | changed | to happen / happens / happened | happens | meaning now matches this surface form / context |
| b2-01 | uitleggen | None | verb | changed | to explain / explained | to explain | meaning now matches this surface form / context |
| b2-01 | zoals | None | conjunction | changed | like / as | like | meaning now matches this surface form / context |
| b2-01 | andere | None | adjective | changed | other | other (inflected) | meaning now matches this surface form / context |
| b2-01 | bedrijven | None | noun | changed | company | companies | meaning now matches this surface form / context |
| b2-01 | maakte | None | verb | changed | to make / makes / made | made | meaning now matches this surface form / context |
| b2-01 | Bedoelt | None | verb | changed | to mean / means / meant | means | meaning now matches this surface form / context |
| b2-01 | beslissingen | None | noun | changed | decision | decisions | meaning now matches this surface form / context |
| b2-01 | neemt | None | verb | changed | to take / takes / took | takes | meaning now matches this surface form / context |
| b2-01 | belangrijke | None | adjective | changed | important | important (inflected) | meaning now matches this surface form / context |
| b2-01 | heeft | None | verb | changed | to have / has / had | has | meaning now matches this surface form / context |
| b2-01 | zit | None | verb | changed | to sit / sits / sat | sits | meaning now matches this surface form / context |
| b2-01 | krijgt | None | verb | changed | to get / received | gets | meaning now matches this surface form / context |
| b2-01 | overtuigen | None | verb | changed | to convince / convinced | to convince | meaning now matches this surface form / context |
| b2-01 | zien | None | verb | changed | to see / sees / saw | to see | meaning now matches this surface form / context |
| b2-01 | kon | None | verb | changed | to can / be able / could | could | meaning now matches this surface form / context |
| b2-01 | diploma's | None | noun | changed | diploma | diplomas | meaning now matches this surface form / context |
| b2-01 | beroemde | None | adjective | changed | famous | famous (inflected) | meaning now matches this surface form / context |
| b2-01 | universiteiten | None | noun | changed | university | universities | meaning now matches this surface form / context |
| b2-01 | ambities | None | noun | changed | ambition | ambitions | meaning now matches this surface form / context |
| b2-01 | vergaderingen | None | noun | changed | meeting | meetings | meaning now matches this surface form / context |
| b2-01 | korter | None | adjective | changed | short | shorter | meaning now matches this surface form / context |
| b2-01 | pauzes | None | noun | changed | break | breaks | meaning now matches this surface form / context |
| b2-01 | nemen | None | verb | changed | to take / takes / took | to take | meaning now matches this surface form / context |
| b2-01 | hief | None | verb | changed | to lift / raise | lifted | meaning now matches this surface form / context |
| b2-01 | elke | None | pronoun | changed | each | each (inflected) | meaning now matches this surface form / context |
| b2-01 | korte | None | adjective | changed | short | short (inflected) | meaning now matches this surface form / context |
| b2-01 | worden | None | verb | changed | to become / becomes / became | to become | meaning now matches this surface form / context |
| b2-01 | rustiger | None | adjective | changed | calm / quiet | calmer | meaning now matches this surface form / context |
| b2-01 | verder | None | adverb | changed | further | on | meaning now matches this surface form / context |
| b2-01 | betere | None | adjective | changed | better | better (inflected) | meaning now matches this surface form / context |
| b2-01 | hoeven | None | verb | changed | to need / have to | to need to | meaning now matches this surface form / context |
| b2-01 | stellen | None | verb | changed | to postpone | to postpone (separable verb) | meaning now matches this surface form / context |
| b2-01 | efficiënter | None | adjective | changed | efficient | more efficient | meaning now matches this surface form / context |
| b2-01 | snuffelde | None | verb | changed | to sniff / sniffed | sniffed | meaning now matches this surface form / context |
| b2-01 | likte | None | verb | changed | to lick / licked | licked | meaning now matches this surface form / context |
| b2-01 | lachte | None | verb | changed | to laugh / laughs / laughed | laughed | meaning now matches this surface form / context |
| b2-01 | geloof | None | verb | changed | to believe / believed | believe | meaning now matches this surface form / context |
| b2-01 | hebt | None | verb | changed | to have / has / had | have | meaning now matches this surface form / context |
| b2-01 | gemaakt | None | verb | changed | to make / makes / made | made | meaning now matches this surface form / context |
| b2-01 | bewezen | None | verb | changed | to prove / proved | proved | meaning now matches this surface form / context |
| b2-01 | praat | None | verb | changed | to talk | talk | meaning now matches this surface form / context |
| b2-01 | weken | None | noun | changed | week | weeks | meaning now matches this surface form / context |
| b2-01 | vreemdste | None | adjective | changed | strange | strangest | meaning now matches this surface form / context |
| b2-01 | gekregen | None | verb | changed | to get / received | received | meaning now matches this surface form / context |
| b2-01 | hing | None | verb | changed | to hang / hung | hung | meaning now matches this surface form / context |
| b2-01 | lag | None | verb | changed | to lie / lies / lay | lay | meaning now matches this surface form / context |
| b2-01 | bracht | None | verb | changed | to bring / brought | brought | meaning now matches this surface form / context |
| b2-01 | nieuwe | None | adjective | changed | new | new (inflected) | meaning now matches this surface form / context |
| b2-01 | waren | None | verb | changed | to be / is / are / was / were / be | were | meaning now matches this surface form / context |
| b2-01 | vonden | None | verb | changed | to find / finds / found | found | meaning now matches this surface form / context |
| b2-01 | gaf | None | verb | changed | to give / gives / gave / given | gave | meaning now matches this surface form / context |
| b2-01 | nam | None | verb | changed | to take / takes / took | took | meaning now matches this surface form / context |
| b2-01 | overtuigd | None | verb | changed | to convince / convinced | convinced | meaning now matches this surface form / context |
| b2-01 | werd | None | verb | changed | to become / becomes / became | was | meaning now matches this surface form / context |
| b2-01 | uitgesteld | None | verb | changed | to postpone / postponed | postponed | meaning now matches this surface form / context |
| b2-01 | dutje | None | noun | changed | nap | a little nap | meaning now matches this surface form / context |
| b2-01 | bepaalde | None | verb | changed | to determine / determined | determined | meaning now matches this surface form / context |
| b2-01 | hoefde | None | verb | changed | to need / have to | needed to | meaning now matches this surface form / context |
| b2-01 | bewijzen | None | verb | changed | to prove / proved | to prove | meaning now matches this surface form / context |

**554 entries changed, added, or removed.**

## Per-occurrence entries

These forms genuinely mean different things in different sentences, so they use the optional `occurrence` field. The entry without `occurrence` is the default for that form.

| Story | Form | Occ | POS | Meaning |
| --- | --- | ---: | --- | --- |
| a1-01 | zijn | 1 | verb | are |
| a1-01 | zijn | 2 | verb | are |
| a1-01 | zijn | 3 | verb | are |
| a1-01 | zijn | 4 | verb | are |
| a1-01 | zijn | 5 | verb | are |
| a1-01 | zijn | 6 | verb | are |
| a1-01 | zijn | 8 | verb | are |
| a1-01 | zijn | 9 | verb | are |
| a1-01 | zijn | 10 | verb | are |
| a1-01 | zijn | 11 | verb | are |
| a2-01 | Het | 1 | article | it |
| a2-01 | aan | 2 | preposition | after |
| a2-01 | aan | 3 | preposition | after |
| b1-01 | zijn | 1 | verb | are |
| b1-01 | als | 3 | conjunction | as |
| b1-01 | die | 1 | pronoun | that one |
| b1-01 | die | 2 | pronoun | that one |
| b1-01 | die | 4 | pronoun | who |
| b1-01 | die | 5 | pronoun | who |
| b1-01 | die | 8 | pronoun | which |
| b1-01 | aan | 3 | preposition | there you go |
| b1-01 | aan | 4 | preposition | of |
| b1-01 | aan | 5 | preposition | to |
| b2-01 | zijn | 9 | verb | to be |
| b2-01 | zijn | 11 | verb | to be |
| b2-01 | zijn | 18 | verb | are |
| b2-01 | te | 2 | adverb | too |
| b2-01 | als | 5 | conjunction | when |
| b2-01 | die | 1 | pronoun | which |
| b2-01 | die | 2 | pronoun | who |
| b2-01 | die | 3 | pronoun | it |
| b2-01 | die | 4 | pronoun | that one |
| b2-01 | ze | 13 | pronoun | they |
| b2-01 | alleen | 2 | adverb | alone |
| b2-01 | zoals | 2 | conjunction | as |
| b2-01 | uit | 1 | adverb | out of |
| b2-01 | uit | 2 | adverb | looked (in 'eruitzien') |
| b2-01 | uit | 3 | adverb | ran over (in 'uitlopen') |
| b2-01 | uit | 4 | adverb | postpone (in 'uitstellen') |

## Uncertain or intentionally combined results

These are the results I could not make a single exact English word for, or where the glossary deliberately keeps a combined gloss. They are not bugs, but they are the honest limits of the audit.

| Story | Form | POS | Meaning | Why it stays combined |
| --- | --- | --- | --- | --- |
| b2-01 | voor | preposition | for / before | both senses occur; kept combined |
| b2-01 | van | preposition | of / from | possessive and origin uses both occur |
| b2-01 | wel | adverb | indeed / well | discourse particle; no single English gloss |
| b2-01 | al | adverb | already / all | two senses occur |
| b2-01 | eens | adverb | once / agreed | idiomatic uses |
| a2-01 | er | adverb | there | existential `er`; English `there` is the closest gloss |
| a2-01 | geen | determiner | no / not a | both English renderings fit different sentences |
| b1-01 | toch | adverb | yet / still / after all | discourse particle |
| b1-01 | maar | conjunction | but / only | both senses occur |
| b1-01 | nou | adverb | now / well | discourse particle |

Other notes:

- Inflected adjectives that translate the same as their lemma carry an explicit ` (inflected)` marker (for example `rode` = `red (inflected)`), so the `GLOSSARY_LEMMA_MEANING` rule can tell them apart without inventing a different English word.
- Noun plurals and diminutives use distinct wording where English has one (for example `Excuses` = `apologies`, `dutje` = `a little nap`).
- Entries with `text_count = 0` are starter words or lemma entries that the story needs for a base line but does not itself use as a token. They are intentional.
