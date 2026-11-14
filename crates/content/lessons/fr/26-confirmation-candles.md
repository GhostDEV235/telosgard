+++
id = 26
stage = 5
title = "Les bougies de confirmation"
objective = "reconnaître une pin bar, une bougie d’avalement et une inside bar, et expliquer pourquoi elles ne comptent qu’à un endroit précis, une fois clôturées."
hook = "Le prix atteint ta zone de support. Tu achètes tout de suite, ou tu attends d’abord quelque chose ?"
analogy = "Pense à une poignée de main. En arrivant à un rendez-vous, tu attends que l’autre personne tende la main avant de la serrer. Une bougie de confirmation, c’est le marché qui te tend la main sur ton niveau : la preuve que des acheteurs ou des vendeurs sont vraiment venus, pas seulement l’espoir qu’ils viendront."
analogy_tag = "poignée de main"
example = "L’or descend dans ta zone de support, de 4 080 à 4 084. Une bougie H1 plonge à 4 076, puis clôture à 4 089 avec une longue mèche basse : une pin bar haussière. Les vendeurs ont poussé sous la zone et ont été repoussés. Tu chercherais à acheter après la clôture, avec un stop sous la mèche, vers 4 074. (Prix donnés à titre d’exemple.)"
mistake = "Trader une figure au milieu de nulle part. Une pin bar entre deux niveaux, ou contre la tendance, n’est qu’une bougie. L’endroit d’abord, la bougie ensuite."
takeaways = [
  "Une bougie de confirmation montre que l’autre camp a été repoussé sur ton niveau.",
  "Pin bar : une longue mèche et un petit corps à l’autre bout. Avalement : un corps qui recouvre le corps précédent, dans l’autre sens. Inside bar : à l’intérieur de l’amplitude de la bougie précédente.",
  "N’agis qu’une fois la bougie clôturée, jamais pendant qu’elle se forme.",
  "Une figure ne compte qu’à un endroit précis : sur une zone, dans le sens de la tendance de l’unité supérieure.",
]
drills = ["name_candle"]

[[steps]]
heading = "Pourquoi attendre une confirmation"
text = "Dans une zone, le prix peut rebondir ou casser. Une **bougie de confirmation** est la preuve que l’autre camp est venu : le prix est entré dans la zone et a été repoussé. Tu agis une fois qu’elle a **clôturé**, jamais pendant qu’elle se forme."
why = "Une bougie peut ressembler à une pin bar parfaite à la 50e minute et clôturer tout autrement à la 60e. Seule la clôture compte."

[[steps]]
heading = "La pin bar"
text = "Une **pin bar** a une longue mèche d’un côté, environ deux tiers de la bougie ou plus, et un petit corps à l’autre bout. Une longue mèche basse sur un support : les acheteurs ont refusé les prix plus bas (**haussière**). Une longue mèche haute sur une résistance : **baissière**."
why = "CME Group l’explique par le corps : un petit corps près du haut de la bougie veut dire que les acheteurs avaient la main à la clôture ; près du bas, c’étaient les vendeurs. La longue mèche montre où l’autre camp a échoué."

[[steps]]
heading = "La bougie d’avalement"
text = "Une bougie d’**avalement** (engulfing) a un corps qui recouvre tout le corps de la bougie précédente, dans l’autre sens. Un avalement haussier sur un support : les vendeurs ont eu une bougie, puis les acheteurs ont tout repris, et plus encore."
why = "Compare les corps, pas les mèches : l’écart entre l’ouverture et la clôture de la bougie d’avalement doit couvrir celui de la bougie précédente."

[[steps]]
heading = "L’inside bar"
text = "Une **inside bar** reste entièrement entre le plus haut et le plus bas de la bougie précédente : le marché fait une pause. Sur un niveau, les traders attendent que le prix sorte de l’amplitude de la grande bougie, dans le sens de la tendance."
why = "Seule, une inside bar dit juste « pause ». Son sens vient de la cassure qui suit, donc elle demande plus de patience qu’une pin bar."

[[steps]]
heading = "L’endroit d’abord"
text = "Une figure ne compte qu’**à un endroit précis** : sur une zone, dans le sens de l’unité de temps supérieure (leçons 22 à 24). La même pin bar entre deux niveaux, ou contre la tendance, n’est qu’une bougie. L’endroit, puis la confirmation, puis le trade, avec un stop au-delà de la mèche."
why = "C’est le cœur du Price Action classique : une bougie de confirmation sur un niveau, dans le sens de la tendance. Les leçons 27 et 28 en font deux jeux complets."

[[videos]]
id = "DC-ghSmJy9s"
title = "A pinbar trading strategy that works"
channel = "Rayner Teo"
duration = "13:13"
watch_for = "Comment il ne lit une pin bar que sur un niveau, en contexte. Ses entrées exactes sont des exemples, pas les règles de ce cours."
fallback_search = "pin bar candlestick explained"
checked = "2026-11-13"

[[videos]]
id = "k98oyKtGUns"
title = "Candlestick Pattern Trading #3: What is a Bullish Engulfing Pattern"
channel = "Rayner Teo"
duration = "4:05"
watch_for = "Ce qui fait un avalement : le corps qui recouvre le corps précédent."
fallback_search = "bullish engulfing pattern explained"
checked = "2026-11-13"

