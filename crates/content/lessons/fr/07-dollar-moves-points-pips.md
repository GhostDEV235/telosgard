+++
id = 7
stage = 1
title = "Mouvements en dollars, points et pips"
objective = "mesurer n’importe quel mouvement en dollars et convertir les « pips » des autres avant de leur faire confiance."
hook = "Une vidéo dit : « Sur l’or, je ne risque que 50 pips. » C’est beaucoup ou peu ?"
analogy = "Pense à une règle graduée sur trois échelles : centimètres, millimètres et mètres. Un crayon mesure 15 cm, 150 mm ou 0,15 m : la même longueur, trois nombres différents. Mouvements en dollars, points et pips sont trois échelles sur la même règle. Les ennuis commencent quand deux personnes utilisent des échelles différentes mais disent le même mot."
analogy_tag = "règle à trois échelles"
example = "L’or passe de 4 150,00 à 4 153,00. C’est un mouvement de 3 $. Dans ce cours, ça fait aussi 300 points (à 0,01 $ chacun) ou 30 pips (à 0,10 $ chacun). Mais quelqu’un qui appelle 1 $ un pip dirait « 3 pips », et quelqu’un qui appelle 0,01 $ un pip dirait « 300 pips ». Même mouvement, trois nombres différents. (Prix donnés à titre d’exemple.)"
mistake = "Croire qu’un pip veut dire la même chose partout, ou que les pips sont une somme d’argent. Un pip n’est qu’une distance, et pour l’or, chacun utilise sa propre taille. Convertis toujours d’abord en mouvement en dollars."
takeaways = [
  "Ce cours mesure toute distance en Mouvement en dollars.",
  "Ici, un point vaut 0,01 $ et un pip 0,10 $.",
  "Quand d’autres disent « pip » pour l’or, ils peuvent parler de 0,01 $, 0,10 $ ou 1 $.",
  "Convertis toute affirmation en « pips » en dollars avant de la croire ou de la copier.",
]
drills = ["pip_convert"]

[[videos]]
id = "3jwixTmgHUg"
title = "How to Calculate Pips in Forex"
channel = "The Moving Average"
duration = "6:27"
watch_for = "L’idée qu’un pip est une distance, pas de l’argent. La vidéo prend des paires de devises, dont la taille de pip diffère de celle de l’or ; convertis les chiffres pour l’or avec la règle de cette leçon."
fallback_search = "how to calculate pips explained"
checked = "2026-11-10"

[[sources]]
claim = "L’or est coté en dollars américains par once troy, donc un mouvement peut toujours se mesurer en dollars."
name = "LBMA : About LBMA daily auction prices"
url = "https://www.lbma.org.uk/prices-and-data/about-lbma-daily-auction-prices"
checked = "2026-11-10"

[[quiz]]
id = "q1"
q = "L’or passe de 4 150,00 à 4 153,00. Quel est le Mouvement en dollars ?"
options = ["0,30 $", "3 $", "30 $", "300 $"]
answer = 1
why = "4 153 − 4 150 = 3 $."

[[quiz]]
id = "q2"
q = "Dans ce cours, combien vaut un pip sur l’or ?"
options = ["0,01 $", "0,10 $", "1 $", "10 $"]
answer = 1
why = "Ici, un pip = 0,10 $ et un point = 0,01 $."

[[quiz]]
id = "q3"
q = "Un mouvement de 5 $, c’est combien de pips dans ce cours ?"
options = ["5", "50", "500", "0,5"]
answer = 1
why = "5 $ ÷ 0,10 $ par pip = 50 pips."

[[quiz]]
id = "q4"
q = "Quelqu’un parle d’un « stop de 30 pips sur l’or ». Que dois-tu faire en premier ?"
options = ["Le copier tel quel", "Demander ou deviner quelle taille de pip il utilise, puis convertir en dollars", "Multiplier par 100", "Ignorer complètement les stops"]
answer = 1
why = "« 30 pips » peut vouloir dire 0,30 $, 3 $ ou 30 $ selon la personne. Convertis d’abord."

[[quiz]]
id = "q5"
q = "Quelle affirmation est vraie ?"
options = ["Un pip a la même taille sur tous les graphiques", "Un pip est une somme d’argent", "Un pip est une distance, et sa taille dépend de qui emploie le mot", "Les pips ne concernent que les actions"]
answer = 2
why = "Les pips mesurent une distance, et pour l’or leur taille varie selon la source."

