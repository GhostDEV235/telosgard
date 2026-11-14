+++
id = 29
stage = 6
title = "Tracer Fibonacci correctement"
objective = "tracer une grille de Fibonacci sur la bonne jambe : une impulsion nette qui a cassé la structure, du creux au sommet de swing en tendance haussière (du sommet au creux en tendance baissière), de mèche à mèche."
hook = "Deux traders posent Fibonacci sur le même graphique de l’or et obtiennent des niveaux différents. Les deux grilles peuvent-elles être justes ?"
analogy = "Pense à un mètre ruban. Accroche-le au tout début de la planche et tire-le jusqu’au tout bout : chaque graduation entre les deux est juste. Accroche-le quelques centimètres plus loin, ou déroule-le à l’envers, et toutes les graduations suivantes sont fausses, même si tu les lis avec soin."
analogy_tag = "mètre ruban"
example = "Tendance haussière en H4. L’or monte d’un creux de swing à 4 060 jusqu’à un sommet de swing à 4 100, en clôturant au passage au-dessus de l’ancien sommet à 4 088 : un BOS, donc c’est la jambe d’impulsion. Tu traces de la mèche basse à 4 060 jusqu’à la mèche haute à 4 100, une jambe de 40 $. L’outil place alors 38,2 % à 4 084,72, 50 % à 4 080,00 et 61,8 % à 4 075,28 : jusqu’où un repli est revenu. (Prix donnés à titre d’exemple.)"
mistake = "Le tracer à l’envers. En tendance haussière, tirer du sommet vers le creux inverse la grille : la ligne marquée 38,2 % tombe là où devrait être 61,8 %, et chaque niveau que tu surveilles est au mauvais endroit."
takeaways = [
  "Fibonacci mesure jusqu’où un repli est revenu dans le dernier mouvement : 38,2 %, 50 % et 61,8 % sont les niveaux principaux.",
  "Trace-le sur la dernière jambe d’impulsion nette qui a cassé la structure, sur ton unité de temps de tendance.",
  "Tendance haussière : du creux au sommet de swing. Tendance baissière : du sommet au creux de swing. De mèche à mèche.",
  "Un nouveau sommet avant le repli, c’est une nouvelle jambe : retrace-la. Un prix sous le creux de la jambe, c’est une jambe cassée : supprime la grille.",
]
drills = ["pick_leg"]

[[steps]]
heading = "Ce que mesure Fibonacci"
text = "L’outil **Fibonacci** indique jusqu’où un repli est revenu dans le dernier mouvement. Ses niveaux principaux sont **38,2 %**, **50 %** et **61,8 %** de ce mouvement. Le cours de CME Group précise que 50 % n’est pas du tout un ratio de Fibonacci, mais que les traders le surveillent quand même."
why = "Les ratios viennent de la suite de Fibonacci (0, 1, 1, 2, 3, 5, 8, 13…, chaque nombre étant la somme des deux précédents) : divise un nombre par celui placé deux rangs plus loin et tu obtiens environ 0,382 ; 1 − 0,618 donne la même chose."

[[steps]]
heading = "Choisis la bonne jambe"
text = "Trace-le sur la **jambe d’impulsion** : le dernier mouvement fort dans le sens de la tendance qui a **cassé la structure** (un BOS, leçon 25), sur ton unité de temps de tendance, H4 dans ce cours. Pas sur le repli en cours, ni sur une jambe plus ancienne."
why = "Une jambe qui n’a pas cassé la structure n’est que du bruit dans un range. La plupart des désaccords entre les grilles de deux traders viennent du choix de la jambe, pas de l’outil."

[[steps]]
heading = "Du creux au sommet en hausse"
text = "En **tendance haussière**, trace du **creux de swing** au **sommet de swing**. En **tendance baissière**, du sommet au creux. L’outil mesure alors les replis contre le mouvement. Tracée à l’envers, la grille s’inverse et ses étiquettes désignent les mauvais prix."
why = "Vérification rapide : en tendance haussière, la ligne 0 % doit être en haut (le sommet) et 100 % en bas (le creux), avec 38,2 % au plus près du sommet."

