+++
id = 34
stage = 7
title = "Rapport gain/risque et espérance"
objective = "calculer le taux de réussite qu’il faut à un rapport gain/risque pour être à l’équilibre, juger un plan par son espérance en R, et voir pourquoi un petit risque fixe résiste aux séries de pertes."
hook = "Une stratégie ne gagne que 4 trades sur 10. C’est une mauvaise stratégie ?"
analogy = "Pense à un gardien de but pendant une séance de tirs au but. La plupart des penaltys entrent, et pourtant un gardien qui en arrête un sur trois peut faire gagner son équipe, parce que chaque arrêt compte plus que chaque but encaissé. C’est le score qui décide, pas le taux d’arrêts."
analogy_tag = "tirs au but"
example = "Tu risques 1R sur chaque trade et tu vises 2R. Sur 100 trades, tu en gagnes 40 et tu en perds 60 : 40 × 2R = +80R, 60 × 1R = −60R. Bilan +20R, soit +0,2R par trade. À 100 $ par R, ça fait +2 000 $, en te trompant 60 % du temps. (Chiffres donnés à titre d’exemple ; les coûts se retirent en premier.)"
mistake = "Juger une stratégie à son seul taux de réussite, ou chercher un taux élevé avec de tout petits objectifs. 70 % de gains à 0,3 pour 1 perd de l’argent : 70 × 0,3R = 21R gagnés, 30 × 1R = 30R perdus."
takeaways = [
  "Le rapport gain/risque compare la distance de l’objectif à celle du stop ; ce cours demande au moins 2 pour 1.",
  "Taux de réussite d’équilibre = 1 ÷ (1 + rapport gain/risque) : 50 % à 1 pour 1, environ 33 % à 2 pour 1, 25 % à 3 pour 1.",
  "L’espérance est le résultat moyen par trade, en R : taux de réussite × gain moyen − taux de pertes × perte moyenne.",
  "À 1 % par trade, dix pertes d’affilée laissent environ 90 % du compte ; à 5 %, environ 60 %.",
]

[[steps]]
heading = "Le rapport gain/risque"
text = "Le **rapport gain/risque** compare la distance de l’objectif à celle du stop. Stop à 10 $, objectif à 20 $ : **2 pour 1**, soit 2R pour 1R. Le minimum de ce cours est 2 pour 1 (leçon 27). Le rapport se fixe avant d’entrer, à partir du graphique."
why = "R, c’est la somme que tu perds si le stop est touché (leçon 2). Parler en R permet de comparer des trades de tailles différentes : une perte de 96 $ et une de 50 $ valent toutes les deux −1R si chacune était le risque prévu."

[[steps]]
heading = "Le taux de réussite d’équilibre"
text = "Chaque rapport a un **taux de réussite d’équilibre** : 1 ÷ (1 + rapport gain/risque). À **1 pour 1**, il faut gagner plus de **50 %** ; à **2 pour 1**, plus d’environ **33 %** ; à **3 pour 1**, plus de **25 %**. Au-dessus, tu gagnes sur beaucoup de trades ; en dessous, tu perds."
why = "À 2 pour 1, un gain (+2R) paie deux pertes (−2R). Gagner un trade sur trois, c’est donc l’équilibre exact. En pratique, le spread et la commission (leçon 11) relèvent un peu cette ligne."
widget = "win_rate"

[[steps]]
heading = "L’espérance : la moyenne du plan"
text = "L’**espérance** est ce que vaut un trade en moyenne, en R : **taux de réussite × gain moyen − taux de pertes × perte moyenne**. 40 % de gains à 2R : 0,4 × 2 − 0,6 × 1 = **+0,2R** par trade. Le but, c’est une espérance positive ; elle ne se voit que sur des dizaines de trades."
why = "Dix trades ne prouvent presque rien : un plan positif peut perdre six fois d’affilée. C’est pour ça que « Prêt pour le réel » demande 50 trades démo avant tout argent réel."

[[steps]]
heading = "Un petit risque survit aux séries"
text = "Les séries de pertes arrivent, même aux bons plans. À **1 %** par trade, dix pertes d’affilée laissent environ **90 %** du compte. À **5 %**, la même série en laisse environ **60 %**. Un risque petit et fixe laisse à une espérance positive le temps de jouer."
why = "0,99 multiplié dix fois par lui-même donne environ 0,904 ; 0,95 dix fois, environ 0,599. Les tableaux de CME Group montrent le même effet : un pourcentage fixe réduit chaque perte à mesure que le compte baisse."

[[videos]]
id = "fuFlkAnvSXI"
title = "The Ugly Truth About Risk To Reward Ratio (95% Of Traders Get It Wrong)"
channel = "Rayner Teo"
duration = "6:23"
watch_for = "Pourquoi un rapport ne veut rien dire sans le taux de réussite qui va avec. Un grand rapport avec un tout petit taux de réussite peut quand même perdre."
fallback_search = "risk to reward ratio win rate explained"
checked = "2026-11-16"

[[videos]]
id = "cbhXErBHuNM"
title = "Why Win Rate Alone Tells You Little: Risk-to-Reward Explained"
channel = "QuantCrawler"
duration = "9:37"
watch_for = "Le taux de réussite d’équilibre de chaque rapport, et comment l’espérance combine les deux."
fallback_search = "break even win rate risk reward"
checked = "2026-11-16"

[[sources]]
claim = "Les résultats en trading dépendent de ce que tu gagnes quand tu as raison et du peu que tu perds quand tu as tort, pas du fait d’avoir raison ; la peur des pertes pousse à vendre trop tôt les trades gagnants et à garder trop longtemps les perdants."
name = "CME Group Education: Misconceptions of Taking Losses"
url = "https://www.cmegroup.com/education/courses/trade-and-risk-management/misconceptions-of-taking-losses.html"
checked = "2026-11-16"

