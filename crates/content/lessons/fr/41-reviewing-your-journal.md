+++
id = 41
stage = 9
title = "Relire ton journal"
objective = "chercher les fuites dans ton propre journal : regrouper tes trades, comparer leurs résultats en R, trouver l’habitude qui coûte le plus, et changer une seule règle à la fois."
hook = "Cinquante trades démo sont dans ton journal. Quelque part dedans se cache l’habitude qui te coûte le plus. Comment la trouver ?"
analogy = "Pense au budget d’un foyer. Le total dépensé ne dit pas grand-chose ; trier les tickets entre courses, transport et plats à emporter montre où part vraiment l’argent. Il y a souvent une catégorie qui fuit, et rarement celle qu’on croyait."
analogy_tag = "trier les tickets"
example = "60 trades démo, en moyenne +0,15R. Une fois triés : les trades du matin à Londres, 32, en moyenne +0,4R. Les trades après midi, heure de New York, 14, en moyenne −0,3R. Les 7 notés FOMO font en moyenne −0,8R, et 5 d’entre eux ont enfreint une règle. La fuite n’est pas le setup : ce sont les trades de fin de journée et la course après le prix. Un seul changement : une nouvelle version du plan qui ne trade que jusqu’à 12 h, heure de New York. Puis 20 trades de plus avant de juger."
mistake = "Changer trois choses après une mauvaise semaine. Avec plusieurs changements à la fois, impossible de savoir lequel a aidé, et une semaine compte trop peu de trades pour montrer autre chose que du bruit."
takeaways = [
  "Regroupe tes trades : par séance, par type de Scénario, par émotion notée, par règle enfreinte.",
  "Compare le résultat moyen en R de chaque groupe ; une fuite, c’est un groupe nettement négatif ou qui concentre les infractions.",
  "Les petits groupes trompent : attends une vingtaine de trades dans un groupe avant de conclure.",
  "Change une seule règle à la fois, dans une nouvelle version du plan, et juge-la sur la série de trades suivante.",
]

[[steps]]
heading = "Du carnet aux données"
text = "Une entrée du journal, c’est un carnet ; cinquante, ce sont des **données**. Une fois que tu les as, relis tes entrées et **regroupe-les** : par séance, par type de Scénario (repli, rebond), par **émotion notée**, par règle enfreinte. Note le nombre et les résultats de chaque groupe dans un petit tableau."
why = "Le cours de CME Group sur le plan de trading présente le journal comme un outil pour voir la vue d’ensemble : en un coup d’œil, tu vois ton historique et tu repères tes réussites et tes erreurs."

[[steps]]
heading = "Le R moyen par groupe"
text = "Pour chaque groupe, additionne les résultats en **R** et divise par le nombre de trades : c’est son **R moyen**. Tout ton journal peut faire +0,15R en moyenne, alors qu’un groupe fait +0,4R et un autre −0,3R. Les groupes sous zéro sont tes **fuites**."
why = "Mesurer en R plutôt qu’en dollars garde les groupes comparables, même si ton solde ou ton risque a changé entre-temps (leçon 34)."

[[steps]]
heading = "Regarde les émotions et les infractions"
text = "Deux groupes méritent une attention particulière : les trades notés **FOMO, revanche ou excès de confiance**, et ceux qui ont **enfreint le plan**. Ils représentent souvent peu de trades et beaucoup de pertes. Si c’est le cas, ton avantage va bien ; c’est ta discipline qui fuit."
why = "C’est la leçon 36 avec tes propres chiffres : les habitudes qui transforment de bons trades en mauvais apparaissent en général clairement une fois comptées."

[[steps]]
heading = "Les petits échantillons mentent"
text = "Cinq trades dans un groupe peuvent paraître brillants ou terribles par pur hasard. Traite un groupe comme une **preuve** seulement quand il compte une **vingtaine de trades** ; avant, c’est une piste à surveiller. Pareil pour une seule bonne ou mauvaise semaine."
why = "Avec une espérance positive, tu peux quand même perdre six fois d’affilée (leçon 34). Les petits groupes montrent surtout la chance."

[[steps]]
heading = "Un changement, puis on mesure"
text = "Choisis **la** plus grosse fuite et transforme le remède en règle : enregistre-la dans une **nouvelle version du plan**, et écris pourquoi dans ton Bilan de la semaine. Puis trade la vingtaine de trades suivante avant de juger. Une fuite réparée, puis la suivante."
why = "Changer plusieurs règles à la fois empêche de savoir lequel des changements a aidé. Les versions gardent la trace de ce qui a changé, et quand (leçon 39)."

[[videos]]
id = "WjWC5Qlpqns"
title = "The EXACT thought process, entries, exits, and results of my recent trades..."
channel = "Rayner Teo"
duration = "14:23"
watch_for = "Comment un trader revient sur une série de trades : le raisonnement, l’entrée et la sortie, et le résultat, côte à côte. Ses trades et ses marchés sont les siens."
fallback_search = "review my recent trades thought process entries exits"
checked = "2026-11-16"

