use crate::cli;
use anyhow::{ bail, ensure};
use camino::Utf8PathBuf;

pub fn run_new(name: &str, path: &Utf8PathBuf) -> anyhow::Result<()> {
    if name.is_empty() {
        bail!("`Project name` cannot be empty");
    }
    ensure!(
        cli::validate_project_name(name),
        "The project name is invalide: '{}'. ",
        name
    );
    let new_name = cli::convertir_in_module_name(&name);
    let destination = cli::resolve_and_validate_destination(path, &new_name)?;
    println!(
        " project destination: {} and name: {}",
        destination, new_name
    );

    Ok(())
}
