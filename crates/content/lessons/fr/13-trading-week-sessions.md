+++
id = 13
stage = 2
title = "La semaine de trading et les séances"
objective = "décrire la semaine de trading de l’or et ses trois séances, en heure de New York et dans ton heure locale."
hook = "Si l’or se trade presque 24 h sur 24, l’heure à laquelle tu trades compte-t-elle ?"
analogy = "Pense à un relais autour du monde. Tokyo porte le témoin pendant la journée asiatique, le passe à Londres, et Londres le passe à New York. La course a lieu chaque jour de semaine, mais certains relais ont bien plus de coureurs que d’autres."
analogy_tag = "course de relais"
example = "Mardi, 3 h à New York. En hiver, il est 8 h à Londres et 17 h à Tokyo : la séance de Tokyo vient de finir et celle de Londres commence. L’or était ouvert tout ce temps ; ce qui a changé, c’est qui le trade. (Horaires d’hiver ; en été, Londres a une heure de décalage en plus ou en moins.)"
mistake = "Croire que l’or est aussi actif toute la journée. Il est ouvert presque 24 h sur 24 en semaine, mais certaines heures sont calmes. L’essentiel de l’action se passe quand Londres et New York tradent."
takeaways = [
  "L’or se trade du dimanche 18 h au vendredi 17 h, heure de New York, avec une pause d’une heure chaque jour de semaine à 17 h.",
  "Trois séances se relaient autour du monde : asiatique (Tokyo), puis Londres, puis New York.",
  "Londres et New York sont les deux plus grandes places de l’or.",
  "L’horloge des séances de l’appli affiche chaque séance dans ton heure locale, changements d’heure compris.",
]
drills = ["session"]

[[steps]]
heading = "La semaine de trading de l’or"
text = "L’or se trade presque 24 h sur 24 en semaine. La semaine ouvre **le dimanche à 18 h, heure de New York**, et ferme **le vendredi à 17 h**. Chaque jour de semaine, il y a **une pause d’une heure à 17 h, heure de New York**, quand la journée de trading bascule. Le week-end, l’or ne se trade pas."
why = "Ce sont les horaires des contrats à terme sur l’or sur la plateforme Globex du CME, que les courtiers CFD suivent dans les grandes lignes ; les horaires de ton courtier peuvent différer un peu, vérifie la fiche du contrat. L’heure de New York sert de référence parce que c’est là qu’on compte la journée de trading."

[[steps]]
heading = "Trois séances autour du monde"
text = "Chaque jour passe par trois grandes séances : **asiatique** (Tokyo, 9 h–16 h heure de Tokyo), **Londres** (8 h–17 h heure de Londres) et **New York** (8 h–17 h heure de New York). Une séance, c’est simplement le moment où les banques et les traders de cette ville sont à leur bureau."
widget = "session_clock"
why = "Les horaires des séances ne sont pas des cloches d’ouverture officielles ; ce sont des conventions, et les sources diffèrent d’une heure ici ou là. Ce cours utilise partout ces définitions, pour que l’horloge des séances, ton journal et les Règles strictes de ton plan soient toujours d’accord."

[[steps]]
heading = "Londres et New York en tête"
text = "Londres est le centre mondial du marché de gré à gré de l’or, et New York accueille le plus grand marché à terme de l’or (COMEX). Deux fois par jour, à **10 h 30 et 15 h, heure de Londres**, les enchères du **LBMA Gold Price** fixent un prix de référence utilisé dans le monde entier."
why = "Le LBMA Gold Price est fixé lors d’enchères électroniques organisées par ICE Benchmark Administration. Tu ne traderas pas ces enchères, mais elles rappellent où se passe l’essentiel du trading de l’or."

