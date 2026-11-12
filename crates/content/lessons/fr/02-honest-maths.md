+++
id = 2
stage = 1
title = "Les vrais calculs du trading"
objective = "expliquer pourquoi la plupart des débutants perdent, et pourquoi un trader peut se tromper souvent et gagner quand même."
hook = "Si tu gagnes plus de trades que tu n’en perds, est-ce que tu gagnes forcément de l’argent ?"
analogy = "Pense à un étal de marché qui vend la plupart de ses articles à perte pour attirer les clients, mais quelques-uns avec une grosse marge. Compte les ventes une par une : il a l’air de couler. Fais le total de la semaine : il s’en sort bien. Un trader se juge de la même façon : sur les totaux, pas trade par trade."
analogy_tag = "étal de marché"
example = "Dix trades, chacun risquant 100 $. Cinq touchent leur stop : −500 $. Cinq atteignent un objectif deux fois plus loin : +1 000 $. Raison une fois sur deux seulement, et pourtant +500 $. Maintenant, inverse : gagne 6 trades sur 10, mais 50 $ par gain et 150 $ par perte, et tu es à −300 $."
mistake = "Te juger sur la fréquence à laquelle tu as raison. Un taux de réussite élevé avec de grosses pertes fait quand même perdre de l’argent, et vouloir « avoir raison » pousse à garder les trades perdants et à couper trop tôt les gagnants."
takeaways = [
  "La plupart des comptes CFD de particuliers perdent de l’argent ; les régulateurs obligent les courtiers à le dire.",
  "Le résultat dépend de la taille des gains face aux pertes, pas seulement de la fréquence des gains.",
  "Avoir raison une fois sur deux peut suffire si les gains sont plus gros que les pertes.",
  "Des pertes petites et prévues, et une méthode répétable, valent mieux que la conviction.",
]

[[steps]]
heading = "La plupart des débutants perdent"
text = "Au Royaume-Uni et dans l’UE, un courtier en CFD doit t’indiquer la part de ses comptes de particuliers qui perdent de l’argent. Le régulateur européen a constaté qu’elle se situe en général entre **74 et 89 %**. Ce n’est pas de la malchance : c’est ce qui arrive quand on trade sans plan pour ses pertes."
why = "L’avertissement de chaque courtier doit donner son propre pourcentage. La fourchette de 74 à 89 % vient des analyses des régulateurs nationaux dans les pays de l’UE. C’est pour ça que ce cours te demande de prouver ta discipline sur un compte démo avant de te parler d’argent réel."

[[steps]]
heading = "Avoir raison n’est pas le but"
text = "Ce qui compte, c’est combien tu gagnes quand tu as raison face à combien tu perds quand tu as tort. Gagne 6 trades sur 10 avec 50 $ par gain et 150 $ par perte, et tu es à **−300 $**."
why = "6 gains × 50 $ = 300 $ gagnés. 4 pertes × 150 $ = 600 $ perdus. Résultat : −300 $, malgré 60 % de trades gagnants."

[[steps]]
heading = "Petites pertes, gains plus gros"
text = "Risque 100 $ par trade. Perds 5 trades sur 10 à 100 $ (−500 $) et gagnes-en 5 à 200 $ (+1 000 $) : **+500 $**, alors que tu n’as eu raison qu’une fois sur deux. Garder les pertes petites et laisser les gains être plus gros, c’est tout le jeu."
widget = "win_rate"
why = "Les traders appellent la perte prévue **1R** (une unité de risque). Un gain deux fois plus gros vaut +2R. En R, le résultat ci-dessus donne 5 × (+2R) + 5 × (−1R) = +5R sur dix trades. Tu verras le R en détail dans les leçons sur le risque."

[[steps]]
heading = "La méthode bat la conviction"
text = "Un trader n’est pas quelqu’un qui se sent sûr de lui. C’est quelqu’un qui suit les mêmes règles à chaque fois : un plan écrit, un stop-loss sur chaque trade, une taille choisie avant d’entrer. Tu construiras cette méthode dans ce cours, étape par étape."
why = "La confiance ressemble à du talent, mais le marché se moque de ta certitude. Une méthode répétable peut se mesurer, se revoir et s’améliorer ; une impression, non."

[[videos]]
id = "aKZsireNBIM"
title = "The Risk to Reward Ratio Explained in One Minute: From Definition and \"Formula\" to Examples"
channel = "One Minute Economics"
duration = "1:36"
watch_for = "Comment comparer ce que tu peux perdre avec ce que tu peux gagner décide si un trade vaut la peine."
fallback_search = "risk to reward ratio explained"
checked = "2026-11-10"

[[videos]]
id = "tNNqFSLXAOk"
title = "Win Rate, Risk To Reward Ratio, Expected Value (Trading Math)"
channel = "Mind Math Money"
duration = "11:14"
watch_for = "Comment le taux de réussite et la taille des gains face aux pertes se combinent en un résultat attendu par trade."
fallback_search = "win rate risk reward expected value trading"
checked = "2026-11-10"

[[sources]]
claim = "Dans les pays de l’UE, 74 à 89 % des comptes CFD de particuliers perdent généralement de l’argent ; les fournisseurs doivent afficher un avertissement standard avec leur propre pourcentage de comptes perdants et offrir une protection contre le solde négatif par compte."
name = "ESMA : ESMA agrees to prohibit binary options and restrict CFDs to protect retail investors"
url = "https://www.esma.europa.eu/press-news/esma-news/esma-agrees-prohibit-binary-options-and-restrict-cfds-protect-retail-investors"
checked = "2026-11-10"

