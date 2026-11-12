+++
id = 5
stage = 1
title = "Acheter ou vendre"
objective = "expliquer les positions longues et courtes, et quand chacune gagne ou perd de l’argent."
hook = "Peut-on gagner de l’argent quand le prix de l’or baisse ?"
analogy = "Prendre une position longue, c’est comme acheter un parapluie avant la saison des pluies : tu t’attends à ce qu’il vaille plus plus tard. Prendre une position courte, c’est comme emprunter le parapluie d’un ami, le vendre 20 $ aujourd’hui, puis en racheter un identique 12 $ quand la pluie s’arrête pour le lui rendre. Tu gardes les 8 $ d’écart, mais si les parapluies avaient augmenté, tu aurais dû payer plus cher pour le remplacer."
analogy_tag = "parapluies"
example = "Position longue : tu achètes à 4 150,00 et tu clôtures à 4 160,00, un mouvement de 10 $ en ta faveur. Position courte : tu vends à 4 150,00 et tu clôtures en rachetant à 4 140,00, là aussi un mouvement de 10 $ en ta faveur. Si la position courte avait été clôturée à 4 160,00, ce même mouvement de 10 $ aurait été une perte. (Prix donnés à titre d’exemple.)"
mistake = "Croire qu’il faut posséder de l’or avant de pouvoir le vendre, ou que « vendre » veut seulement dire clôturer un achat. Avec un courtier, tu peux ouvrir un trade en vendant et le clôturer plus tard en achetant."
takeaways = [
  "Position longue = acheter d’abord, vendre ensuite ; tu gagnes si le prix monte.",
  "Position courte = vendre d’abord, racheter ensuite ; tu gagnes si le prix baisse.",
  "Chacune perd de l’argent si le prix va dans l’autre sens.",
  "Avec un courtier, tu peux ouvrir un trade dans les deux sens.",
]
drills = ["candle_direction"]

[[videos]]
id = "fXnCtGcvqdk"
title = "Long Trade vs  Short Trade (Explained In Less Than 4 Minutes)"
channel = "Whyze"
duration = "3:33"
watch_for = "Comment un même mouvement de prix est un gain pour un sens et une perte pour l’autre."
fallback_search = "long vs short trade explained"
checked = "2026-11-10"

[[sources]]
claim = "Dans MetaTrader 5, un achat s’exécute au prix Ask et une vente au prix Bid, et les trades peuvent s’ouvrir dans les deux sens."
name = "Aide MetaTrader 5 : Types of Orders"
url = "https://www.metatrader5.com/en/mobile-trading/iphone/help/trade/general_concept/order_types"
checked = "2026-11-10"

[[sources]]
claim = "Les produits comme les CFD permettent de spéculer à la fois sur la hausse et sur la baisse du prix d’un actif."
name = "FCA : Understanding high-risk investments"
url = "https://www.fca.org.uk/investsmart/understanding-high-risk-investments"
checked = "2026-11-10"

[[quiz]]
id = "q1"
q = "Tu penses que l’or va monter. Quel trade correspond à cette idée ?"
options = ["Une position longue (acheter)", "Une position courte (vendre)", "L’un ou l’autre, ça ne change rien", "Aucun : on ne gagne que sur les baisses"]
answer = 0
why = "Une position longue gagne quand le prix monte."

[[quiz]]
id = "q2"
q = "Tu ouvres une position courte à 4 150,00 et tu clôtures à 4 138,00. Quel est le résultat ?"
options = ["Un gain de 12 $", "Une perte de 12 $", "Aucun changement", "Un gain de 1,20 $"]
answer = 0
why = "Tu as vendu à 4 150 et racheté à 4 138, 12 $ plus bas. Le prix a baissé, donc la position courte a gagné."

[[quiz]]
id = "q3"
q = "Tu ouvres une position longue à 4 150,00 et tu clôtures à 4 143,00. Quel est le résultat ?"
options = ["Un gain de 7 $", "Une perte de 7 $", "Aucun changement", "Une perte de 70 $"]
answer = 1
why = "Tu as acheté à 4 150 et vendu à 4 143 : le prix a baissé de 7 $ contre ta position longue."

[[quiz]]
id = "q4"
q = "Faut-il posséder de l’or avant d’ouvrir une position courte chez un courtier ?"
options = ["Oui, toujours", "Non, tu peux ouvrir un trade en vendant", "Seulement le week-end", "Seulement si tu as un lingot"]
answer = 1
why = "Avec un courtier, tu trades le prix, donc tu peux ouvrir une position en vendant."

