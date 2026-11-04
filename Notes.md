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