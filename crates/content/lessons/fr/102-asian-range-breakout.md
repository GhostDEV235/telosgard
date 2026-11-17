+++
id = 102
stage = 10
title = "Cassure de la fourchette asiatique"
objective = "marquer le plus haut et le plus bas de la séance asiatique, ne trader sa cassure que dans une fenêtre fixe après l’ouverture de Londres, et savoir exactement où l’idée est prouvée fausse."
hook = "Cette nuit, pendant que Tokyo tradait, l’or a dérivé dans une boîte de 13 $. Londres va ouvrir. Par où va-t-il sortir de la boîte, et comment sauras-tu que tu as mal lu le sens ?"
analogy = "Pense à une bouilloire sur feu doux. Pendant un moment, il ne se passe pas grand-chose ; la pression monte. Quand on augmente le feu, ça déborde d’un côté ou de l’autre. Tu ne devines pas le côté à l’avance : tu regardes par où ça part, et tu décides d’avance ce qui voudrait dire que tu as mal lu."
analogy_tag = "la bouilloire"
example = "Fourchette asiatique : de 4 118 à 4 131, 13 $ de large, avec un ATR H1 de 7 $. La bougie de 3 h à 4 h, heure de New York, clôture à 4 136, au-dessus de la boîte et dans le sens de la tendance haussière H4. Entrée à 4 136, stop sous le milieu de la boîte (4 124,50) à 4 124 : 12 $ de risque. Objectif 4 160, deux fois le risque, juste avant la zone H4 suivante à 4 165. Sur un compte de 5 000 $ à 1 %, ça fait 0,04 lot. (Prix donnés à titre d’exemple.)"
mistake = "Trader la cassure quel que soit le moment. Une cassure à 11 h, heure de New York, des heures après l’ouverture de Londres, n’est plus ce setup : l’élan de l’ouverture est passé et la direction du jour est peut-être déjà fixée. Hors de la fenêtre, il n’y a pas de trade sur la fourchette asiatique."
takeaways = [
  "Marque la fourchette asiatique : le plus haut et le plus bas entre le début de la séance asiatique et l’ouverture de Londres.",
  "Ne trade qu’une clôture H1 au-delà de la fourchette dans une fenêtre fixe, les trois premières heures après l’ouverture de Londres, de préférence avec la tendance H4.",
  "L’idée est fausse si le prix repasse le milieu de la fourchette : le stop va là, et l’objectif vaut au moins deux fois le risque.",
  "Écarte-la quand la fourchette est trop large pour un objectif à 2 pour 1, quand la cassure va contre la tendance H4, ou quand la fenêtre est fermée.",
]

[[steps]]
heading = "Marque la boîte"
text = "En H1, marque le **plus haut** et le **plus bas** entre le début de la séance asiatique et **l’ouverture de Londres** (3 h, heure de New York ; l’horloge des séances de l’appli l’affiche dans ton heure). C’est la **fourchette asiatique**. Son milieu, à mi-chemin entre les deux, comptera plus tard."
why = "Les heures asiatiques sont en général plus calmes pour l’or que Londres et New York, donc le prix dérive souvent en latéral et construit un range. Le cours de CME Group appelle ça une consolidation : aucun camp n’arrive à pousser le prix au-delà."

[[steps]]
heading = "Une fenêtre fixe"
text = "Ne trade une cassure que pendant les **trois premières heures** après l’ouverture de Londres : de 3 h à 6 h, heure de New York. Il te faut une **bougie H1 qui clôture** au-delà de la fourchette. C’est la cassure **dans le sens de la tendance H4** qu’on prend. Si la fenêtre se ferme sans cassure, pas de trade aujourd’hui."
why = "Londres est le centre du marché de gré à gré de l’or. Son ouverture apporte souvent l’activité qui casse la fourchette de la nuit ; plus tard dans la journée, la cassure de l’ancienne boîte veut dire beaucoup moins."