[[sources]]
claim = "Sur une série de cinq pertes, une règle fixe de 1 % mène un compte de 50 000 $ à 47 550 $ et une règle de 2 % à 45 198 $ : un pourcentage fixe réduit la taille de chaque trade suivant et ralentit la baisse du compte."
name = "CME Group Education: Controlling Risk"
url = "https://www.cmegroup.com/education/courses/trade-and-risk-management/controlling-risk.html"
checked = "2026-11-16"

[[quiz]]
id = "q1"
q = "Stop à 8 $, objectif à 24 $. Quel est le rapport gain/risque ?"
options = ["3 pour 1", "1 pour 3", "2 pour 1", "1 pour 1"]
answer = 0
why = "24 $ ÷ 8 $ = 3 : trois fois le risque."

[[quiz]]
id = "q2"
q = "À 2 pour 1, quel taux de réussite donne environ l’équilibre (avant coûts) ?"
options = ["Environ 33 %", "50 %", "66 %", "10 %"]
answer = 0
why = "1 ÷ (1 + 2) = 1/3 : un gain de +2R paie deux pertes de −1R."

[[quiz]]
id = "q3"
q = "Tu gagnes 40 % de tes trades à 2R et tu en perds 60 % à 1R. Quelle est l’espérance ?"
options = ["+0,2R par trade", "−0,2R par trade", "+0,8R par trade", "Zéro"]
answer = 0
why = "0,4 × 2 − 0,6 × 1 = 0,8 − 0,6 = +0,2R."

[[quiz]]
id = "q4"
q = "Un plan gagne 70 % du temps mais prend 0,3R sur ses gains et perd 1R sur ses pertes. Sur 100 trades ?"
options = ["−9R : il perd de l’argent malgré son taux de réussite", "+70R", "+21R", "L’équilibre"]
answer = 0
why = "70 × 0,3R = 21R gagnés ; 30 × 1R = 30R perdus ; bilan −9R."

[[quiz]]
id = "q5"
q = "Dix pertes d’affilée à 1 % chacune laissent environ…"
options = ["90 % du compte", "50 %", "99 %", "0 %"]
answer = 0
why = "0,99¹⁰ ≈ 0,904, environ 90 %."

[[quiz]]
id = "q6"
q = "Tes 8 premiers trades d’un plan positif : 6 pertes. Qu’est-ce que ça t’apprend ?"
options = ["Pas grand-chose encore : l’espérance se voit sur des dizaines de trades", "Le plan est cassé, change-le aujourd’hui", "Double le risque pour te refaire", "Arrête d’utiliser des stops"]
answer = 0
why = "Les courtes séries sont normales ; juge un plan sur un grand nombre de trades, si tu l’as suivi."

[[recall]]
id = "r1"
q = "Le taux de réussite d’équilibre à 1 pour 1, c’est…"
options = ["50 %", "33 %", "25 %"]
answer = 0
why = "1 ÷ (1 + 1)."

[[recall]]
id = "r2"
q = "L’espérance, c’est…"
options = ["Le résultat moyen par trade, en R", "Ton taux de réussite", "Ton plus gros gain"]
answer = 0
why = "Taux de réussite × gain moyen − taux de pertes × perte moyenne."

[[recall]]
id = "r3"
q = "Un petit risque fixe par trade compte parce que…"
options = ["Il survit aux séries de pertes et laisse à l’avantage le temps de jouer", "Il garantit les gains"]
answer = 0
why = "Dix pertes à 1 % laissent environ 90 %."

[explain_back]
prompt = "Pourquoi une stratégie qui se trompe la plupart du temps peut-elle quand même gagner de l’argent, et qu’est-ce qui te permet de durer assez pour le savoir ?"
must_include = ["le rapport gain/risque fixe un taux de réussite d’équilibre égal à 1 divisé par 1 plus le rapport, environ 33 % à 2 pour 1, donc gagner moins d’une fois sur deux peut rester rentable", "l’espérance, taux de réussite fois gain moyen moins taux de pertes fois perte moyenne en R, dit si un plan gagne de l’argent, et elle ne se voit que sur beaucoup de trades", "risquer une petite somme fixe comme 1 % fait qu’une série de dix pertes laisse environ 90 % du compte"]
misconceptions = ["un taux de réussite élevé veut dire une stratégie rentable", "quelques pertes d’affilée prouvent qu’un plan ne marche pas"]

[[explain_back.fixtures]]
answer = "Parce que ce qui compte, c’est le nombre de gains fois leur taille face aux pertes fois la leur. À 2 pour 1, un gain paie deux pertes, donc le taux de réussite d’équilibre est d’environ 33 %, soit 1 ÷ (1 + 2). L’espérance réunit tout : taux de réussite fois gain moyen moins taux de pertes fois perte moyenne, en R ; 40 % à 2R donne +0,2R par trade, mais ça ne se voit que sur beaucoup de trades. Risquer seulement 1 % par trade me fait tenir pendant les séries : dix pertes d’affilée laissent encore environ 90 %."
expect = "got_it"

[[explain_back.fixtures]]
answer = "À 2 pour 1, un gain paie deux pertes, donc il me suffit de gagner environ un trade sur trois pour être à l’équilibre, et l’espérance en R montre si le plan gagne de l’argent dans la durée."
expect = "almost"

[[explain_back.fixtures]]
answer = "Une bonne stratégie gagne la plupart de ses trades. Si je perds trois fois d’affilée, elle est cassée et je dois en changer."
expect = "almost_or_not_yet"
+++
