use anyhow::{Context, ensure};
use camino::Utf8PathBuf;
use regex::Regex;
use std::sync::OnceLock;

const RUST_KEYWORDS: &[&str] = &[
    // Strict keywords.
    "as", "async", "await", "break", "const", "continue", "crate",
    "dyn", "else", "enum", "extern", "false", "fn", "for", "if",
    "impl", "in", "let", "loop", "match", "mod", "move", "mut",
    "pub", "ref", "return", "self", "static", "struct", "super",
    "trait", "true", "type", "unsafe", "use", "where", "while",
    // Reserved keywords.
    "abstract", "become", "box", "do", "final", "gen", "macro",
    "override", "priv", "try", "typeof", "unsized", "virtual",
    "yield",
];

pub fn convert_to_module_name(project_name: &str) -> String {
    project_name.replace('-', "_")
}

pub fn is_rust_keyword(project_name: &str) -> bool {
    let module_name = convert_to_module_name(project_name);
    RUST_KEYWORDS.contains(&module_name.as_str())
}

pub fn validate_project_name(project_name: &str) -> bool {
    static REGEX_NAME: OnceLock<Regex> = OnceLock::new();

    let regex = REGEX_NAME.get_or_init(|| {
        // Start with a lowercase ASCII letter, followed by lowercase
        // ASCII letters, digits, hyphens, or underscores.
        Regex::new(r"^[a-z][a-z0-9_-]*$")
            .expect("the project name regex should be valid")
    });

    regex.is_match(project_name) && !is_rust_keyword(project_name)
}

pub fn resolve_and_validate_destination(
    base_path: &Utf8PathBuf,
    project_name: &str,
) -> anyhow::Result<Utf8PathBuf> {
    let destination = base_path.join(project_name);

    if destination.exists() {
        ensure!(
            !destination.is_file(),
            "The destination {destination} is a file. Cannot create the project."
        );

        let mut entries = std::fs::read_dir(destination.as_std_path())
            .with_context(|| {
                format!("Failed to read directory {destination}")
            })?;

        ensure!(
            entries.next().is_none(),
            "The destination {destination} is not empty. Cannot create the project."
        );
    }

    Ok(destination)
}