# Comprendre le projet : LGP et CartPole

## Objectif général

Le projet consiste à entraîner un programme à résoudre une tâche de
**reinforcement learning** (apprentissage par renforcement). L’agent observe
l’état d’un environnement, choisit une action, puis reçoit une récompense.
L’objectif est d’obtenir une récompense cumulée aussi élevée que possible au
fil d’un épisode.

L’environnement étudié ici est **CartPole** : il faut maintenir un pendule en
équilibre sur un chariot en choisissant dans quelle direction pousser.

## Les notions importantes

- **Environnement** : le système simulé. Il conserve son état, applique une
  action et renvoie le nouvel état, une récompense et un indicateur de fin
  d’épisode.
- **État** : les quatre valeurs observées dans CartPole : position et vitesse
  du chariot, angle et vitesse angulaire du pendule.
- **Action** : le choix du sens dans lequel pousser le chariot.
- **Épisode** : une exécution de l’environnement depuis son initialisation
  jusqu’à sa fin.
- **Score** : le total des récompenses reçues pendant l’épisode. Dans
  l’environnement actuel, chaque étape rapporte un point ; un score élevé
  signifie donc généralement que le pendule est resté en équilibre longtemps.

## Pourquoi utiliser LGP ?

**Linear Genetic Programming** (LGP) représente un agent par un petit programme
composé d’instructions. Ce programme reçoit l’état courant en entrée, effectue
des calculs et fournit une action.

Au lieu d’écrire manuellement le meilleur programme, on fait évoluer une
population de programmes :

1. **Évaluation** : mesurer le score de chaque programme en le faisant jouer
   dans l’environnement.
2. **Sélection** : garder des programmes selon la méthode indiquée dans le
   sujet.
3. **Mutation** : modifier les programmes conservés afin de produire une
   nouvelle population.
4. **Répétition** : recommencer l’évaluation et l’évolution sur plusieurs
   générations, puis observer si les scores s’améliorent.

Les choix précis de représentation des programmes, de sélection et de mutation
font partie du travail demandé : il faut suivre les contraintes et les
indications de l’énoncé pour les concevoir.

## Ce que contient déjà le dépôt

- `env/src/lib.rs` définit l’interface commune des environnements, ainsi que
  les types d’état et de score.
- `env/src/cartpole.rs` contient la simulation CartPole et son calcul de
  récompense.
- `env/src/pcgrng.rs` fournit un générateur pseudo-aléatoire avec graine.
- `lgp/src/main.rs` lance actuellement CartPole avec des actions aléatoires et
  affiche le score de l’épisode. C’est un exemple de lancement, pas encore
  l’algorithme d’apprentissage LGP.

## Grandes étapes de travail

1. Comprendre l’interface de l’environnement et vérifier le comportement du
   programme actuel.
2. Concevoir, conformément à l’énoncé, la représentation d’un programme LGP et
   vérifier qu’un programme peut être exécuté sur un état.
3. Définir comment un programme choisit une action à partir de son résultat.
4. Évaluer une population de programmes sur des épisodes et calculer leurs
   scores.
5. Appliquer les méthodes de sélection et de mutation demandées pour former
   les générations suivantes.
6. Faire tourner l’apprentissage, puis comparer ses résultats à une référence
   simple, par exemple le comportement aléatoire du programme initial.

## Comment lancer le programme actuel

Depuis la racine du dépôt, exécuter :

```text
cargo run -p prog64
```

Cette commande compile puis lance le binaire `prog64`. Pour l’instant, elle
affiche le score d’un épisode joué avec des actions aléatoires ; elle ne lance
pas encore un entraînement LGP.

## À garder en tête

- L’évaluation dépend des états de départ aléatoires ; utiliser des graines
  reproductibles aide à comparer les essais.
- Un score sur un seul épisode peut être trompeur. Comparer plusieurs épisodes
  permet de mieux estimer les performances.
- Vérifier séparément l’exécution d’un programme, le calcul du score et
  l’évolution de la population facilite le diagnostic des erreurs.
- Les règles exactes et les critères de réussite sont ceux de l’énoncé du
  projet ; ces notes donnent une vue d’ensemble et ne les remplacent pas.


# Resumé (Zeineb):
  Détaillons le parcours du programme dans main.rs, puis ce que fait l’environnement dans cartpole.rs.

1. L’état du CartPole
L’environnement représente le chariot et le pendule par quatre nombres, dans cet ordre :

x : position du chariot ;
x_dot : vitesse du chariot ;
theta : angle du pendule par rapport à la verticale ;
theta_dot : vitesse angulaire du pendule.
Un état proche de [0, 0, 0, 0] signifie que le chariot est au centre, immobile, avec le pendule vertical et immobile. Au démarrage, les quatre valeurs sont plutôt tirées au hasard dans une petite plage autour de zéro : de -0,05 à 0,05.

2. Les actions tirées par main
main crée un générateur aléatoire avec la graine 1, puis tire une valeur entre 0 et 3 à chaque tour. Cette valeur est passée à env.step.

Dans l’environnement, l’action est interprétée seulement selon son signe :

0 produit une force de -10 ;
1, 2 et 3 produisent une force de +10.
Ainsi, les quatre actions ne correspondent pas à quatre mouvements différents : il n’y a que deux directions possibles, et le tirage choisit la direction positive trois fois sur quatre. L’action ne dépend pas de l’état du pendule ; le programme ne cherche donc pas à corriger sa position.

3. Ce que calcule un pas
À chaque appel à step, le programme utilise la position et les vitesses actuelles pour estimer les accélérations du chariot (x_dd) et du pendule (t_dd). Le calcul prend en compte la force appliquée, la gravité, les masses et l’angle du pendule.

Ensuite, il met à jour les positions et les vitesses avec tau, qui vaut 0,02 seconde. C’est une approximation du mouvement continu : le programme avance par petits intervalles de temps. La valeur de tau signifie que 500 pas représentent environ 10 secondes de simulation.

L’environnement renvoie alors le nouvel état, une récompense et un booléen indiquant si l’épisode est terminé. Le nouvel état est bien renvoyé à main, mais celui-ci l’ignore avec _ : il ne s’en sert pas pour décider de la prochaine action.

4. Quand l’épisode se termine et ce que signifie le score
L’épisode se termine si le chariot dépasse -2,4 ou +2,4, si le pendule dépasse -12° ou +12°, ou après 500 pas. Chaque pas rapporte 1 point, donc total_reward correspond essentiellement au nombre de pas effectués avant la fin.

Deux détails à noter :

Le commentaire de main dit que l’environnement est réinitialisé, mais le programme ne appelle pas reset : les valeurs initiales sont déjà créées par CartPole::default().
step reçoit aussi le générateur aléatoire, mais ne l’utilise pas. La graine 1 sert ici à choisir les actions ; la graine 0 utilisée dans CartPole::default() sert à produire l’état initial.