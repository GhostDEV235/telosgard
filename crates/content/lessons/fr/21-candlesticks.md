+++
id = 21
stage = 4
title = "Les chandeliers"
objective = "lire l’ouverture, le plus haut, le plus bas et la clôture d’une bougie, dire qui a gagné sa période, et expliquer pourquoi une bougie est un compte rendu, pas une prévision."
hook = "Une seule bougie de ton graphique résume quatre prix en une forme. Sais-tu les lire tous les quatre d’un coup d’œil ?"
analogy = "Une bougie, c’est comme le compte rendu d’un match. Il te donne le score au coup d’envoi (l’ouverture), le score final (la clôture), et la plus grosse avance de chaque équipe pendant le match (le plus haut et le plus bas). Il te dit qui a gagné et à quel point le match a été disputé. Il ne te dit pas qui gagnera le match suivant."
analogy_tag = "compte rendu de match"
example = "Sur un graphique 1 heure, une bougie ouvre à 4 100,00, monte à 4 112,00, descend à 4 094,00 et clôture à 4 108,00. Le corps va de 4 100 à 4 108 : les acheteurs ont gagné l’heure de 8 $. La mèche haute montre que le prix a atteint 4 112 sans s’y maintenir ; la mèche basse, que les vendeurs l’ont poussé jusqu’à 4 094 et ont perdu. (Prix donnés à titre d’exemple.)"
mistake = "Lire une bougie comme une prédiction : « grosse bougie des acheteurs, donc ça va continuer de monter ». Une bougie raconte ce qui s’est déjà passé pendant sa période. La suivante peut partir dans un sens comme dans l’autre ; l’endroit où se forme une bougie compte plus que sa taille, et l’étape 5 du cours montre pourquoi."
takeaways = [
  "Chaque bougie montre quatre prix pour une période : ouverture, plus haut, plus bas et clôture (OHLC).",
  "Le corps va de l’ouverture à la clôture : clôture au-dessus de l’ouverture, les acheteurs ont gagné ; en dessous, les vendeurs.",
  "Les mèches atteignent le plus haut et le plus bas : jusqu’où le prix est allé avant d’être repoussé.",
  "Une bougie est un compte rendu de ce qui s’est passé, pas une prévision de la suite.",
]
drills = ["build_candle"]

[[steps]]
heading = "Quatre prix en une seule forme"
text = "Chaque bougie couvre une période : une heure sur un graphique 1 heure, une journée sur un graphique journalier. Elle enregistre quatre prix : l’**ouverture** (le premier prix de la période), le **plus haut**, le **plus bas** et la **clôture** (le dernier prix). Les traders les appellent **OHLC**, d’après leurs noms anglais."
why = "Les leçons de CME Group décrivent la bougie en deux parties : le corps pour l’ouverture et la clôture, la mèche pour les prix les plus hauts et les plus bas échangés. Un graphique en barres montre les mêmes quatre prix avec de petits traits au lieu d’un corps."

[[steps]]
heading = "Le corps : de l’ouverture à la clôture"
text = "La partie épaisse, le **corps**, va de l’ouverture à la clôture. Clôture **au-dessus** de l’ouverture : les acheteurs ont gagné la période. Clôture **en dessous** : les vendeurs ont gagné. Dans l’appli, les corps creux ont clôturé plus haut et les corps pleins plus bas ; la plupart des plateformes les colorent plutôt en vert et en rouge."
why = "Les couleurs sont un réglage : vérifie la légende de ta plateforme. Quand l’ouverture et la clôture sont presque au même prix, le corps se réduit à un trait : aucun camp n’a gagné. Les traders appellent ça un doji."
widget = "candle_builder"

[[steps]]
heading = "Les mèches : jusqu’où le prix est allé"
text = "Les traits fins au-dessus et au-dessous du corps sont les **mèches** (on dit aussi les ombres). Le bout de la mèche haute, c’est le **plus haut** ; le bout de la mèche basse, le **plus bas**. Une longue mèche veut dire que le prix est allé jusque-là et a été repoussé avant la clôture."
why = "CME Group le dit ainsi : une mèche courte ou absente veut dire qu’un camp a dominé toute la période ; de longues mèches, que les acheteurs et les vendeurs ont tous deux été actifs. Une longue mèche haute sur une bougie des vendeurs : les acheteurs ont poussé d’abord, puis les vendeurs ont repris la main."