[[sources]]
claim = "Un petit corps près du bas d’une bougie veut dire que les vendeurs avaient pris la main à sa fin ; un petit corps près du haut, que c’étaient les acheteurs ; de longues mèches, que les deux camps ont été actifs."
name = "CME Group Education: Chart Types: candlestick, line, bar"
url = "https://www.cmegroup.com/education/courses/technical-analysis/chart-types-candlestick-line-bar.html"
checked = "2026-11-13"

[[quiz]]
id = "q1"
q = "Sur un support, une bougie H1 plonge nettement sous la zone, puis clôture près de son plus haut, avec une longue mèche basse. Qu’est-ce que c’est ?"
options = ["Une pin bar haussière", "Une pin bar baissière", "Une inside bar", "Un avalement baissier"]
answer = 0
why = "Une longue mèche basse et un petit corps près du haut : les prix plus bas ont été refusés."

[[quiz]]
id = "q2"
q = "Une bougie baissière, puis une bougie haussière dont le corps recouvre tout le corps baissier. Qu’est-ce que c’est ?"
options = ["Un avalement haussier", "Une inside bar", "Une pin bar baissière", "Pas de figure"]
answer = 0
why = "Le nouveau corps avale le précédent, dans l’autre sens."

[[quiz]]
id = "q3"
q = "Le plus haut et le plus bas d’une bougie sont tous les deux dans l’amplitude de la bougie précédente. Qu’est-ce que c’est ?"
options = ["Une inside bar : le marché fait une pause", "Un avalement haussier", "Une cassure garantie", "Une pin bar"]
answer = 0
why = "À l’intérieur de l’amplitude précédente, c’est une pause ; la cassure qui suit donne le sens."

[[quiz]]
id = "q4"
q = "C’est la 45e minute d’une bougie H1 sur ta zone, et elle ressemble à une pin bar. Que fais-tu ?"
options = ["Attendre la clôture de la bougie", "Entrer maintenant avant qu’il soit trop tard", "Rapprocher le stop", "Passer en M1"]
answer = 0
why = "Seule la clôture compte ; la bougie peut encore changer de forme."

[[quiz]]
id = "q5"
q = "Une pin bar haussière parfaite se forme à mi-chemin entre deux zones, dans une tendance baissière en H4. Est-ce un setup ?"
options = ["Non : elle n’est pas à un endroit précis et elle va contre la tendance", "Oui, toute pin bar est un achat", "Oui, si la mèche est longue", "Seulement si elle est verte"]
answer = 0
why = "L’endroit d’abord. Ailleurs, ce n’est qu’une bougie."

[[quiz]]
id = "q6"
q = "Après la clôture d’une pin bar haussière sur un support, avec son plus bas à 4 076, où va le stop ?"
options = ["Sous la mèche et la zone, vers 4 074", "Juste au-dessus de l’entrée", "Au milieu de la zone", "Pas besoin de stop : c’est confirmé"]
answer = 0
why = "Au-delà de la mèche : si le prix repasse dessous, le rejet a échoué."

[[recall]]
id = "r1"
q = "Une pin bar a…"
options = ["Une longue mèche et un petit corps à l’autre bout", "Aucune mèche", "Deux corps"]
answer = 0
why = "La longue mèche montre le camp repoussé."

[[recall]]
id = "r2"
q = "Tu agis sur une bougie de confirmation…"
options = ["Une fois clôturée", "Pendant qu’elle se forme", "Avant que le prix atteigne la zone"]
answer = 0
why = "La clôture décide de ce qu’elle est vraiment."

[[recall]]
id = "r3"
q = "Une bougie de confirmation compte…"
options = ["Seulement à un endroit précis, avec la tendance", "N’importe où sur le graphique"]
answer = 0
why = "L’endroit d’abord, la bougie ensuite."

[explain_back]
prompt = "Qu’est-ce qu’une bougie de confirmation, et quand compte-t-elle ?"
must_include = ["une bougie de confirmation montre que l’autre camp a été repoussé sur un niveau : une pin bar, une bougie d’avalement ou une inside bar", "tu attends la clôture de la bougie avant d’agir", "elle ne compte qu’à un endroit précis (une zone, dans le sens de la tendance de l’unité supérieure) ; ailleurs, ce n’est qu’une bougie"]
misconceptions = ["toute pin bar est un signal de trade", "on peut entrer pendant que la bougie se forme encore"]

[[explain_back.fixtures]]
answer = "Une bougie de confirmation, c’est une bougie sur mon niveau qui montre que l’autre camp a été repoussé, comme une pin bar avec une longue mèche, un avalement ou une inside bar. J’attends sa clôture avant de faire quoi que ce soit. Et elle ne compte qu’à un endroit précis, sur une zone et dans le sens de la tendance de l’unité supérieure ; la même figure au milieu de nulle part n’est qu’une bougie."
expect = "got_it"

[[explain_back.fixtures]]
answer = "C’est une pin bar, un avalement ou une inside bar qui montre que les acheteurs ou les vendeurs ont été repoussés, et on attend sa clôture."
expect = "almost"

[[explain_back.fixtures]]
answer = "Dès que je vois une pin bar, j’achète tout de suite, même avant la clôture, parce que ça veut dire que le prix va monter."
expect = "almost_or_not_yet"
+++
