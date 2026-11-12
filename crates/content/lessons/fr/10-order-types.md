+++
id = 10
stage = 1
title = "Types d’ordres, stop-loss et take-profit"
objective = "choisir un ordre au marché, limite ou stop selon ton intention, avec un stop-loss et un take-profit attachés."
hook = "L’or te plaît, mais seulement à un prix plus bas. Dois-tu rester devant l’écran jusqu’à ce qu’il y arrive ?"
analogy = "Pense à un magasin. Payer à la caisse tout de suite, c’est un ordre au marché. Laisser une offre écrite, « je l’achète si le prix descend à 80 $ », c’est un ordre limite. Dire au magasin « vends-m’en un seulement s’il devient populaire et dépasse 120 $ », c’est un ordre stop. Chacun dit à quel moment tu acceptes d’agir."
analogy_tag = "caisse et offres écrites"
example = "L’or est à 4 100,00. Un achat au marché s’exécute tout de suite, près de l’ask. Un buy limit à 4 080,00 attend un repli et peut ne jamais s’exécuter. Un buy stop à 4 120,00 ne se déclenche que si le prix monte au-delà de 4 120. Attache un stop-loss à 4 090,00 et un take-profit à 4 130,00 à un achat à 4 100,00, et le trade se clôture tout seul à l’un des deux niveaux. (Prix donnés à titre d’exemple.)"
mistake = "Croire qu’un ordre limite s’exécute toujours, ou qu’un stop-loss garantit le prix de sortie exact. Une limite peut ne jamais être atteinte, et sur un marché rapide un stop peut s’exécuter à un prix pire que celui que tu as fixé."
takeaways = [
  "Ordre au marché : acheter ou vendre maintenant, au prix actuel.",
  "Ordre limite : acheter plus bas ou vendre plus haut que maintenant ; il peut ne jamais s’exécuter.",
  "Ordre stop : acheter au-dessus ou vendre en dessous du prix actuel, une fois le niveau atteint.",
  "Le stop-loss et le take-profit clôturent ton trade automatiquement.",
]
drills = ["order_type"]

[[videos]]
id = "yWTBndni56g"
title = "Market Order, Buy Limit, Sell Limit, Buy Stop, Sell Stop"
channel = "The Moving Average"
duration = "3:27"
watch_for = "Où se place chaque ordre en attente par rapport au prix actuel : les limites du côté du « meilleur prix », les stops de l’autre côté."
fallback_search = "buy limit sell limit buy stop sell stop explained"
checked = "2026-11-10"

[[videos]]
id = "Tiyystl8x40"
title = "Understanding Market, Limit, and Stop Orders"
channel = "Charles Schwab"
duration = "5:31"
watch_for = "Pourquoi un ordre au marché te fait entrer tout de suite, alors qu’un ordre limite échange la certitude d’être exécuté contre un meilleur prix. Les exemples portent sur des actions ; les types d’ordres fonctionnent pareil."
fallback_search = "market limit stop orders explained"
checked = "2026-11-10"

[[sources]]
claim = "MetaTrader 5 définit les ordres en attente Buy Limit, Sell Limit, Buy Stop et Sell Stop, ainsi que les ordres Stop Loss et Take Profit."
name = "Aide MetaTrader 5 : Types of Orders"
url = "https://www.metatrader5.com/en/mobile-trading/iphone/help/trade/general_concept/order_types"
checked = "2026-11-10"

[[sources]]
claim = "En exécution au marché, c’est le courtier qui fixe le prix d’exécution, qui peut donc différer du prix que tu as vu."
name = "Aide MetaTrader 5 : Types of Execution"
url = "https://www.metatrader5.com/en/mobile-trading/iphone/help/trade/general_concept/execution_types"
checked = "2026-11-10"

[[quiz]]
id = "q1"
q = "L’or est à 4 100. Tu veux acheter seulement s’il redescend à 4 080. Quel ordre ?"
options = ["Achat au marché", "Buy limit à 4 080", "Buy stop à 4 080", "Sell limit à 4 080"]
answer = 1
why = "Un buy limit attend sous le prix actuel pour une meilleure entrée."

[[quiz]]
id = "q2"
q = "L’or est à 4 100. Tu veux acheter seulement s’il franchit 4 120. Quel ordre ?"
options = ["Buy limit à 4 120", "Buy stop à 4 120", "Sell stop à 4 120", "Achat au marché maintenant"]
answer = 1
why = "Un buy stop se place au-dessus du prix actuel et se déclenche quand le prix y monte."

[[quiz]]
id = "q3"
q = "Quel ordre te fait entrer sur le marché immédiatement ?"
options = ["L’ordre au marché", "Le buy limit", "Le sell stop", "Le take-profit"]
answer = 0
why = "Un ordre au marché s’exécute tout de suite, au prix actuel."

