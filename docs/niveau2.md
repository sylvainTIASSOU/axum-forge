# Étape suivante : créer le socle HTTP Axum

L’objectif de cette étape doit rester limité :

> Générer une application Axum compilable et testable, avec `/hello`, `/health` et la configuration du port.

N’ajoute pas encore :

- OpenAPI et Swagger UI ;
- `request_id` ;
- les logs HTTP JSON ;
- les middlewares personnalisés.

Ils viendront dans les étapes suivantes.

## 1. Arborescence à produire

Dans `templates/standard/`, vise cette structure :

```text
standard/
├── src/
│   ├── main.rs.j2
│   ├── lib.rs
│   ├── app.rs
│   ├── config.rs
│   ├── error.rs
│   ├── state.rs
│   ├── controllers/
│   │   ├── mod.rs
│   │   ├── hello.rs
│   │   └── health.rs
│   └── models/
│       ├── mod.rs
│       └── message.rs
├── tests/
│   └── api.rs
├── migrations/
│   └── .gitkeep
├── .env.example
├── .gitignore
├── Cargo.toml.j2
└── README.md.j2
```

Seuls les fichiers qui contiennent réellement une variable ont besoin de l’extension `.j2` :

- `Cargo.toml.j2` utilise `project_name` ;
- `main.rs.j2` utilise `crate_name` ;
- `README.md.j2` utilise `project_name`.

Les autres fichiers peuvent rester des fichiers statiques.

## 2. Manifeste du projet généré

Pour ce socle, le projet généré aura besoin des dépendances suivantes.

### Dépendances principales

- `axum` ;
- `tokio`, avec au minimum les fonctionnalités `macros`, `rt-multi-thread` et `net` ;
- `serde`, avec `derive` ;
- `serde_json` ;
- `thiserror` ;
- `dotenvy`.

### Dépendances de test

- `tower`, avec `util`, pour appeler le routeur avec `oneshot` ;
- `http-body-util`, pour lire les corps de réponse.

Ne choisis pas les versions au moment où l’utilisateur exécute le générateur. Sélectionne-les une fois pendant ton développement, vérifie leur compatibilité, puis écris-les explicitement dans `templates/standard/Cargo.toml.j2`.

## 3. Responsabilités des fichiers

### `src/lib.rs`

Expose les modules utilisables par le binaire et les tests :

```rust
pub mod app;
pub mod config;
pub mod controllers;
pub mod error;
pub mod models;
pub mod state;
```

### `src/main.rs.j2`

Ce fichier doit rester minimal. Il doit :

1. charger la configuration ;
2. initialiser éventuellement le tracing de base ;
3. appeler le lancement du serveur ;
4. retourner l’erreur éventuelle.

C’est ici que tu utiliseras :

```rust
use {{ crate_name }}::app;
use {{ crate_name }}::config::Config;
```

Ainsi, `mon-api` produira un import depuis `mon_api`.

### `src/config.rs`

Crée une structure `Config` contenant au minimum :

```text
host : 0.0.0.0
port : 3030 par défaut
```

Comportement attendu :

- `dotenvy::dotenv().ok()` charge facultativement `.env` ;
- si `PORT` est absent, utiliser `3030` ;
- si `PORT` est présent, le convertir en `u16` ;
- si sa valeur est invalide, retourner une erreur claire.

Sépare si possible la lecture de l’environnement de la conversion du port. Une petite fonction pure comme `parse_port(&str)` sera beaucoup plus facile à tester.

Exemple de `.env.example` :

```dotenv
PORT=3030
```

### `src/state.rs`

Crée un état partagé minimal :

```rust
#[derive(Clone, Default)]
pub struct AppState;
```

Il sera volontairement vide pour le MVP, mais il permettra d’ajouter ensuite des services sans modifier toute l’architecture.

### `src/models/message.rs`

Définis des réponses sérialisables avec Serde. Par exemple :

```text
MessageResponse { message }
HealthResponse { status }
```

Évite de renvoyer directement des `serde_json::json!` partout : les structures typées documentent mieux l’API.

### `src/controllers/hello.rs`

Le contrôleur doit répondre à :

```http
GET /hello
```

avec :