[[steps]]
heading = "De mèche à mèche"
text = "Accroche la grille aux **points extrêmes** de la jambe : la mèche la plus basse et la mèche la plus haute, pas les corps des bougies. Le guide de Fidelity décrit l’outil comme tracé entre deux points extrêmes, et c’est ce qu’utilisent toutes les personnes qui regardent le graphique."
why = "Les niveaux ne comptent que parce que beaucoup de traders surveillent les mêmes. T’accrocher aux corps décale tes niveaux de quelques dollars par rapport aux leurs."

[[steps]]
heading = "Retracer, ou supprimer"
text = "Une grille appartient à une seule jambe. Si le prix fait un **nouveau sommet** avant de se replier, la jambe s’arrête là : **retrace-la** depuis le même creux. Si le prix passe **sous le creux de la jambe**, elle est cassée : **supprime** la grille ; ses niveaux ne veulent plus rien dire."
why = "La leçon 30 ajoute une ligne plus stricte utilisée dans ce cours : une clôture au-delà de 78,6 % met déjà fin au jeu du repli."

[[videos]]
id = "mnEe3uM4pyQ"
title = "How to Draw FIBONACCI Retracement For Trading"
channel = "Mind Math Money"
duration = "2:59"
watch_for = "Sur quel swing il accroche la grille, et dans quel sens il la tire en tendance haussière et baissière."
fallback_search = "how to draw fibonacci retracement correctly"
checked = "2026-11-13"

[[videos]]
id = "nYKpik-o8zI"
title = "Fibonacci Retracement explained in under 5 minutes"
channel = "The Moving Average"
duration = "3:49"
watch_for = "Un court tour des niveaux, et pourquoi la jambe choisie compte. Prends les taux de réussite cités comme des exemples, pas des promesses."
fallback_search = "fibonacci retracement explained"
checked = "2026-11-13"

[[sources]]
claim = "La suite de Fibonacci additionne les deux nombres précédents ; diviser un nombre par celui placé deux rangs à droite donne 38,2 %, soit aussi 1 − 0,618 ; 50 % n’est pas un ratio de Fibonacci mais les traders l’incluent ; 61,8, 38,2 et 50 forment la base des études de Fibonacci."
name = "CME Group Education: Fibonacci Retracements and Extensions"
url = "https://www.cmegroup.com/education/courses/technical-analysis/fibonacci-retracements-and-extensions"
checked = "2026-11-13"

[[sources]]
claim = "Les retracements de Fibonacci s’affichent en traçant d’abord une ligne entre deux points extrêmes ; les pourcentages s’appliquent à l’écart entre le plus haut et le plus bas."
name = "Fidelity Learning Center: Fibonacci Retracement"
url = "https://www.fidelity.com/learning-center/trading-investing/technical-analysis/technical-indicator-guide/fibonacci-retracement"
checked = "2026-11-13"

[[quiz]]
id = "q1"
q = "Que mesure l’outil Fibonacci ?"
options = ["Jusqu’où un repli est revenu dans le dernier mouvement", "Où le prix va tourner, à coup sûr", "La taille de ta position", "Le spread"]
answer = 0
why = "Il marque des niveaux à surveiller, pas une prévision."

[[quiz]]
id = "q2"
q = "Tendance haussière en H4. Sur quelle jambe traces-tu Fibonacci ?"
options = ["Le dernier mouvement fort à la hausse qui a cassé la structure", "Le repli en cours", "N’importe quelle jambe, tant qu’elle est longue", "Le premier mouvement de la semaine"]
answer = 0
why = "La dernière impulsion nette qui a cassé la structure (un BOS)."

