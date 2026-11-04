use anyhow::{Context, Result};
use camino::{Utf8Path, Utf8PathBuf};
use include_dir::{Dir, DirEntry, include_dir};
use minijinja::{AutoEscape, Environment, context};

static STANDARD_TEMPLATE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/standard");

pub struct TemplateProfile {
    name: &'static str,
    directory: &'static Dir<'static>,
}

pub struct RenderedFile {
    /// relatif path in the projet generated
    pub path: Utf8PathBuf,

    /// content of file after rendered
    pub content: String,
}

pub fn load_standard() -> TemplateProfile {
    TemplateProfile {
        name: "standard",
        directory: &STANDARD_TEMPLATE,
    }
}

impl TemplateProfile {
    pub fn name(&self) -> &str {
        self.name
    }

    pub fn render(&self, project_name: &str, crate_name: &str) -> Result<Vec<RenderedFile>> {
        let mut environment = Environment::new();

        //  the template produit rust, TOML and Markdown, not html
        environment.set_auto_escape_callback(|_| AutoEscape::None);
        let mut files = Vec::new();

        render_directory(
            self.directory,
            &environment,
            project_name,
            crate_name,
            &mut files,
        )?;
        Ok(files)
    }
}

fn render_directory(
    directory: &Dir<'_>,
    environment: &Environment,
    project_name: &str,
    crate_name: &str,
    output: &mut Vec<RenderedFile>,
) -> Result<()> {
    for entry in directory.entries() {
        match entry {
            DirEntry::Dir(subdirectory) => {
                render_directory(subdirectory, environment, project_name, crate_name, output)?;
            }
            DirEntry::File(file) => {
                let relative_path = Utf8Path::from_path(file.path()).with_context(|| {
                    format!("Template path is not valid UTF-8: {:?}", file.path())
                })?;

                let source = file.contents_utf8().with_context(|| {
                    format!("Template file is not valid UTF-8: {:?}", relative_path)
                })?;
                let content = environment
                    .template_from_str(source)
                    .with_context(|| format!("Invalid template: {}", relative_path))?
                    .render(context! {
                        project_name => project_name,
                        crate_name => crate_name,
                    })
                    .with_context(|| format!("Failded to render template: {}", relative_path))?;

                let output_path = relative_path
                    .as_str()
                    .strip_suffix(".j2")
                    .map(Utf8PathBuf::from)
                    .unwrap_or_else(|| relative_path.to_path_buf());

                output.push(RenderedFile {
                    path: output_path,
                    content,
                });
            }
        }
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_the_standard_profile() {
        let files = load_standard()
            .render("mon-api", "mon_api")
            .unwrap();

        let cargo_toml = files
            .iter()
            .find(|file| file.path == Utf8PathBuf::from("Cargo.toml"))
            .expect("Cargo.toml should be generated");

        assert!(cargo_toml.content.contains("name = \"mon-api\""));

        assert!(files.iter().any(|file| {
            file.path == Utf8PathBuf::from("src/main.rs")
        }));

        assert!(files.iter().all(|file| {
            !file.path.as_str().ends_with(".j2")
        }));
    }
}