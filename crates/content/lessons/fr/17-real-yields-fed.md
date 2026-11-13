+++
id = 17
stage = 3
title = "Les taux réels et la Fed"
objective = "expliquer pourquoi des taux réels en hausse pèsent en général sur l’or, et comment les décisions de taux de la Fed les influencent."
hook = "L’or ne te rapporte rien pendant que tu le gardes. Alors que devient son attrait quand une obligation d’État américaine, sans risque, se met à rapporter davantage ?"
analogy = "Imagine deux tirelires. L’une, c’est l’or : elle ne te rapporte jamais rien, mais elle garde bien sa valeur. L’autre, c’est une obligation d’État qui verse des intérêts après inflation. Quand ce versement réel augmente, la tirelire en or paraît moins attirante et certains déplacent leur argent. Quand il baisse, l’or redevient plus intéressant."
analogy_tag = "deux tirelires"
example = "Une obligation américaine à 10 ans rapporte 4,5 % par an, et l’inflation attendue est de 2,5 %. Le taux réel est d’environ 2 %. Si la Fed laisse entendre des taux plus élevés et que le taux réel grimpe à 2,5 %, garder de l’or, qui ne rapporte rien, te coûte plus en revenu manqué, et l’or glisse souvent, disons de 4 100,00 à 4 060,00. (Chiffres donnés à titre d’exemple.)"
mistake = "Croire que « taux en hausse, or en baisse » est automatique. Ce qui compte le plus, c’est le taux réel (après inflation) et ce que les marchés attendaient. L’or peut monter pendant que les taux montent, par exemple quand la peur ou les achats des banques centrales sont plus forts."
takeaways = [
  "L’or ne verse pas d’intérêts : le garder a un coût d’opportunité, le revenu auquel tu renonces.",
  "Le taux réel, c’est en gros le rendement d’une obligation moins l’inflation attendue ; le rendement des TIPS à 10 ans est la mesure habituelle.",
  "Des taux réels en hausse pèsent en général sur l’or ; des taux réels en baisse le soutiennent en général.",
  "Les décisions et les indices de la Fed font bouger les rendements, c’est pourquoi les jours de FOMC font bouger l’or.",
]

[[steps]]
heading = "Le coût caché de l’or"
text = "L’or ne verse pas d’intérêts. Le garder, c’est renoncer au revenu que tu pourrais toucher ailleurs, par exemple avec une obligation d’État américaine sans risque. Ce revenu manqué, c’est le **coût d’opportunité** de l’or. Plus il est grand, moins l’or a tendance à attirer."
why = "Le World Gold Council range le coût d’opportunité (le dollar américain et les rendements obligataires ensemble) parmi les grands moteurs de l’or. Tu as vu la moitié « dollar » à la leçon 16 ; cette leçon est la moitié « rendements »."

[[steps]]
heading = "Le taux réel : l’intérêt après inflation"
text = "Le **taux réel** d’une obligation, c’est en gros son taux d’intérêt moins l’inflation attendue. Une obligation à 4,5 % avec 2,5 % d’inflation attendue a un taux réel d’environ **2 %**. Les traders suivent comme mesure le **rendement des TIPS à 10 ans**, l’obligation d’État américaine protégée contre l’inflation."
why = "Les TIPS (Treasury Inflation-Protected Securities) sont ajustés de l’inflation : leur rendement est donc déjà un taux réel. La Réserve fédérale le publie chaque jour ; sur FRED, c’est la série DFII10."

[[steps]]
heading = "Taux réels en hausse, or en général en baisse"
text = "Quand les taux réels **montent**, l’obligation sans risque rapporte plus après inflation, donc garder de l’or coûte plus en revenu manqué : l’or **baisse en général**. Quand les taux réels **baissent**, le coût d’opportunité de l’or diminue et l’or **monte en général**. En général, pas toujours."
why = "Le World Gold Council s’attend à ce que des rendements en hausse et un dollar plus fort fassent sortir de l’argent des fonds en or, et à ce que des taux plus bas l’aident. Mais ce n’est qu’un moteur parmi d’autres : à certaines périodes, d’autres forces, comme la peur ou les achats des banques centrales, ont fait monter l’or alors que les taux réels étaient élevés."