[[sources]]
claim = "Un journal de trades donne une vue d’ensemble de ton historique pour repérer réussites et erreurs ; note pourquoi tu as pris chaque trade et, après chaque journée, attache-toi au pourquoi et au comment de tes résultats plus qu’au gain ou à la perte."
name = "CME Group Education: Keep a Trade Log"
url = "https://www.cmegroup.com/education/courses/building-a-trade-plan/keep-a-trade-log.html"
checked = "2026-11-16"

[[quiz]]
id = "q1"
q = "Ton journal fait +0,15R en moyenne, mais les trades après midi, heure de New York, font −0,3R sur 14 trades. Qu’est-ce que c’est ?"
options = ["Une fuite probable : un groupe à la moyenne négative", "La preuve que tout le plan est cassé", "Rien à noter", "Une raison de trader plus l’après-midi"]
answer = 0
why = "Un groupe sous zéro est une fuite à examiner."

[[quiz]]
id = "q2"
q = "Pourquoi comparer les groupes en R plutôt qu’en dollars ?"
options = ["Le R reste comparable même si ton solde ou ton risque a changé", "Les dollars sont interdits dans un journal", "Le R paraît toujours meilleur", "Il n’y a aucune différence"]
answer = 0
why = "Une perte de −1R veut dire la même chose à n’importe quelle taille."

[[quiz]]
id = "q3"
q = "Quatre trades le vendredi, tous perdants. Que conclus-tu ?"
options = ["Pas grand-chose encore : quatre trades, c’est trop peu ; continue d’observer", "Ne plus jamais trader le vendredi", "Le vendredi porte malheur", "Doubler la taille le vendredi pour te refaire"]
answer = 0
why = "Attends une vingtaine de trades dans un groupe avant de conclure."

[[quiz]]
id = "q4"
q = "Tu trouves deux fuites. Quel est le plan ?"
options = ["Réparer d’abord la plus grosse avec une seule nouvelle règle, mesurer, puis passer à l’autre", "Changer les deux règles et le setup d’un coup", "Les ignorer", "Arrêter le journal"]
answer = 0
why = "Un changement à la fois, sinon tu ne sais pas ce qui a marché."

[[quiz]]
id = "q5"
q = "Les 7 trades notés FOMO font en moyenne −0,8R, et 5 ont enfreint une règle. Qu’est-ce que ça suggère ?"
options = ["La fuite vient de la discipline, pas du setup", "Le setup ne marche pas", "Les trades FOMO devraient être plus gros", "Les émotions notées ne comptent pas"]
answer = 0
why = "Peu de trades, beaucoup de pertes."

[[recall]]
id = "r1"
q = "Une fuite, c’est…"
options = ["Un groupe de trades au R moyen nettement négatif", "N’importe quel trade perdant", "Une connexion internet lente"]
answer = 0
why = "Les trades perdants sont normaux ; les groupes perdants sont des fuites."

[[recall]]
id = "r2"
q = "Avant de conclure sur un groupe, attends environ…"
options = ["20 trades", "2 trades", "1 semaine"]
answer = 0
why = "Les petits échantillons montrent surtout la chance."

[[recall]]
id = "r3"
q = "Quand tu répares une fuite, tu…"
options = ["Enregistres une seule nouvelle règle dans une nouvelle version du plan et tu mesures", "Changes tout d’un coup"]
answer = 0
why = "Un changement, puis on mesure."

[explain_back]
prompt = "Comment trouves-tu une fuite dans ton journal, et que fais-tu ensuite ?"
must_include = ["regrouper tes trades par séance, type de Scénario, émotion notée et règle enfreinte, et comparer le résultat moyen en R de chaque groupe", "une fuite est un groupe nettement négatif ou qui concentre les infractions, mais les petits groupes trompent, donc on attend une vingtaine de trades avant de conclure", "changer une seule règle à la fois dans une nouvelle version du plan et la juger sur la série de trades suivante"]
misconceptions = ["une mauvaise semaine prouve qu’une règle est mauvaise", "changer plusieurs choses à la fois répare les fuites plus vite"]

[[explain_back.fixtures]]
answer = "Je trie mes trades en groupes, par séance, type de setup, émotion notée et règle enfreinte, et je calcule le R moyen de chaque groupe. Un groupe nettement sous zéro, ou qui concentre mes infractions, est une fuite, à condition qu’il compte une vingtaine de trades ; en dessous, ça peut être de la chance. Ensuite je ne répare que la plus grosse fuite avec une seule nouvelle règle, je l’enregistre dans une nouvelle version du plan, et je lui laisse la vingtaine de trades suivante avant de juger."
expect = "got_it"

[[explain_back.fixtures]]
answer = "Je regroupe mes trades par séance et par émotion notée, je calcule le R moyen de chacun, et ceux sous zéro sont mes fuites."
expect = "almost"

[[explain_back.fixtures]]
answer = "Après une mauvaise semaine, je change mon setup, mes séances et mon risque d’un coup pour que la semaine suivante soit meilleure."
expect = "almost_or_not_yet"
+++
