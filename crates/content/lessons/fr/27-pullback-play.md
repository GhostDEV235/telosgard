+++
id = 27
stage = 5
title = "Le jeu du repli"
objective = "trader pas à pas un repli dans une tendance : zone, confirmation, stop, objectif, et dire quand y renoncer."
hook = "L’or monte joliment, mais tu as raté le début. Tu lui cours après, ou tu attends qu’il revienne vers toi ?"
analogy = "Pense à la marée qui monte. Même quand l’eau monte, chaque vague glisse en arrière avant que la suivante aille plus loin sur le sable. Tu ne cours pas après une vague sur la plage ; tu attends qu’elle reflue, puis tu entres quand la suivante commence."
analogy_tag = "marée et vagues"
example = "Tendance haussière en H4, dernier creux plus haut à 4 090. En H1, le prix se replie dans la zone 4 088 à 4 093, une ancienne résistance devenue support, et un avalement haussier clôture à 4 096. Entrée 4 096, stop sous la figure à 4 085 : 11 $ de risque. Objectif le dernier sommet à 4 118 : 22 $ plus loin, deux fois le risque. Taille la position pour qu’une perte fasse 1 % du compte. (Prix donnés à titre d’exemple.)"
mistake = "Acheter le repli avant qu’il s’arrête. Un prix qui descend dans une zone n’est pas un setup tant qu’une bougie de confirmation n’y a pas clôturé ; certains replis continuent et deviennent des retournements (un CHoCH, leçon 25)."
takeaways = [
  "Le jeu du repli a besoin d’une tendance claire sur l’unité supérieure.",
  "Attends que le prix se replie dans une zone, puis qu’une bougie de confirmation dans le sens de la tendance y clôture.",
  "Stop au-delà de la mèche de la bougie et de la zone ; objectif la zone suivante ; au moins deux fois le risque.",
  "Pas de tendance claire, pas de zone, pas de confirmation ou pas assez de gain possible : renonce, et note-le comme setup écarté.",
]
drills = ["name_candle"]

[[steps]]
heading = "La tendance d’abord"
text = "Le jeu du repli n’existe que dans une **tendance** : en H4, des sommets et des creux plus hauts, ou l’inverse (leçon 25). Pas de tendance claire, pas de jeu du repli. C’est le premier des deux jeux du Price Action classique."
why = "Les débutants l’apprennent en général en premier parce qu’il trade avec le grand mouvement, pas contre lui."

[[steps]]
heading = "Attends le repli vers une zone"
text = "Au lieu de courir après le prix, attends qu’il se **replie** contre la tendance dans une zone : souvent une ancienne résistance devenue support (leçon 24), ou près du dernier creux plus haut. Regarde-le arriver en H1."
why = "Courir après, c’est acheter après le mouvement, avec un stop très loin. Attendre place ton entrée près d’un niveau, donc le stop peut être plus proche et la position bien dimensionnée."

[[steps]]
heading = "La confirmation dans la zone"
text = "Une fois le prix dans la zone, attends qu’une **bougie de confirmation** dans le sens de la tendance (leçon 26) **clôture**. Pas de confirmation, pas de trade : le repli peut continuer et se transformer en retournement."
why = "Si le prix casse plutôt le dernier creux plus haut, c’est un CHoCH : la tendance s’essouffle peut-être, donc le jeu est annulé."

[[steps]]
heading = "Stop, objectif et taille"
text = "Place le **stop** au-delà de la mèche de la bougie de confirmation et de la zone. Vise la **zone suivante** dans le sens de la tendance, en général le dernier sommet. Le gain possible doit faire au moins **deux fois** le risque ; sinon, renonce. Ensuite, taille la position pour qu’une perte fasse environ 1 %."
why = "Le calculateur de taille de position transforme la distance du stop en taille de lot (leçons 8 et 9). L’étape 6 du cours ajoute Fibonacci pour affiner l’endroit où les replis marquent une pause."

[[steps]]
heading = "Quand renoncer"
text = "Renonce quand la tendance H4 n’est pas claire, que la zone est au milieu de nulle part, qu’il n’y a pas de confirmation, que le gain possible fait moins de deux fois le risque, ou qu’une Fenêtre sans trade approche (leçon 15). Un setup écarté est une bonne décision ; note-le comme tel."
why = "Les setups écartés vont aussi dans ton journal. Ils ne comptent jamais pour l’accès au réel, mais ils montrent que ton plan fonctionne."

[[videos]]
id = "LpWz8hswCQo"
title = "Pullback Trading Explained (For Beginners)"
channel = "Rayner Teo"
duration = "6:43"
watch_for = "Pourquoi trader le repli vaut mieux que courir après le mouvement. Ses règles d’entrée exactes peuvent différer de celles de ce cours."
fallback_search = "pullback trading explained beginners"
checked = "2026-11-13"

[[videos]]
id = "4MPR-7bickE"
title = "Do You Make This Pullback Trading Mistake?"
channel = "Rayner Teo"
duration = "20:05"
watch_for = "L’erreur d’acheter un repli avant qu’il montre des signes d’arrêt."
fallback_search = "pullback trading mistake"
checked = "2026-11-13"

[[sources]]
claim = "Les marchés ont tendance à faire une pause aux anciens plus hauts et plus bas, et une résistance cassée a tendance à devenir support quand le prix y redescend."
name = "CME Group Education: Support and Resistance"
url = "https://www.cmegroup.com/education/courses/technical-analysis/support-and-resistance.html"
checked = "2026-11-13"

