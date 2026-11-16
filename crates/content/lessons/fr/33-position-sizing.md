+++
id = 33
stage = 7
title = "La taille de position"
objective = "transformer un solde, un risque d’au plus 1 % et une distance de stop en taille en lots, arrondie vers le bas, en moins de 30 secondes, et savoir quand un trade est trop petit pour être pris."
hook = "Deux traders prennent le même trade avec le même stop. L’un perd 96 $ quand il échoue, l’autre 1 600 $. Qu’ont-ils fait de différent ?"
analogy = "Pense à une recette pour quatre quand tu cuisines pour deux. Tu ne changes ni le four ni le temps de cuisson ; tu ajustes les quantités. En trading, le graphique fixe le stop, et tu ajustes la taille pour que la perte tienne dans ton plan."
analogy_tag = "ajuster une recette"
example = "Le trade de la leçon 32 : le stop est à 16 $ de l’entrée. Ton solde est de 10 000 $ et tu risques 1 % : 100 $. Un lot perd 16 $ × 100 oz = 1 600 $ si le stop est touché. 100 $ ÷ 1 600 $ = 0,0625, arrondi vers le bas à 0,06 lot. Si le stop est touché, tu perds 96 $, juste sous ton 1 %. (Prix donnés à titre d’exemple.)"
mistake = "Faire l’inverse : choisir une taille qui te plaît (« 0,50 lot, ça me va ») puis déplacer le stop pour que la perte ait l’air acceptable. Le stop vient du graphique ; la taille est le seul chiffre que tu as le droit de changer."
takeaways = [
  "Le stop décide de la taille, jamais l’inverse.",
  "Lots = (solde × % de risque) ÷ (distance du stop × 100 oz), avec la taille de contrat de ton courtier.",
  "Arrondis vers le bas au 0,01 lot, pour ne jamais risquer plus que prévu.",
  "Le plafond de ce cours est 1 % du solde par trade ; si même 0,01 lot risque davantage, le trade est trop petit pour être pris.",
]
drills = ["size_it"]

[[steps]]
heading = "Le stop d’abord"
text = "La taille se calcule une fois le stop placé (leçon 32). Il te faut deux chiffres : la **distance du stop** en dollars, et la **somme que tu acceptes de perdre**. La taille est ce qui met ces deux chiffres d’accord. Un stop large donne une petite taille ; un stop serré, une plus grande."
why = "Le cours de CME Group sur la taille de position le dit aussi : pour fixer une taille, il faut savoir où est ton stop et quelle part du compte tu acceptes de risquer."

[[steps]]
heading = "Risque au plus 1 %"
text = "Fixe la somme en risque comme une part du solde. Le plafond de ce cours est **1 %** : 100 $ sur 10 000 $, 50 $ sur 5 000 $. Moins, c’est toujours permis. Comme c’est un pourcentage, ton risque baisse quand le compte baisse et ne grandit qu’avec lui."
why = "CME Group conseille de commencer avec un risque modeste de 1 à 3 % par trade. Ce cours prend le bas de cette fourchette, pour qu’une série de pertes reste supportable ; la leçon 35 montre pourquoi."
widget = "one_percent"

[[steps]]
heading = "Le calcul de la taille"
text = "**Lots = somme en risque ÷ (distance du stop × 100)**. Le 100, ce sont les onces d’un lot (leçon 8). 100 $ en risque avec un stop à 16 $ : un lot perdrait 1 600 $, donc 100 $ ÷ 1 600 $ = **0,0625 lot**."
why = "Vérifie d’abord la taille de contrat de ton courtier : si un lot fait 10 oz chez lui, le calcul utilise 10 au lieu de 100. Le calculateur de l’onglet Outils te permet de la régler."

[[steps]]
heading = "Toujours vers le bas"
text = "Les plateformes avancent par pas de **0,01 lot**, donc 0,0625 devient **0,06**, jamais 0,07. Arrondir vers le bas risque un peu moins que prévu (96 $) ; vers le haut, plus (112 $). Le calculateur de l’onglet **Outils** fait le calcul et l’arrondi pour toi."
why = "0,06 lot × 100 oz = 6 oz ; × 16 $ = 96 $. À 0,07 lot, 7 oz × 16 $ = 112 $, ce qui dépasse le plafond de 1 %."

[[steps]]
heading = "Trop petit pour trader"
text = "Sur un petit compte, un stop large peut échouer même à **0,01 lot**, la plus petite taille. 500 $ à 1 %, c’est 5 $, mais 0,01 lot avec un stop à 8 $ perd 8 $ (1,6 %). La réponse est alors **pas de trade**, pas un stop plus serré."
why = "Le calculateur le signale comme trop petit pour trader sans risque excessif. Un setup avec un stop plus proche, et honnête, viendra peut-être ; celui-ci n’est pas pour toi."

[[videos]]
id = "VFVcvWpnbd4"
title = "How to Measure LOT SIZE For GOLD Trading. XAUUSD FREE Position Size Calculator Explained"
channel = "VasilyTrader"
duration = "4:27"
watch_for = "Le calcul d’une taille en lots sur l’or à partir du compte, du risque et du stop. Il mesure le stop dans les unités de sa plateforme ; convertis-le d’abord en dollars de prix, comme dans la leçon 7."
fallback_search = "xauusd lot size calculation"
checked = "2026-11-16"

[[videos]]
id = "q5UiDAk1740"
title = "Forex Trading: Risk Management And Position Sizing (Video 6 of 13)"
channel = "Rayner Teo"
duration = "9:49"
watch_for = "Pourquoi le stop vient d’abord et la taille ensuite. Ses exemples portent sur des paires de devises ; la logique est la même pour l’or avec 100 oz par lot."
fallback_search = "risk management position sizing explained"
checked = "2026-11-16"

