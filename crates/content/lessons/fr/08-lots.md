+++
id = 8
stage = 1
title = "Les lots : quelle est la taille de ton trade ?"
objective = "transformer une taille en lots et un mouvement en dollars en argent gagné ou perdu."
hook = "Deux traders profitent du même mouvement de 4 $ sur l’or. L’un gagne 4 $, l’autre 400 $. Comment ?"
analogy = "Imagine que tu achètes par cartons. Un lot est un carton de 100 onces d’or. Un dixième de lot (0,10) fait 10 onces, et un centième (0,01) une seule once. Quand le prix de l’or bouge, chaque once du carton bouge avec lui : un plus gros carton, ce sont de plus gros gains et de plus grosses pertes."
analogy_tag = "cartons d’onces"
example = "L’or monte de 4 150,00 à 4 154,00, un mouvement de 4 $. Avec 1 lot (100 oz), ça fait 100 × 4 $ = 400 $. Avec 0,20 lot (20 oz), 20 × 4 $ = 80 $. Avec 0,01 lot (1 oz), 4 $. Même mouvement, sommes très différentes. Si l’or avait baissé de 4 $, ce seraient toutes des pertes. (Prix donnés à titre d’exemple ; 100 oz par lot.)"
mistake = "Croire qu’un lot est une somme d’argent fixe, ou que le gain ne dépend que de la distance parcourue par le prix. L’argent dépend à la fois du mouvement et de la taille de ton trade."
takeaways = [
  "Un lot est une taille : dans ce cours, 1 lot = 100 onces d’or.",
  "Argent = lots × 100 × Mouvement en dollars.",
  "0,01 lot est en général le plus petit trade possible.",
  "Ton courtier fixe la taille du contrat : vérifie-la avant de trader.",
]
drills = ["dollar_distance"]

[[videos]]
id = "GJ-Bjq1Xvks"
title = "Forex Leverage for Beginners Explained (lot sizes and pips)"
channel = "The Moving Average"
duration = "7:17"
watch_for = "Comment la taille d’un trade change l’argent gagné ou perdu sur un même mouvement. La vidéo prend des paires de devises ; pour l’or, utilise le carton de 100 onces et les Mouvements en dollars de cette leçon."
fallback_search = "lot size explained beginners"
checked = "2026-11-10"

[[sources]]
claim = "Dans MetaTrader 5, le volume minimal d’un trade et son pas sont fixés par le courtier."
name = "Aide MetaTrader 5 : Trade"
url = "https://www.metatrader5.com/en/mobile-trading/iphone/help/trade"
checked = "2026-11-10"

[[quiz]]
id = "q1"
q = "Dans ce cours, combien d’onces d’or fait 1 lot ?"
options = ["1", "10", "100", "1 000"]
answer = 2
why = "On utilise le réglage courant 1 lot = 100 onces. Vérifie-le toujours dans la fiche de ton courtier."

[[quiz]]
id = "q2"
q = "Tu as 0,20 lot et l’or monte de 4 $. Combien gagnes-tu ?"
options = ["0,80 $", "8 $", "80 $", "800 $"]
answer = 2
why = "0,20 × 100 × 4 $ = 80 $."

[[quiz]]
id = "q3"
q = "Tu as 1 lot et l’or baisse de 3 $ contre toi. Quel est le résultat ?"
options = ["−3 $", "−30 $", "−300 $", "−3 000 $"]
answer = 2
why = "1 × 100 × 3 $ = 300 $, et le mouvement est allé contre toi : c’est une perte de 300 $."

[[quiz]]
id = "q4"
q = "Quelle taille de trade fait 1 $ pour chaque dollar de mouvement de l’or ?"
options = ["0,01 lot", "0,10 lot", "1 lot", "10 lots"]
answer = 0
why = "0,01 lot, c’est 1 once : 0,01 × 100 × 1 $ = 1 $."

[[quiz]]
id = "q5"
q = "Deux traders profitent du même mouvement de 5 $. Pourquoi l’un pourrait-il gagner bien plus que l’autre ?"
options = ["Leurs courtiers affichent des prix différents", "Ils ont tradé des tailles de lot différentes", "L’un a un meilleur graphique", "Les pips valent plus le vendredi"]
answer = 1
why = "Même mouvement, taille différente : argent = lots × 100 × Mouvement en dollars."

