+++
id = 3
stage = 1
title = "Qu’est-ce que l’or, et pourquoi le monde en veut-il ?"
objective = "citer trois raisons pour lesquelles on achète de l’or et les relier à son prix."
hook = "Pourquoi un métal jaune et brillant compte-t-il encore dans un monde d’applis et d’argent numérique ?"
analogy = "Vois l’or comme la tirelire de secours du monde. La plupart du temps, elle reste sur l’étagère. Quand les gens s’inquiètent pour leur emploi, les banques ou une guerre, ils tendent la main vers la tirelire, et quand tout le monde tend la main vers la même tirelire en même temps, ce qu’elle contient prend de la valeur."
analogy_tag = "tirelire de secours"
example = "Imagine l’or à 4 100,00 un lundi calme. Mardi, une nouvelle inquiétante tombe et des investisseurs du monde entier placent de l’argent dans l’or. Plus d’acheteurs que de vendeurs poussent le prix à 4 150,00 : un mouvement de 50 $ en une journée. Le métal n’a pas changé ; seul le nombre de personnes qui en voulaient a changé. (Prix donnés à titre d’exemple.)"
mistake = "Croire que l’or ne fait que monter, ou qu’il te rapporte quelque chose pendant que tu le gardes. L’or peut baisser pendant de longues périodes, et il ne verse ni intérêts ni dividendes : tu ne gagnes que si quelqu’un paie plus tard plus cher que toi."
takeaways = [
  "On veut de l’or pour les bijoux, l’investissement, les réserves des banques centrales et la technologie.",
  "L’or est vu comme une valeur qui se conserve : la demande monte souvent quand on a peur.",
  "Plus de demande que d’offre fait monter le prix ; moins de demande le laisse baisser.",
  "L’or ne verse pas d’intérêts, et il ne fait pas que monter.",
]

[[videos]]
id = "ezolUaQ37xE"
title = "CNA Explains: Why is gold a safe haven asset?"
channel = "CNA"
duration = "3:55"
watch_for = "Pourquoi les investisseurs placent leur argent dans l’or quand ils s’inquiètent, et ce que ça fait à son prix."
fallback_search = "why is gold a safe haven asset explained"
checked = "2026-11-10"

[[videos]]
id = "qOpzs7IP51U"
title = "The Volatility of the Gold Market, Explained"
channel = "WSJ | The Wall Street Journal"
duration = "5:41"
watch_for = "L’idée que le prix de l’or peut varier fortement, dans les deux sens."
fallback_search = "gold market volatility explained"
checked = "2026-11-10"

[[sources]]
claim = "La demande d’or vient des bijoux, des lingots et des pièces, des ETF, des banques centrales et de la technologie."
name = "World Gold Council : Gold Demand Trends"
url = "https://www.gold.org/goldhub/research/gold-demand-trends"
checked = "2026-11-10"

[[quiz]]
id = "q1"
q = "Laquelle de ces sources n’est PAS une des principales sources de demande d’or ?"
options = ["Les bijoux", "Les banques centrales", "Des intérêts versés chaque mois", "La technologie"]
answer = 2
why = "L’or ne verse pas d’intérêts. La demande vient des bijoux, de l’investissement, des banques centrales et de la technologie."

[[quiz]]
id = "q2"
q = "Pourquoi achète-t-on souvent de l’or quand on a peur ?"
options = ["Sa hausse est garantie", "Il est vu comme gardant sa valeur en cas de crise", "Les courtiers le donnent gratuitement", "Il verse une prime en période difficile"]
answer = 1
why = "L’or est vu comme une valeur refuge : quelque chose qui garde sa valeur quand le reste semble risqué."

[[quiz]]
id = "q3"
q = "Aujourd’hui, beaucoup plus de gens veulent acheter de l’or que le vendre. Que fait généralement le prix ?"
options = ["Il monte", "Il baisse", "Il reste exactement le même", "Le trading s’arrête"]
answer = 0
why = "Quand les acheteurs sont plus nombreux que les vendeurs, ils surenchérissent et font monter le prix."

[[quiz]]
id = "q4"
q = "L’or passe de 4 100,00 à 4 150,00 après une nouvelle inquiétante. De combien a-t-il bougé ?"
options = ["5 $", "50 $", "500 $", "0,50 $"]
answer = 1
why = "4 150 − 4 100 = 50 $."

