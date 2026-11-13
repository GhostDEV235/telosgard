+++
id = 22
stage = 4
title = "Unités de temps et analyse du haut vers le bas"
objective = "expliquer ce qu’est une unité de temps, utiliser une unité supérieure pour la direction et une inférieure pour l’entrée, et dire quoi faire quand elles ne sont pas d’accord."
hook = "Le graphique 15 minutes dit que l’or baisse. Le graphique 4 heures dit qu’il monte. Lequel a raison ?"
analogy = "Pense à une appli de cartes. En vue éloignée, tu vois dans quel sens la route traverse le pays. En vue rapprochée, tu vois le prochain virage. Tu prépares l’itinéraire sur la grande carte et tu tiens le volant avec la petite. Si la vue rapprochée semble pointer dans l’autre sens, c’est un virage : c’est la grande carte qui dit où tu vas."
analogy_tag = "zoom de carte"
example = "Le graphique 4 heures de l’or monte depuis deux semaines, de 4 020,00 à 4 110,00. Sur le graphique 1 heure, le prix redescend à 4 092,00 et commence à remonter. Lu du haut vers le bas, c’est un repli dans une tendance haussière : un endroit où chercher un achat, avec un stop sous le repli. Vendre ici, ce serait aller contre la grande tendance. (Prix donnés à titre d’exemple.)"
mistake = "Passer d’une unité de temps à l’autre jusqu’à en trouver une qui donne raison au trade que tu veux déjà. Choisis tes deux unités avant de regarder, lis d’abord la plus haute, et laisse-la décider du sens."
takeaways = [
  "Une unité de temps, c’est la durée de chaque bougie : M5, H1, H4, D1. Les grandes bougies sont des petites bougies regroupées.",
  "Du haut vers le bas : lis d’abord l’unité supérieure pour la direction (le biais), puis l’inférieure pour l’entrée.",
  "La paire de départ de ce cours : H4 pour la direction, H1 pour les entrées ; les swing traders utilisent D1 et H4.",
  "Quand les unités ne sont pas d’accord, la plus haute l’emporte : trade dans son sens, ou reste à l’écart.",
]

[[steps]]
heading = "Un marché, plusieurs unités de temps"
text = "Les mêmes prix de l’or peuvent se dessiner avec une bougie toutes les 5 minutes, toutes les heures, toutes les 4 heures ou chaque jour. Ce sont les **unités de temps**, notées **M5, H1, H4 et D1**. Une unité supérieure montre plus d’historique avec moins de bougies, plus grandes."
why = "Quatre bougies de 15 minutes font une bougie de 1 heure : son ouverture est la première ouverture, sa clôture la dernière clôture, et son plus haut et son plus bas sont les extrêmes des quatre. Rien de nouveau ne s’est passé ; ce sont les mêmes échanges, regroupés autrement."
widget = "candle_merge"

[[steps]]
heading = "Du haut vers le bas : la vue d’ensemble d’abord"
text = "L’**analyse du haut vers le bas** consiste à lire d’abord l’unité supérieure, puis à descendre. Le graphique supérieur donne ton **biais** : le sens dans lequel tu traderais. Le graphique inférieur sert à choisir le moment de l’**entrée**. Les leçons de CME Group décrivent des graphiques courts utilisés pour des trades **alignés sur la tendance de fond**."
why = "Le biais, c’est simplement ton sens préféré pour la séance : hausse, baisse ou aucun. Sans biais clair, le plan du jour peut être d’attendre, et c’est un bon plan."

[[steps]]
heading = "La paire du débutant : H4 et H1"
text = "Ce cours te fait commencer avec deux graphiques : **H4 pour la direction** et **H1 pour les entrées**. Les swing traders, qui gardent leurs trades plusieurs jours, montent à **D1 et H4**. Deux graphiques suffisent : chaque unité de plus est une façon de plus de te convaincre de prendre un trade."
why = "Cette paire est un choix du cours, pas une loi du marché. Les très petites unités comme M5 montrent plus de bruit, et les coûts comme le spread (leçon 6) pèsent plus lourd sur de petits mouvements."