[[steps]]
heading = "Les horloges changent, les séances bougent"
text = "Londres et New York passent tous deux à l’heure d’été, mais pas le **même dimanche**. Quelques semaines chaque printemps et chaque automne, l’écart entre les deux est de 4 heures au lieu de 5, et les séances se décalent dans ton heure locale. Ne retiens pas les heures par cœur : laisse l’horloge de l’appli convertir."
why = "Les États-Unis changent le deuxième dimanche de mars et le premier dimanche de novembre ; le Royaume-Uni le dernier dimanche de mars et d’octobre. Tokyo ne change jamais. L’horloge des séances fait ce calcul pour ton propre fuseau horaire, à partir des réglages de ton appareil."

[[videos]]
id = "znpD7T_9fMQ"
title = "Forex Trading Sessions Explained!"
channel = "SimulationFX"
duration = "6:39"
watch_for = "Comment les séances asiatique, de Londres et de New York se passent le relais, et pourquoi certaines heures sont plus actives que d’autres."
fallback_search = "trading sessions explained Asian London New York"
checked = "2026-11-12"

[[videos]]
id = "tW3F-Yv0ZwI"
title = "What's the Best Time to Trade Forex? | 3 Major Market Sessions"
channel = "TraderNick"
duration = "10:10"
watch_for = "Où se placent les trois séances dans la journée et où elles se chevauchent. La vidéo parle des devises, mais l’or suit la même horloge."
fallback_search = "best time to trade three major market sessions"
checked = "2026-11-12"

[[sources]]
claim = "Les contrats à terme sur l’or se tradent du dimanche au vendredi, de 18 h à 17 h (heure de l’Est), avec une pause de 60 minutes chaque jour à partir de 17 h."
name = "CME Group : E-mini Gold contract specs"
url = "https://cmegroup.com/markets/metals/precious/e-mini-gold.contractSpecs.html"
checked = "2026-11-12"

[[sources]]
claim = "Le LBMA Gold Price est fixé deux fois par jour lors d’enchères organisées par ICE Benchmark Administration, à partir de 10 h 30 et 15 h, heure de Londres."
name = "LBMA : LBMA Gold Price"
url = "https://www.lbma.org.uk/prices-and-data/lbma-gold-price/lbma-gold-price"
checked = "2026-11-12"

[[sources]]
claim = "Le marché de gré à gré de Londres et le marché à terme de New York comptent parmi les grandes places de l’or, et le fuseau de Londres fait le pont entre les heures asiatiques et américaines."
name = "World Gold Council : Global gold market"
url = "https://gold.org/gold-market-structure/global-gold-market"
checked = "2026-11-12"

[[sources]]
claim = "Au Royaume-Uni, on avance l’heure le dernier dimanche de mars et on la recule le dernier dimanche d’octobre."
name = "GOV.UK : When do the clocks change?"
url = "https://gov.uk/when-do-the-clocks-change"
checked = "2026-11-12"

[[sources]]
claim = "Aux États-Unis, l’heure d’été commence le deuxième dimanche de mars et se termine le premier dimanche de novembre."
name = "US Naval Observatory : Daylight time"
url = "https://aa.usno.navy.mil/faq/daylight_time"
checked = "2026-11-12"

[[quiz]]
id = "q1"
q = "La pause quotidienne du trading de l’or a lieu à…"
options = ["17 h, heure de New York", "Minuit à Londres", "9 h, heure de Tokyo", "Il n’y a pas de pause quotidienne"]
answer = 0
why = "Chaque jour de semaine, l’or fait une pause d’une heure à partir de 17 h, heure de New York, quand la journée de trading bascule."

[[quiz]]
id = "q2"
q = "Dans quel ordre les grandes séances se succèdent-elles chaque jour de semaine ?"
options = ["New York → Londres → Asie", "Asie → Londres → New York", "Londres → Asie → New York", "Les trois en même temps"]
answer = 1
why = "La journée avance vers l’ouest avec le soleil : d’abord Tokyo, puis Londres, puis New York."

[[quiz]]
id = "q3"
q = "Il est 9 h à Londres, un mercredi d’hiver. Quelle séance est ouverte ?"
options = ["Londres et New York", "Seulement l’Asie", "Seulement Londres", "Aucune : l’or est fermé"]
answer = 2
why = "9 h à Londres, c’est 4 h à New York (avant sa séance) et 18 h à Tokyo (après sa séance) : seule Londres est ouverte."

