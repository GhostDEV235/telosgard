+++
id = 35
stage = 7
title = "Perte maximale du jour et drawdown"
objective = "fixer une Perte maximale du jour de 2 à 3R et t’arrêter quand elle est atteinte, et expliquer pourquoi une perte est plus dure à rattraper qu’elle n’en a l’air."
hook = "Deux stops touchés avant midi. Le troisième setup a l’air parfait. Tu le prends ?"
analogy = "Pense à une soirée au casino avec une somme fixe en poche et la carte bancaire laissée à la maison. La limite ne prédit pas que tu vas perdre ; c’est une décision prise au calme, pour qu’une mauvaise heure ne devienne pas un mauvais mois."
analogy_tag = "la carte laissée à la maison"
example = "Ton plan risque 1 % par trade, avec une Perte maximale du jour de 2R. Mardi, le premier trade touche son stop (−1R), puis le deuxième (−2R au total). La limite est atteinte : tu fermes la plateforme, tu remplis le journal et tu t’arrêtes. Le lendemain repart de zéro. Sans limite, une série « pour me refaire » de cinq pertes de plus aurait coûté environ 7 % en une journée."
mistake = "Traiter la limite comme un conseil quand le setup suivant a l’air bon. Le problème, ce ne sont pas les setups après deux pertes ; c’est ton jugement après deux pertes. La limite existe justement pour le jour où tu as envie de la dépasser."
takeaways = [
  "La Perte maximale du jour est une Règle stricte de ton Plan de trading : une fois les pertes du jour arrivées à ce niveau, plus aucune entrée ce jour-là.",
  "Ce cours la fixe à 2R, et jamais plus de 3R.",
  "Une perte est plus dure à rattraper qu’elle n’en a l’air : −10 % demande +11 %, −20 % demande +25 %, −50 % demande +100 %.",
  "Un petit risque par trade et une limite par jour gardent les drawdowns peu profonds, là où se refaire reste réaliste.",
]

[[steps]]
heading = "Une limite décidée au calme"
text = "La **Perte maximale du jour** est le plus que tu peux perdre en une journée, comptée en R. Une fois les pertes clôturées du jour arrivées à ce niveau, toute nouvelle entrée **enfreint ton Plan de trading**. Tu la fixes avant la séance, dans la Checklist avant la séance, pas au milieu d’une mauvaise matinée."
why = "Dans l’appli, c’est une Règle stricte : le journal la vérifie par le calcul, donc une entrée après la limite est notée comme ne respectant pas le plan, quel que soit son résultat."

[[steps]]
heading = "2R, jamais plus de 3R"
text = "La limite de ce cours est **2R** : deux pertes complètes et ta journée est finie. Tu peux la monter à **3R** au maximum. À 1 % par trade, une mauvaise journée s’arrête ainsi à 2 ou 3 % du compte, un recul dont on se remet."
why = "CME Group rappelle qu’un trader actif peut enchaîner cinq pertes en une seule journée. À 1 %, une limite de 2R arrête cette journée à 2 % au lieu de 5 %."

[[steps]]
heading = "Le calcul du rattrapage"
text = "Une perte se mesure sur ce que tu avais ; le rattrapage, sur ce qu’il reste. Perds **10 %** et il te faut **+11 %** pour revenir. Perds **20 %** : **+25 %**. Perds **50 %** : **+100 %**, il faut doubler ce qu’il reste. Plus le trou est profond, plus la pente est raide."
why = "10 000 $ en baisse de 50 %, ça fait 5 000 $ ; revenir à 10 000 $ veut dire gagner 5 000 $ sur 5 000 $ : +100 %. Le tableau de CME Group continue : −70 % demande +233 % et −90 % demande +900 %."

[[steps]]
heading = "Le drawdown et ta Série de préparation"
text = "Le **drawdown**, c’est l’écart entre le compte et son dernier plus haut. Ton Plan de trading fixe aussi une **perte totale maximale** pour chaque Série de préparation en démo. La dépasser met fin à la série ; une nouvelle repart avec un solde neuf. Un petit risque et une limite par jour t’en tiennent loin."
why = "Le profit n’est pas exigé pour « Prêt pour le réel », mais rester sous la perte totale maximale l’est. L’habitude que tu prouves, c’est que tes pertes restent petites."

[[steps]]
heading = "Que faire à la limite"
text = "Quand tu l’atteins : **ferme la plateforme**, note tes trades et écris une ligne sur ce que tu as ressenti. Ne cherche pas « un dernier ». Demain repart de zéro, avec une checklist neuve. La leçon 36 parle de l’envie de te refaire."
why = "S’arrêter est une compétence qui s’entraîne, comme placer un stop. Chaque jour où tu respectes la limite est un jour de Respect du plan, même s’il a perdu de l’argent."

[[videos]]
id = "mPzxtXtwFzM"
title = "How To Overcome Trading Losses And Losing Streak"
channel = "Rayner Teo"
duration = "6:01"
watch_for = "Pourquoi les séries de pertes sont normales pour un plan qui fonctionne, et comment un petit risque permet de les traverser."
fallback_search = "how to handle a losing streak trading"
checked = "2026-11-16"

[[videos]]
id = "wjOcqjeO7S4"
title = "What Your Daily Loss Should Be DAY TRADING!"
channel = "Patrick Wieland"
duration = "8:58"
watch_for = "Fixer la limite du jour avant la séance et s’en aller quand elle est touchée. Ses montants en dollars sont les siens ; utilise ta limite en R."
fallback_search = "daily loss limit trading"
checked = "2026-11-16"

