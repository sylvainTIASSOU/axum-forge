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
    /// Path relative to the generated project directory.
    pub path: Utf8PathBuf,

    /// File content after template rendering, when applicable.
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

        // Templates generate Rust, TOML, and Markdown, not HTML.
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

                output.push(render_file(
                    relative_path,
                    source,
                    environment,
                    project_name,
                    crate_name,
                )?);
            }
        }
    }
    Ok(())
}

fn render_file(
    relative_path: &Utf8Path,
    source: &str,
    environment: &Environment,
    project_name: &str,
    crate_name: &str,
) -> Result<RenderedFile> {
    let template_path = relative_path.as_str().strip_suffix(".j2");

    let output_path = template_path
        .map(Utf8PathBuf::from)
        .unwrap_or_else(|| relative_path.to_path_buf());

    let content = if template_path.is_some() {
        environment
            .template_from_str(source)
            .with_context(|| format!("Invalid template: {relative_path}"))?
            .render(context! {
                project_name => project_name,
                crate_name => crate_name,
            })
            .with_context(|| format!("Failed to render template: {relative_path}"))?
    } else {
        // Non-Jinja files are copied verbatim.
        source.to_owned()
    };

    Ok(RenderedFile {
        path: output_path,
        content,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_the_standard_profile() {
        let files = load_standard().render("mon-api", "mon_api").unwrap();

        let cargo_toml = files
            .iter()
            .find(|file| file.path.as_str() == "Cargo.toml")
            .expect("Cargo.toml should be generated");

        assert!(cargo_toml.content.contains("name = \"mon-api\""));

        assert!(
            files
                .iter()
                .any(|file| { file.path.as_str() == "src/bin/main.rs" })
        );

        assert!(
            files
                .iter()
                .all(|file| { !file.path.as_str().ends_with(".j2") })
        );
    }

    #[test]
    fn renders_jinja_template_files() {
        let file = render_file(
            Utf8Path::new("example.txt.j2"),
            "Project: {{ project_name }}",
            &Environment::new(),
            "mon-api",
            "mon_api",
        )
        .unwrap();

        assert_eq!(file.path.as_str(), "example.txt");
        assert_eq!(file.content, "Project: mon-api");
    }

    #[test]
    fn copies_non_template_files_verbatim() {
        let source = "{{ project_name }} {{ not_a_template }}";

        let file = render_file(
            Utf8Path::new("example.txt"),
            source,
            &Environment::new(),
            "mon-api",
            "mon_api",
        )
        .unwrap();

        assert_eq!(file.path.as_str(), "example.txt");
        assert_eq!(file.content, source);
    }
}
