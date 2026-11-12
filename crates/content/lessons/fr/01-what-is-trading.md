+++
id = 1
stage = 1
title = "Qu’est-ce que le trading ?"
objective = "expliquer ce dont un trader cherche vraiment à tirer profit."
hook = "Si tu achètes un lingot d’or et que tu le gardes dix ans dans un tiroir, est-ce que tu fais du trading ?"
analogy = "Pense à la revente de places de concert. Tu achètes une place 50 $ avant que le groupe annonce sa tournée d’adieu. La demande s’envole, et tu la revends 80 $. Tu n’as jamais mis les pieds au concert : tes 30 $ viennent du changement de prix. Si le concert avait été annulé, le prix aurait tout aussi bien pu baisser, et tu aurais revendu à perte."
analogy_tag = "places de concert"
example = "L’or est à 4 100,00 et tu achètes. Plus tard, il est à 4 120,00 et tu vends. L’or a bougé de 20 $ dans ton sens, et c’est de ce mouvement de 20 $ que vient ton gain. S’il était tombé à 4 080,00, le même mouvement de 20 $ aurait été une perte. (Prix donnés à titre d’exemple.)"
mistake = "Croire que trader, c’est posséder de l’or sur le long terme. Ça, c’est investir. Le trading porte sur le changement de prix sur une durée plus courte, et chaque trade peut perdre autant que gagner."
takeaways = [
  "Un trader gagne sur un changement de prix, pas en possédant la chose.",
  "Tu achètes et tu vends à des prix différents ; l’écart est ton gain ou ta perte.",
  "Pas besoin de posséder de l’or physique pour trader son prix.",
  "Le prix peut aller contre toi : chaque trade peut te faire perdre de l’argent réel.",
]

[[videos]]
id = "dmqoqVwFopE"
title = "The Difference Between Trading and Investing"
channel = "The Plain Bagel"
duration = "10:29"
watch_for = "Comment la durée pendant laquelle on garde quelque chose, et ce qu’on espère gagner, distinguent un trader d’un investisseur."
fallback_search = "difference between trading and investing explained"
checked = "2026-11-10"

[[sources]]
claim = "Les produits comme les CFD servent à spéculer sur la hausse et la baisse du prix d’un actif, avec une forte probabilité de perdre de l’argent."
name = "FCA : Understanding high-risk investments"
url = "https://www.fca.org.uk/investsmart/understanding-high-risk-investments"
checked = "2026-11-10"

[[quiz]]
id = "q1"
q = "De quoi un trader cherche-t-il à tirer profit ?"
options = ["Posséder de l’or pendant des années", "Un changement de prix", "Les intérêts versés par le courtier", "Stocker de l’or en sécurité"]
answer = 1
why = "Le trading gagne sur la différence entre le prix d’achat et le prix de vente."

[[quiz]]
id = "q2"
q = "Tu achètes de l’or à 4 100,00 et tu vends à 4 115,00. De combien le prix a-t-il bougé ?"
options = ["1,50 $", "15 $", "150 $", "1 500 $"]
answer = 1
why = "4 115 − 4 100 = 15 $. C’est la taille du mouvement, et il est allé dans ton sens."

[[quiz]]
id = "q3"
q = "Tu achètes à 4 140,00 et tu vends à 4 130,00. Que s’est-il passé ?"
options = ["Un gain de 10 $", "Une perte de 10 $", "Rien : le prix est revenu", "Un gain de 100 $"]
answer = 1
why = "Le prix a baissé de 10 $ après ton achat : le mouvement est allé contre toi."

[[quiz]]
id = "q4"
q = "Faut-il posséder un vrai lingot d’or pour trader XAUUSD en ligne ?"
options = ["Oui, le courtier te l’envoie", "Oui, il est gardé dans un coffre à ton nom", "Non, tu trades le prix", "Seulement pour les gros trades"]
answer = 2
why = "Le trading d’or en ligne règle la différence de prix en argent ; aucun lingot ne change de main."

