+++
id = 4
stage = 1
title = "Lire « XAUUSD »"
objective = "lire le symbole et la cotation comme des dollars américains par once troy d’or."
hook = "Tu ouvres une appli de trading et tu vois « XAUUSD 4 150,00 ». Qu’est-ce que tu regardes, au juste ?"
analogy = "C’est une étiquette de prix dans un magasin. L’étiquette d’une baguette indique ce que coûte une baguette. XAUUSD est l’étiquette de prix d’une once troy d’or, en dollars américains. Quand l’étiquette change, le prix de cette once a changé."
analogy_tag = "étiquette de prix"
example = "XAUUSD 4 150,00 signifie qu’une once troy d’or coûte 4 150 $. Si la cotation passe à 4 155,00, l’or est devenu 5 $ plus cher en dollars. Si elle passe à 4 140,00, l’or est devenu 10 $ moins cher. (Prix donnés à titre d’exemple.)"
mistake = "Croire que le nombre est le prix d’un gramme, d’un kilo ou d’un lingot entier, ou que XAU est une entreprise. C’est le prix d’une once troy d’or, en dollars américains, et rien d’autre."
takeaways = [
  "XAU est le code de l’or ; USD, celui du dollar américain.",
  "XAUUSD est le prix d’une once troy d’or en dollars.",
  "Quand le nombre monte, l’or coûte plus de dollars ; quand il baisse, il en coûte moins.",
  "L’once troy est l’unité de poids standard des métaux précieux.",
]
drills = ["dollar_distance"]

[[videos]]
id = "K72sISknAH4"
title = "Troy oz VS Regular oz | What is the difference?"
channel = "Chards Coin and Bullion Dealer"
duration = "1:37"
watch_for = "Qu’une once troy est une unité à part pour les métaux précieux, pas l’once d’une balance de cuisine."
fallback_search = "troy ounce vs regular ounce"
checked = "2026-11-10"

[[sources]]
claim = "XAU est le code ISO 4217 de l’or (code numérique 959)."
name = "SIX (agence de maintenance ISO 4217) : liste des codes de devises"
url = "https://www.six-group.com/dam/download/financial-information/data-center/iso-currrency/lists/list-one.xml"
checked = "2026-11-10"

[[sources]]
claim = "Le prix de référence LBMA Gold Price est fixé en dollars américains par once troy."
name = "LBMA : About LBMA daily auction prices"
url = "https://www.lbma.org.uk/prices-and-data/about-lbma-daily-auction-prices"
checked = "2026-11-10"

[[quiz]]
id = "q1"
q = "Que signifie XAU ?"
options = ["Une société minière aurifère", "L’or", "Le dollar américain", "Une bourse à Londres"]
answer = 1
why = "XAU est le code officiel de l’or. « Au » vient d’aurum, « or » en latin."

[[quiz]]
id = "q2"
q = "XAUUSD = 4 150,00. Qu’est-ce que ça veut dire ?"
options = ["Un gramme d’or coûte 4 150 $", "Une once troy d’or coûte 4 150 $", "Un kilo d’or coûte 4 150 $", "Une action « or » coûte 4 150 $"]
answer = 1
why = "La cotation est le prix d’une once troy d’or en dollars américains."

[[quiz]]
id = "q3"
q = "XAUUSD passe de 4 150,00 à 4 162,00. Que s’est-il passé ?"
options = ["L’or a baissé de 12 $", "L’or a augmenté de 12 $", "Le dollar a augmenté de 12 $", "Rien : c’est un arrondi normal"]
answer = 1
why = "Un nombre plus élevé veut dire qu’une once d’or coûte maintenant plus de dollars : 4 162 − 4 150 = 12 $."

[[quiz]]
id = "q4"
q = "XAUUSD passe de 4 150,00 à 4 135,50. De combien a-t-il bougé, et dans quel sens ?"
options = ["14,50 $ à la baisse", "14,50 $ à la hausse", "1,45 $ à la baisse", "145 $ à la baisse"]
answer = 0
why = "4 150,00 − 4 135,50 = 14,50 $, et le nombre a baissé : l’or est devenu moins cher."

