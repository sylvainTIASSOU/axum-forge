#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use camino::Utf8PathBuf;
    use cargo_axum_forge::project::{
        convert_to_module_name, resolve_and_validate_destination, validate_project_name,
    };
    use predicates::prelude::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_validation_noms() {
        // Cas acceptés
        assert!(validate_project_name("mon-api"));
        assert!(validate_project_name("mon_api"));
        assert!(validate_project_name("api2"));
        assert!(validate_project_name("a"));

        // Cas refusés
        assert!(!validate_project_name("Mon-API")); // Majuscules
        assert!(!validate_project_name("mon api")); // Espace
        assert!(!validate_project_name("../mon-api")); // Séparateurs de chemin
        assert!(!validate_project_name(".")); // Point
        assert!(!validate_project_name("2api")); // Commence par un chiffre
    }

    #[test]
    fn test_conversion_module() {
        assert_eq!(convert_to_module_name("mon-api"), "mon_api");
        assert_eq!(
            convert_to_module_name("mon-super-projet-axum"),
            "mon_super_projet_axum"
        );
        assert_eq!(convert_to_module_name("deja_valide"), "deja_valide");
    }

    /// convert the temporary folde to Utf8PathBuf.
    fn base_path(temp_dir: &TempDir) -> Utf8PathBuf {
        Utf8PathBuf::from_path_buf(temp_dir.path().to_path_buf())
            .expect("temporary directory path should be valid UTF-8")
    }

    #[test]
    fn accepts_a_destination_that_does_not_exist() {
        let temp_dir = tempfile::tempdir().unwrap();
        let base_path = base_path(&temp_dir);

        let destination = resolve_and_validate_destination(&base_path, "mon-api").unwrap();

        assert_eq!(destination, base_path.join("mon-api"));

        // La fonction valide le chemin, mais ne doit pas le créer.
        assert!(!destination.exists());
    }

    #[test]
    fn accepts_an_existing_empty_directory() {
        let temp_dir = tempfile::tempdir().unwrap();
        let base_path = base_path(&temp_dir);
        let expected_destination = base_path.join("mon-api");

        fs::create_dir(&expected_destination).unwrap();

        let destination = resolve_and_validate_destination(&base_path, "mon-api").unwrap();

        assert_eq!(destination, expected_destination);
        assert!(destination.is_dir());
    }

    #[test]
    fn rejects_a_destination_that_is_a_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let base_path = base_path(&temp_dir);
        let destination = base_path.join("mon-api");

        fs::write(&destination, "existing content").unwrap();

        let error = resolve_and_validate_destination(&base_path, "mon-api")
            .expect_err("a file should not be accepted as a destination");

        assert!(error.to_string().contains("is a file"));

        // Le fichier existant ne doit pas avoir été modifié.
        assert_eq!(fs::read_to_string(destination).unwrap(), "existing content");
    }

    #[test]
    fn rejects_a_non_empty_directory_containing_a_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let base_path = base_path(&temp_dir);
        let destination = base_path.join("mon-api");

        fs::create_dir(&destination).unwrap();
        fs::write(destination.join("important.txt"), "keep me").unwrap();

        let error = resolve_and_validate_destination(&base_path, "mon-api")
            .expect_err("a non-empty directory should be rejected");

        assert!(error.to_string().contains("is not empty"));

        // Le contenu existant doit être conservé.
        assert_eq!(
            fs::read_to_string(destination.join("important.txt")).unwrap(),
            "keep me"
        );
    }

    #[test]
    fn rejects_a_non_empty_directory_containing_a_subdirectory() {
        let temp_dir = tempfile::tempdir().unwrap();
        let base_path = base_path(&temp_dir);
        let destination = base_path.join("mon-api");

        fs::create_dir(&destination).unwrap();
        fs::create_dir(destination.join("src")).unwrap();

        let result = resolve_and_validate_destination(&base_path, "mon-api");

        assert!(result.is_err());
        assert!(destination.join("src").is_dir());
    }

    #[test]
    fn rejects_a_directory_containing_only_a_hidden_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let base_path = base_path(&temp_dir);
        let destination = base_path.join("mon-api");

        fs::create_dir(&destination).unwrap();
        fs::write(destination.join(".gitkeep"), "").unwrap();

        let result = resolve_and_validate_destination(&base_path, "mon-api");

        assert!(result.is_err());
    }

    #[test]
    fn creates_a_project_from_the_standard_profile() {
        let temp_dir = tempfile::tempdir().unwrap();

        let mut command = Command::cargo_bin("cargo-axum-forge").unwrap();

        command
            .args(["axum-forge", "new", "mon-api"])
            .arg("--path")
            .arg(temp_dir.path())
            .assert()
            .success()
            .stdout(predicate::str::contains(
                "Project `mon-api` created successfully.",
            ))
            .stdout(predicate::str::contains("Profile: standard"))
            .stdout(predicate::str::contains("cargo run"));

        let project_dir = temp_dir.path().join("mon-api");

        let manifest = fs::read_to_string(project_dir.join("Cargo.toml")).unwrap();
        assert!(manifest.contains("name = \"mon-api\""));

        assert!(project_dir.join("src/bin/main.rs").is_file());
        assert!(project_dir.join("README.md").is_file());
        assert!(project_dir.join(".gitignore").is_file());
    }
}
