+++
id = 9
stage = 1
title = "Effet de levier et marge"
objective = "expliquer la marge comme un dépôt, et pourquoi l’effet de levier grossit les pertes autant que les gains."
hook = "Comment un petit compte peut-il contrôler des centaines de milliers de dollars d’or, et pourquoi est-ce dangereux ?"
analogy = "C’est comme acheter une maison avec un crédit. Un petit apport te permet de contrôler un gros bien. Si la maison prend 10 %, ton apport grossit beaucoup. Mais si elle perd 10 %, cette perte sort d’abord de ton apport, et une baisse assez forte peut l’effacer complètement."
analogy_tag = "apport d’un crédit immobilier"
example = "1 lot d’or à 4 150,00, c’est 100 onces, soit environ 415 000 $. Avec un effet de levier de 100:1, ton courtier peut demander un dépôt (la marge) d’environ 4 150 $. Un mouvement de 41,50 $ contre toi ne représente que 1 % de la valeur de la position, mais c’est 4 150 $ de perte : 100 % de ce dépôt. (Prix et levier donnés à titre d’exemple.)"
mistake = "Croire que l’effet de levier est de l’argent gratuit, ou que plus de levier veut dire plus de gains. Le levier grossit chaque mouvement dans les deux sens. C’est la taille de ta position, pas le chiffre du levier, qui décide combien tu peux perdre."
takeaways = [
  "La marge est un dépôt que ton courtier bloque pendant qu’un trade est ouvert, pas le coût du trade.",
  "L’effet de levier permet à un petit dépôt de contrôler une grosse position.",
  "Il grossit les pertes exactement autant que les gains, et peut vider un compte très vite.",
  "C’est la taille de la position, pas le levier, qui fixe ton vrai risque.",
]

[[videos]]
id = "b8j6s0Z8s18"
title = "Forex Leverage: 90% Of Beginners Make This Mistake When Trading With Margin..."
channel = "The Trading Channel (The Trading Channel)"
duration = "15:16"
watch_for = "Pourquoi le chiffre du levier n’est pas ton risque, alors que la taille de position l’est. La vidéo prend des exemples de devises ; la logique est la même pour l’or."
fallback_search = "leverage and margin explained beginners mistake"
checked = "2026-11-10"

[[sources]]
claim = "Au Royaume-Uni, les règles limitent l’effet de levier des CFD vendus aux particuliers entre 30:1 et 2:1 selon le marché."
name = "FCA : FCA confirms permanent restrictions on the sale of CFDs to retail consumers"
url = "https://www.fca.org.uk/news/press-releases/fca-confirms-permanent-restrictions-sale-cfds-and-cfd-options-retail-consumers"
checked = "2026-11-10"

[[sources]]
claim = "Les produits à haut risque comme les CFD comportent une forte probabilité de perdre tout son argent."
name = "FCA : Understanding high-risk investments"
url = "https://www.fca.org.uk/investsmart/understanding-high-risk-investments"
checked = "2026-11-10"

[[quiz]]
id = "q1"
q = "Qu’est-ce que la marge ?"
options = ["Les frais d’ouverture d’un trade", "Un dépôt bloqué pendant que ton trade est ouvert", "Ton gain sur un trade", "De l’argent offert par le courtier"]
answer = 1
why = "La marge est un dépôt. Tu la récupères à la clôture, moins une éventuelle perte."

[[quiz]]
id = "q2"
q = "Avec un levier de 100:1, quelle marge demande environ une position de 415 000 $ ?"
options = ["415 $", "4 150 $", "41 500 $", "415 000 $"]
answer = 1
why = "415 000 $ ÷ 100 = 4 150 $."

[[quiz]]
id = "q3"
q = "Tu as 1 lot d’or et le prix bouge de 41,50 $ contre toi. Combien perds-tu ?"
options = ["41,50 $", "415 $", "4 150 $", "Rien, grâce au levier"]
answer = 2
why = "1 × 100 × 41,50 $ = 4 150 $. Le levier ne change pas la formule de l’argent ; il permet seulement d’ouvrir le trade avec un dépôt plus petit."

[[quiz]]
id = "q4"
q = "Que fait l’effet de levier aux pertes ?"
options = ["Il les réduit", "Il les grossit autant que les gains", "Il les supprime", "Il ne touche que les gains"]
answer = 1
why = "Le levier grossit les deux sens de la même façon."