[[quiz]]
id = "q5"
q = "Lequel de ces cas est du trading plutôt que de l’investissement ?"
options = ["Acheter des pièces d’or pour la retraite", "Acheter le matin et revendre l’après-midi après une hausse de 12 $", "Hériter de bijoux en or", "Garder de l’or dans un coffre pendant 20 ans"]
answer = 1
why = "Le trading vise des mouvements de prix plus courts ; l’investissement garde sur le long terme."

[explain_back]
prompt = "Avec tes propres mots : de quoi un trader cherche-t-il vraiment à tirer profit ?"
must_include = ["le gain vient du changement de prix", "on achète et on vend à des prix différents", "un trade peut perdre autant que gagner"]
misconceptions = ["le trading, c’est la même chose qu’investir sur le long terme", "il faut posséder de l’or physique pour le trader"]

[[explain_back.fixtures]]
answer = "Un trader cherche à gagner de l’argent grâce au changement de prix. On achète à un prix, on revend à un autre, et la différence est le gain, mais si le prix part dans le mauvais sens, on perd de l’argent."
expect = "got_it"

[[explain_back.fixtures]]
answer = "Il veut profiter du changement de prix, en achetant à un prix et en revendant plus cher."
expect = "almost"

[[explain_back.fixtures]]
answer = "Un trader achète des lingots d’or et les garde en sécurité pendant des années pour qu’ils valent plus cher à sa retraite."
expect = "almost_or_not_yet"

[[steps]]
heading = "Le trading, c’est le changement"
text = "Trader, c’est acheter à un prix et vendre à un autre, en visant une différence en ta faveur. Un trader ne se pose qu’une question : le prix sera-t-il plus haut ou plus bas plus tard ? **Ton gain ou ta perte, c’est le changement de prix** entre ton entrée et ta sortie."
widget = "price_drag"
[[steps]]
heading = "Pas besoin de posséder l’or"
text = "Quand tu trades l’or en ligne, aucun métal ne change de main. Tu ouvres une position à un prix, tu la fermes à un autre, et la différence est réglée en argent. Tu trades **le prix**, et c’est aussi pour ça que tu peux gagner quand il baisse (leçon 5)."
why = "C’est pour ça que tu peux trader l’or depuis ton téléphone, dans ta cuisine, sans jamais toucher un lingot."
[[steps]]
heading = "L’écart peut aller contre toi"
text = "Si tu achètes et que le prix baisse, l’écart est une perte : de l’argent réel qui quitte ton compte. Il n’existe aucune version du trading où l’on ne fait que gagner. Les bons traders perdent souvent ; ce qui les garde dans la partie, c’est que leurs pertes sont petites et prévues."
why = "C’est l’essentiel de ce cours : faire en sorte que, quand tu te trompes, ça coûte peu, et seulement ce que tu avais prévu."
[[steps]]
heading = "Trader n’est pas investir"
text = "Un **investisseur** achète et garde pendant des années, en attendant que la valeur augmente. Un **trader** entre et sort en quelques minutes, heures ou jours, pour profiter de mouvements plus courts. Deux métiers, deux savoir-faire. Ce cours t’apprend celui du trader."
why = "Chaque décision, plus tard, revient à une question : *où le prix va-t-il ensuite, à mon avis, et que m’arrive-t-il si je me trompe ?* Retiens que le trading porte sur le changement de prix, et que ce changement peut aller contre toi."
[[recall]]
id = "r1"
q = "Le gain d’un trader vient…"
options = ["Du changement de prix", "De la possession du métal", "Des intérêts du courtier"]
answer = 0
why = "Le trading gagne sur la différence entre ton prix d’achat et ton prix de vente."
[[recall]]
id = "r2"
q = "Tu achètes à 4 100 et tu vends à 4 090. Que s’est-il passé, par once ?"
options = ["Une perte de 10 $", "Un gain de 10 $", "Rien"]
answer = 0
why = "Le prix a baissé de 10 $ après ton achat."
[[recall]]
id = "r3"
q = "Garder de l’or pendant 20 ans, c’est…"
options = ["Investir", "Trader", "Ni l’un ni l’autre"]
answer = 0
why = "Garder longtemps, c’est investir ; le trading vise des mouvements plus courts."
+++
