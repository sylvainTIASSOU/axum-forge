Je te conseille de commencer **uniquement par le squelette du CLI**, sans écrire le template Axum. Ton premier objectif est d’obtenir cette commande :

```bash
cargo axum-forge new mon-api
```

avec une CLI correctement parsée, des validations fiables et des tests qui exécutent réellement le binaire. La génération des fichiers viendra juste après.

J’utilise **`cargo-axum-forge`** comme nom de travail ; tu pourras le changer avant publication.

## 1. Initialise le dépôt

```bash
cargo new --bin cargo-axum-forge
cd cargo-axum-forge
```

Ajoute dès maintenant :

- un `README.md` décrivant l’objectif et la commande prévue ;
- une licence open source — le couple MIT + Apache-2.0 est courant dans l’écosystème Rust ;
- un `.gitignore` contenant au minimum `/target` ;
- le `Cargo.lock` dans Git, puisque tu construis une application exécutable.

Installe seulement les dépendances nécessaires à cette première étape :

```bash
cargo add clap --features derive
cargo add camino thiserror

cargo add --dev assert_cmd predicates tempfile
```

N’ajoute pas encore `toml_edit` : si le générateur produit un `Cargo.toml` complet à partir d’un template, il n’a rien à modifier. `minijinja` et `include_dir` seront utiles dès que tu brancheras les templates.

## 2. Adopte une arborescence simple

```text
cargo-axum-forge/
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── cli.rs
│   ├── commands/
│   │   ├── mod.rs
│   │   └── new.rs
│   ├── project.rs
│   └── templates.rs
├── templates/
│   └── standard/
│       └── .gitkeep
├── tests/
│   └── cli_new.rs
├── Cargo.toml
├── Cargo.lock
├── README.md
└── LICENSE-MIT / LICENSE-APACHE
```

Les responsabilités sont les suivantes :

- `main.rs` : parse la CLI, appelle la commande, affiche le résultat ou l’erreur ;
- `lib.rs` : expose les modules pour pouvoir tester la logique sans passer systématiquement par le binaire ;
- `cli.rs` : uniquement les types Clap ;
- `commands/new.rs` : orchestration de la commande ;
- `project.rs` : nom du projet, destination et validations ;
- `templates.rs` : futur chargement et rendu des templates ;
- `templates/standard/` : futur projet Axum à générer.

Dans `lib.rs` :

```rust
pub mod cli;
pub mod commands;
pub mod project;
pub mod templates;
```

## 3. Attention au fonctionnement des sous-commandes Cargo

Pour un binaire nommé `cargo-axum-forge`, Cargo transmet également `axum-forge` comme argument lors de l’exécution de :

```bash
cargo axum-forge new mon-api
```

Ta structure Clap doit donc représenter cette hiérarchie :

```text
cargo
└── axum-forge
    └── new
```

Un point de départ pour `src/cli.rs` :

```rust
use camino::Utf8PathBuf;
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "cargo", bin_name = "cargo", version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CargoCommand,
}

#[derive(Debug, Subcommand)]
pub enum CargoCommand {
    #[command(name = "axum-forge")]
    AxumForge(GeneratorCli),
}

#[derive(Debug, Args)]
pub struct GeneratorCli {
    #[command(subcommand)]
    pub command: GeneratorCommand,
}

#[derive(Debug, Subcommand)]
pub enum GeneratorCommand {
    /// Crée une nouvelle application Axum.
    New(NewArgs),
}

#[derive(Debug, Args)]
pub struct NewArgs {
    /// Nom du projet et du futur crate Cargo.
    pub name: String,

    /// Dossier parent dans lequel créer le projet.
    #[arg(long, default_value = ".")]
    pub path: Utf8PathBuf,
}
```

Ne crée pas encore d’option `--profile` : le MVP ne possède que `standard`. Tu peux garder ce choix comme constante interne pour le moment.

Dans `main.rs`, garde le code minimal :

```rust
use std::process::ExitCode;

use cargo_axum_forge::{cli::Cli, commands};
use clap::Parser;

fn main() -> ExitCode {
    let cli = Cli::parse();

    match commands::run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("erreur : {error}");
            ExitCode::FAILURE
        }
    }
}
```

La fonction `commands::run` dispatche ensuite vers `commands::new::execute`.

## 4. Définis précisément les validations

Avant d’écrire leur implémentation, fixe les règles.

### Nom du projet

Pour un MVP volontairement strict, tu peux accepter uniquement :

- une première lettre ASCII minuscule ;
- ensuite des lettres minuscules, chiffres, `-` ou `_` ;
- aucun espace, séparateur de chemin, `.` ou `..`.

Exemples :

```text
mon-api       accepté
mon_api       accepté
api2          accepté
2api          refusé
Mon-API       refusé
mon api       refusé
../mon-api    refusé
.             refusé
```

Prévois aussi une méthode pour obtenir le nom du module Rust :

```text
mon-api → mon_api
```

### Destination

Avec :

```bash
cargo axum-forge new mon-api --path ./sandbox
```

la destination est :

```text
./sandbox/mon-api
```

Règles recommandées :