[[quiz]]
id = "q5"
q = "Qu’est-ce qui fixe vraiment combien tu peux perdre sur un trade ?"
options = ["Le chiffre du levier", "La taille de ta position et la distance jusqu’à ton stop", "Le logo du courtier", "L’heure de la journée"]
answer = 1
why = "Lots × 100 × Mouvement en dollars jusqu’au stop : la taille et la distance du stop décident de la perte."

[explain_back]
prompt = "Pourquoi l’effet de levier est-il dangereux pour un débutant, même s’il « permet de trader plus gros » ?"
must_include = ["la marge est un dépôt qui permet de contrôler une position bien plus grosse (ce n’est pas le coût total du trade)", "le levier grossit les pertes autant que les gains", "il peut vider un compte très vite", "c’est la taille de position, pas le levier, qui fixe le risque"]
misconceptions = ["l’effet de levier est de l’argent gratuit", "plus de levier veut dire plus de gains"]

[[explain_back.fixtures]]
answer = "La marge n’est qu’un dépôt qui permet de contrôler une position bien plus grosse. Comme la position est grosse, un petit mouvement contre toi fait une grosse perte par rapport au dépôt, tout comme les gains sont plus gros, donc un compte peut être vidé très vite. Ce qui fixe vraiment le risque, c’est la taille de la position, pas le chiffre du levier."
expect = "got_it"

[[explain_back.fixtures]]
answer = "Parce que le levier rend les pertes aussi grosses que les gains, et qu’un petit mouvement contre toi peut vider ton compte."
expect = "almost"

[[explain_back.fixtures]]
answer = "Ce n’est pas vraiment dangereux. Plus de levier, c’est plus de gains, comme si le courtier te donnait de l’argent en plus pour trader."
expect = "almost_or_not_yet"

[[steps]]
heading = "La marge est un dépôt"
text = "Ton courtier ne demande pas la valeur totale d’un trade. Il bloque une **marge** : un dépôt mis de côté pendant que le trade est ouvert, libéré à la clôture, plus le gain ou moins la perte."
why = "Le rapport entre la valeur totale de la position et le dépôt, c’est l’**effet de levier**. 1 lot autour de 4 150, c’est environ 415 000 $ d’or. Avec un levier de 100:1, il faut environ 4 150 $ de marge ; avec 20:1, environ 20 750 $."
[[steps]]
heading = "Le levier grossit dans les deux sens"
text = "Le levier ne change pas argent = lots × 100 × Mouvement en dollars ; il permet seulement à un petit dépôt d’ouvrir un gros trade. À 100:1, 1 lot demande environ 4 150 $, et un mouvement de **41,50 $** contre toi coûte la totalité."
why = "Ce mouvement représente environ 1 % du prix de l’or, le genre de mouvement qu’il peut faire en une journée. Le même mouvement dans ton sens doublerait le dépôt : c’est pour ça que le levier attire, et pour ça qu’il vide autant de comptes."
widget = "leverage_toggle"
[[steps]]
heading = "Limites et clôtures forcées"
text = "Les régulateurs plafonnent le levier pour les particuliers ; au Royaume-Uni, de 30:1 à 2:1 selon le marché. Si les pertes deviennent trop grandes, ton courtier clôture tes trades automatiquement. Tu n’as pas le droit d’attendre que ça remonte."
[[steps]]
heading = "Le vrai contrôle : ta taille"
text = "Un levier élevé ne t’oblige pas à trader gros ; il le permet. Ce qui décide de ton risque, c’est **le nombre de lots** et **la distance de ton stop**. À 100:1, trader 0,01 lot risque bien moins que trader 1 lot à 20:1."
why = "Ne demande donc jamais « quel levier puis-je avoir ? ». Demande « quelle taille peut avoir ce trade pour que, si mon stop est touché, je ne perde que ce que j’avais prévu ? ». La leçon 33 en fait un calculateur."
[[recall]]
id = "r1"
q = "La marge, c’est…"
options = ["Un dépôt bloqué pendant qu’un trade est ouvert", "Les frais d’un trade", "Ton gain"]
answer = 0
why = "Tu la récupères à la clôture, plus ou moins le résultat."
[[recall]]
id = "r2"
q = "L’effet de levier grossit…"
options = ["Les gains et les pertes autant", "Seulement les gains", "Seulement les pertes"]
answer = 0
why = "Il agrandit chaque mouvement, dans les deux sens."
[[recall]]
id = "r3"
q = "Qu’est-ce qui fixe vraiment combien tu peux perdre ?"
options = ["Ta taille et la distance de ton stop", "Le chiffre du levier", "L’heure de la journée"]
answer = 0
why = "Lots × 100 × la distance jusqu’à ton stop."
+++
