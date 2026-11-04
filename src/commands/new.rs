use std::fs;
use crate::{cli, templates};
use anyhow::{bail, ensure, Context, Result};
use camino::{Utf8Path, Utf8PathBuf};
use crate::cli::NewArgs;
use crate::templates::RenderedFile;

fn write_file(
    destination: &Utf8Path,
    files: &[RenderedFile],
) -> Result<usize> {
    fs::create_dir_all(destination.as_std_path()).with_context(|| {
        format!("Failded to create project directory {}", destination)
    })?;

    for file in files {
        let output_path = destination.join(&file.path);

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent.as_std_path()).with_context(|| {
                format!("Faild to create directory {}", parent)
            })?;
        }
        fs::write(output_path.as_std_path(), &file.content).with_context(|| {
            format!("Faild to write file {}", output_path)
        })?;
    }
    Ok(files.len())
}

fn print_summary(
    project_name: &str,
    profile_name: &str,
    destination: &Utf8PathBuf,
    files_created: usize,
) {
    println!("Project `{project_name}` created successfully.");
    println!();
    println!("  Profile: {profile_name}");
    println!("  Location: {destination}");
    println!("  Files created: {files_created}");
    println!();
    println!("Next steps:");
    println!("  cd \"{destination}\"");
    println!("  cargo run");
}

pub fn run_new(args: NewArgs) -> anyhow::Result<()> {
    if args.name.is_empty() {
        bail!("`Project name` cannot be empty");
    }
    ensure!(
        cli::validate_project_name(&args.name),
        "The project name is invalide: '{}'. ",
        args.name
    );
    let new_name = cli::convertir_in_module_name(&args.name);
    let destination = cli::resolve_and_validate_destination(&args.path, &new_name)?;
    let profile = templates::load_standard();

    let files = profile.render(new_name.as_str(), &new_name)?;

    let files_created = write_file(&destination, &files)?;

    print_summary(
        new_name.as_str(),
        profile.name(),
        &destination,
        files_created,
    );


    Ok(())
}
