+++
id = 15
stage = 2
title = "Les fenêtres à éviter"
objective = "citer les trois Fenêtres sans trade et expliquer pourquoi chacune est dangereuse pour un débutant."
hook = "Qu’ont en commun la pause quotidienne de l’or, sa réouverture du dimanche et une grosse statistique américaine ?"
analogy = "Pense à traverser une route. La plupart du temps, tu regardes des deux côtés et tu traverses. Mais dans un virage sans visibilité, ou quand un camion arrive déjà, tu attends, même si le temps presse. Les Fenêtres sans trade sont les virages sans visibilité de l’or : quelques minutes où tu ne vois pas ce qui arrive."
analogy_tag = "virage sans visibilité"
example = "Mercredi, 8 h 27 heure de New York, et l’inflation américaine (CPI) sort à 8 h 30. Tu repères une configuration, mais la règle dit d’attendre 8 h 35. À 8 h 30, l’or bondit de 15 $ en quelques secondes, puis retombe de 10 $. Attendre ne t’a rien coûté ; un trade pendant ces secondes aurait pu s’exécuter à plusieurs dollars de ton prix. (Prix donnés à titre d’exemple.)"
mistake = "Croire qu’un stop-loss te protège totalement dans ces fenêtres. Quand le prix saute, ton stop peut s’exécuter bien au-delà de son niveau. C’est le slippage, et il peut transformer une perte prévue de 100 $ en 150 $."
takeaways = [
  "Aucun nouveau trade pendant le rollover quotidien, de 17 h à 18 h heure de New York.",
  "Aucun nouveau trade dans la première heure après l’ouverture du dimanche.",
  "Aucun nouveau trade de 5 minutes avant à 5 minutes après une grande statistique américaine (CPI, NFP, FOMC).",
  "Ce sont des Règles strictes : le journal vérifie l’heure d’entrée de chaque Trade démo.",
]
drills = ["no_trade"]

[[steps]]
heading = "Le rollover quotidien"
text = "À **17 h, heure de New York**, la journée de trading se termine et les positions ouvertes basculent sur le jour suivant. Beaucoup de traders s’éloignent, le marché se vide et **les spreads s’élargissent souvent**. Les courtiers prélèvent ou versent aussi un **swap** sur les positions gardées. N’ouvre pas de nouveau trade de 17 h à 18 h."
widget = "session_clock"
why = "Les contrats à terme sur l’or font une pause d’une heure à 17 h à New York. Les courtiers CFD peuvent continuer à coter, mais les prix sont maigres. Certains courtiers prélèvent trois jours de swap le mercredi soir pour couvrir le week-end ; vérifie le tableau de rollover de ton courtier."

[[steps]]
heading = "L’ouverture du dimanche"
text = "Le week-end, l’actualité continue mais l’or ne se trade pas. Quand il rouvre à **18 h, heure de New York, le dimanche**, le prix peut faire un **gap** : sauter directement de la clôture du vendredi à un nouveau niveau. Les spreads restent souvent larges un moment. Attends une heure avant de trader."
why = "Un gap veut dire qu’il n’y a eu aucun échange entre les deux prix : un stop-loss ne peut donc pas s’exécuter à son niveau ; il s’exécute au premier prix disponible après la réouverture."

[[steps]]
heading = "Les grandes statistiques américaines"
text = "Les plus grosses publications américaines (l’inflation **CPI**, l’emploi **NFP** et la décision de taux de la Fed, le **FOMC**) peuvent faire bouger l’or de beaucoup de dollars en quelques secondes. Les spreads s’élargissent et les ordres glissent. Reste en dehors **de 5 minutes avant à 5 minutes après** chacune ; ta Checklist avant la séance les liste."
why = "Le CPI et le NFP sont publiés à 8 h 30, heure de New York ; les décisions du FOMC à 14 h, avec une conférence de presse vers 14 h 30. Tu verras ce que veulent dire ces statistiques, et pourquoi elles font bouger l’or, à l’étape 3."

[[steps]]
heading = "Le slippage : pourquoi un stop n’est pas magique"
text = "Un stop-loss devient un ordre au marché quand il est touché. Si le prix saute par-dessus, il s’exécute au **prochain prix disponible**, qui peut être pire. C’est le **slippage**. C’est pour ça que ces fenêtres sont des règles, pas des suggestions."
why = "Certains courtiers vendent des stops garantis, moyennant des frais. Ce cours ne compte pas dessus ; il évite simplement les moments où le slippage est le plus probable."