[[quiz]]
id = "q3"
q = "En tendance haussière, dans quel sens le traces-tu ?"
options = ["Du creux de swing au sommet de swing", "Du sommet de swing au creux de swing", "Du prix actuel au sommet", "Peu importe"]
answer = 0
why = "Du creux au sommet en hausse ; du sommet au creux en baisse. À l’envers, les niveaux s’inversent."

[[quiz]]
id = "q4"
q = "La jambe va de 4 060 à 4 100. Où est le niveau 50 % ?"
options = ["4 080,00", "4 075,28", "4 084,72", "4 100,00"]
answer = 0
why = "La moitié de 40 $, c’est 20 $ : 4 100 − 20 = 4 080. 4 075,28, c’est 61,8 %, et 4 084,72, c’est 38,2 %."

[[quiz]]
id = "q5"
q = "Avant de se replier, le prix fait un nouveau sommet à 4 112. Que devient ta grille tracée jusqu’à 4 100 ?"
options = ["Tu la retraces depuis le même creux jusqu’au nouveau sommet", "Tu la gardes : les anciens niveaux comptent toujours", "Tu abandonnes Fibonacci pour de bon", "Tu remontes le creux"]
answer = 0
why = "La jambe s’est terminée au nouveau sommet, donc la grille suit."

[[quiz]]
id = "q6"
q = "Où accroches-tu la grille à chaque bout ?"
options = ["Sur les mèches extrêmes de la jambe", "Sur les corps des bougies", "Sur des chiffres ronds près du swing", "Là où c’est le plus joli"]
answer = 0
why = "De mèche à mèche : le vrai plus haut et le vrai plus bas de la jambe, les points qu’utilisent les autres traders."

[[recall]]
id = "r1"
q = "Les trois niveaux principaux de Fibonacci sont…"
options = ["38,2 %, 50 % et 61,8 %", "10 %, 20 % et 30 %", "100 %, 200 % et 300 %"]
answer = 0
why = "50 % n’est pas un ratio de Fibonacci, mais les traders le surveillent."

[[recall]]
id = "r2"
q = "En tendance baissière, tu traces depuis…"
options = ["Le sommet de swing jusqu’au creux de swing", "Le creux de swing jusqu’au sommet de swing"]
answer = 0
why = "Toujours du début du mouvement à sa fin."

[[recall]]
id = "r3"
q = "Le prix passe sous le creux de la jambe. Ta grille…"
options = ["Est cassée : supprime-la", "Reste valable"]
answer = 0
why = "Le mouvement qu’elle mesurait a été entièrement effacé."

[explain_back]
prompt = "Comment traces-tu Fibonacci correctement, et quand le retraces-tu ou le supprimes-tu ?"
must_include = ["tu le traces sur la dernière jambe d’impulsion nette qui a cassé la structure, sur l’unité de temps de tendance", "en tendance haussière du creux au sommet de swing (en baisse du sommet au creux), accroché aux mèches ; tracé à l’envers, les niveaux sont faux", "un nouveau sommet avant le repli veut dire retracer, et un prix sous le creux de la jambe la casse, donc tu supprimes la grille"]
misconceptions = ["le sens du tracé n’a pas d’importance", "les niveaux de Fibonacci prédisent exactement où le prix va tourner"]

[[explain_back.fixtures]]
answer = "Je prends la dernière jambe forte dans le sens de la tendance qui a cassé la structure en H4. En tendance haussière, je tire du creux de swing au sommet de swing, de mèche à mèche ; à l’envers, les niveaux seraient inversés. Si le prix fait un nouveau sommet avant de se replier, je retrace jusqu’à ce sommet, et s’il passe sous le creux de la jambe, elle est cassée et je supprime la grille."
expect = "got_it"

[[explain_back.fixtures]]
answer = "Je le trace sur la dernière jambe d’impulsion qui a cassé la structure, du creux au sommet de swing en tendance haussière, en utilisant les mèches."
expect = "almost"

[[explain_back.fixtures]]
answer = "On peut le tirer dans n’importe quel sens, et le prix tourne toujours à 61,8 %, donc j’achète là."
expect = "almost_or_not_yet"
+++