[[quiz]]
id = "q5"
q = "Quelle affirmation est vraie ?"
options = ["L’or ne fait que monter", "L’or verse des dividendes chaque année", "L’or peut baisser pendant de longues périodes", "Le prix de l’or ne bouge jamais les jours calmes"]
answer = 2
why = "L’or a connu de longues périodes de baisse. Ce n’est pas un pari à sens unique."

[explain_back]
prompt = "Pourquoi les gens se précipitent-ils sur l’or quand ils ont peur ?"
must_include = ["l’or est vu comme une valeur qui se conserve dans le temps", "c’est une valeur refuge en cas de peur ou de crise", "plus de demande fait monter le prix"]
misconceptions = ["l’or ne fait que monter", "l’or verse des intérêts ou des dividendes"]

[[explain_back.fixtures]]
answer = "Parce que l’or a gardé sa valeur pendant très longtemps, les gens le voient comme un endroit sûr pour leur argent quand tout semble risqué. Quand beaucoup de gens en veulent en même temps, cette demande en plus fait monter le prix."
expect = "got_it"

[[explain_back.fixtures]]
answer = "Quand les gens ont peur, ils veulent mettre leur argent à l’abri, et l’or est vu comme sûr."
expect = "almost"

[[explain_back.fixtures]]
answer = "Ils achètent de l’or parce qu’il prend toujours de la valeur et qu’il leur verse des intérêts pendant qu’ils le gardent."
expect = "almost_or_not_yet"

[[steps]]
heading = "Qui veut de l’or ?"
text = "Le World Gold Council classe la demande mondiale en quelques catégories : les **bijoux**, l’**investissement** (lingots, pièces et fonds), les **banques centrales** qui constituent des réserves, et un peu de **technologie**. Chacune achète plus ou moins selon les prix, les revenus et l’humeur du moment."
why = "Les banques centrales gardent de l’or dans leurs réserves comme une épargne qui ne dépend d’aucun gouvernement. Additionne tous ces groupes et tu obtiens la demande mondiale d’or."
[[steps]]
heading = "Pourquoi la peur compte"
text = "L’or ne dépend ni des bénéfices d’une entreprise ni des promesses d’un gouvernement, alors beaucoup le voient comme **une valeur qui se conserve dans le temps**. Quand le monde paraît incertain, l’argent se dirige vers l’or. C’est pour ça qu’on l’appelle une **valeur refuge**."
[[steps]]
heading = "Comment la demande devient un prix"
text = "Un prix, c’est le point où acheteurs et vendeurs acceptent de se rencontrer. Plus d’acheteurs que de vendeurs, et les acheteurs doivent offrir plus : **le prix monte**. Quand la demande retombe, **le prix baisse**. Pas besoin de prédire l’actualité, seulement de comprendre que l’actualité change la demande."
[[steps]]
heading = "Ce que l’or ne fait pas"
text = "L’or ne te rapporte rien pendant que tu le gardes : ni intérêts, ni dividende. Et il ne fait pas que monter ; il a connu de longues périodes de baisse. Quiconque te présente l’or comme une valeur sûre à 100 % essaie de te vendre quelque chose."
why = "Plus tard, tu verras pourquoi ne rien rapporter compte : quand les taux d’intérêt sont élevés, garder quelque chose qui ne rapporte rien devient moins intéressant."
[[recall]]
id = "r1"
q = "Laquelle est une vraie source de demande d’or ?"
options = ["Les banques centrales", "Des dividendes mensuels en or", "Les bonus des courtiers"]
answer = 0
why = "Les banques centrales gardent de l’or dans leurs réserves."
[[recall]]
id = "r2"
q = "Les acheteurs deviennent soudain plus nombreux que les vendeurs. Le prix, en général…"
options = ["Monte", "Baisse", "Reste le même"]
answer = 0
why = "Les acheteurs doivent offrir plus pour en obtenir."
[[recall]]
id = "r3"
q = "L’or verse-t-il des intérêts pendant que tu le gardes ?"
options = ["Non", "Oui, chaque mois", "Seulement aux gros détenteurs"]
answer = 0
why = "L’or ne rapporte rien par lui-même ; tu ne gagnes que si son prix monte."
+++