[[sources]]
claim = "Pour déterminer la taille d’une position, il faut savoir où l’on place son stop et quel pourcentage ou montant du compte on accepte de risquer ; quand on débute, mieux vaut risquer un modeste 1 à 3 % du compte par trade ; une fois le stop fixé, le risque par contrat est connu et le nombre de contrats en découle."
name = "CME Group Education: Proper Position Size"
url = "https://www.cmegroup.com/education/courses/trade-and-risk-management/proper-position-size.html"
checked = "2026-11-16"

[[sources]]
claim = "Avec une règle en pourcentage fixe, la somme en risque baisse à chaque trade quand le compte baisse : à 1 %, 500 $ sur 50 000 $, puis 495 $, 490 $ et ainsi de suite pendant une série de pertes, ce qui ralentit la baisse du compte."
name = "CME Group Education: Controlling Risk"
url = "https://www.cmegroup.com/education/courses/trade-and-risk-management/controlling-risk.html"
checked = "2026-11-16"

[[quiz]]
id = "q1"
q = "Quel chiffre changes-tu pour que la perte tienne dans ton plan ?"
options = ["La taille", "Le stop", "L’entrée", "L’objectif"]
answer = 0
why = "Le stop vient du graphique ; seule la taille est à toi."

[[quiz]]
id = "q2"
q = "Solde 5 000 $, risque 1 %, stop à 10 $. Quelle taille ?"
options = ["0,05 lot", "0,50 lot", "0,10 lot", "0,01 lot"]
answer = 0
why = "50 $ en risque ; un lot perd 10 $ × 100 = 1 000 $ ; 50 $ ÷ 1 000 $ = 0,05."

[[quiz]]
id = "q3"
q = "Le calcul donne 0,0875 lot. Que trades-tu ?"
options = ["0,08 lot", "0,09 lot", "0,10 lot", "0,0875 lot"]
answer = 0
why = "Arrondis vers le bas au pas de 0,01 pour ne jamais risquer plus que prévu."

[[quiz]]
id = "q4"
q = "Solde 10 000 $, risque 1 %, stop à 7 $. Combien perds-tu si le stop est touché ?"
options = ["98 $, avec 0,14 lot", "100 $ pile", "700 $", "105 $, avec 0,15 lot"]
answer = 0
why = "100 $ ÷ 700 $ = 0,1428… → 0,14 lot ; 14 oz × 7 $ = 98 $."

[[quiz]]
id = "q5"
q = "Un compte de 500 $, 1 % de risque, et le graphique demande un stop à 8 $. Quelle est la réponse ?"
options = ["Pas de trade : même 0,01 lot perdrait 8 $, soit 1,6 %", "Resserrer le stop à 5 $", "Trader 0,01 lot quand même", "Trader sans stop"]
answer = 0
why = "Quand la plus petite taille dépasse le plafond, le trade est trop petit pour être pris."

[[quiz]]
id = "q6"
q = "Chez ton courtier, le contrat or fait 10 oz par lot. Solde 10 000 $, 1 %, stop à 5 $. Quelle taille ?"
options = ["2,00 lots", "0,20 lot", "20,00 lots", "0,02 lot"]
answer = 0
why = "Un lot perd 5 $ × 10 = 50 $ ; 100 $ ÷ 50 $ = 2,00. Vérifie toujours la taille de contrat."

[[recall]]
id = "r1"
q = "Qu’est-ce qui vient en premier ?"
options = ["Le stop, puis la taille", "La taille, puis le stop", "Aucun : on devine les deux"]
answer = 0
why = "Le stop décide de la taille, jamais l’inverse."

[[recall]]
id = "r2"
q = "Lots = …"
options = ["(solde × % de risque) ÷ (distance du stop × 100)", "solde ÷ distance du stop", "% de risque × 100"]
answer = 0
why = "La somme en risque divisée par ce qu’un lot perd au stop."

[[recall]]
id = "r3"
q = "Quand le calcul ne tombe pas sur un pas de 0,01, tu…"
options = ["Arrondis vers le bas", "Arrondis vers le haut", "Arrondis au plus proche"]
answer = 0
why = "Vers le bas, pour que la perte reste dans le plan."

[explain_back]
prompt = "Comment calcules-tu ta taille de position, et que fais-tu quand elle est trop petite ?"
must_include = ["le stop vient d’abord du graphique et décide de la taille, jamais l’inverse", "les lots valent la somme en risque, au plus 1 % du solde, divisée par la distance du stop fois les onces par lot, en général 100, arrondie vers le bas à 0,01", "si même 0,01 lot risque plus que le plan, le trade est trop petit et tu l’écartes au lieu de resserrer le stop"]
misconceptions = ["on choisit d’abord une taille et on adapte le stop autour", "arrondir vers le haut, ce n’est pas grave, c’est proche"]

[[explain_back.fixtures]]
answer = "D’abord le graphique me donne le stop, puis je calcule la taille à partir de lui, jamais l’inverse. Je prends au plus 1 % de mon solde comme somme en risque, je la divise par la distance du stop fois 100 oz, ou la taille de contrat de mon courtier, et j’arrondis vers le bas à 0,01 lot. Si même 0,01 lot perdrait plus de 1 %, le trade est trop petit pour mon compte et je l’écarte au lieu de serrer le stop."
expect = "got_it"

[[explain_back.fixtures]]
answer = "Je prends 1 % de mon solde et je le divise par la distance du stop fois 100, puis j’arrondis vers le bas à 0,01 lot."
expect = "almost"

[[explain_back.fixtures]]
answer = "Je trade toujours 0,50 lot et je rapproche juste mon stop si la perte paraît trop grosse."
expect = "almost_or_not_yet"
+++