[[steps]]
heading = "Quand les unités ne sont pas d’accord"
text = "Ça arrive souvent. H4 qui monte pendant que H1 baisse, c’est normal : c’est un **repli** (pullback) dans le grand mouvement. L’unité supérieure l’emporte : dans une tendance haussière en H4, tu cherches des achats en H1, pas des ventes. Si H4 n’a pas de direction claire, rester à l’écart est un plan valable."
why = "Un repli dans une tendance, c’est justement là que le Price Action classique cherche son entrée (leçon 27). Aller contre l’unité supérieure, c’est trader contre l’essentiel de l’élan récent du marché."

[[videos]]
id = "5N3X2WPnG2Q"
title = "How to Analyze Multiple Timeframes (Without Getting Confused)"
channel = "Rayner Teo"
duration = "3:39"
watch_for = "Comment une unité supérieure donne la direction et une inférieure le moment. Sa paire exacte peut différer ; ce cours commence avec H4 et H1."
fallback_search = "multiple timeframe analysis beginners"
checked = "2026-11-13"

[[videos]]
id = "POJFoQYVXnM"
title = "Which is the best timeframe to trade (for beginners)?"
channel = "Rayner Teo"
duration = "5:17"
watch_for = "Pourquoi les débutants s’en sortent en général mieux sur des unités de temps plus hautes que sur de très courtes."
fallback_search = "best timeframe to trade for beginners"
checked = "2026-11-13"

[[sources]]
claim = "Les graphiques peuvent être réglés sur différentes unités de temps ; un trader de long terme peut suivre des graphiques hebdomadaires ou mensuels, un trader de court terme des graphiques 60 minutes ou 5 minutes."
name = "CME Group Education: Chart Types: candlestick, line, bar"
url = "https://www.cmegroup.com/education/courses/technical-analysis/chart-types-candlestick-line-bar.html"
checked = "2026-11-13"

[[sources]]
claim = "Le graphique 1 heure sert à préparer des trades alignés sur la tendance de fond du marché, le graphique journalier donne la vue d’ensemble, et combiner les unités de temps couvre les mouvements de court et de long terme."
name = "CME Group Education: Trading Simulator How-To Guide: Chart Time Frames"
url = "https://www.cmegroup.com/education/courses/using-the-trading-simulator/trading-sim-in-30-seconds-chart-time-frames.html"
checked = "2026-11-13"

[[quiz]]
id = "q1"
q = "Sur un graphique H1, que couvre chaque bougie ?"
options = ["Une heure", "Une journée", "Une minute", "Un trade"]
answer = 0
why = "H1 = une heure par bougie ; H4 = quatre heures ; D1 = une journée."

[[quiz]]
id = "q2"
q = "Dans la paire du débutant de ce cours, quelle unité donne la direction ?"
options = ["H4", "H1", "M5", "Celle qui a l’air la meilleure aujourd’hui"]
answer = 0
why = "H4 pour la direction, H1 pour les entrées."

[[quiz]]
id = "q3"
q = "H4 est en nette tendance haussière. En H1, le prix baisse depuis six bougies. Lu du haut vers le bas, qu’est-ce que c’est ?"
options = ["Sans doute un repli dans la tendance haussière : cherche des achats, pas des ventes", "Un signal pour vendre tout de suite", "La preuve que la tendance H4 est finie", "Une raison de passer en M5"]
answer = 0
why = "L’unité supérieure l’emporte ; une baisse sur l’unité inférieure est souvent un repli, là où les acheteurs peuvent revenir."