[[sources]]
claim = "Une série de cinq pertes d’affilée peut arriver à un trader actif en une seule journée ; après une perte, le gain nécessaire pour revenir est plus grand que la perte : 10 % demande 11,11 %, 20 % demande 25 %, 50 % demande 100 %, 70 % demande 233 % et 90 % demande 900 %."
name = "CME Group Education: Controlling Risk"
url = "https://www.cmegroup.com/education/courses/trade-and-risk-management/controlling-risk.html"
checked = "2026-11-16"

[[sources]]
claim = "Des limites strictes sur la taille de position aident à contenir les pertes les plus probables quand on débute ; quand on débute, mieux vaut risquer un modeste 1 à 3 % du compte par trade."
name = "CME Group Education: Proper Position Size"
url = "https://www.cmegroup.com/education/courses/trade-and-risk-management/proper-position-size.html"
checked = "2026-11-16"

[[quiz]]
id = "q1"
q = "Ta Perte maximale du jour est de 2R et tu as perdu 2R à 11 h. Un super setup apparaît. Que fais-tu ?"
options = ["J’arrête pour la journée : la limite est atteinte", "Je le prends avec une demi-taille", "Je le prends avec une taille doublée pour me refaire", "Je le prends s’il a l’air vraiment bon"]
answer = 0
why = "Toute entrée après la limite enfreint le plan, aussi bon que le setup paraisse."

[[quiz]]
id = "q2"
q = "Un compte baisse de 50 %. Quel gain lui faut-il pour revenir à son point de départ ?"
options = ["+100 %", "+50 %", "+75 %", "+150 %"]
answer = 0
why = "La moitié restante doit doubler : 5 000 $ → 10 000 $."

[[quiz]]
id = "q3"
q = "Un compte baisse de 20 %. Quel gain le fait revenir ?"
options = ["+25 %", "+20 %", "+40 %", "+10 %"]
answer = 0
why = "8 000 $ ont besoin de 2 000 $ de plus : 2 000 ÷ 8 000 = 25 %."

[[quiz]]
id = "q4"
q = "À 1 % par trade, à combien une Perte maximale du jour de 2R limite-t-elle une mauvaise journée ?"
options = ["Environ 2 % du compte", "Environ 20 %", "Environ 0,2 %", "À rien : c’est seulement un repère"]
answer = 0
why = "Deux pertes de 1R, d’environ 1 % chacune."

[[quiz]]
id = "q5"
q = "Quand fixer la limite de pertes du jour ?"
options = ["Avant la séance, dans la Checklist avant la séance", "Après la première perte", "En fin de journée", "Seulement les mauvais jours"]
answer = 0
why = "Une limite décidée au calme, c’est celle que tu tiendras."

[[quiz]]
id = "q6"
q = "Qu’est-ce que le drawdown ?"
options = ["L’écart entre le compte et son dernier plus haut", "Le spread", "Ton plus gros gain", "La distance jusqu’à ton objectif"]
answer = 0
why = "Il mesure la profondeur du trou, depuis le sommet."

[[recall]]
id = "r1"
q = "La Perte maximale du jour de ce cours, c’est…"
options = ["2R, jamais plus de 3R", "10R", "Il n’y en a pas"]
answer = 0
why = "Deux ou trois pertes et la journée est finie."

[[recall]]
id = "r2"
q = "Une perte de 50 % demande…"
options = ["+100 % pour revenir", "+50 % pour revenir"]
answer = 0
why = "Le rattrapage se mesure sur ce qu’il reste."

[[recall]]
id = "r3"
q = "Quand la limite est atteinte, tu…"
options = ["Fermes la plateforme, remplis le journal et t’arrêtes", "Cherches un dernier setup", "Remontes la limite"]
answer = 0
why = "Demain repart de zéro."

[explain_back]
prompt = "Qu’est-ce que la Perte maximale du jour, et pourquoi les pertes comptent-elles plus qu’elles n’en ont l’air ?"
must_include = ["la Perte maximale du jour est le plus que tu peux perdre en une journée, en R, fixée avant la séance à 2R et jamais plus de 3R, et une fois atteinte tu arrêtes de trader ce jour-là", "une perte est plus dure à rattraper qu’elle n’en a l’air parce que le rattrapage se mesure sur ce qu’il reste : 20 % demande 25 % et 50 % demande 100 %", "un petit risque par trade et une limite par jour gardent les drawdowns peu profonds, sous la perte totale maximale du plan"]
misconceptions = ["on peut continuer après la limite si le setup a l’air bon", "une perte de 50 % se rattrape avec un gain de 50 %"]

[[explain_back.fixtures]]
answer = "C’est le plus que j’ai le droit de perdre en une journée, compté en R et fixé dans ma checklist avant de commencer : 2R, 3R au grand maximum. Quand mes pertes y arrivent, je ferme la plateforme et j’arrête, même si le setup suivant a l’air bon. Les pertes font plus mal qu’elles n’en ont l’air parce qu’on se refait sur un solde plus petit : −20 % demande +25 %, et −50 % demande +100 %. Garder le risque à 1 % et une limite par jour garde le drawdown peu profond, bien sous la perte totale maximale de mon plan."
expect = "got_it"

[[explain_back.fixtures]]
answer = "C’est une limite de 2R par jour, fixée avant la séance, et quand je l’atteins j’arrête de trader pour la journée."
expect = "almost"

[[explain_back.fixtures]]
answer = "Si je perds 50 %, il me suffit de regagner 50 %, donc une mauvaise journée, ce n’est pas grave. Je continue jusqu’à me refaire."
expect = "almost_or_not_yet"
+++