[[sources]]
claim = "Au Royaume-Uni, les sociétés de CFD doivent fournir un avertissement standard indiquant le pourcentage de comptes de particuliers en perte, et garantir qu’un client ne peut pas perdre plus que les fonds de son compte CFD."
name = "FCA : FCA confirms permanent restrictions on the sale of CFDs to retail consumers"
url = "https://www.fca.org.uk/news/press-releases/fca-confirms-permanent-restrictions-sale-cfds-and-cfd-options-retail-consumers"
checked = "2026-11-10"

[[quiz]]
id = "q1"
q = "Selon le régulateur européen, quelle part des comptes CFD de particuliers perd généralement de l’argent ?"
options = ["Environ 10 à 20 %", "Environ 40 à 50 %", "Environ 74 à 89 %", "Presque aucun"]
answer = 2
why = "Les régulateurs ont constaté que 74 à 89 % des comptes de particuliers perdent généralement de l’argent ; c’est pour ça que les courtiers doivent te prévenir."

[[quiz]]
id = "q2"
q = "Tu gagnes 5 trades sur 10. Chaque gain fait +200 $ et chaque perte −100 $. Quel est le total ?"
options = ["−500 $", "0 $", "+500 $", "+1 500 $"]
answer = 2
why = "5 × 200 $ = 1 000 $ gagnés ; 5 × 100 $ = 500 $ perdus ; résultat +500 $."

[[quiz]]
id = "q3"
q = "Tu gagnes 7 trades sur 10, avec 30 $ par gain, mais tu perds 100 $ par perte. Quel est le total ?"
options = ["+210 $", "−90 $", "+90 $", "−300 $"]
answer = 1
why = "7 × 30 $ = 210 $ gagnés ; 3 × 100 $ = 300 $ perdus ; résultat −90 $. 70 % de trades gagnants, et pourtant une perte."

[[quiz]]
id = "q4"
q = "Qu’est-ce qui décide si un trader gagne de l’argent sur beaucoup de trades ?"
options = ["Uniquement la fréquence à laquelle il a raison", "À quel point il se sent sûr de lui", "Le taux de réussite combiné à la taille des gains face aux pertes", "Trader le plus souvent possible"]
answer = 2
why = "La fréquence des gains et leur taille par rapport aux pertes comptent toutes les deux."

[[quiz]]
id = "q5"
q = "Quelle habitude décrit le mieux un trader plutôt qu’un joueur ?"
options = ["Suivre les mêmes règles écrites à chaque fois", "Miser plus gros après une perte pour se refaire", "Faire confiance à une forte intuition", "Se passer du stop-loss quand on est sûr"]
answer = 0
why = "C’est une méthode répétable, pas la conviction, qui sépare le trading du jeu."

[[recall]]
id = "r1"
q = "Avoir raison sur la plupart des trades garantit de gagner de l’argent."
options = ["Faux", "Vrai"]
answer = 0
why = "Si les pertes sont bien plus grosses que les gains, un taux de réussite élevé peut quand même faire perdre."

[[recall]]
id = "r2"
q = "5 gains de 200 $ et 5 pertes de 100 $. Total ?"
options = ["+500 $", "0 $", "−500 $"]
answer = 0
why = "1 000 $ gagnés moins 500 $ perdus."

[[recall]]
id = "r3"
q = "Pourquoi les courtiers en CFD doivent-ils afficher un avertissement sur les risques ?"
options = ["La plupart des comptes de particuliers perdent de l’argent", "C’est une tradition marketing", "Pour faire la publicité de leurs bonus"]
answer = 0
why = "Les régulateurs l’exigent parce que la plupart des comptes CFD de particuliers perdent de l’argent."

[explain_back]
prompt = "Peut-on perdre plus de trades qu’on n’en gagne et quand même gagner de l’argent ? Comment ?"
must_include = ["la plupart des comptes CFD de particuliers perdent de l’argent", "le résultat dépend de la taille des gains par rapport aux pertes, pas seulement du taux de réussite", "garder les pertes petites et prévues (avec un stop) tout en laissant les gains être plus gros"]
misconceptions = ["c’est avoir raison la plupart du temps qui fait gagner de l’argent", "le trading est un moyen rapide de devenir riche"]

[[explain_back.fixtures]]
answer = "Oui. La plupart des comptes CFD de particuliers perdent de l’argent, donc il faut être prudent. Ce qui compte, c’est la taille des gains par rapport aux pertes : si je garde chaque perte petite avec un stop et que mes gains sont environ deux fois plus gros, je peux me tromper une fois sur deux et finir quand même gagnant."
expect = "got_it"

[[explain_back.fixtures]]
answer = "Oui, si tes gains sont plus gros que tes pertes, tu peux perdre plus souvent et gagner quand même au total."
expect = "almost"

[[explain_back.fixtures]]
answer = "Non. Pour gagner de l’argent, il faut avoir raison la plupart du temps, donc l’astuce est de trouver une stratégie qui gagne 80 ou 90 % des trades, puis de trader beaucoup pour devenir riche vite."
expect = "almost_or_not_yet"
+++