1. Si la destination n’existe pas, elle peut être créée.
2. Si c’est un dossier vide, il peut être utilisé.
3. Si c’est un dossier non vide, échouer **avant tout ajout de fichier**.
4. Si c’est un fichier ordinaire, échouer.
5. Ne jamais supprimer ni écraser du contenu existant.

Tu peux modéliser ces règles avec de petits types plutôt qu’avec des chaînes dispersées :

```rust
pub struct ProjectName(String);

impl ProjectName {
    pub fn parse(value: &str) -> Result<Self, ProjectError>;

    pub fn as_str(&self) -> &str;

    pub fn rust_crate_name(&self) -> String {
        self.0.replace('-', "_")
    }
}
```

Utilise `thiserror` pour distinguer les erreurs utiles :

```rust
#[derive(Debug, thiserror::Error)]
pub enum ProjectError {
    #[error("nom du projet invalide : {0}")]
    InvalidName(String),

    #[error("la destination existe mais n’est pas un dossier : {0}")]
    DestinationNotDirectory(String),

    #[error("le dossier de destination n’est pas vide : {0}")]
    DestinationNotEmpty(String),

    #[error("impossible d’examiner la destination : {0}")]
    DestinationIo(String),
}
```

Commence par les tests unitaires de ces validations : ils sont rapides et ne dépendent pas de Clap.

## 5. Prépare la commande `new`

L’orchestration dans `commands/new.rs` devrait rester lisible :

```text
1. Valider le nom.
2. Calculer la destination.
3. Vérifier la destination.
4. Charger le profil standard.
5. Générer les fichiers.
6. Afficher le récapitulatif.
```

Pour la première implémentation, tu peux t’arrêter après l’étape 3 et renvoyer une erreur explicite du type « génération pas encore implémentée ». Mais il est encore mieux de terminer une toute petite tranche fonctionnelle : générer uniquement un `Cargo.toml` et un `src/main.rs` triviaux depuis `templates/standard/`.

Cela te permettra de valider tôt :

- le chargement des templates ;
- la création des dossiers ;
- les chemins relatifs ;
- le récapitulatif ;
- les tests de bout en bout.

La fonction principale peut avoir cette forme :

```rust
pub fn execute(args: NewArgs) -> Result<NewProjectReport, NewProjectError>;
```

Et le rapport peut contenir :

```rust
pub struct NewProjectReport {
    pub project_dir: Utf8PathBuf,
    pub files_created: usize,
}
```

Le récapitulatif final ressemblera à :

```text
Projet mon-api créé dans ./mon-api

Pour le démarrer :
  cd mon-api
  cargo run
```

## 6. Écris les premiers tests du binaire

Dans `tests/cli_new.rs`, utilise `assert_cmd` et `tempfile`. N’oublie pas l’argument initial `axum-forge`, nécessaire à cause du fonctionnement des sous-commandes Cargo.

Exemple pour un nom invalide :

```rust
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn rejects_an_invalid_project_name() {
    let temp = tempfile::tempdir().unwrap();

    let mut command = Command::cargo_bin("cargo-axum-forge").unwrap();

    command
        .args(["axum-forge", "new", "../mon-api"])
        .arg("--path")
        .arg(temp.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("nom du projet invalide"));
}
```

Exemple pour un dossier non vide :

```rust
use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn refuses_to_overwrite_a_non_empty_directory() {
    let temp = tempfile::tempdir().unwrap();
    let destination = temp.path().join("mon-api");

    fs::create_dir(&destination).unwrap();
    fs::write(destination.join("important.txt"), "à conserver").unwrap();

    let mut command = Command::cargo_bin("cargo-axum-forge").unwrap();

    command
        .args(["axum-forge", "new", "mon-api"])
        .arg("--path")
        .arg(temp.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("n’est pas vide"));

    assert_eq!(
        fs::read_to_string(destination.join("important.txt")).unwrap(),
        "à conserver"
    );
}
```

Ajoute ensuite ces cas :

```text
new mon-api réussit dans un dossier temporaire
new mon-api accepte un dossier de destination vide
new mon-api crée l’arborescence attendue
new mon-api --path <dossier> utilise le bon dossier parent
new mon-api affiche les commandes de lancement
```

Vérifie toujours, dans les tests d’échec, qu’aucun fichier existant n’a été modifié.

## 7. Ordre de travail recommandé

Fais de petits commits vérifiables :

1. **Squelette du dépôt** : README, licences, modules et dépendances.
2. **Parsing Clap** : la commande et l’aide fonctionnent.
3. **Validation du nom** : tests unitaires.
4. **Validation de la destination** : tests unitaires et tests du binaire.
5. **Premier template minimal** : génération d’un projet Rust trivial.
6. **Récapitulatif final** : chemin créé et commandes de lancement.
7. **Template Axum complet** : seulement après cette base.

À chaque étape, exécute :

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

**Ta première tâche concrète est donc : créer le squelette Clap et faire passer les tests « nom invalide » et « dossier non vide ».** C’est la meilleure fondation avant d’introduire `minijinja`, `include_dir` et toute la complexité du template Axum.