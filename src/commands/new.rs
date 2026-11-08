use crate::project::{
    convert_to_module_name, is_rust_keyword,
    resolve_and_validate_destination, validate_project_name,
};
use crate::templates::RenderedFile;
use crate::{cli, templates};
use anyhow::{Context, Result, ensure};
use camino::{Utf8Path, Utf8PathBuf};
use std::fs;

fn write_files(
    destination: &Utf8Path,
    files: &[RenderedFile],
) -> Result<usize> {
    fs::create_dir_all(destination.as_std_path()).with_context(|| {
        format!("Failed to create project directory {destination}")
    })?;

    for file in files {
        let output_path = destination.join(&file.path);

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent.as_std_path()).with_context(|| {
                format!("Failed to create directory {parent}")
            })?;
        }

        fs::write(output_path.as_std_path(), &file.content)
            .with_context(|| {
                format!("Failed to write file {output_path}")
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

pub fn run_new(args: cli::NewArgs) -> Result<()> {
    ensure!(
        !is_rust_keyword(&args.name),
        "The project name `{}` is a reserved Rust keyword.",
        args.name
    );

    ensure!(
        validate_project_name(&args.name),
        "Invalid project name `{}`. Project names must start with a \
         lowercase ASCII letter and contain only lowercase letters, \
         digits, hyphens, or underscores.",
        args.name
    );

    // Cargo package names may contain hyphens, but Rust crate names
    // use underscores instead.
    let crate_name = convert_to_module_name(&args.name);

    let destination =
        resolve_and_validate_destination(&args.path, &args.name)?;

    let profile = templates::load_standard();
    let files = profile.render(&args.name, &crate_name)?;
    let files_created = write_files(&destination, &files)?;

    print_summary(
        &args.name,
        profile.name(),
        &destination,
        files_created,
    );

    Ok(())
}