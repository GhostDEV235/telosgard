+++
id = 32
stage = 7
title = "Le stop au-delà de la structure, dosé avec l’ATR"
objective = "placer un stop là où le graphique te donne tort : au-delà de la structure, à environ 1 à 1,5 fois l’ATR, et à l’écart d’un chiffre rond évident."
hook = "Ton stop était 1 $ sous le plus bas de la bougie. L’or a plongé de 2 $, t’a sorti, puis est monté jusqu’à ton objectif sans toi. C’était l’idée qui était fausse, ou le stop ?"
analogy = "Pense à une voiture garée au bord d’une route passante. Gare-toi pile sur la ligne et les voitures qui passent accrochent ton rétroviseur, sans même te viser. Gare-toi à une distance raisonnable et seul un vrai accident t’atteint. L’ATR te dit de combien la circulation s’écarte."
analogy_tag = "garer sa voiture"
example = "Achat sur repli. Le plus bas de la bougie de confirmation H1 est à 4 105 et l’ATR(14) en H1 vaut 6 $. Un ATR sous le plus bas, c’est 4 099, juste au-dessus du chiffre rond 4 100, où beaucoup de stops se trouvent déjà. Le stop va donc quelques dollars plus loin, à 4 097 : 8 $ sous le plus bas, environ 1,3 fois l’ATR. Entrée à 4 113, donc 16 $ de risque, et l’objectif doit être au moins à 4 145. (Prix donnés à titre d’exemple.)"
mistake = "Choisir le stop d’après ce que tu as envie de perdre (« 5 $, ça me va ») puis le coincer sous la bougie. Un stop placé dans le bruit normal se fait toucher par des mouvements ordinaires ; c’est la taille que tu ajustes, jamais le stop."
takeaways = [
  "Le graphique décide du stop : au-delà de la structure qui te donne tort, la mèche de la bougie de confirmation et la zone.",
  "L’ATR (Average True Range) mesure le mouvement normal ; place le stop à environ 1 à 1,5 fois l’ATR au-delà de la structure.",
  "Ne mets pas un stop sur un chiffre rond évident, ni quelques cents après : donne-lui quelques dollars de plus.",
  "Si le stop nécessaire ne laisse pas de place pour un objectif à 2 pour 1, écarte le trade au lieu de resserrer le stop.",
]
drills = ["place_stop"]

[[steps]]
heading = "Le graphique décide du stop"
text = "Un stop se place là où l’idée du trade est **prouvée fausse** : au-delà de la **structure**, c’est-à-dire la mèche de la bougie de confirmation et la zone (leçons 26 et 27). Si le prix y arrive, le setup a échoué. Un stop placé ailleurs n’est qu’un chiffre qui te plaisait."
why = "Le cours de CME Group sur la taille de position le dit : un stop ne se place pas au hasard, mais à un niveau logique qui te montre que tu as tort, et pas là où un mouvement normal du marché pourrait facilement le déclencher."

[[steps]]
heading = "L’ATR mesure le bruit"
text = "L’**ATR (Average True Range)** est la taille moyenne des bougies récentes, gaps compris, en général sur **14** bougies. Ajoute-le depuis la liste d’indicateurs de ta plateforme, sur l’unité de temps où tu entres (H1 ici). Un ATR de 6 $ veut dire qu’une bougie H1 normale couvre environ 6 $."
why = "Le guide de Fidelity : l’amplitude réelle d’une bougie est la plus grande des trois valeurs plus haut moins plus bas, plus haut moins clôture précédente, clôture précédente moins plus bas ; l’ATR en fait la moyenne, en général sur 14 périodes. L’ATR montre de combien le prix bouge, pas dans quel sens."

[[steps]]
heading = "1 à 1,5 ATR au-delà de la structure"
text = "Place le stop à environ **1 à 1,5 × l’ATR au-delà** de la structure : sous le plus bas pour un achat, au-dessus du plus haut pour une vente. Plus près, des mouvements ordinaires te sortent. Beaucoup plus loin, la taille fond et l’objectif devient hors de portée."
why = "Fidelity propose un multiple de l’ATR, comme 1,5, pour ne réagir qu’aux mouvements anormaux. 1 à 1,5 fois, c’est la fourchette de ce cours, pas une loi ; quand le marché s’agite, l’ATR grandit et ton stop s’élargit avec lui, automatiquement."

