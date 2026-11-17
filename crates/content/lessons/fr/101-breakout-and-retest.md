+++
id = 101
stage = 10
title = "Cassure et retest"
objective = "ne trader une cassure qu’après le retour du prix sur la zone cassée et la clôture d’une bougie de confirmation, et repérer les fausses cassures des journées calmes pour les écarter."
hook = "L’or a buté trois fois sur le même plafond. Ce matin, une bougie a enfin clôturé au-dessus. Tu achètes maintenant, ou tu attends ?"
analogy = "Pense à une foule qui pousse une grille de stade. Une fois la grille ouverte, elle devient l’entrée, et la barrière est derrière. Mais deux ou trois personnes qui se faufilent dessous quand personne ne regarde, ça ne veut pas dire que la grille est ouverte. Tu attends de voir si la foule tient de l’autre côté."
analogy_tag = "la grille du stade"
example = "L’or a évolué deux jours sous une zone à 4 150, repoussé trois fois. Mardi à 9 h, heure de New York, une bougie H1 clôture à 4 162. Pas de course après le prix. Mercredi à 4 h, le prix redescend à 4 151 et une pin bar haussière clôture à 4 156, sa mèche à 4 148. L’ATR H1 est de 6 $ : stop à 4 142, soit 14 $ de risque. La zone suivante est à 4 190 : 34 $ de gain, environ 2,4 pour 1. Dimensionné à 1 %, c’est un trade. (Prix donnés à titre d’exemple.)"
mistake = "Acheter la bougie de cassure elle-même. Les cassures retombent souvent dans le range, surtout les jours calmes, et une entrée en haut de la bougie de cassure laisse le stop loin et le gain petit. Si le prix ne revient jamais tester la zone, tu rates simplement celle-là."
takeaways = [
  "Une cassure, c’est une bougie qui clôture nettement au-delà d’une zone que le prix a respectée plusieurs fois ; une mèche qui dépasse n’est pas une cassure.",
  "Ne cours pas après la bougie de cassure : attends que le prix revienne sur la zone cassée et qu’une bougie de confirmation clôture dans le sens de la cassure.",
  "Stop à environ 1 à 1,5 × l’ATR au-delà de la mèche de la bougie de retest, objectif sur la zone suivante, au moins 2 pour 1, taille selon le risque de ton plan.",
  "Méfie-toi des cassures des journées calmes ou contre la tendance H4 : si le prix clôture de nouveau dans l’ancien range, le setup est mort.",
]

[[steps]]
heading = "Ce qui compte comme une cassure"
text = "Pars d’une **zone** que le prix a respectée au moins deux fois (leçon 24). Une **cassure**, c’est une bougie H1 qui **clôture** nettement au-delà. Une mèche qui dépasse puis clôture à l’intérieur n’est pas une cassure. Sur un range, cherche la cassure dans le sens de la **tendance H4**."
why = "Le cours de CME Group décrit la consolidation comme un range au-delà duquel ni les acheteurs ni les vendeurs n’arrivent à pousser le prix, et note que le prix tend à sortir d’une consolidation dans le sens où il y est entré."

[[steps]]
heading = "Attends le retest"
text = "N’achète pas la bougie de cassure. Attends que le prix **revienne sur la zone cassée** : l’ancienne résistance, devenue support (ou l’inverse pour une vente). Puis attends qu’une **bougie de confirmation** (pin bar ou avalement, leçon 26) **clôture** dans le sens de la cassure. Pas de retest, pas de trade."
why = "CME Group note que lorsqu’une résistance est cassée, ce niveau tend à devenir un support quand le prix y revient. Le retest, c’est le moment où tu vérifies si c’est le cas."

[[steps]]
heading = "Stop, objectif, taille"
text = "Place le **stop** à environ 1 à 1,5 × l’ATR au-delà de la mèche de la bougie de retest, ce qui le met aussi dans l’ancien range (leçon 32). Vise la **zone suivante**. Le gain doit valoir au moins **deux fois** le risque. Puis dimensionne selon le risque par trade de ton plan (leçon 33)."
why = "Si le prix retombe dans l’ancien range, la cassure a échoué et l’idée est fausse. C’est exactement là que le stop doit être."