[[steps]]
heading = "Ce sont des Règles strictes"
text = "Les trois **Fenêtres sans trade** font partie des **Règles strictes** de ton Plan de trading. Quand tu notes un Trade démo dans ton journal, l’appli vérifie automatiquement son heure d’entrée. Un trade ouvert dans une fenêtre enfreint ton plan, même s’il gagne de l’argent."
why = "Un gain ne rend pas un trade discipliné. « Prêt pour le réel » compte les trades qui ont suivi ton plan, pas ceux qui ont gagné par hasard."

[[videos]]
id = "bLLj88oGlDs"
title = "What is a Rollover Event When Trading?"
channel = "Plus500"
duration = "1:35"
watch_for = "Ce qui arrive à une position ouverte au rollover quotidien, et pourquoi elle peut être débitée ou créditée."
fallback_search = "what is rollover in trading"
checked = "2026-11-12"

[[videos]]
id = "DiL2LF57znE"
title = "Trading guide - What is a Slippage?"
channel = "IG Singapore"
duration = "1:39"
watch_for = "Pourquoi un ordre peut s’exécuter à un autre prix que celui demandé, et quand c’est le plus probable."
fallback_search = "what is slippage in trading"
checked = "2026-11-12"

[[sources]]
claim = "Les contrats à terme sur l’or se tradent du dimanche au vendredi, de 18 h à 17 h (heure de l’Est), avec une pause de 60 minutes chaque jour à partir de 17 h."
name = "CME Group : E-mini Gold contract specs"
url = "https://cmegroup.com/markets/metals/precious/e-mini-gold.contractSpecs.html"
checked = "2026-11-12"

[[sources]]
claim = "Le rollover s’applique aux positions ouvertes à la clôture de 17 h à New York ; un swap est débité ou crédité."
name = "FOREX.com : Rollover FAQs"
url = "https://forex.com/en-us/help-and-support/rollover"
checked = "2026-11-12"

[[sources]]
claim = "Certains courtiers appliquent trois jours de rollover le mercredi sur l’or (XAU/USD) pour couvrir le week-end."
name = "FXCM : calendrier de rollover des CFD"
url = "https://www.fxcm.com/eu/help/cfd-trading-cfd-rollover-calendar/"
checked = "2026-11-12"

[[sources]]
claim = "Le CPI américain et le rapport sur l’emploi (Employment Situation) sont publiés à 8 h 30, heure de l’Est."
name = "US Bureau of Labor Statistics : calendrier des publications"
url = "https://www.bls.gov/schedule/2026/11_sched_list.htm"
checked = "2026-11-12"

[[sources]]
claim = "Les communiqués du FOMC sont publiés à 14 h, heure de l’Est, et la conférence de presse du président commence vers 14 h 30."
name = "Réserve fédérale : horaires du communiqué et de la conférence de presse du FOMC"
url = "https://www.federalreserve.gov/newsevents/pressreleases/monetary20130313a.htm"
checked = "2026-11-12"

[[quiz]]
id = "q1"
q = "Laquelle est une Fenêtre sans trade dans ce cours ?"
options = ["La première heure de la séance de Londres", "La première heure après l’ouverture du dimanche", "Tous les vendredis", "Les enchères LBMA"]
answer = 1
why = "Le prix peut faire un gap et les spreads sont souvent larges juste après la réouverture du dimanche : tu attends une heure."

[[quiz]]
id = "q2"
q = "Le CPI américain sort à 8 h 30, heure de New York. Quand peux-tu ouvrir un trade au plus tôt ?"
options = ["8 h 30", "8 h 31", "8 h 35", "Pas avant le lendemain"]
answer = 2
why = "La fenêtre va de 5 minutes avant à 5 minutes après la publication : 8 h 35 au plus tôt."

[[quiz]]
id = "q3"
q = "Pourquoi l’or peut-il faire un gap à l’ouverture du dimanche ?"
options = ["Il y a eu de l’actualité le week-end alors qu’il ne se tradait pas", "Les courtiers remettent les prix à zéro chaque dimanche", "Le spread est nul à ce moment-là", "Tokyo ouvre en premier"]
answer = 0
why = "Deux jours d’actualité sont intégrés d’un coup à la reprise : le premier prix peut être loin de la clôture du vendredi."