[[steps]]
heading = "Reste à l’écart du chiffre rond"
text = "Beaucoup de stops s’accumulent **juste après les chiffres ronds** comme 4 100 ou 4 050. Quand le prix les touche, ils partent ensemble et peuvent pousser le prix plus loin, brièvement. Ne gare donc pas le tien là : s’il tombe dessus ou quelques cents après, éloigne-le de **quelques dollars**."
why = "Une étude de la Federal Reserve Bank of New York sur les ordres de change d’une banque a montré que les stop-loss de vente s’accumulaient juste sous les chiffres ronds et les stop-loss d’achat juste au-dessus, et que les prix avancent vite une fois ces niveaux franchis."

[[steps]]
heading = "Trop large ? On écarte, on ne serre pas"
text = "Une fois le stop placé, vérifie l’objectif : il doit toujours être au moins à **deux fois** la distance du stop (leçon 27). Sinon, **écarte** le trade. Ne rapproche jamais le stop pour que les chiffres collent ; la leçon 33 réduit plutôt la taille."
why = "Serrer le stop ne rend pas un trade plus sûr : un mouvement normal devient une perte. La distance du stop décide ensuite de la taille : c’est la leçon suivante."

[[videos]]
id = "zMT1Fs2u9nY"
title = "ATR - Average True Range indicator EXPLAINED in less than 2 minutes"
channel = "ForexSignals TV"
duration = "1:49"
watch_for = "Ce que mesure la ligne de l’ATR et pourquoi un chiffre plus grand veut dire des bougies plus larges. Il montre de combien le prix bouge, jamais dans quel sens."
fallback_search = "average true range explained"
checked = "2026-11-16"

[[videos]]
id = "M79kxiOMJJg"
title = "How to Set a Stop loss in Trading - So You Don't Get Stopped Out Unnecessarily"
channel = "Rayner Teo"
duration = "8:49"
watch_for = "Placer le stop au-delà de la structure avec une marge tirée de l’ATR, plutôt qu’à une distance fixe. Ses multiples peuvent différer du 1 à 1,5 de ce cours."
fallback_search = "where to set stop loss ATR structure"
checked = "2026-11-16"

[[sources]]
claim = "L’ATR est la moyenne des amplitudes réelles sur une période, en général 14, et mesure la volatilité gaps compris ; il n’indique pas de direction. Un stop fondé sur l’ATR s’adapte à la volatilité, contrairement à un stop fixe ; un multiple de l’ATR, comme 1,5 × ATR, sert à repérer les mouvements anormaux."
name = "Fidelity Learning Center: Average True Range (ATR)"
url = "https://www.fidelity.com/learning-center/trading-investing/technical-analysis/technical-indicator-guide/atr"
checked = "2026-11-16"

[[sources]]
claim = "Un stop ne se place pas au hasard : il doit être à un niveau logique, qui te dit si tu te trompes de direction, et pas là où des mouvements normaux du marché pourraient facilement le déclencher."
name = "CME Group Education: Proper Position Size"
url = "https://www.cmegroup.com/education/courses/trade-and-risk-management/proper-position-size.html"
checked = "2026-11-16"

[[sources]]
claim = "Dans le carnet d’ordres de change d’une grande banque, les prix des ordres s’accumulaient sur les chiffres ronds ; les stop-loss d’achat juste au-dessus des chiffres ronds et les stop-loss de vente juste en dessous, et les tendances accélèrent souvent une fois ces niveaux franchis."
name = "Federal Reserve Bank of New York Staff Report 125 (C. L. Osler): Currency Orders and Exchange-Rate Dynamics"
url = "https://www.newyorkfed.org/research/staff_reports/sr125.html"
checked = "2026-11-16"

[[quiz]]
id = "q1"
q = "Qu’est-ce qui décide de l’endroit où va ton stop ?"
options = ["Le graphique : au-delà de la structure qui donne tort à l’idée", "La somme que tu as envie de perdre", "5 $ fixes sur chaque trade", "Le chiffre rond le plus proche"]
answer = 0
why = "Le stop va là où le setup a échoué ; la taille ajuste ensuite la perte à ton plan."

[[quiz]]
id = "q2"
q = "Que t’apprend un ATR H1 de 6 $ ?"
options = ["Une bougie H1 normale a récemment couvert environ 6 $", "Le prix va monter de 6 $", "Ton stop doit être exactement à 6 $", "Le spread vaut 6 $"]
answer = 0
why = "L’ATR mesure de combien le prix bouge, pas dans quel sens."