[[quiz]]
id = "q5"
q = "Le prix de ta position courte monte. Que se passe-t-il pour ton trade ?"
options = ["Il gagne de l’argent", "Il perd de l’argent", "Rien tant que tu ne clôtures pas", "Il devient automatiquement une position longue"]
answer = 1
why = "Une position courte perd quand le prix monte, parce que tu devras racheter plus cher que tu n’as vendu."

[explain_back]
prompt = "Comment peut-on gagner de l’argent quand le prix de l’or baisse ?"
must_include = ["on vend d’abord (position courte) et on rachète plus tard", "on gagne si le prix baisse", "on perd si le prix monte au contraire"]
misconceptions = ["il faut posséder de l’or avant de pouvoir le vendre", "vendre veut seulement dire clôturer un achat"]

[[explain_back.fixtures]]
answer = "On prend une position courte : on vend d’abord au prix actuel et on rachète plus tard. Si l’or baisse, on rachète moins cher et on garde la différence, mais s’il monte, on doit racheter plus cher et on perd."
expect = "got_it"

[[explain_back.fixtures]]
answer = "On vend d’abord, puis on rachète plus tard à un prix plus bas, et on garde la différence."
expect = "almost"

[[explain_back.fixtures]]
answer = "On ne peut pas vraiment. Il faut d’abord acheter de l’or et le posséder, et vendre veut juste dire sortir de cet achat."
expect = "almost_or_not_yet"

[[steps]]
heading = "La position longue"
text = "Prendre une **position longue** (« long »), c’est **acheter d’abord** et **vendre ensuite**. Tu gagnes si le prix monte entre les deux et tu perds s’il baisse. Achat à 4 150,00, vente à 4 160,00 : un mouvement de **+10 $** en ta faveur."
[[steps]]
heading = "La position courte"
text = "Prendre une **position courte** (« short »), c’est **vendre d’abord** et **racheter ensuite**. Tu gagnes si le prix baisse et tu perds s’il monte. Vente à 4 150,00, rachat à 4 140,00 : un mouvement de **+10 $** en ta faveur."
why = "Vendre quelque chose qu’on ne possède pas paraît bizarre. Avec un courtier, tu trades le prix, tu ne livres pas de métal : ouvrir en vendant, c’est parier que le prix va baisser, et clôturer une position courte rachète automatiquement."
[[steps]]
heading = "Même mouvement, résultats opposés"
text = "Un même mouvement de prix est un gain pour un camp et une perte pour l’autre. Si l’or monte de 10 $, toutes les positions longues gagnent et toutes les courtes perdent. Ton travail : choisir un camp pour une bonne raison, avec un plan pour le cas où tu te trompes."
why = "Acheter, long et haussier veulent tous dire qu’on attend une hausse. Vendre, short et baissier veulent tous dire qu’on attend une baisse. Clôturer un trade, c’est faire l’inverse de ce qui l’a ouvert."
[[steps]]
heading = "La perte d’un short n’a pas de plafond"
text = "Une position longue peut perdre au maximum la valeur de l’or, car le prix ne peut pas passer sous zéro. Le prix, lui, n’a **pas de plafond** : la perte d’une position courte continue de grossir quand le prix monte, jusqu’à ce qu’un stop-loss ou ton courtier clôture le trade. C’est pour ça que chaque position courte a besoin d’un stop."
why = "Au Royaume-Uni et dans l’UE, les comptes CFD de particuliers bénéficient d’une protection contre le solde négatif : tu ne peux pas perdre plus que l’argent de ton compte. Mais tu peux quand même tout perdre, et vite."
widget = "short_risk"
[[recall]]
id = "r1"
q = "Une position courte gagne de l’argent quand le prix…"
options = ["Baisse", "Monte", "Ne bouge pas"]
answer = 0
why = "Tu as vendu d’abord, donc tu veux racheter moins cher."
[[recall]]
id = "r2"
q = "Tu ouvres une position courte à 4 150 et tu clôtures à 4 158. Par once, c’est…"
options = ["8 $ perdus", "8 $ gagnés", "0 $"]
answer = 0
why = "Le prix a monté de 8 $ contre ta position courte."
[[recall]]
id = "r3"
q = "Quelle position peut perdre sans plafond si tu ne l’arrêtes pas ?"
options = ["Une position courte", "Une position longue", "Aucune"]
answer = 0
why = "Le prix peut monter sans limite, donc la perte d’une position courte peut continuer de grossir."
+++
