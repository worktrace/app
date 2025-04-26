use crate::fmt::{add_line_before, eol, is_rust_file, rust_line_comment};
use std::{
    fs::{read_dir, read_to_string, write},
    path::{Path, PathBuf},
};

pub fn update_cargo_license(root: &PathBuf, comment: impl AsRef<str>) -> std::io::Result<()> {
    let generator = LicenseNotationGenerator {
        template: read_to_string(root.join(".license.txt"))?,
        comment: comment.as_ref().into(),
        separator: "=== Auto generated, DO NOT EDIT ABOVE ===".into(),
        eol: eol::LF,
    };
    generator.update_dir(&root.join("src"))?;
    generator.update_file(&root.join("build.rs"))
}

pub struct LicenseNotationGenerator {
    pub template: String,
    pub comment: String,
    pub separator: String,
    pub eol: &'static str,
}

impl LicenseNotationGenerator {
    pub fn update_dir(&self, path: &Path) -> std::io::Result<()> {
        self.commented().update_dir_bare(path)
    }

    pub fn update_file(&self, path: &Path) -> std::io::Result<()> {
        self.commented().update_file_bare(path)
    }

    pub fn update_content(&self, raw: String) -> String {
        self.commented().update_content_bare(raw)
    }

    fn update_dir_bare(&self, path: &Path) -> std::io::Result<()> {
        let _ = read_dir(path)?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .map(|path| match path.as_path() {
                path if path.is_dir() => self.update_dir_bare(path),
                path if is_rust_file(path) => self.update_file_bare(path),
                _ => Ok(()),
            })
            .collect::<Vec<_>>();
        Ok(())
    }

    fn update_file_bare(&self, path: &Path) -> std::io::Result<()> {
        let content = read_to_string(path)?;
        let result = self.update_content_bare(content);
        write(path, result)
    }

    fn update_content_bare(&self, raw: String) -> String {
        let separator = add_line_before(&self.separator, self.eol);
        let rest = match raw.split_once(&separator) {
            Some((_before, after)) => after,
            None => &add_line_before(&raw, self.eol),
        };
        let parts = [&self.template, &self.comment, &self.separator, rest];
        parts.join("")
    }

    fn commented(&self) -> LicenseNotationGenerator {
        LicenseNotationGenerator {
            template: rust_line_comment(&self.template),
            comment: rust_line_comment(&self.comment),
            separator: rust_line_comment(&self.separator),
            eol: self.eol,
        }
    }
}
