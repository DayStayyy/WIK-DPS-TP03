# WIK-DPS-TP01

## CONSIGNES
- Mettre en place son environnement de développement et les bases du projet avec le moins de dépendances possibles

- Développer une API qui retourne au format JSON les headers de la requête quand il y une requête HTTP GET sur /ping

- Le serveur doit écouter sur un port configurable via la variable d'environnement : PING_LISTEN_PORT
- Réponse vide avec code 404 si quoi que ça soit d'autre que GET /ping ou code 500 si erreur inconnue

- Réalisation d'un README avec documentation sur le lancement du projet

## LANCEMENT DU PROJET

### Prérequis
- Cargo
- Clone du projet (git clone https://github.com/DayStayyy/WIK-DPS-TP01.git)
  
### Lancement
- Build du projet avec la commande : cargo build
- Lancement du projet avec la commande : cargo run


## DOCUMENTATION

Une seule route est disponible : GET /ping, elle retourne les headers de la requête au format JSON.

Vous pouvez modifier le port du serveur en modifiant la variable d'environnement PING_LISTEN_PORT (par défaut 8080) dans le fichier .env

