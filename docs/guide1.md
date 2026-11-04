
## Contexte du projet

Je souhaite créer un **projet open source en Rust** : un générateur CLI permettant de démarrer rapidement une API Axum correctement structurée, sans répéter manuellement l’ajout des crates, la création des modules et la configuration des middlewares.

Le projet comporte deux parties :

1. **Un générateur CLI**, installable avec Cargo.
2. **Un template d’application Axum**, généré par ce CLI.

Le principe serait :

```bash
cargo <nom-du-generateur> new mon-api
```

Le nom du générateur reste à choisir.

## Objectif du générateur

Une seule commande doit produire un projet :

- immédiatement compilable ;
- directement exécutable ;
- correctement structuré ;
- documenté avec Swagger UI ;
- équipé de logs HTTP JSON ;
- accompagné de tests fonctionnels ;
- sans framework supplémentaire imposé : le résultat doit rester du Rust/Axum standard.

## Fonctionnalités du MVP

Pour la première version, je veux uniquement :

- une commande `new` ;
- un seul profil nommé `standard` ;
- la validation du nom du projet ;
- la vérification du dossier de destination ;
- le refus d’écraser un dossier non vide ;
- la génération de tous les fichiers ;
- un récapitulatif final avec les commandes de lancement.

Ne pas ajouter dans le MVP :

- PostgreSQL ou SQLx ;
- authentification ;
- plusieurs profils ;
- Docker ;
- Redis ;
- OpenTelemetry ;
- génération de contrôleurs supplémentaires.

Ces fonctionnalités viendront après validation du MVP.

## Structure de l’application générée

```text
mon-api/
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── app.rs
│   ├── config.rs
│   ├── error.rs
│   ├── state.rs
│   ├── openapi.rs
│   ├── controllers/
│   │   ├── mod.rs
│   │   ├── hello.rs
│   │   └── health.rs
│   ├── models/
│   │   ├── mod.rs
│   │   └── message.rs
│   └── middlewares/
│       ├── mod.rs
│       ├── request_id.rs
│       └── request_logging.rs
├── tests/
│   └── api.rs
├── migrations/
│   └── .gitkeep
├── .env.example
├── .gitignore
├── Cargo.toml
└── README.md
```

Responsabilités importantes :

- `main.rs` : démarrage minimal ;
- `app.rs` : construction du routeur et lancement du serveur ;
- `config.rs` : configuration et validation ;
- `state.rs` : état partagé Axum ;
- `openapi.rs` : document OpenAPI ;
- `controllers/` : endpoints HTTP ;
- `middlewares/` : corrélation et journalisation.

## Comportement du serveur généré

- Adresse par défaut : `0.0.0.0`.
- Port par défaut : **3030**.
- Port modifiable avec `PORT`.
- Chargement facultatif de `.env` avec `dotenvy`.
- `GET /hello` : réponse JSON `Hello World`.
- `GET /health` : vérification de fonctionnement.
- Swagger UI : `/swagger-ui`.
- Spécification OpenAPI : `/api-docs/openapi.json`.
- Gestion centralisée des erreurs JSON avec `thiserror`.

## Logs HTTP attendus

Chaque requête terminée doit produire une ligne JSON contenant :

```json
{
  "requestId": "abc123",
  "ip": "203.0.113.5",
  "requestTime": "2026-10-28T10:15:30.123Z",
  "httpMethod": "POST",
  "routeKey": "/orders",
  "status": 201,
  "protocol": "HTTP/1.1",
  "responseLength": 512,
  "responseTime": "12ms"
}
```

Contraintes :

- `requestId` généré par défaut avec UUID ;
- identifiant propagé dans `x-request-id` ;
- date UTC au format RFC 3339 ;
- `routeKey` basé sur le modèle de route Axum, pas sur l’URL brute ;
- pas de corps de requête, token, cookie ou mot de passe dans les logs ;
- les en-têtes de proxy comme `X-Forwarded-For` ne doivent être acceptés que si le proxy est explicitement approuvé ;
- les logs ne doivent pas contaminer les réponses HTTP.

## Dépendances envisagées

Pour le générateur :

- `clap` ;
- `anyhow` ou `thiserror` pour les erreurs internes ;
- `minijinja` pour les templates ;
- `include_dir` pour embarquer les templates ;
- `toml_edit` pour les manifests ;
- `camino` pour les chemins ;
- `assert_cmd`, `predicates` et `tempfile` pour les tests.

Pour l’application générée :

- `axum` ;
- `tokio` ;
- `serde` et `serde_json` ;
- `tower` et `tower-http` ;
- `tracing` et `tracing-subscriber` ;
- `thiserror` ;
- `dotenvy` ;
- `utoipa` et `utoipa-swagger-ui` ;
- `uuid` ;
- `time`.

Les versions compatibles doivent être centralisées et validées ensemble. Le générateur ne doit pas récupérer arbitrairement les dernières versions à chaque exécution.

## Tests indispensables

Le générateur doit être testé en exécutant réellement son binaire :

- création d’un projet dans un dossier temporaire ;
- vérification de l’arborescence ;
- refus d’un nom invalide ;
- refus d’écraser un dossier non vide ;
- compilation du projet généré ;
- exécution de ses tests ;
- vérification de `cargo fmt` et Clippy.

Le projet généré doit tester :

- `/hello` ;
- `/health` ;
- la présence des endpoints dans OpenAPI ;
- l’accès à Swagger UI ;
- le port configurable ;
- le format des erreurs ;
- les logs JSON ;
- la propagation de `x-request-id`.

## Première étape demandée

Aide-moi maintenant à **initialiser le dépôt du générateur CLI**, avec :

1. son arborescence ;
2. son `Cargo.toml` ;
3. la commande `new` avec Clap ;
4. la validation du nom et du dossier de destination ;
5. une organisation prête à accueillir les templates Axum ;
6. les premiers tests du CLI.

Ca sera un projet opensource et je veux tout coder moi mem.  Donne moi par quoi commencer.