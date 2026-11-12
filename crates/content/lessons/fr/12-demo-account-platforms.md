+++
id = 12
stage = 1
title = "Ton compte démo et tes plateformes"
objective = "ouvrir un compte démo MT5, lire les caractéristiques du contrat XAUUSD, puis passer et clôturer un trade de 0,01 lot avec un stop-loss et un take-profit."
hook = "Comment s’entraîner à trader sans risquer un seul dollar ?"
analogy = "Un compte démo est un simulateur de vol. Les pilotes s’entraînent sur de vrais instruments, dans des conditions réalistes, où un crash ne coûte rien. Tu as la même chose : de vrais prix de l’or, une vraie plateforme et de l’argent fictif, pour que les erreurs deviennent des leçons au lieu de pertes."
analogy_tag = "simulateur de vol"
example = "Dans ta démo MT5, ouvre la fiche du symbole XAUUSD et note la taille du contrat, le volume minimal, le pas de volume, les taux de swap et les horaires de trading. Passe ensuite un achat de 0,01 lot avec un stop-loss 5 $ sous ton entrée et un take-profit 10 $ au-dessus, observe-le quelques minutes et clôture-le à la main. Avec 0,01 lot, chaque mouvement de 1 $ vaut 1 $."
mistake = "Croire que les résultats en démo garantissent les résultats en réel, ou que le contrat or est le même chez tous les courtiers. En démo, aucun argent réel n’est en jeu, donc les émotions sont différentes, et chaque courtier fixe ses propres caractéristiques de contrat."
takeaways = [
  "Un compte démo utilise de vrais prix avec de l’argent fictif.",
  "Vérifie la taille du contrat, le lot minimal, le swap et les horaires dans la fiche XAUUSD de ton courtier.",
  "Mets un stop-loss sur chaque trade, même en démo.",
  "N’utilise qu’un courtier que tu as vérifié sur le registre officiel de son régulateur.",
]

[[videos]]
id = "QnnDgyJrPl0"
title = "How to Place Orders on MetaTrader 5"
channel = "Pepperstone Australia"
duration = "6:34"
watch_for = "Où se trouve la fenêtre d’ordre et comment saisir le volume, le stop-loss et le take-profit. Ignore les offres propres au courtier ; les étapes sur la plateforme sont les mêmes partout."
fallback_search = "how to place orders metatrader 5"
checked = "2026-11-10"

[[videos]]
id = "CSF6bSFc24A"
title = "How to Set a Stop Loss and Take Profit on MetaTrader 5 (Android, iOS, Desktop, Web)"
channel = "Trading Heroes"
duration = "10:27"
watch_for = "Placer et modifier un stop-loss et un take-profit sur l’appareil que tu utiliseras vraiment."
fallback_search = "set stop loss take profit metatrader 5"
checked = "2026-11-10"

[[sources]]
claim = "MetaTrader 5 permet d’ouvrir un compte démo."
name = "MetaTrader 5 : Trading Platform"
url = "https://www.metatrader5.com/en/trading-platform"
checked = "2026-11-10"

[[sources]]
claim = "MetaTrader 5 prend en charge les ordres Stop Loss et Take Profit sur les positions."
name = "Aide MetaTrader 5 : Types of Orders"
url = "https://www.metatrader5.com/en/mobile-trading/iphone/help/trade/general_concept/order_types"
checked = "2026-11-10"

[[sources]]
claim = "NFA BASIC est un outil gratuit pour se renseigner sur les sociétés et professionnels américains des produits dérivés."
name = "NFA : BASIC"
url = "https://www.nfa.futures.org/basicnet/"
checked = "2026-11-10"

[[sources]]
claim = "Les registres de l’ASIC indiquent les organismes autorisés par l’ASIC à fournir des services financiers en Australie."
name = "ASIC : Search ASIC registers"
url = "https://asic.gov.au/online-services/search-asic-s-registers/"
checked = "2026-11-10"

[[quiz]]
id = "q1"
q = "Qu’est-ce qu’un compte démo ?"
options = ["Un compte réel avec un petit solde", "De vrais prix avec de l’argent fictif", "Un enregistrement de prix passés", "Une formation de trading payante"]
answer = 1
why = "La démo utilise les prix réels du marché mais de l’argent fictif : les erreurs ne coûtent rien."

[[quiz]]
id = "q2"
q = "Que dois-tu vérifier dans la fiche XAUUSD de ton courtier ?"
options = ["La couleur du logo", "La taille du contrat, le lot minimal, le swap et les horaires", "Combien d’autres personnes tradent", "Rien ; tous les courtiers sont pareils"]
answer = 1
why = "Chaque courtier fixe ses propres caractéristiques de contrat, et tous tes calculs en dépendent."

[[quiz]]
id = "q3"
q = "En démo, tu passes un achat de 0,01 lot avec un stop-loss 5 $ sous l’entrée. Si le stop est touché, combien perds-tu environ ?"
options = ["0,05 $", "0,50 $", "5 $", "50 $"]
answer = 2
why = "0,01 × 100 × 5 $ = 5 $ (fictifs, en démo)."

[[quiz]]
id = "q4"
q = "Comment vérifier qu’un courtier est bien réglementé ?"
options = ["Se fier au logo sur son site", "Chercher soi-même dans le registre officiel du régulateur", "Demander dans un groupe sur les réseaux sociaux", "Regarder combien d’abonnés il a"]
answer = 1
why = "Une licence peut être falsifiée. Cherche la société dans le registre du régulateur lui-même."