[[quiz]]
id = "q3"
q = "Achat. Le plus bas de la bougie de confirmation est à 4 105 et l’ATR vaut 6 $. Où va le stop ?"
options = ["4 097 : environ 1,3 ATR sous le plus bas, après le chiffre rond", "4 104,50 : juste sous la mèche", "4 100,00 : pile sur le chiffre rond", "4 087 : à trois ATR"]
answer = 0
why = "8 $ sous le plus bas, c’est environ 1,3 × 6 $, et c’est quelques dollars après 4 100 plutôt que dessus."

[[quiz]]
id = "q4"
q = "Pourquoi ne pas mettre le stop d’un achat à 4 099,90, juste sous 4 100 ?"
options = ["Beaucoup de stops de vente s’accumulent juste sous les chiffres ronds, et le prix peut les traverser", "Les courtiers l’interdisent", "C’est trop loin de l’entrée", "Les chiffres ronds ne comptent jamais"]
answer = 0
why = "L’étude de la Fed de New York a montré que les stop-loss de vente s’accumulaient juste sous les chiffres ronds."

[[quiz]]
id = "q5"
q = "Ton stop fondé sur la structure est à 20 $ et la zone suivante seulement à 25 $. Que fais-tu ?"
options = ["J’écarte le trade : l’objectif ne fait pas deux fois le risque", "Je rapproche le stop à 12 $ pour que ça passe", "Je supprime le stop", "Je double la taille"]
answer = 0
why = "Ne serre jamais le stop pour sauver un trade : écarte-le."

[[quiz]]
id = "q6"
q = "Le marché s’agite et l’ATR double. Que devient ton stop ?"
options = ["Il s’élargit, et la taille baisse pour garder le même montant en risque", "Il reste à la même distance", "Il se resserre", "Tu arrêtes d’utiliser un stop"]
answer = 0
why = "Un stop plus large avec une taille plus petite garde la même perte."

[[recall]]
id = "r1"
q = "Un stop se place…"
options = ["Au-delà de la structure qui donne tort à l’idée", "Là où la perte te paraît confortable", "Sur le chiffre rond le plus proche"]
answer = 0
why = "Le graphique décide du stop."

[[recall]]
id = "r2"
q = "Ce cours place le stop à environ…"
options = ["1 à 1,5 × l’ATR au-delà de la structure", "Pile sur le plus bas de la bougie", "10 × l’ATR"]
answer = 0
why = "Assez loin pour résister au bruit ordinaire, assez près pour que le trade vaille la peine."

[[recall]]
id = "r3"
q = "Un stop sur un chiffre rond, ou quelques cents après…"
options = ["Est là où beaucoup de stops s’accumulent : éloigne-le de quelques dollars", "Est l’endroit le plus sûr"]
answer = 0
why = "Quand ces stops partent ensemble, le prix peut filer un instant."

[explain_back]
prompt = "Comment décides-tu où va un stop, et en quoi l’ATR t’aide-t-il ?"
must_include = ["le stop va au-delà de la structure qui donne tort au trade, la mèche de la bougie de confirmation et la zone, pas à un montant choisi d’avance", "l’ATR mesure le mouvement normal, et le stop se place à environ 1 à 1,5 fois l’ATR au-delà de la structure", "tu le gardes à l’écart d’un chiffre rond évident, et si le stop ne laisse pas de place pour un objectif à 2 pour 1, tu écartes le trade au lieu de resserrer le stop"]
misconceptions = ["un stop plus serré veut toujours dire moins de risque", "on choisit le stop d’après ce qu’on veut perdre"]

[[explain_back.fixtures]]
answer = "C’est le graphique qui décide : le stop va au-delà de la structure qui montre que j’ai tort, donc après la mèche de la bougie de confirmation et la zone. L’ATR me dit de combien l’or bouge normalement, alors je mets le stop à environ 1 à 1,5 ATR au-delà de ce point, et si ça tombe sur un chiffre rond comme 4 100, ou juste après, je l’éloigne de quelques dollars parce que les stops s’y accumulent. Si l’objectif ne fait pas au moins deux fois ce stop, j’écarte le trade au lieu de serrer le stop."
expect = "got_it"

[[explain_back.fixtures]]
answer = "Je mets le stop après la mèche de la bougie de confirmation et la zone, à environ 1 à 1,5 fois l’ATR, parce que l’ATR montre de combien l’or bouge normalement."
expect = "almost"

[[explain_back.fixtures]]
answer = "Je décide que perdre 5 $ me va, donc je mets le stop à 5 $ de mon entrée. Un stop serré est toujours plus sûr."
expect = "almost_or_not_yet"
+++