```json
{
  "message": "Hello World"
}
```

### `src/controllers/health.rs`

Le contrôleur doit répondre à :

```http
GET /health
```

avec un statut HTTP `200` et, par exemple :

```json
{
  "status": "ok"
}
```

### `src/controllers/mod.rs`

Expose les deux modules :

```rust
pub mod health;
pub mod hello;
```

### `src/app.rs`

Prévois deux fonctions distinctes :

```rust
pub fn build_router(state: AppState) -> Router
```

et :

```rust
pub async fn run(config: Config) -> anyhow::Result<()>
```

`build_router` enregistre les routes suivantes :

```text
GET /hello
GET /health
```

La fonction `run` doit :

1. construire l’adresse à partir de `Config` ;
2. créer un `tokio::net::TcpListener` ;
3. construire le routeur ;
4. appeler `axum::serve`.

Cette séparation permettra aux tests d’appeler le routeur sans démarrer un vrai serveur.

### `src/error.rs`

Crée une erreur centrale avec `thiserror`, puis implémente `IntoResponse` pour les erreurs HTTP.

Pour le moment, une réponse d’erreur simple suffit :

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "message": "Internal server error"
  }
}
```

N’expose pas de détail interne sensible dans la réponse HTTP.

## 4. Tests du projet généré

Dans `templates/standard/tests/api.rs`, ajoute au minimum les tests suivants.

### Endpoint `/hello`

Le test doit :

1. construire le routeur ;
2. créer une requête `GET /hello` ;
3. appeler le routeur avec `oneshot` ;
4. vérifier le statut `200` ;
5. vérifier le type de contenu JSON ;
6. lire le corps ;
7. vérifier `"message": "Hello World"`.

### Endpoint `/health`

Applique le même principe :

- statut `200` ;
- réponse JSON ;
- présence de `"status": "ok"`.

### Configuration

Teste séparément les cas suivants :

```text
PORT absent       -> 3030
PORT=3031         -> 3031
PORT=abc          -> erreur
PORT=70000        -> erreur
PORT vide         -> erreur
```

Privilégie les tests d’une fonction pure de parsing plutôt que la modification globale des variables d’environnement dans des tests parallèles.

## 5. Ajouter un test côté générateur

Dans le dépôt `axum-forge`, ajoute un test d’intégration qui exécute réellement :

```bash
cargo-axum-forge axum-forge new mon-api --path <tempdir>
```

Ce test doit :

1. vérifier la présence de toute l’arborescence ;
2. vérifier que `Cargo.toml` contient `name = "mon-api"` ;
3. vérifier que `main.rs` importe `mon_api` ;
4. exécuter ensuite :

```bash
cargo test --manifest-path <tempdir>/mon-api/Cargo.toml
```

Ce dernier point est essentiel : il garantit que le générateur ne produit pas seulement les bons fichiers, mais un projet réellement compilable et testé.

## 6. Vérification manuelle

Quand les tests automatiques passent, exécute :

```bash
cargo run -- axum-forge new demo-api --path /tmp
cargo run --manifest-path /tmp/demo-api/Cargo.toml
```

Dans un autre terminal :

```bash
curl http://127.0.0.1:3030/hello
curl http://127.0.0.1:3030/health
```

Puis vérifie le port configurable :

```bash
PORT=3031 cargo run --manifest-path /tmp/demo-api/Cargo.toml
```

Enfin, dans un autre terminal :

```bash
curl http://127.0.0.1:3031/health
```

## Critères pour terminer cette étape

Tu peux considérer le socle terminé lorsque :

- le générateur produit toute l’arborescence attendue ;
- `mon-api` et `mon_api` sont utilisés aux bons endroits ;
- le projet généré compile ;
- `/hello` et `/health` fonctionnent ;
- le port par défaut est `3030` ;
- `PORT` est pris en compte et validé ;
- les tests du projet généré passent ;
- les tests, le formatage et Clippy du générateur passent ;
- la CI GitHub est verte.

## Étapes suivantes

Après cela, l’ordre le plus naturel sera :

1. OpenAPI et Swagger UI ;
2. propagation de `x-request-id` ;
3. logs HTTP JSON ;

avec leurs tests à chaque étape.