[[quiz]]
id = "q5"
q = "Tu gagnes en démo depuis deux semaines. Qu’est-ce que ça prouve pour le trading réel ?"
options = ["Tu gagneras forcément en réel", "Rien n’est garanti ; l’argent réel change ce que tu ressens et ce que tu fais", "Le trading réel est plus facile", "Tu peux te passer du stop-loss"]
answer = 1
why = "La démo construit des compétences et des habitudes, mais l’argent réel apporte des émotions que la démo ne peut pas reproduire."

[explain_back]
prompt = "Pourquoi s’entraîner d’abord en démo, et que dois-tu vérifier dans la fiche XAUUSD de ton courtier ?"
must_include = ["une démo utilise de vrais prix avec de l’argent fictif", "vérifier la taille du contrat, le lot minimal, le swap et les horaires", "toujours mettre un stop", "utiliser un courtier réglementé"]
misconceptions = ["les résultats en démo garantissent les résultats en réel", "le contrat or est le même chez tous les courtiers"]

[[explain_back.fixtures]]
answer = "Une démo a les vrais prix du marché mais de l’argent fictif, donc les erreurs sont gratuites pendant que j’apprends. Dans la fiche XAUUSD, je vérifie la taille du contrat, le lot minimal, les taux de swap et les horaires, je mets toujours un stop sur chaque ordre, et je m’assure que le courtier est réglementé."
expect = "got_it"

[[explain_back.fixtures]]
answer = "Parce que ce sont de vrais prix avec de l’argent fictif, donc je peux m’entraîner sans risque. Je vérifierais combien d’onces il y a dans un lot."
expect = "almost"

[[explain_back.fixtures]]
answer = "Pour prouver que je gagne, parce que si ça marche en démo, ça marchera forcément en réel. La fiche n’a pas d’importance puisque l’or est le même chez tous les courtiers."
expect = "almost_or_not_yet"

[[steps]]
heading = "Deux outils"
text = "Tu utiliseras **MetaTrader 5 (MT5)**, une plateforme proposée par beaucoup de courtiers, pour passer tes trades d’entraînement, et **TradingView**, un site de graphiques, pour étudier le prix de l’or. Tu te serviras vraiment de TradingView à partir de l’étape 4."
[[steps]]
heading = "Choisis ton courtier avec soin"
text = "Choisis un courtier **agréé par un régulateur financier** de ton pays, et vérifie-le toi-même dans le registre du régulateur : la FCA au Royaume-Uni, l’ASIC en Australie, NFA BASIC aux États-Unis. Si tu ne le trouves pas, passe ton chemin."
why = "Même pour une démo, choisis un courtier à qui tu pourrais un jour confier de l’argent réel. Un numéro de licence sur un site peut être faux ; le registre du régulateur, non. Si le nom ne correspond pas exactement, passe ton chemin. En France, c’est le registre REGAFI et la liste noire de l’AMF qui servent à vérifier."
[[steps]]
heading = "Ouvre une démo et lis la fiche"
text = "Ouvre un **compte démo** MT5 : prix réels, argent fictif, avec un solde proche de ce que tu traderais vraiment. Dans la fiche du symbole XAUUSD, note la **taille du contrat**, le **lot minimal et son pas**, le **swap** et les **horaires de trading**."
why = "Ces chiffres entrent dans chacun de tes calculs : garde-les sous la main. Un solde fictif énorme donne de mauvaises habitudes."
[[steps]]
heading = "Ton premier trade d’entraînement"
text = "Ouvre un ordre de **0,01 lot**, où chaque mouvement de 1 $ vaut 1 $. Avant de confirmer, place un stop-loss à environ 5 $ et un take-profit à environ 10 $. Regarde-le commencer légèrement négatif (le spread), puis clôture-le."
why = "Retrouve-le ensuite dans ton historique et vérifie que le résultat correspond à lots × 100 × Mouvement en dollars. Tu ne cherches pas encore à gagner de l’argent ; tu apprends où sont les boutons, pour qu’ils ne te distraient jamais plus tard."
widget = "stop_first"
[[steps]]
heading = "La démo, c’est l’entraînement, pas une preuve"
text = "Trader en démo construit des compétences et des habitudes. Ce n’est pas la preuve que tu gagneras en réel : sans argent réel en jeu, il est facile de garder des trades perdants ou de prendre des trades risqués. C’est pour ça que ce cours juge ta démo sur la **discipline**, pas sur le gain."
[[recall]]
id = "r1"
q = "Un compte démo utilise…"
options = ["De vrais prix et de l’argent fictif", "D’anciens prix et de l’argent réel", "Des prix fictifs"]
answer = 0
why = "Les prix réels du marché, de l’argent simulé."
[[recall]]
id = "r2"
q = "Où vérifies-tu la licence d’un courtier ?"
options = ["Dans le registre du régulateur lui-même", "Sur le site du courtier", "Dans un groupe sur les réseaux sociaux"]
answer = 0
why = "Une licence peut être falsifiée ; le registre, non."
[[recall]]
id = "r3"
q = "0,01 lot avec un stop de 5 $ : si le stop est touché, tu perds environ…"
options = ["5 $", "50 $", "0,50 $"]
answer = 0
why = "0,01 × 100 × 5 $ = 5 $."
+++
