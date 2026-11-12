+++
id = 11
stage = 1
title = "Les vrais coûts : spread, commission, slippage, swap"
objective = "citer les quatre coûts d’un trade et les estimer avant de trader."
hook = "Ton trade a bougé de 5 $ en ta faveur. Pourquoi ton compte a-t-il moins grossi que prévu ?"
analogy = "Une course en taxi comporte plus d’un frais : la prise en charge quand tu montes (le spread), des frais de réservation (la commission), le compteur qui tourne dans un embouteillage imprévu (le slippage) et le parking de nuit si tu gardes la voiture (le swap). Le tarif affiché n’est jamais toute la note."
analogy_tag = "course en taxi"
example = "Tu achètes 0,10 lot. Le spread est de 0,30 $, donc entrer coûte 0,10 × 100 × 0,30 $ = 3 $. Pendant un pic d’actualité, ton stop-loss s’exécute 1 $ plus mal que prévu : encore 0,10 × 100 × 1 $ = 10 $. Tu gardes la position trois nuits, et le swap est prélevé chaque nuit au taux publié par ton courtier. Rien de tout ça n’apparaît dans le mouvement du prix, mais tout sort de ton compte. (Chiffres donnés à titre d’exemple.)"
mistake = "Croire que les coûts ne comptent que pour les gros comptes, que le swap est un frais unique, ou que les stops s’exécutent toujours exactement à leur prix. Les coûts grossissent avec ta taille, le swap revient chaque nuit, et les marchés rapides causent du slippage."
takeaways = [
  "Spread : payé à chaque ouverture de trade.",
  "Commission : des frais par lot que certains courtiers facturent.",
  "Slippage : une exécution moins bonne que prévu quand le prix bouge vite.",
  "Swap : prélevé (ou parfois versé) pour chaque nuit où un trade reste ouvert.",
]

[[videos]]
id = "Z4z3aTgfWLg"
title = "Lesson 6.1:  What is swap in forex trading?"
channel = "Financial Wars"
duration = "5:37"
watch_for = "Que le swap est prélevé pour chaque nuit où une position reste ouverte. La vidéo prend des paires de devises ; vérifie le taux de swap de l’or chez ton courtier."
fallback_search = "what is swap in forex trading"
checked = "2026-11-10"

[[videos]]
id = "DiL2LF57znE"
title = "Trading guide - What is a Slippage?"
channel = "IG Singapore"
duration = "1:39"
watch_for = "Pourquoi le prix obtenu peut différer du prix demandé quand le marché bouge vite."
fallback_search = "what is slippage in trading"
checked = "2026-11-10"

[[sources]]
claim = "MetaTrader 5 affiche le swap des positions ouvertes, et le volume minimal d’un trade est fixé par le courtier."
name = "Aide MetaTrader 5 : Trade"
url = "https://www.metatrader5.com/en/mobile-trading/iphone/help/trade"
checked = "2026-11-10"

[[sources]]
claim = "En exécution au marché, le courtier fixe le prix d’exécution, qui peut donc différer du prix demandé."
name = "Aide MetaTrader 5 : Types of Execution"
url = "https://www.metatrader5.com/en/mobile-trading/iphone/help/trade/general_concept/execution_types"
checked = "2026-11-10"

[[quiz]]
id = "q1"
q = "Tu achètes 0,10 lot avec un spread de 0,30 $. Combien te coûte le spread ?"
options = ["0,30 $", "3 $", "30 $", "Rien avant la clôture"]
answer = 1
why = "0,10 × 100 × 0,30 $ = 3 $, payés à l’ouverture."

[[quiz]]
id = "q2"
q = "Ton stop-loss s’exécute 2 $ plus mal que son niveau, sur 0,20 lot. Combien le slippage t’a-t-il coûté en plus ?"
options = ["2 $", "4 $", "40 $", "400 $"]
answer = 2
why = "0,20 × 100 × 2 $ = 40 $."

[[quiz]]
id = "q3"
q = "À quelle fréquence le swap est-il prélevé ?"
options = ["Une fois, à l’ouverture", "Pour chaque nuit où le trade reste ouvert", "Seulement sur les trades gagnants", "Jamais sur l’or"]
answer = 1
why = "Le swap est un frais de nuit : garde la position trois nuits, il est appliqué trois fois."

[[quiz]]
id = "q4"
q = "Quel coût vient du fait que le prix est passé vite au-delà de ton ordre ?"
options = ["Le spread", "La commission", "Le slippage", "Le swap"]
answer = 2
why = "Le slippage est l’écart entre le prix attendu et le prix obtenu."