[[steps]]
heading = "La taille montre la force"
text = "Un long corps veut dire que le prix a beaucoup bougé entre l’ouverture et la clôture : un camp a poussé fort. Un corps court veut dire que l’ouverture et la clôture étaient proches : pas beaucoup de force dans un sens ou dans l’autre. Mesure les deux en Mouvements en dollars : un corps de 4 100,00 à 4 108,00 est un mouvement de 8 $."
why = "« Long » veut seulement dire long par rapport aux bougies voisines du même graphique. Un corps de 8 $ est grand sur un graphique 5 minutes et ordinaire sur un graphique journalier."

[[steps]]
heading = "Un compte rendu, pas une prévision"
text = "Une bougie te dit ce qui s’est passé pendant sa période. Elle ne te dit pas la suite : une forte bougie des acheteurs peut être suivie d’une bougie des vendeurs. C’est pour ça que tu ne trades jamais une bougie seule ; l’étape 5 du cours montre où les bougies commencent à compter : sur des niveaux."
why = "Souviens-toi de l’exercice de la leçon 2 : deviner la prochaine bougie, c’est presque pile ou face. Une bougie de confirmation ne compte qu’à un endroit précis, dans le sens de la tendance (leçon 26)."

[[videos]]
id = "jmoOrgTP5XQ"
title = "Candlestick charts: The ULTIMATE beginners guide to reading a candlestick chart"
channel = "The Trading Channel"
duration = "16:25"
watch_for = "Comment le corps et les mèches se construisent à partir de l’ouverture, du plus haut, du plus bas et de la clôture. Laisse de côté les noms de figures pour l’instant : l’étape 5 y reviendra."
fallback_search = "how to read candlestick charts beginners"
checked = "2026-11-13"

[[videos]]
id = "zEBvX_El3LY"
title = "Forex Trading for Beginners #7: How to Read a Forex Chart"
channel = "Rayner Teo"
duration = "5:30"
watch_for = "Comment lire un graphique en bougies et changer d’unité de temps. C’est un graphique de devises, mais les bougies se lisent de la même façon sur l’or ; quand il parle en pips, ce cours parle en Mouvements en dollars."
fallback_search = "how to read a price chart candlesticks beginners"
checked = "2026-11-13"

[[sources]]
claim = "Une bougie a un corps qui montre les prix d’ouverture et de clôture et une mèche qui montre les prix les plus hauts et les plus bas échangés sur la période ; la barre est en général verte quand la clôture est au-dessus de l’ouverture et rouge en dessous, selon les réglages de couleur du logiciel."
name = "CME Group Education: Chart Types: candlestick, line, bar"
url = "https://www.cmegroup.com/education/courses/technical-analysis/chart-types-candlestick-line-bar.html"
checked = "2026-11-13"

[[sources]]
claim = "Plus le corps est long, plus le prix a bougé pendant la période ; une mèche courte ou absente signifie qu’un camp a dominé toute la période ; de longues mèches, que les acheteurs et les vendeurs ont tous deux été actifs."
name = "CME Group Education: Chart Types: candlestick, line, bar"
url = "https://www.cmegroup.com/education/courses/technical-analysis/chart-types-candlestick-line-bar.html"
checked = "2026-11-13"

[[quiz]]
id = "q1"
q = "Une bougie 1 heure : ouverture 4 100,00, plus haut 4 112,00, plus bas 4 094,00, clôture 4 108,00. Où va son corps ?"
options = ["De 4 100,00 à 4 108,00", "De 4 094,00 à 4 112,00", "De 4 108,00 à 4 112,00", "De 4 094,00 à 4 100,00"]
answer = 0
why = "Le corps va de l’ouverture à la clôture. Le plus haut et le plus bas sont les bouts des mèches."

[[quiz]]
id = "q2"
q = "Une bougie ouvre à 4 120,00 et clôture à 4 111,00. Qui a gagné la période ?"
options = ["Les vendeurs : clôture 9 $ sous l’ouverture", "Les acheteurs : clôture 9 $ au-dessus de l’ouverture", "Personne : les bougies ne le montrent pas", "Les acheteurs, parce que le plus haut était au-dessus de l’ouverture"]
answer = 0
why = "Clôture sous l’ouverture : les vendeurs ont gagné ; le corps est un mouvement de 9 $ à la baisse."