[[steps]]
heading = "Une invalidation claire"
text = "Si le prix repasse le **milieu de la fourchette**, la cassure a échoué. Place le **stop** juste au-delà de ce milieu. L’objectif vaut au moins **deux fois le risque**, et doit pouvoir être atteint avant la zone H4 suivante. Dimensionne selon le risque de ton plan."
why = "Une cassure nette ne devrait pas revenir à mi-chemin dans la boîte. Utiliser le milieu garde le stop sur un niveau qui veut vraiment dire quelque chose, pas sur un chiffre arbitraire."

[[steps]]
heading = "Quand l’écarter"
text = "Écarte-la quand la fourchette est **trop large** pour un 2 pour 1 avant la zone suivante, quand la cassure va **contre la tendance H4**, quand une bougie clôture de nouveau dans la fourchette avant ton entrée, ou un **jour férié à Londres**. Et comme toute stratégie avancée, ajoute-la d’abord dans une **nouvelle version du plan**."
why = "Une fourchette large, c’est un stop loin, une petite taille et un objectif qui n’arrivera peut-être jamais. Moins de trades, plus propres, valent mieux que d’en forcer un chaque matin."

[[videos]]
id = "h2Q7SGbd4o4"
title = "London Breakout Strategy the Right Way"
channel = "The Moving Average"
duration = "2:45"
watch_for = "Comment on marque une fourchette construite avant l’ouverture de Londres et comment on la trade une fois Londres lancé. Ses instruments et réglages peuvent différer ; la fenêtre et les règles de stop de ce cours passent d’abord."
fallback_search = "london breakout strategy asian range"
checked = "2026-11-17"

[[videos]]
id = "8P9BHSVD_vI"
title = "How To Trade The Opening Range Breakout (ORB)"
channel = "BrandonTrades"
duration = "8:22"
watch_for = "La même idée appliquée à la fourchette d’ouverture d’un marché d’actions : marquer la fourchette, attendre la cassure, savoir où on a tort. Un autre marché ; la logique reste valable."
fallback_search = "opening range breakout explained"
checked = "2026-11-17"

[[sources]]
claim = "La consolidation, c’est quand le prix n’arrive pas à sortir d’un range défini parce que ni les acheteurs ni les vendeurs ne peuvent le pousser au-delà d’un certain prix ; le prix tend à sortir d’une consolidation dans le sens où il y est entré."
name = "CME Group Education: Trend and Continuation Patterns"
url = "https://www.cmegroup.com/education/courses/technical-analysis/trend-and-continuation-patterns.html"
checked = "2026-11-17"

[[sources]]
claim = "Le marché de gré à gré de Londres et le marché à terme de New York font partie des grandes places du marché de l’or."
name = "World Gold Council: Global gold market"
url = "https://gold.org/gold-market-structure/global-gold-market"
checked = "2026-11-17"

[[sources]]
claim = "Une étude des contrats à terme sur l’or du COMEX a montré que les schémas intrajournaliers ont un effet profond sur la dynamique de la volatilité de l’or."
name = "Journal of Futures Markets (2001): What moves the gold market?"
url = "https://ideas.repec.org/a/wly/jfutmk/v21y2001i3p257-278.html"
checked = "2026-11-17"

[[quiz]]
id = "q1"
q = "Qu’est-ce que la fourchette asiatique ?"
options = ["Le plus haut et le plus bas entre le début de la séance asiatique et l’ouverture de Londres", "Le range complet de la journée", "Le range de la dernière bougie H4", "La distance entre deux chiffres ronds"]
answer = 0
why = "C’est la boîte de la nuit, avant Londres."

[[quiz]]
id = "q2"
q = "Fourchette 4 118–4 131. À 10 h 30, heure de New York, une bougie H1 clôture à 4 140. Tu la trades comme une cassure de la fourchette asiatique ?"
options = ["Non : la fenêtre de 3 h à 6 h est fermée", "Oui : elle a clôturé au-dessus de la fourchette", "Oui, avec une taille plus petite", "Seulement si Londres est encore ouvert"]
answer = 0
why = "Hors de la fenêtre, pas de trade sur la fourchette asiatique."

