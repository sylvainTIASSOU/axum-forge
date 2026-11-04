use anyhow::{Context, ensure};
use camino::Utf8PathBuf;
use regex::Regex;
use std::sync::OnceLock;

pub fn validate_project_name(project_name: &str) -> bool {
    static REGEX_NAME: OnceLock<Regex> = OnceLock::new();

    let re = REGEX_NAME.get_or_init(|| {
        // ^[a-z] : Commence par une minuscule
        // [a-z0-9_-]*$ : Suivi de zéro ou plusieurs minuscules, chiffres, - ou _
        Regex::new(r"^[a-z][a-z0-9_-]*$").unwrap()
    });
    re.is_match(project_name)
}

pub fn convert_to_module_name(project_name: &str) -> String {
    project_name.replace('-', "_")
}

pub fn resolve_and_validate_destination(
    base_path: &Utf8PathBuf,
    project_name: &str,
) -> anyhow::Result<Utf8PathBuf> {
    // make the final path: ./sandbox/my-api
    let destination = base_path.join(project_name);
    if destination.exists() {
        // Rule 4: if is ordinary file
        ensure!(
            !destination.is_file(),
            "The destination {} is a file. Impossible to creat the project",
            destination
        );

        // Rule  2 & 3: verify if the folde is en*mpty
        let mut entry = std::fs::read_dir(destination.as_std_path())
            .with_context(|| format!("Failed to read directory {}", destination))?;

        let is_empty = entry.next().is_none();

        ensure!(
            is_empty,
            "The destination {} is not empty. Impossible to create the project.",
            destination
        );
    }
    // Rule 1 : if folde not rxiste or empty, return the valide path.
    Ok(destination)
}