[[quiz]]
id = "q4"
q = "Tu achètes à 4 100 avec un stop-loss à 4 090 et un take-profit à 4 130. Combien prévois-tu de perdre au maximum, par once ?"
options = ["1 $", "10 $", "30 $", "40 $"]
answer = 1
why = "4 100 − 4 090 = 10 $ par once, si le stop s’exécute à son prix."

[[quiz]]
id = "q5"
q = "Une grosse nouvelle tombe et le prix saute par-dessus ton stop-loss. Que peut-il se passer ?"
options = ["Il s’exécute toujours exactement à ton stop", "Il peut s’exécuter à un prix pire", "Le trade reste ouvert", "Le courtier te rembourse la différence"]
answer = 1
why = "Lors d’un mouvement rapide, un stop devient un ordre au marché et peut s’exécuter au prochain prix disponible."

[explain_back]
prompt = "Tu veux acheter seulement si l’or redescend à 4 080. Quel ordre utilises-tu, et pourquoi pas un ordre au marché ?"
must_include = ["utiliser un ordre buy limit à 4 080", "un ordre au marché s’exécute immédiatement au prix actuel", "un ordre limite attend et peut ne jamais s’exécuter", "un stop-loss et un take-profit clôturent le trade automatiquement"]
misconceptions = ["un ordre limite s’exécute toujours", "un stop-loss garantit le prix de sortie exact"]

[[explain_back.fixtures]]
answer = "Un buy limit à 4 080. Un ordre au marché achèterait tout de suite, au prix actuel plus élevé, alors qu’une limite attend le repli et peut ne jamais s’exécuter si l’or n’y arrive pas. J’y attache un stop-loss et un take-profit pour qu’il se clôture tout seul dans les deux cas."
expect = "got_it"

[[explain_back.fixtures]]
answer = "Un buy limit à 4 080, parce qu’un ordre au marché achèterait tout de suite au prix actuel au lieu d’attendre."
expect = "almost"

[[explain_back.fixtures]]
answer = "Un buy stop à 4 080, et il est garanti d’être exécuté exactement à 4 080, donc aucun risque d’un prix pire."
expect = "almost_or_not_yet"

[[steps]]
heading = "Entrer maintenant : l’ordre au marché"
text = "Un **ordre au marché** achète ou vend tout de suite, au prix actuel : l’achat près de l’ask, la vente près du bid. C’est simple et sûr d’être exécuté, mais tu prends le prix disponible, qui sur un marché rapide peut différer de celui que tu as vu."
[[steps]]
heading = "Les ordres limites : un meilleur prix"
text = "Un **buy limit** se place **sous** le prix actuel : « achète seulement si l’or devient moins cher, à ce niveau. » Un **sell limit** se place **au-dessus**. Les limites attendent un meilleur prix, et **peuvent ne jamais s’exécuter** si le prix n’y arrive pas."
[[steps]]
heading = "Les ordres stop : au franchissement d’un niveau"
text = "Un **buy stop** se place **au-dessus** du prix actuel : « achète seulement si l’or monte jusqu’ici. » Un **sell stop** se place **en dessous**. Les ordres stop ne se déclenchent qu’une fois que le prix a traversé ton niveau."
[[steps]]
heading = "Sortir : stop-loss et take-profit"
text = "Un **stop-loss** clôture le trade si le prix va contre toi jusqu’à ton niveau ; un **take-profit** le clôture à ton objectif. Fixe les deux à l’ouverture, et la plateforme s’occupe du reste, même pendant que tu dors."
why = "Achète à 4 100,00 avec un stop-loss à 4 090,00 et un take-profit à 4 130,00, et tu as décidé à l’avance : risquer 10 $ par once pour tenter d’en gagner 30."
widget = "stop_first"
[[steps]]
heading = "Un stop n’est pas une garantie"
text = "Quand le prix atteint ton stop, le trade se clôture au prochain prix disponible, en général à ton niveau ou tout près. Lors d’un saut, une grosse nouvelle par exemple, ça peut être pire : c’est le **slippage** (glissement). Malgré tout, chaque trade a un stop, fixé dès son ouverture."
why = "C’est la seule règle du cours sans aucune exception. Une sortie un peu moins bonne vaut bien mieux que pas de sortie du tout."
[[recall]]
id = "r1"
q = "L’or est à 4 100. Tu veux acheter seulement s’il redescend à 4 080. Utilise un…"
options = ["Buy limit", "Buy stop", "Ordre au marché"]
answer = 0
why = "Un buy limit attend sous le prix pour une meilleure entrée."
[[recall]]
id = "r2"
q = "Tu veux acheter seulement si l’or franchit 4 120 à la hausse. Utilise un…"
options = ["Buy stop", "Buy limit", "Sell stop"]
answer = 0
why = "Un buy stop se place au-dessus du prix et se déclenche à la montée."
[[recall]]
id = "r3"
q = "Un stop-loss garantit-il ton prix de sortie exact ?"
options = ["Non, il peut glisser lors d’un mouvement rapide", "Oui, toujours"]
answer = 0
why = "Il se clôture au prochain prix disponible."
+++
