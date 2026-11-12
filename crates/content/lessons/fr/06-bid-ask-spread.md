+++
id = 6
stage = 1
title = "Bid, ask et spread"
objective = "expliquer pourquoi chaque trade commence légèrement négatif, et ce que te coûte le spread."
hook = "Pourquoi un trade tout juste ouvert affiche-t-il déjà une petite perte ?"
analogy = "Imagine un bureau de change à l’aéroport. Le panneau affiche deux prix : « Nous achetons à » et « Nous vendons à ». Si tu changes 100 $ en euros puis tout de suite en dollars, tu récupères moins de 100 $. Tu n’as pas pris une mauvaise décision : tu as payé l’écart entre les deux prix. Le trading de l’or marche pareil."
analogy_tag = "bureau de change"
example = "Ta plateforme affiche Bid 4 150,20 / Ask 4 150,50. Le spread est de 4 150,50 − 4 150,20 = 0,30 $. Si tu achètes, tu paies l’ask, 4 150,50, mais ta position est évaluée au bid, 4 150,20. Le trade s’ouvre donc à −0,30 $, et l’or doit monter de 0,30 $ rien que pour que tu rentres dans tes frais. (Prix donnés à titre d’exemple.)"
mistake = "Croire que le spread n’est prélevé qu’à la clôture, ou qu’il a toujours la même taille. Tu le paies dès l’ouverture, et il peut s’élargir fortement quand le marché est calme ou autour d’une grosse annonce."
takeaways = [
  "Chaque cotation a deux prix : le bid (le plus bas) et l’ask (le plus haut).",
  "Tu achètes à l’ask et tu vends au bid.",
  "L’écart entre les deux est le spread, un coût que tu paies à l’ouverture.",
  "Le spread peut s’élargir : vérifie-le avant de trader.",
]
drills = ["bid_ask"]

[[videos]]
id = "r1N22WzhzPM"
title = "Forex Spread Explained (Video 8 of 13)"
channel = "Rayner Teo"
duration = "7:53"
watch_for = "Comment les deux prix apparaissent sur une plateforme et pourquoi le spread est un coût. La vidéo prend des paires de devises ; l’idée est identique pour l’or."
fallback_search = "forex spread explained beginner"
checked = "2026-11-10"

[[sources]]
claim = "Dans MetaTrader 5, les ordres d’achat s’exécutent au prix Ask et les ordres de vente au prix Bid."
name = "Aide MetaTrader 5 : Types of Orders"
url = "https://www.metatrader5.com/en/mobile-trading/iphone/help/trade/general_concept/order_types"
checked = "2026-11-10"

[[quiz]]
id = "q1"
q = "L’or affiche Bid 4 150,20 / Ask 4 150,50. Quel est le spread ?"
options = ["0,30 $", "3,00 $", "30 $", "0,03 $"]
answer = 0
why = "4 150,50 − 4 150,20 = 0,30 $."

[[quiz]]
id = "q2"
q = "Tu veux acheter de l’or tout de suite. Quel prix paies-tu ?"
options = ["Le bid", "L’ask", "Le milieu entre les deux", "La clôture d’hier"]
answer = 1
why = "Tu achètes à l’ask, le plus haut des deux prix."

[[quiz]]
id = "q3"
q = "Tu veux vendre (position courte) tout de suite. Quel prix obtiens-tu ?"
options = ["Le bid", "L’ask", "Le plus haut des deux", "Celui que tu choisis"]
answer = 0
why = "Tu vends au bid, le plus bas des deux prix."

[[quiz]]
id = "q4"
q = "Quand paies-tu le spread ?"
options = ["Seulement à la clôture", "À l’ouverture du trade", "Une fois par mois", "Seulement si le trade perd"]
answer = 1
why = "Le spread se paie à l’entrée : c’est pour ça qu’un nouveau trade commence légèrement négatif."

[[quiz]]
id = "q5"
q = "Le spread est de 0,40 $ et tu achètes. De combien l’or doit-il monter pour que tu rentres dans tes frais ?"
options = ["0,04 $", "0,40 $", "4,00 $", "Tu es déjà à l’équilibre"]
answer = 1
why = "Tu pars avec 0,40 $ de retard, donc le prix doit monter de 0,40 $ pour revenir à zéro."