[[quiz]]
id = "q4"
q = "Quatre bougies de 15 minutes : ouvertures 4 100, 4 104, 4 109, 4 101 ; plus hauts 4 105, 4 112, 4 110, 4 106 ; plus bas 4 098, 4 102, 4 099, 4 100 ; la dernière clôture à 4 105. Quelle est la bougie de 1 heure ?"
options = ["Ouverture 4 100, plus haut 4 112, plus bas 4 098, clôture 4 105", "Ouverture 4 101, plus haut 4 106, plus bas 4 100, clôture 4 105", "Ouverture 4 100, plus haut 4 105, plus bas 4 098, clôture 4 104", "Ouverture 4 105, plus haut 4 112, plus bas 4 098, clôture 4 100"]
answer = 0
why = "Première ouverture, plus haut le plus élevé, plus bas le plus bas, dernière clôture."

[[quiz]]
id = "q5"
q = "Un swing trader garde ses trades plusieurs jours. Quelle paire lui convient ?"
options = ["D1 pour la direction, H4 pour les entrées", "M1 et M5", "H1 pour la direction, M5 pour les entrées", "Seulement le graphique hebdomadaire"]
answer = 0
why = "Monte les deux unités d’un cran : D1 et H4."

[[quiz]]
id = "q6"
q = "Tu veux vendre. H4 monte, alors tu regardes H1, puis M15, puis M5 jusqu’à en trouver une qui baisse. Qu’est-ce qui ne va pas ?"
options = ["Tu as cherché un graphique qui te donne raison au lieu de laisser l’unité supérieure décider", "Rien : plus d’unités, c’est toujours mieux", "Tu aurais dû commencer en M1", "Tu aurais dû prendre un plus gros lot"]
answer = 0
why = "Choisis les unités d’abord, lis la plus haute en premier, et suis-la."

[[recall]]
id = "r1"
q = "L’analyse du haut vers le bas commence par…"
options = ["L’unité de temps supérieure", "La plus petite unité de temps", "La dernière bougie"]
answer = 0
why = "La vue d’ensemble d’abord, puis l’entrée."

[[recall]]
id = "r2"
q = "La paire du débutant de ce cours, c’est…"
options = ["H4 pour la direction, H1 pour les entrées", "M1 et M5", "D1 seulement"]
answer = 0
why = "Les swing traders montent à D1 et H4."

[[recall]]
id = "r3"
q = "Quand les unités de temps ne sont pas d’accord…"
options = ["La plus haute l’emporte, ou tu restes à l’écart", "La plus basse l’emporte", "Tu trades dans les deux sens"]
answer = 0
why = "Un mouvement de l’unité inférieure contre la tendance est souvent juste un repli."

[explain_back]
prompt = "Comment utilises-tu deux unités de temps ensemble, et que fais-tu quand elles ne sont pas d’accord ?"
must_include = ["l’unité supérieure donne la direction (le biais) et l’inférieure le moment de l’entrée : H4 puis H1 dans ce cours", "tu lis l’unité supérieure en premier, avant de regarder l’inférieure", "quand elles ne sont pas d’accord, l’unité supérieure l’emporte : tu trades dans son sens ou tu restes à l’écart, jamais contre elle"]
misconceptions = ["la plus petite unité de temps est la plus précise", "il faut changer d’unité de temps jusqu’à en trouver une qui donne raison à ton trade"]

[[explain_back.fixtures]]
answer = "Je regarde d’abord H4 pour décider du sens, puis H1 pour trouver l’entrée. S’ils ne sont pas d’accord, par exemple H4 monte et H1 baisse, je le vois comme un repli et je ne cherche que des achats, ou je reste à l’écart. Je ne vends pas contre la tendance H4."
expect = "got_it"

[[explain_back.fixtures]]
answer = "J’utilise H4 pour voir la tendance et H1 pour choisir le moment de mon entrée, en regardant H4 en premier."
expect = "almost"

[[explain_back.fixtures]]
answer = "La plus petite unité de temps est la plus précise, alors je regarde M1 et M5 et je change de graphique jusqu’à en trouver un qui montre le mouvement que je veux."
expect = "almost_or_not_yet"
+++