[[steps]]
heading = "Là où intervient la Fed"
text = "La **Réserve fédérale** fixe le taux d’intérêt américain à court terme huit fois par an, lors des réunions du **FOMC**. Ses décisions, et ses indices sur la suite, changent ce que les marchés attendent pour tous les taux, et donc les taux réels. C’est pourquoi l’or bouge souvent fortement les jours de FOMC."
why = "Les marchés tradent des anticipations. Si tout le monde attend déjà une hausse de taux, la hausse elle-même peut à peine faire bouger l’or ; c’est une surprise, ou un changement de ton de la Fed sur l’avenir, qui le fait bouger. C’est tout le sujet de la leçon 19."

[[videos]]
id = "AkMsMDk_brU"
title = "How the Fed Steers Interest Rates to Guide the Entire Economy"
channel = "WSJ | The Wall Street Journal"
duration = "5:19"
watch_for = "Comment les décisions de taux de la Fed se propagent aux autres taux d’intérêt, y compris les rendements obligataires qui comptent pour l’or."
fallback_search = "how the Fed steers interest rates explained"
checked = "2026-11-12"

[[videos]]
id = "F9CMGTbqU_w"
title = "How Do Real Interest Rates Impact Gold Prices?"
channel = "Financial Source"
duration = "4:33"
watch_for = "Pourquoi c’est le taux d’intérêt après inflation, et non le taux affiché, qui fait concurrence à l’or."
fallback_search = "real interest rates gold prices explained"
checked = "2026-11-12"

[[sources]]
claim = "Le coût d’opportunité, l’attrait de l’or face au dollar américain et aux rendements obligataires, est un des moteurs de l’or ; des rendements en hausse et un dollar plus fort pèsent dessus, des taux plus bas et un dollar plus faible le soutiennent."
name = "World Gold Council : Gold Mid-Year Outlook 2025"
url = "https://www.gold.org/goldhub/research/gold-mid-year-outlook-2025"
checked = "2026-11-12"

[[sources]]
claim = "Le rendement des emprunts d’État américains indexés sur l’inflation (TIPS) à 10 ans est publié chaque jour par la Réserve fédérale (H.15), série DFII10."
name = "FRED : Market Yield on U.S. Treasury Securities at 10-Year Constant Maturity, Inflation-Indexed (DFII10)"
url = "https://fred.stlouisfed.org/series/DFII10"
checked = "2026-11-12"

[[sources]]
claim = "Le FOMC tient huit réunions programmées par an, dont quatre avec un résumé des projections économiques."
name = "Réserve fédérale : calendrier des réunions du FOMC"
url = "https://www.federalreserve.gov/monetarypolicy/fomccalendars.htm"
checked = "2026-11-12"

[[quiz]]
id = "q1"
q = "Pourquoi l’or a-t-il un « coût d’opportunité » ?"
options = ["Il ne verse pas d’intérêts, donc tu renonces à un revenu possible ailleurs", "Les courtiers facturent des frais pour le garder", "Il perd du poids avec le temps", "Il est taxé chaque mois"]
answer = 0
why = "Garder de l’or, c’est se passer des intérêts qu’une obligation sans risque verserait."

[[quiz]]
id = "q2"
q = "Une obligation rapporte 4,0 % et l’inflation attendue est de 2,5 %. Quel est à peu près son taux réel ?"
options = ["1,5 %", "6,5 %", "4,0 %", "2,5 %"]
answer = 0
why = "Taux réel ≈ rendement − inflation attendue : 4,0 % − 2,5 % = 1,5 %."

[[quiz]]
id = "q3"
q = "Le rendement des TIPS à 10 ans passe de 2,0 % à 2,6 % en un mois. Que fait l’or en général ?"
options = ["Il est en général sous pression et baisse", "Il monte en général fortement", "Il ne peut pas bouger", "Il monte toujours d’exactement 0,6 %"]
answer = 0
why = "Des taux réels plus élevés augmentent le coût d’opportunité de l’or, ce qui pèse en général dessus."