[[steps]]
heading = "Les fausses cassures des jours calmes"
text = "Les cassures échouent le plus souvent quand peu de monde trade : la **séance asiatique**, les jours fériés, la fin d’après-midi du vendredi, ou toute journée de toutes petites bougies. Si le prix **clôture de nouveau dans** le range avant ta confirmation, le setup est **mort** ; note-le comme écarté et passe à autre chose."
why = "Sans volume derrière, une cassure n’est souvent que quelques ordres qui poussent le prix dans un marché peu liquide. Les vraies cassures arrivent plutôt quand Londres ou New York tradent."

[[steps]]
heading = "Une nouvelle version du plan"
text = "La cassure et retest est un nouveau type de **Scénario**. Avant de la trader en réel, ajoute-la dans une **nouvelle version de ton Plan de trading** avec ces règles, et entraîne-toi d’abord en démo. Note-la à part dans ton journal pour que ton bilan (leçon 41) puisse la comparer à tes setups classiques."
why = "Chaque stratégie avancée a besoin de ses propres règles écrites et de son propre historique. Sinon, impossible de savoir si elle t’aide ou si elle fuit."

[[videos]]
id = "CIsq8mHftAo"
title = "Why 90% of Traders FAIL the Break and Retest"
channel = "Tom Crown"
duration = "8:46"
watch_for = "Les façons les plus courantes de rater une cassure et retest, comme entrer avant que le retest ait montré son jeu. Ses marchés et ses chiffres sont les siens."
fallback_search = "break and retest mistakes"
checked = "2026-11-17"

[[videos]]
id = "WX0nV7mHxis"
title = "The ULTIMATE Break & Retest Strategy (Most Traders Fail Without This)"
channel = "Pro Trading School"
duration = "16:57"
watch_for = "Comment un niveau cassé change de rôle au retest, et pourquoi attendre une confirmation à cet endroit compte. Ignore les promesses ; les règles de ton plan passent d’abord."
fallback_search = "break and retest strategy explained"
checked = "2026-11-17"

[[sources]]
claim = "La consolidation, c’est quand le prix n’arrive pas à sortir d’un range défini parce que ni les acheteurs ni les vendeurs ne peuvent le pousser au-delà d’un certain prix ; le prix tend à sortir d’une consolidation dans le sens où il y est entré."
name = "CME Group Education: Trend and Continuation Patterns"
url = "https://www.cmegroup.com/education/courses/technical-analysis/trend-and-continuation-patterns.html"
checked = "2026-11-17"

[[sources]]
claim = "Quand le prix casse un support ou une résistance, le support devient résistance et la résistance devient support."
name = "CME Group Education: Support and Resistance"
url = "https://www.cmegroup.com/education/courses/technical-analysis/support-and-resistance.html"
checked = "2026-11-17"

[[quiz]]
id = "q1"
q = "Résistance à 4 150. La mèche d’une bougie H1 atteint 4 156 mais elle clôture à 4 147. Est-ce une cassure ?"
options = ["Non : elle a clôturé sous la zone", "Oui : le prix est passé au-dessus de 4 150", "Oui, si c’était pendant Londres", "Seulement en H4"]
answer = 0
why = "Une cassure demande une clôture au-delà de la zone."

[[quiz]]
id = "q2"
q = "Une bougie H1 clôture nettement au-dessus de la résistance. Que fais-tu ensuite ?"
options = ["J’attends que le prix revienne sur la zone cassée et une bougie de confirmation", "J’achète à la clôture de la bougie de cassure", "Je vends, parce que les cassures échouent", "Je monte mon objectif"]
answer = 0
why = "Pas de course après le prix : le retest et la confirmation, c’est le setup."

[[quiz]]
id = "q3"
q = "Après une cassure au-dessus de 4 150, le prix redescend et clôture à 4 141, dans l’ancien range, avant toute confirmation. Et maintenant ?"
options = ["Le setup est mort : je le note comme écarté", "J’achète : c’est moins cher", "J’attends encore quelques heures le retest", "J’élargis le stop"]
answer = 0
why = "Une clôture dans le range veut dire que la cassure a échoué."