[[quiz]]
id = "q5"
q = "Quelle affirmation est vraie ?"
options = ["Les coûts ne comptent que pour les gros comptes", "Des trades plus gros veulent dire des coûts plus gros en argent", "La commission, c’est la même chose que le swap", "Le spread est remboursé si tu perds"]
answer = 1
why = "Chaque coût est multiplié par ta taille, comme les gains et les pertes."

[explain_back]
prompt = "Ton trade a bougé de 5 $ dans ton sens, mais ton compte a moins grossi que prévu. Qu’est-ce qui peut l’expliquer ?"
must_include = ["le spread payé à l’entrée", "la commission par lot, selon le courtier", "le slippage quand le prix saute au-delà de ton ordre", "le swap prélevé pour chaque nuit où le trade est gardé"]
misconceptions = ["les coûts ne comptent que pour les gros comptes", "le swap est un frais unique", "les stops s’exécutent toujours exactement à leur prix"]

[[explain_back.fixtures]]
answer = "Les coûts en ont mangé une partie : le spread payé à l’ouverture, une éventuelle commission par lot, du slippage si le prix a sauté au-delà de mon ordre, et le swap si je l’ai gardé la nuit, qui est prélevé chaque nuit."
expect = "got_it"

[[explain_back.fixtures]]
answer = "Le spread et la commission du courtier prennent une partie du gain."
expect = "almost"

[[explain_back.fixtures]]
answer = "Sûrement un frais de swap unique que prend le courtier, mais ce genre de coût ne compte vraiment que pour les gros comptes."
expect = "almost_or_not_yet"

[[steps]]
heading = "1. Le spread"
text = "Tu paies le spread à chaque ouverture de trade, et comme le reste, il grossit avec la taille : **coût du spread = lots × 100 × spread**. Un spread de 0,30 $ sur 0,10 lot, c’est 3 $ ; sur 1 lot, 30 $, à chaque trade."
[[steps]]
heading = "2. La commission"
text = "Certains courtiers facturent une **commission** par lot, souvent en échange d’un spread plus étroit. D’autres n’en prennent pas et se rémunèrent sur le spread. Aucun n’est forcément moins cher : additionne les deux pour comparer les courtiers."
[[steps]]
heading = "3. Le slippage"
text = "Le **slippage** (glissement) est l’écart entre le prix attendu et le prix obtenu. Il arrive quand le prix bouge plus vite que les ordres ne s’exécutent : autour des grosses annonces, après un week-end, lors de pics soudains. Prévois qu’il te coûte quelque chose."
why = "Le slippage peut parfois jouer en ta faveur, mais quand tu gardes un trade pendant une annonce, attends-toi à ce qu’il te coûte."
[[steps]]
heading = "4. Le swap"
text = "Garde un trade après la fin de la journée de trading, et ton courtier applique un **swap** : un petit frais, ou parfois un crédit, pour **chaque nuit** où il reste ouvert. MetaTrader 5 affiche le swap de chaque position ouverte."
why = "Certains courtiers appliquent un swap plus gros un jour précis de la semaine pour couvrir le week-end : vérifie le calendrier. Pour des trades de quelques minutes ou heures, le swap compte à peine ; sur des jours ou des semaines, il peut peser vraiment sur le résultat."
[[steps]]
heading = "Fais le total avant de trader"
text = "Avant un trade, additionne ce que coûtent l’entrée, la détention et la sortie : spread × taille, plus une éventuelle commission, plus le swap si tu gardes la nuit, plus une marge pour le slippage si une annonce est prévue. Si ça mange une grosse part de ton objectif, passe ton tour."
why = "Les coûts ne rendent pas le trading impossible. Ils veulent dire que les petits trades fréquents demandent de très bonnes raisons, et que ton journal doit noter le vrai résultat, après coûts."
[[recall]]
id = "r1"
q = "0,20 lot avec un spread de 0,30 $ te coûte…"
options = ["6 $", "0,60 $", "60 $"]
answer = 0
why = "0,20 × 100 × 0,30 $ = 6 $."
[[recall]]
id = "r2"
q = "Le swap est prélevé…"
options = ["Pour chaque nuit où un trade reste ouvert", "Une fois, à l’ouverture"]
answer = 0
why = "Il revient chaque nuit où tu gardes la position."
[[recall]]
id = "r3"
q = "Le slippage est le plus probable…"
options = ["Autour des grosses annonces et des pics soudains", "Les jours calmes et lents"]
answer = 0
why = "Il arrive quand le prix bouge plus vite que les ordres ne s’exécutent."
+++