[[quiz]]
id = "q4"
q = "Tu as acheté à 4 100,00 avec un stop à 4 095,00. Le prix saute directement de 4 100,00 à 4 090,00. Où ton stop peut-il s’exécuter ?"
options = ["Exactement à 4 095,00, toujours", "Vers 4 090,00, plus mal que 4 095,00", "À 4 100,00", "Il ne s’exécute pas du tout"]
answer = 1
why = "Le stop devient un ordre au marché et s’exécute au prochain prix disponible, ici vers 4 090,00. Ces 5 $ en plus, c’est le slippage."

[[quiz]]
id = "q5"
q = "Un trade démo ouvert à 17 h 20, heure de New York, un mardi, a fait un gain. A-t-il suivi le plan ?"
options = ["Oui : il a gagné de l’argent", "Non : il a été ouvert pendant la fenêtre du rollover", "Oui : le rollover ne compte que le vendredi", "Seulement si le spread était petit"]
answer = 1
why = "17 h–18 h heure de New York est une Fenêtre sans trade. Le gain ne change rien au fait que le trade a enfreint les règles."

[[quiz]]
id = "q6"
q = "Que se passe-t-il au rollover quotidien ?"
options = ["La journée de trading se termine, les swaps s’appliquent et les spreads s’élargissent souvent", "Le marché ferme toute la nuit", "Le prix de l’or est remis à zéro", "L’enchère LBMA a lieu"]
answer = 0
why = "À 17 h à New York, la journée de trading bascule : les positions sont débitées ou créditées d’un swap, et le peu d’échanges élargit souvent les spreads."

[[recall]]
id = "r1"
q = "Quand a lieu la fenêtre du rollover quotidien ?"
options = ["17 h–18 h, heure de New York", "8 h–9 h, heure de Londres", "Minuit UTC"]
answer = 0
why = "La journée de trading se termine à 17 h, heure de New York."

[[recall]]
id = "r2"
q = "Combien de temps attends-tu après l’ouverture du dimanche ?"
options = ["Une heure", "Cinq minutes", "Tout le lundi"]
answer = 0
why = "La première heure après la réouverture du dimanche est une Fenêtre sans trade."

[[recall]]
id = "r3"
q = "Qu’est-ce que le slippage ?"
options = ["Un ordre exécuté à un prix pire que celui demandé", "Des frais mensuels du courtier", "L’écart entre deux séances"]
answer = 0
why = "Quand le prix saute, les ordres, stops compris, s’exécutent au prochain prix disponible."

[explain_back]
prompt = "Cite les trois Fenêtres sans trade et explique pourquoi un débutant doit rester en dehors."
must_include = ["le rollover quotidien vers 17 h–18 h, heure de New York", "la première heure après l’ouverture du dimanche, quand le prix peut faire un gap", "5 minutes de part et d’autre d’une grande statistique américaine comme le CPI, le NFP ou le FOMC", "les spreads s’élargissent et les stops peuvent glisser, donc une perte peut dépasser ce qui était prévu"]
misconceptions = ["un stop-loss s’exécute toujours exactement à son prix", "les annonces sont le meilleur moment pour trader parce que les prix bougent vite"]

[[explain_back.fixtures]]
answer = "Pas de trade pendant le rollover quotidien de 17 h à 18 h heure de New York, ni dans la première heure après la réouverture de l’or le dimanche parce que le prix peut faire un gap, ni de 5 minutes avant à 5 minutes après les grandes statistiques américaines comme le CPI, le NFP et la décision du FOMC. À ces moments-là, les spreads s’élargissent et les stops peuvent glisser, donc une perte peut finir plus grosse que prévu."
expect = "got_it"

[[explain_back.fixtures]]
answer = "Éviter le rollover à 17 h à New York et les grosses annonces comme le CPI, parce que les spreads deviennent larges."
expect = "almost"

[[explain_back.fixtures]]
answer = "Les annonces sont le meilleur moment pour trader parce que l’or bouge vite, et de toute façon ton stop-loss te protège toujours."
expect = "almost_or_not_yet"
+++