[[quiz]]
id = "q4"
q = "À quelle fréquence le FOMC tient-il ses réunions programmées ?"
options = ["Huit fois par an", "Chaque semaine", "Une fois par an", "Seulement en cas d’urgence"]
answer = 0
why = "Il y a huit réunions programmées par an, plus d’autres si besoin."

[[quiz]]
id = "q5"
q = "Les taux réels montent depuis des semaines, et pourtant l’or continue de grimper. Quelle explication colle à cette leçon ?"
options = ["Un autre moteur, comme la peur ou les achats des banques centrales, est plus fort pour l’instant", "Les taux réels n’ont aucun effet sur l’or", "Le graphique doit être faux", "L’or verse maintenant des intérêts"]
answer = 0
why = "Les taux réels sont un moteur parmi d’autres ; d’autres peuvent l’emporter un moment."

[[quiz]]
id = "q6"
q = "Tout le monde attend une hausse de taux de la Fed, et elle a lieu. Pourquoi l’or bougerait-il à peine ?"
options = ["La hausse était déjà attendue et intégrée dans les prix", "L’or ne se trade pas les jours de FOMC", "La Fed fixe le prix de l’or", "Les hausses de taux ne comptent jamais"]
answer = 0
why = "Les marchés bougent sur les surprises et sur les indices de la Fed pour l’avenir, pas sur ce que tout le monde attendait déjà."

[[recall]]
id = "r1"
q = "Des taux réels en hausse font en général…"
options = ["Baisser l’or", "Monter l’or", "Verser des intérêts à l’or"]
answer = 0
why = "Ils augmentent le coût d’opportunité de l’or."

[[recall]]
id = "r2"
q = "Le taux réel, c’est en gros…"
options = ["Le rendement moins l’inflation attendue", "Le rendement plus l’inflation", "Le prix de l’or divisé par 100"]
answer = 0
why = "L’intérêt après inflation."

[[recall]]
id = "r3"
q = "Qui tient les réunions du FOMC ?"
options = ["La Réserve fédérale américaine", "Le marché de l’or de Londres", "Le World Gold Council"]
answer = 0
why = "Le comité de la Fed qui fixe les taux se réunit huit fois par an."

[explain_back]
prompt = "Pourquoi des taux réels en hausse pèsent-ils en général sur l’or, et où intervient la Fed ?"
must_include = ["l’or ne verse pas d’intérêts, donc le garder a un coût d’opportunité", "le taux réel est l’intérêt après inflation ; quand il monte, l’or baisse en général (et inversement)", "les décisions et les indices de la Fed font bouger les rendements, donc les nouvelles du FOMC font bouger l’or", "c’est une tendance, pas une règle : d’autres moteurs peuvent l’emporter"]
misconceptions = ["des taux d’intérêt plus élevés font toujours baisser l’or", "l’or verse des intérêts comme une obligation"]

[[explain_back.fixtures]]
answer = "L’or ne verse pas d’intérêts, donc le garder, c’est renoncer à ce qu’une obligation sans risque rapporte après inflation, son taux réel. Quand les taux réels montent, ce revenu manqué est plus grand, donc l’or baisse en général, et quand ils baissent l’or monte en général. Les décisions et les indices de la Fed aux réunions du FOMC font bouger ces rendements, c’est pour ça que l’or réagit. Ce n’est qu’une tendance : la peur ou les achats des banques centrales peuvent faire monter l’or quand même."
expect = "got_it"

[[explain_back.fixtures]]
answer = "L’or ne verse pas d’intérêts, donc quand les taux réels montent, les obligations paraissent meilleures et l’or baisse en général."
expect = "almost"

[[explain_back.fixtures]]
answer = "Chaque fois que la Fed monte ses taux, l’or baisse toujours, parce que l’or verse des intérêts qui diminuent quand les taux montent."
expect = "almost_or_not_yet"
+++