[[quiz]]
id = "q6"
q = "Qu’est-ce qui est vrai à propos du spread ?"
options = ["Il ne change jamais", "Il peut s’élargir quand le marché est calme ou autour d’une grosse annonce", "Il vaut toujours exactement 1 $", "Il t’est remboursé à la clôture"]
answer = 1
why = "Les spreads bougent. Ils sont souvent plus larges quand il y a peu d’échanges ou quand une grosse nouvelle tombe."

[explain_back]
prompt = "Pourquoi chaque trade commence-t-il légèrement « dans le rouge » ?"
must_include = ["on achète à l’ask et on vend au bid", "l’ask est plus haut que le bid", "le spread est un coût payé à l’entrée"]
misconceptions = ["le spread n’est prélevé qu’à la clôture", "le spread a toujours la même taille"]

[[explain_back.fixtures]]
answer = "Parce qu’on achète au prix ask, plus haut, mais que la position est évaluée au bid, plus bas. L’écart entre les deux est le spread, un coût qu’on paie dès l’ouverture du trade."
expect = "got_it"

[[explain_back.fixtures]]
answer = "À cause du spread : il y a deux prix et l’écart entre eux est un coût."
expect = "almost"

[[explain_back.fixtures]]
answer = "Parce que le courtier prélève le spread à la fin, quand on clôture le trade, et que c’est toujours exactement le même montant."
expect = "almost_or_not_yet"

[[steps]]
heading = "Toujours deux prix"
text = "Chaque cotation a deux prix. Le **bid** est le prix auquel tu peux **vendre** : le plus bas. L’**ask** est le prix auquel tu peux **acheter** : le plus haut. Les plateformes les affichent souvent simplement comme Sell et Buy (Vendre et Acheter)."
why = "Dans MetaTrader 5, la plateforme sur laquelle tu t’entraîneras, les ordres d’achat s’exécutent à l’ask et les ordres de vente au bid."
widget = "bid_ask"
[[steps]]
heading = "Le spread"
text = "L’écart entre les deux est le **spread** : ask − bid. Avec 4 150,20 / 4 150,50, le spread est de **0,30 $**. C’est ainsi que beaucoup de courtiers gagnent leur argent : il est inclus dans les prix auxquels tu trades, pas facturé à part."
[[steps]]
heading = "Pourquoi un nouveau trade commence négatif"
text = "Tu achètes, tu paies l’ask ; pour sortir tout de suite, tu vendrais au bid, plus bas. Dès l’ouverture, le trade vaut donc exactement le spread en moins. Le prix doit bouger au moins de cette distance dans ton sens avant que tu gagnes quoi que ce soit."
why = "C’est pareil pour une position courte : tu vends au bid et tu devrais racheter à l’ask."
[[steps]]
heading = "Les spreads changent"
text = "Les spreads sont étroits quand beaucoup de monde trade, et plus larges quand le marché est calme ou qu’une grosse nouvelle tombe. Un spread de 0,30 $ peut devenir plusieurs dollars lors d’un pic d’actualité. Jette un œil aux deux prix avant chaque trade."
why = "Demande-toi : quelle part du mouvement attendu le spread va-t-il manger ? Si tu vises 3 $ avec un spread de 0,30 $, un dixième de ton idée est parti avant même de commencer. Si le spread passe soudain à 2 $, le trade ne vaut peut-être plus la peine."
[[recall]]
id = "r1"
q = "Tu achètes maintenant. Quel prix paies-tu ?"
options = ["L’ask", "Le bid", "Le milieu"]
answer = 0
why = "Tu achètes à l’ask, le prix le plus haut."
[[recall]]
id = "r2"
q = "Bid 4 150,20 / Ask 4 150,60. Le spread est de…"
options = ["0,40 $", "4,00 $", "0,04 $"]
answer = 0
why = "4 150,60 − 4 150,20 = 0,40 $."
[[recall]]
id = "r3"
q = "Quand paies-tu le spread ?"
options = ["À l’ouverture", "À la clôture", "Une fois par mois"]
answer = 0
why = "Il se paie à l’entrée, donc un nouveau trade commence légèrement négatif."
+++