[explain_back]
prompt = "Tu achètes 0,20 lot et l’or monte de 4 $. Combien as-tu gagné, et pourquoi ?"
must_include = ["ce que représente le 100 : 1 lot fait en général 100 onces (à vérifier chez le courtier), donc 0,20 lot fait 20 onces", "argent = lots × 100 × mouvement en dollars", "donc 0,20 lot × 4 $ = 80 $"]
misconceptions = ["un lot est une somme d’argent fixe", "le gain ne dépend que du mouvement du prix, pas de la taille"]

[[explain_back.fixtures]]
answer = "Un lot standard fait 100 onces, même s’il faut vérifier chez son courtier, donc 0,20 lot fait 20 onces. Chaque once a gagné 4 $, ce qui donne 0,20 × 100 × 4 $ = 80 $."
expect = "got_it"

[[explain_back.fixtures]]
answer = "80 $, parce que 0,20 lot fois 4 $ fois 100 font 80."
expect = "almost"

[[explain_back.fixtures]]
answer = "4 $, parce que le prix a monté de 4 $ et que c’est ce qu’on gagne, quel que soit le nombre de lots."
expect = "almost_or_not_yet"

[[steps]]
heading = "Ce qu’est un lot"
text = "Un **lot** est une taille de trade standard. Ce cours utilise le réglage courant **1 lot = 100 onces**. Donc 0,10 lot fait 10 onces, et 0,01 lot, en général la plus petite taille, fait 1 once."
why = "| Lots | Onces | Argent par mouvement de 1 $ |\n|---|---|---|\n| 1,00 | 100 | 100 $ |\n| 0,10 | 10 | 10 $ |\n| 0,01 | 1 | 1 $ |\n\nDans MetaTrader 5, la taille minimale et ses paliers sont fixés par ton courtier ; tu les vérifieras à la leçon 12."
[[steps]]
heading = "La formule de l’argent"
text = "Chaque résultat vient de trois nombres : **argent = lots × 100 × Mouvement en dollars**. 0,20 lot et l’or qui monte de 4 $ : 0,20 × 100 × 4 = **80 $**. Le même mouvement de 4 $ avec 1 lot, c’est 400 $."
why = "Un mouvement de 10 $, que l’or peut faire en quelques minutes un jour agité, vaut 1 000 $ avec 1 lot. C’est pour ça que la taille, et pas seulement le sens, décide si une erreur est une leçon ou une catastrophe."
widget = "lots_money"
[[steps]]
heading = "Décide d’abord de la perte"
text = "Avant chaque trade, décide du maximum que tu acceptes de perdre. Un plafond courant est **environ 1 % de ton compte** par trade : 100 $ sur 10 000 $. Choisis ensuite une taille pour que toucher ton stop-loss coûte à peu près cette somme."
why = "De petites pertes te gardent dans la partie : dix pertes d’affilée à 1 % chacune laissent encore environ 90 % du compte. La leçon 33 en fait un calculateur que tu utiliseras à chaque trade."
widget = "one_percent"
[[steps]]
heading = "Vérifie la taille du contrat chez ton courtier"
text = "Tous les courtiers n’utilisent pas 100 onces par lot pour XAUUSD. La **taille du contrat** figure dans la fiche du symbole chez ton courtier. Tant que tu ne l’as pas vérifiée, considère « 1 lot = 100 onces » comme une hypothèse, pas comme un fait."
[[recall]]
id = "r1"
q = "0,10 lot et l’or bouge de 5 $ dans ton sens. Tu gagnes…"
options = ["50 $", "5 $", "500 $"]
answer = 0
why = "0,10 × 100 × 5 $ = 50 $."
[[recall]]
id = "r2"
q = "1 % d’un compte de 5 000 $, c’est…"
options = ["50 $", "500 $", "5 $"]
answer = 0
why = "5 000 $ × 1 % = 50 $."
[[recall]]
id = "r3"
q = "La plus petite taille que la plupart des plateformes acceptent est en général…"
options = ["0,01 lot", "1 lot", "0,5 lot"]
answer = 0
why = "0,01 lot, c’est 1 once : 1 $ par mouvement de 1 $."
+++