[[quiz]]
id = "q5"
q = "Dans quelle unité de poids XAUUSD est-il coté ?"
options = ["En kilogrammes", "En grammes", "En onces troy", "En onces ordinaires"]
answer = 2
why = "Les métaux précieux sont cotés en onces troy, une unité différente de l’once ordinaire."

[explain_back]
prompt = "Que signifie XAUUSD = 4 150, et qu’est-ce que ça veut dire quand le nombre monte ?"
must_include = ["XAU, c’est l’or et USD, le dollar américain", "le nombre est le prix d’une once troy en dollars", "une hausse veut dire que l’or coûte plus de dollars"]
misconceptions = ["le nombre est le prix d’un gramme ou d’un kilo", "XAU est une entreprise ou une action"]

[[explain_back.fixtures]]
answer = "XAU veut dire l’or et USD le dollar américain, donc 4 150 signifie qu’une once troy d’or coûte 4 150 $. Si ça monte à 4 155, l’or est devenu 5 $ plus cher en dollars."
expect = "got_it"

[[explain_back.fixtures]]
answer = "Ça veut dire que l’or coûte 4 150 dollars américains, et si le nombre monte, l’or est plus cher."
expect = "almost"

[[explain_back.fixtures]]
answer = "XAUUSD est le cours de l’action d’une grande société minière aurifère, et 4 150 est le prix d’une action."
expect = "almost_or_not_yet"

[[steps]]
heading = "Découper le symbole"
text = "**XAU** est le code officiel de l’or ; **USD**, celui du dollar américain. Ensemble, XAUUSD veut dire **l’or coté en dollars américains**. Tu le verras aussi écrit XAU/USD ; c’est la même chose."
why = "XAU vient de la norme internationale des codes de devises, celle qui donne USD au dollar et EUR à l’euro. « Au » est le symbole chimique de l’or, du latin *aurum*."
[[steps]]
heading = "Ce que veut dire le nombre"
text = "Le nombre est le prix d’**une once troy** d’or, en dollars. XAUUSD **4 150,00** signifie qu’une once troy coûte **4 150 $**. L’once troy est l’unité standard des métaux précieux, pas l’once d’une balance de cuisine."
why = "Le principal prix de référence de l’or, fixé deux fois par jour à Londres, utilise exactement la même unité : des dollars américains par once troy. Pas besoin de retenir son poids ; retiens juste que la cotation est toujours par once troy."
[[steps]]
heading = "Lire un changement"
text = "Si le nombre monte, l’or coûte plus de dollars ; s’il baisse, moins. La distance, c’est une simple soustraction : de 4 150,00 à 4 155,00, c’est un **mouvement de 5 $**. Ce cours mesure toujours la distance ainsi, en **Mouvement en dollars**."
widget = "price_drag"
[[steps]]
heading = "Deux décimales"
text = "Les cotations ont en général deux décimales, comme 4 150,37. Ce sont des cents, donc le plus petit mouvement que tu verras d’habitude est **0,01 $**. Ta plateforme peut écrire 4150.37, avec un point : c’est le même prix. La leçon 7 montre les autres unités utilisées, et pourquoi elles embrouillent les débutants."
why = "Entraîne-toi : 4 120,00 → 4 131,00, c’est +11 $. 4 131,00 → 4 118,50, c’est −12,50 $. 4 118,50 → 4 118,75, c’est +0,25 $. Une cotation n’est qu’un prix, et un mouvement n’est que la différence entre deux prix."
[[recall]]
id = "r1"
q = "XAU signifie…"
options = ["L’or", "L’argent (le métal)", "Le dollar américain"]
answer = 0
why = "XAU est le code officiel de l’or."
[[recall]]
id = "r2"
q = "XAUUSD 4 150 signifie…"
options = ["Une once troy coûte 4 150 $", "Un gramme coûte 4 150 $", "Un kilo coûte 4 150 $"]
answer = 0
why = "La cotation est par once troy, en dollars."
[[recall]]
id = "r3"
q = "L’or passe de 4 120,00 à 4 131,00. Le mouvement est de…"
options = ["11 $ à la hausse", "11 $ à la baisse", "1,10 $ à la hausse"]
answer = 0
why = "4 131 − 4 120 = 11 $, et le nombre a monté."
+++