[[quiz]]
id = "q4"
q = "Quand commencent les enchères du LBMA Gold Price ?"
options = ["8 h 30, heure de New York", "10 h 30 et 15 h, heure de Londres", "Toutes les heures pile", "Seulement le vendredi"]
answer = 1
why = "Les deux enchères quotidiennes commencent à 10 h 30 et 15 h, heure de Londres."

[[quiz]]
id = "q5"
q = "Pourquoi une séance peut-elle commencer une heure plus tôt ou plus tard dans ton heure locale pendant quelques semaines par an ?"
options = ["Les courtiers déplacent les séances pour les jours fériés", "L’or se trade moins d’heures en été", "Les États-Unis et le Royaume-Uni changent d’heure à des dates différentes", "Tokyo passe à l’heure d’été"]
answer = 2
why = "Les États-Unis et le Royaume-Uni changent d’heure à des dimanches différents : pendant quelques semaines, l’écart entre eux est de 4 heures, pas 5."

[[quiz]]
id = "q6"
q = "Quand la semaine de trading de l’or se termine-t-elle ?"
options = ["Vendredi 17 h, heure de New York", "Samedi midi, heure de Londres", "Vendredi minuit UTC", "Elle ne se termine jamais"]
answer = 0
why = "La semaine se termine le vendredi à 17 h, heure de New York, et reprend le dimanche à 18 h."

[[recall]]
id = "r1"
q = "Quand la semaine de trading de l’or ouvre-t-elle ?"
options = ["Dimanche 18 h, heure de New York", "Lundi 9 h, heure de Londres", "Samedi minuit UTC"]
answer = 0
why = "La semaine ouvre le dimanche soir, heure de New York."

[[recall]]
id = "r2"
q = "Quelle séance vient juste après la séance asiatique ?"
options = ["Londres", "New York", "Aucune : le marché ferme"]
answer = 0
why = "Tokyo passe le relais à Londres, et Londres à New York."

[[recall]]
id = "r3"
q = "Pourquoi utiliser l’horloge de l’appli plutôt que de retenir les horaires des séances ?"
options = ["Les changements d’heure les décalent dans ton heure locale", "Les séances changent chaque semaine", "Les courtiers ne publient pas leurs horaires"]
answer = 0
why = "Les changements d’heure aux États-Unis et au Royaume-Uni décalent les séances dans ton fuseau ; l’horloge s’en charge pour toi."

[explain_back]
prompt = "Décris la semaine de trading de l’or et ses trois séances. Pourquoi l’heure de la journée compte-t-elle ?"
must_include = ["l’or se trade du dimanche soir au vendredi soir, heure de New York, avec une courte pause chaque jour", "les séances asiatique, de Londres et de New York se succèdent autour du monde", "l’activité n’est pas la même toute la journée : Londres et New York sont les places les plus actives"]
misconceptions = ["l’or est aussi actif à chaque heure d’ouverture", "l’or se trade le week-end"]

[[explain_back.fixtures]]
answer = "L’or est ouvert du dimanche 18 h au vendredi 17 h, heure de New York, avec une pause d’une heure chaque jour de semaine à 17 h. La journée passe de la séance asiatique à Tokyo à Londres, puis à New York. Ce n’est pas aussi actif toute la journée : Londres et New York sont les plus grandes places, donc l’essentiel de l’action se passe quand elles tradent."
expect = "got_it"

[[explain_back.fixtures]]
answer = "L’or se trade presque 24 h sur 24 à travers les séances asiatique, de Londres et de New York, l’une après l’autre."
expect = "almost"

[[explain_back.fixtures]]
answer = "L’or se trade 24 h sur 24, 7 jours sur 7, week-end compris, et toutes les heures se valent, donc peu importe quand on trade."
expect = "almost_or_not_yet"
+++