[[quiz]]
id = "q3"
q = "Fourchette 4 118–4 131. Où l’idée devient-elle fausse après une cassure vers le haut ?"
options = ["Quand le prix repasse le milieu, vers 4 124,50", "À 4 131, le haut de la fourchette", "À 4 100", "Il n’y a pas de point fixe"]
answer = 0
why = "Une cassure nette ne devrait pas revenir à mi-chemin dans la boîte."

[[quiz]]
id = "q4"
q = "La fourchette fait 40 $ de large et la zone H4 suivante est 25 $ au-dessus du haut. Que fais-tu ?"
options = ["Je l’écarte : le stop est trop loin pour un objectif à 2 pour 1", "Je trade avec un stop plus serré", "Je trade avec une taille doublée", "J’attends 11 h"]
answer = 0
why = "Une fourchette large rend le rapport gain/risque impossible."

[[quiz]]
id = "q5"
q = "Deux cassures dans la fenêtre : une avec la tendance haussière H4, une contre. Laquelle respecte les règles ?"
options = ["Celle avec la tendance H4", "Celle contre", "Les deux", "Aucune"]
answer = 0
why = "Une cassure contre la tendance H4 s’écarte."

[[recall]]
id = "r1"
q = "La fenêtre de trading, c’est…"
options = ["Les trois premières heures après l’ouverture de Londres", "Toute la journée", "La séance asiatique"]
answer = 0
why = "De 3 h à 6 h, heure de New York."

[[recall]]
id = "r2"
q = "Le stop va au-delà…"
options = ["Du milieu de la fourchette", "Du plus haut du jour", "D’un chiffre rond de ton choix"]
answer = 0
why = "C’est là que la cassure a échoué."

[[recall]]
id = "r3"
q = "Une cassure demande…"
options = ["Une bougie H1 qui clôture au-delà de la fourchette", "Une mèche qui touche la fourchette"]
answer = 0
why = "Une clôture, pas un contact."

[explain_back]
prompt = "Comment trades-tu la cassure de la fourchette asiatique, et où est-elle prouvée fausse ?"
must_include = ["marquer le plus haut et le plus bas entre le début de la séance asiatique et l’ouverture de Londres", "ne trader qu’une clôture H1 au-delà de la fourchette dans les trois premières heures après l’ouverture de Londres, de préférence avec la tendance H4", "le stop va juste au-delà du milieu de la fourchette, l’objectif vaut au moins deux fois le risque, et on écarte les fourchettes trop larges pour ça ou les cassures hors de la fenêtre"]
misconceptions = ["on peut trader la cassure de la fourchette asiatique à n’importe quelle heure", "le stop peut aller n’importe où tant qu’il est petit"]

[[explain_back.fixtures]]
answer = "Je marque en H1 le plus haut et le plus bas entre le début de la séance asiatique et l’ouverture de Londres. Ensuite je ne trade qu’une bougie H1 qui clôture au-delà de cette boîte dans les trois premières heures après l’ouverture de Londres, de 3 h à 6 h à New York, et de préférence avec la tendance H4. Si le prix repasse le milieu de la fourchette, la cassure a échoué, donc mon stop va juste au-delà du milieu, et il me faut au moins deux fois le risque avant la zone suivante. Si la fourchette est trop large pour ça, si la cassure va contre la tendance H4 ou si la fenêtre est fermée, j’écarte."
expect = "got_it"

[[explain_back.fixtures]]
answer = "Je marque le haut et le bas de l’Asie, et j’achète ou je vends quand une bougie H1 clôture en dehors après l’ouverture de Londres, avec un objectif à 2 pour 1."
expect = "almost"

[[explain_back.fixtures]]
answer = "Dès que l’or casse la fourchette de la nuit, le matin ou l’après-midi, je saute dedans avec un petit stop juste derrière la bougie."
expect = "almost_or_not_yet"
+++