[explain_back]
prompt = "Un youtubeur parle d’un « stop de 50 pips sur l’or ». Que fais-tu avant de lui faire confiance ?"
must_include = ["le convertir en mouvement en dollars", "la taille du pip varie selon les sources", "dans ce cours, un point vaut 0,01 $ et un pip 0,10 $"]
misconceptions = ["un pip veut dire la même chose partout", "les pips sont une somme d’argent"]

[[explain_back.fixtures]]
answer = "Je le convertis d’abord en mouvement en dollars, parce que tout le monde n’utilise pas la même taille de pip pour l’or. Dans ce cours, un pip vaut 0,10 $ et un point 0,01 $, donc si c’est ce qu’il veut dire, 50 pips font un stop de 5 $, mais il peut parler d’autre chose."
expect = "got_it"

[[explain_back.fixtures]]
answer = "Je le convertis en dollars, parce que 50 pips peut vouloir dire différentes choses selon la personne."
expect = "almost"

[[explain_back.fixtures]]
answer = "Rien, 50 pips c’est 50 pips sur tous les graphiques. Ça veut dire qu’on perdrait 50 $."
expect = "almost_or_not_yet"

[[steps]]
heading = "D’abord, le Mouvement en dollars"
text = "Tu mesures déjà un mouvement par soustraction : de 4 150,00 à 4 153,00, c’est un **mouvement de 3 $**. Ce cours appelle ça un **Mouvement en dollars**, et l’utilise partout : stops, objectifs, ton journal et ton Plan de trading."
[[steps]]
heading = "Points et pips"
text = "Un **point** vaut **0,01 $**, le plus petit pas qu’affiche une cotation de l’or. **Dans ce cours, un pip vaut 0,10 $.** Un mouvement de 3 $ fait donc 300 points ou 30 pips, et un mouvement de 5 $, 500 points ou 50 pips."
why = "| Mouvement en dollars | Points (0,01 $) | Pips (0,10 $) |\n|---|---|---|\n| 0,30 $ | 30 | 3 |\n| 3 $ | 300 | 30 |\n| 5 $ | 500 | 50 |\n| 10 $ | 1 000 | 100 |"
[[steps]]
heading = "Le problème des pips"
text = "L’or n’a pas de taille de pip commune. Certains parlent de **0,01 $**, d’autres de **0,10 $**, d’autres de **1 $**. « 50 pips » peut donc valoir **0,50 $, 5 $ ou 50 $** : un écart de un à cent quand quelqu’un te dit où placer ton stop-loss."
why = "Le mot pip vient du trading des devises, où il a un sens clair. L’or n’en a jamais eu un qui fasse consensus."
widget = "pip_convert"
[[steps]]
heading = "Ta règle"
text = "Chaque fois que tu entends une distance en pips ou en points pour l’or, **convertis-la d’abord en Mouvement en dollars**. Si tu ne sais pas quelle taille la personne utilise, ne te fie pas au nombre. Et souviens-toi : un pip est une distance, pas une somme d’argent."
why = "Ce que vaut pour toi un mouvement de 5 $ dépend de la taille de ton trade : c’est la leçon suivante. « J’ai fait 50 pips » ne dit rien sur l’argent que quelqu’un a gagné."
[[recall]]
id = "r1"
q = "L’or passe de 4 150,00 à 4 153,00. Le Mouvement en dollars est de…"
options = ["3 $", "30 $", "0,30 $"]
answer = 0
why = "4 153 − 4 150 = 3 $."
[[recall]]
id = "r2"
q = "Dans ce cours, un pip sur l’or vaut…"
options = ["0,10 $", "0,01 $", "1 $"]
answer = 0
why = "Un point vaut 0,01 $ et un pip 0,10 $."
[[recall]]
id = "r3"
q = "Quelqu’un parle d’un « stop de 30 pips sur l’or ». C’est…"
options = ["0,30 $, 3 $ ou 30 $, selon ce qu’il veut dire", "Toujours 3 $", "Toujours 30 $"]
answer = 0
why = "La taille du pip ne fait pas consensus pour l’or : convertis avant de te fier au chiffre."
+++