[[sources]]
claim = "Le graphique 1 heure sert à préparer des trades alignés sur la tendance de fond du marché."
name = "CME Group Education: Trading Simulator How-To Guide: Chart Time Frames"
url = "https://www.cmegroup.com/education/courses/using-the-trading-simulator/trading-sim-in-30-seconds-chart-time-frames.html"
checked = "2026-11-13"

[[quiz]]
id = "q1"
q = "De quoi le jeu du repli a-t-il besoin avant tout ?"
options = ["D’une tendance claire sur l’unité supérieure", "D’un chiffre rond", "D’une grosse annonce économique", "De trois indicateurs d’accord"]
answer = 0
why = "Pas de tendance, pas de repli."

[[quiz]]
id = "q2"
q = "H4 est en tendance haussière. Le prix se replie dans une zone de support en H1. Qu’attends-tu ?"
options = ["Qu’une bougie de confirmation haussière clôture dans la zone", "Rien : j’achète à l’arrivée", "Un avalement baissier", "Le week-end"]
answer = 0
why = "Une confirmation dans le sens de la tendance, après la clôture."

[[quiz]]
id = "q3"
q = "Entrée 4 096, stop 4 085. La zone suivante est à 4 118. Le gain possible fait-il au moins deux fois le risque ?"
options = ["Oui : 22 $ de gain possible pour 11 $ de risque", "Non : c’est moins de 1 pour 1", "Exactement 1 pour 1", "Impossible à dire sans indicateurs"]
answer = 0
why = "4 118 − 4 096 = 22 $ ; 4 096 − 4 085 = 11 $ ; 22 ÷ 11 = 2."

[[quiz]]
id = "q4"
q = "Entrée 4 096, stop 4 085, zone suivante 4 104. Que fais-tu ?"
options = ["Renoncer : 8 $ de gain possible pour 11 $ de risque, ce n’est pas assez", "Le prendre quand même", "Remonter le stop à 4 095", "Doubler la taille"]
answer = 0
why = "Sous deux fois le risque, renonce. Ne rapproche jamais le stop pour arranger les chiffres."

[[quiz]]
id = "q5"
q = "Pendant le repli, le prix clôture sous le dernier creux plus haut. Et maintenant ?"
options = ["Le jeu est annulé : c’est un CHoCH, la tendance s’essouffle peut-être", "Acheter plus, à meilleur prix", "C’est un setup encore meilleur", "Descendre le stop"]
answer = 0
why = "Un creux plus haut cassé veut dire que la structure de la tendance a changé."

[[quiz]]
id = "q6"
q = "Où va le stop dans le jeu du repli ?"
options = ["Au-delà de la mèche de la bougie de confirmation et de la zone", "Exactement au prix d’entrée", "À l’intérieur de la zone", "Là où la perte est la plus petite"]
answer = 0
why = "Au-delà de la mèche et de la zone, pour que le bruit normal ne le touche pas."

[[recall]]
id = "r1"
q = "Le jeu du repli trade…"
options = ["Avec la tendance, après une pause", "Contre la tendance", "Seulement dans les ranges"]
answer = 0
why = "La tendance d’abord, puis le repli."

[[recall]]
id = "r2"
q = "Le gain possible minimum par rapport au risque, c’est…"
options = ["Deux fois le risque", "Autant que le risque", "La moitié du risque"]
answer = 0
why = "En dessous de 2 pour 1, renonce."

[[recall]]
id = "r3"
q = "Un setup écarté, c’est…"
options = ["Une bonne décision, notée comme écartée", "Un échec"]
answer = 0
why = "Suivre le plan, c’est aussi savoir renoncer."

[explain_back]
prompt = "Décris le jeu du repli du début à la fin. Quand y renoncerais-tu ?"
must_include = ["il faut une tendance claire sur l’unité supérieure ; tu attends que le prix se replie dans une zone au lieu de lui courir après", "tu n’entres qu’une fois qu’une bougie de confirmation dans le sens de la tendance a clôturé dans la zone", "le stop va au-delà de la mèche et de la zone et l’objectif sur la zone suivante, avec au moins deux fois le risque ; sinon, ou sans tendance, zone ou confirmation, tu renonces"]
misconceptions = ["on peut acheter dès que le prix se replie", "un repli prolonge toujours la tendance"]

[[explain_back.fixtures]]
answer = "D’abord il me faut une tendance claire en H4. Ensuite j’attends que le prix se replie dans une zone au lieu de lui courir après, et je n’entre qu’une fois qu’une bougie de confirmation dans le sens de la tendance y a clôturé. Mon stop va au-delà de la mèche de cette bougie et de la zone, et mon objectif, c’est la zone suivante ; si ça fait moins de deux fois mon risque, ou s’il n’y a pas de tendance claire, de zone ou de confirmation, je renonce et je le note comme setup écarté."
expect = "got_it"

[[explain_back.fixtures]]
answer = "Dans une tendance H4, j’attends un repli vers une zone, puis une bougie de confirmation, avec mon stop au-delà de la mèche et mon objectif sur la zone suivante."
expect = "almost"

[[explain_back.fixtures]]
answer = "Quand le prix baisse dans une tendance haussière, j’achète tout de suite, parce que les replis prolongent toujours la tendance."
expect = "almost_or_not_yet"
+++