[[quiz]]
id = "q3"
q = "Que montre le bout de la mèche haute ?"
options = ["Le prix le plus haut de la période", "Le prix de clôture", "Le prochain prix à venir", "Le prix moyen"]
answer = 0
why = "La mèche haute atteint le plus haut ; la mèche basse, le plus bas."

[[quiz]]
id = "q4"
q = "Une bougie a un petit corps près de son plus bas et une longue mèche haute. Que s’est-il passé pendant la période ?"
options = ["Le prix est monté loin, puis a été repoussé avant la clôture", "Le prix n’a pas bougé", "Les acheteurs ont gardé leurs gains jusqu’à la clôture", "Le courtier a fait une erreur"]
answer = 0
why = "La longue mèche haute montre jusqu’où le prix est monté ; la clôture près du plus bas montre qu’il n’a pas tenu."

[[quiz]]
id = "q5"
q = "Deux bougies sur le même graphique : l’une a un corps de 2 $, l’autre un corps de 14 $. Que te dit le plus grand corps ?"
options = ["Le prix a bougé beaucoup plus loin entre l’ouverture et la clôture pendant cette période", "Le prix va continuer dans ce sens", "Cette bougie a duré plus longtemps", "Cette bougie avait un spread plus large"]
answer = 0
why = "La taille du corps montre jusqu’où le prix a bougé pendant la période. Elle ne dit rien de sûr sur la suivante."

[[quiz]]
id = "q6"
q = "Trois fortes bougies des acheteurs d’affilée. Que te disent-elles sur la bougie suivante ?"
options = ["Rien de sûr : les bougies racontent ce qui s’est passé, pas la suite", "Ce sera aussi une bougie des acheteurs", "Ce sera une bougie des vendeurs", "Elle sera plus grande"]
answer = 0
why = "Une bougie est un compte rendu, pas une prévision. L’endroit où elle se forme compte, et c’est l’étape 5 du cours."

[[recall]]
id = "r1"
q = "Le corps d’une bougie va…"
options = ["De l’ouverture à la clôture", "Du plus haut au plus bas", "De la clôture à l’ouverture suivante"]
answer = 0
why = "Les mèches couvrent le plus haut et le plus bas."

[[recall]]
id = "r2"
q = "Une bougie qui clôture sous son ouverture a été gagnée par…"
options = ["Les vendeurs", "Les acheteurs", "Personne"]
answer = 0
why = "Clôture sous l’ouverture : les vendeurs ont gagné la période."

[[recall]]
id = "r3"
q = "Une grosse bougie te dit-elle où va le prix ensuite ?"
options = ["Non, elle raconte ce qui s’est déjà passé", "Oui, toujours dans le même sens"]
answer = 0
why = "Un compte rendu, pas une prévision."

[explain_back]
prompt = "Que te dit une bougie, et qu’est-ce qu’elle ne peut pas te dire ?"
must_include = ["une bougie montre quatre prix pour sa période : ouverture, plus haut, plus bas et clôture", "le corps va de l’ouverture à la clôture et montre qui a gagné (clôture au-dessus de l’ouverture : les acheteurs ; en dessous : les vendeurs), et les mèches atteignent le plus haut et le plus bas", "elle raconte ce qui s’est déjà passé pendant cette période, pas ce qui va se passer ensuite"]
misconceptions = ["une grosse bougie prédit le prochain mouvement", "c’est la couleur ou les mèches qui disent qui a gagné, et non l’ouverture et la clôture"]

[[explain_back.fixtures]]
answer = "Une bougie montre l’ouverture, le plus haut, le plus bas et la clôture de sa période. Le corps va de l’ouverture à la clôture : si elle clôture au-dessus de l’ouverture, les acheteurs ont gagné, en dessous ce sont les vendeurs. Les mèches montrent jusqu’où le prix est monté et descendu. Mais elle me dit seulement ce qui s’est passé pendant cette heure ou cette journée, pas ce que fera la bougie suivante."
expect = "got_it"

[[explain_back.fixtures]]
answer = "Une bougie montre l’ouverture, le plus haut, le plus bas et la clôture. Le corps va de l’ouverture à la clôture et dit si les acheteurs ou les vendeurs ont gagné, et les mèches montrent le plus haut et le plus bas."
expect = "almost"

[[explain_back.fixtures]]
answer = "Une bougie verte veut dire que le prix va continuer de monter, et les mèches montrent où les acheteurs vont revenir la prochaine fois."
expect = "almost_or_not_yet"
+++