[[quiz]]
id = "q4"
q = "Mèche de la bougie de retest à 4 148, ATR H1 de 6 $, entrée à 4 156. Où va le stop ?"
options = ["Vers 4 142, environ 1 ATR au-delà de la mèche", "4 150, sur la zone", "4 147, juste sous la mèche", "4 120, loin par sécurité"]
answer = 0
why = "1 à 1,5 × l’ATR au-delà de la mèche, de retour dans l’ancien range."

[[quiz]]
id = "q5"
q = "Quelle cassure mérite le plus de méfiance ?"
options = ["Une cassure pendant une séance asiatique calme un jour férié", "Une cassure pendant Londres avec la tendance H4", "Une cassure qui est retestée et confirmée", "Une cassure un matin chargé à New York"]
answer = 0
why = "Les marchés calmes produisent de fausses cassures."

[[quiz]]
id = "q6"
q = "Que fais-tu avant de trader la cassure et retest en réel ?"
options = ["Je l’ajoute dans une nouvelle version du Plan de trading et je m’entraîne en démo", "Rien : c’est comme les setups classiques", "Je double le risque parce que c’est avancé", "J’abandonne la règle du 2 pour 1 pour les cassures"]
answer = 0
why = "Nouveau Scénario, nouvelles règles écrites, démo d’abord."

[[recall]]
id = "r1"
q = "Une cassure demande une bougie qui…"
options = ["Clôture au-delà de la zone", "Touche la zone", "A une longue mèche"]
answer = 0
why = "Une mèche seule n’est pas une cassure."

[[recall]]
id = "r2"
q = "Tu entres sur…"
options = ["Le retest, après une bougie de confirmation", "La bougie de cassure", "Le premier contact avec la zone"]
answer = 0
why = "Pas de retest, pas de trade."

[[recall]]
id = "r3"
q = "Une clôture de retour dans l’ancien range veut dire…"
options = ["Que le setup est mort", "Une meilleure entrée"]
answer = 0
why = "La cassure a échoué."

[explain_back]
prompt = "Comment trades-tu une cassure et retest, et quand l’écartes-tu ?"
must_include = ["une cassure est une bougie qui clôture nettement au-delà d’une zone respectée au moins deux fois, pas une mèche qui dépasse", "attendre que le prix revienne sur la zone cassée et qu’une bougie de confirmation clôture dans le sens de la cassure, puis mettre le stop à environ 1 à 1,5 ATR au-delà de sa mèche, viser la zone suivante à 2 pour 1 ou mieux, avec une taille selon le risque du plan", "écarter les cassures des jours calmes ou contre la tendance H4, et abandonner le setup si le prix clôture de nouveau dans l’ancien range"]
misconceptions = ["il faut acheter la bougie de cassure dès que le prix franchit le niveau", "n’importe quelle mèche à travers un niveau est une cassure"]

[[explain_back.fixtures]]
answer = "Il me faut d’abord une zone que le prix a respectée au moins deux fois, et une bougie qui clôture nettement au-delà ; une mèche qui dépasse ne compte pas. Je n’achète pas la bougie de cassure. J’attends que le prix revienne sur la zone cassée et qu’une pin bar ou un avalement clôture dans le sens de la cassure, puis je mets le stop à environ 1 à 1,5 ATR au-delà de sa mèche, je vise la zone suivante à 2 pour 1 ou mieux, et je dimensionne selon le risque de mon plan. Je me méfie des cassures des jours calmes ou contre la tendance H4, et si le prix clôture de nouveau dans l’ancien range, le setup est mort."
expect = "got_it"

[[explain_back.fixtures]]
answer = "Quand une bougie clôture au-delà d’une zone, j’attends que le prix y revienne et une bougie de confirmation, puis j’entre avec un stop au-delà de la mèche et un objectif à 2 pour 1."
expect = "almost"

[[explain_back.fixtures]]
answer = "Dès que le prix dépasse la résistance, j’achète tout de suite, parce qu’attendre, c’est rater le mouvement."
expect = "almost_or_not_yet"
+++
