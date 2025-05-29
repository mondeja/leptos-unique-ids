use std::path::Path;

fn extract_version_from_cargo_toml_content(content: &str, match_: &str) -> Option<String> {
    for line in content.lines() {
        if line.starts_with(match_) {
            return line.split('"').nth(1).map(|s| s.to_string());
        }
    }
    None
}

fn extract_versions_from_content(content: &str, matcher: &str) -> Vec<String> {
    let mut versions = Vec::new();
    for line in content.lines() {
        if line.contains(matcher) {
            if let Some(version) = line.split(matcher).nth(1).unwrap().split('"').nth(1) {
                versions.push(version.to_string());
            }
        }
    }
    versions
}

/// Lint directories iterator.
struct LintDirectories(std::fs::ReadDir);

impl LintDirectories {
    fn new() -> Self {
        let lints_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("lints");
        let candidates = std::fs::read_dir(lints_dir).expect("Failed to read lints directory");
        Self(candidates)
    }
}

impl Iterator for LintDirectories {
    type Item = (String, std::path::PathBuf);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(Ok(entry)) = self.0.next() {
            let path = entry.path();
            if !path.is_dir() {
                return self.next();
            }
            let lint_name = path
                .file_name()
                .and_then(|name| name.to_str())
                .expect("Failed to get lint name");
            if lint_name == "helpers" || lint_name == "src" {
                return self.next();
            }
            Some((lint_name.to_string(), path))
        } else {
            None
        }
    }
}

fn get_main_cargo_toml_version() -> String {
    let main_cargo_toml_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("Cargo.toml");
    let main_cargo_toml_content =
        std::fs::read_to_string(&main_cargo_toml_path).expect("Failed to read Cargo.toml");
    extract_version_from_cargo_toml_content(&main_cargo_toml_content, "version =")
        .expect("Failed to extract version from Cargo.toml")
}

/// The README.md file contains an example of how to install the crate.
///
/// This test checks that the version in the README.md file matches the version
/// in the Cargo.toml file.
#[test]
fn crate_version_is_updated_in_readme() {
    let raw_version = get_main_cargo_toml_version();

    // from X.Y.Z to X.Y
    let expected_version = raw_version
        .split('.')
        .take(2)
        .collect::<Vec<&str>>()
        .join(".");

    // README is generated from src/lib.rs
    let src_lib_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("src")
        .join("lib.rs");
    let src_lib_content =
        std::fs::read_to_string(&src_lib_path).expect("Failed to read src/lib.rs");

    let src_lib_versions =
        extract_versions_from_content(&src_lib_content, "//! leptos-unique-ids =");
    assert_eq!(
        src_lib_versions.len(),
        1,
        "Expected exactly one version in src/lib.rs, found {src_lib_versions:?}",
    );

    for version in src_lib_versions {
        assert_eq!(
            version, expected_version,
            "Version in src/lib.rs ({version}) does not match Cargo.toml version ({expected_version})",
        );
    }
}

/// The README.md file contains an example of how to use the lints,
/// which includes the tag version of the library.
///
/// This test checks that the tag version in the README.md file
/// matches the version in the Cargo.toml file of the lint library.
#[test]
fn lints_library_tag_version_is_updated_in_readme() {
    let expected_version = get_main_cargo_toml_version();

    // README is generated from src/lib.rs
    let src_lib_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("src")
        .join("lib.rs");
    let src_lib_content =
        std::fs::read_to_string(&src_lib_path).expect("Failed to read src/lib.rs");

    let src_lib_versions = extract_versions_from_content(&src_lib_content, "tag =");
    assert_eq!(
        src_lib_versions.len(),
        1,
        "Expected exactly one tag version in src/lib.rs, found {src_lib_versions:?}",
    );

    for tag_version in src_lib_versions {
        assert_eq!(
            tag_version,
            format!("v{expected_version}"),
            "Tag version in src/lib.rs ({tag_version}) does not match Cargo.toml version ({expected_version})",
        );
    }
}

/// Lints declare a documentation comment inside the macro provided by dylint_linting.
///
/// These comments are used to generate the READMEs for the lints. This test will
/// fail if the documentation comment is not updated, but will pass on a second
/// run, when the documentation comment is updated.
#[test]
fn lints_readmes_are_updated() {
    fn extract_lint_lib_doc_comment(lint_lib_rs_content: &str) -> Option<String> {
        let mut in_doc_comment = false;
        let mut doc_comment = String::new();

        for line in lint_lib_rs_content.lines() {
            if line.trim().starts_with("///") {
                if !in_doc_comment {
                    in_doc_comment = true;
                }

                let new_line = &line.trim().trim_start_matches("///");
                if new_line.is_empty() {
                    doc_comment.push('\n');
                } else {
                    doc_comment.push_str(&new_line[1..]);
                    doc_comment.push('\n');
                }
            } else if in_doc_comment {
                break; // End of the documentation comment
            }
        }

        if doc_comment.is_empty() {
            None
        } else {
            Some(doc_comment.trim().to_string())
        }
    }

    let mut updated = vec![];
    for (lint_name, path) in LintDirectories::new() {
        let lint_lib_rs_path = path.join("src").join("lib.rs");
        let readme_path = path.join("README.md");
        let readme_content = std::fs::read_to_string(&readme_path)
            .unwrap_or_else(|_| panic!("Failed to read lib/{lint_name}/README.md",));
        let lint_lib_rs_content = std::fs::read_to_string(&lint_lib_rs_path)
            .unwrap_or_else(|_| panic!("Failed to read lib/{lint_name}/src/lib.rs",));

        let doc_comment = extract_lint_lib_doc_comment(&lint_lib_rs_content).unwrap_or_else(|| {
            panic!("Failed to extract documentation comment from lib/{lint_name}/src/lib.rs",)
        });

        let warning_comment = "<!-- WARNING: This file has been autogenerated. Don't edit it!\n\
            Instead, edit the documentation comment in the lint's src/lib.rs file. \
            -->";
        let expected_readme_content =
            format!("{warning_comment}\n\n# {lint_name}\n\n{doc_comment}\n",);

        if readme_content != expected_readme_content {
            std::fs::write(&readme_path, &expected_readme_content)
                .unwrap_or_else(|_| panic!("Failed to write lib/{lint_name}/README.md",));
            updated.push(lint_name);
        }
    }

    let in_ci = std::env::var("CI").is_ok();
    let message = if in_ci {
        format!(
            "The next lints READMEs are not updated: {updated:?}. \
             Run `cargo test -p tests` locally to update them and commit the changes."
        )
    } else {
        format!(
            "The next lints READMEs have been updated: {updated:?}. \
             If you run again this test, it should pass."
        )
    };

    assert!(updated.is_empty(), "{message}");
}

/// Ensure that each lint has a help link pointing to its README.md file.
#[test]
fn lints_have_help_link() {
    for (lint_name, path) in LintDirectories::new() {
        let lint_lib_rs_path = path.join("src").join("lib.rs");
        let lint_lib_rs_content = std::fs::read_to_string(&lint_lib_rs_path)
            .unwrap_or_else(|_| panic!("Failed to read lib/{lint_name}/src/lib.rs",));

        assert!(
            lint_lib_rs_content.contains("for further information visit"),
            "Lint help does not contains the sentence \"for further information visit\" at file {}.",
            lint_lib_rs_path.display(),
        );

        let expected_link = format!(
            "https://github.com/mondeja/leptos-unique-ids/tree/main/lints/{lint_name}#readme"
        );
        assert!(
            lint_lib_rs_content.contains(&expected_link),
            "Lint help does not contains the link to README.md at file {}. Expected link: {}",
            lint_lib_rs_path.display(),
            expected_link,
        );
    }
}
