// Copyright (c) 2025-present All contributors of this project
// All contributors are listed in CONTRIBUTORS.yaml at repository root.
//
// WorkTrace is licensed under Mulan PSL v2.
// You can use this software according to the terms
// and conditions of the Mulan PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//
//   http://license.coscl.org.cn/MulanPSL2
//
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS,
// WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED,
// INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT,
// MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.
//
// 上述开源协议注释乃程序自动生成，请勿编辑
// === Auto generated, DO NOT EDIT ABOVE ===

use std::{
    fs::{ReadDir, read_dir, read_to_string, write},
    path::Path,
    slice::Iter,
};

pub struct LicenseNotationGenerator<'a> {
    pub template: String,
    pub comment: String,
    pub separator: String,
    pub options: LicenseNotationOptions<'a>,
}

impl LicenseNotationGenerator<'_> {
    pub fn cargo(
        root: &Path,
        comment: impl AsRef<str>,
    ) -> std::io::Result<Self> {
        Ok(LicenseNotationGenerator {
            template: read_to_string(root.join(".license.txt"))?,
            comment: comment.as_ref().to_string(),
            separator: "=== Auto generated, DO NOT EDIT ABOVE ===".to_string(),
            options: LicenseNotationOptions::rust(),
        })
    }

    pub fn update_cargo(&self, root: &Path, children: Iter<impl AsRef<str>>) {
        self.update_file(&root.join("build.rs")).ok();
        if let Ok(dir) = read_dir(root.join("src")) {
            self.update_dir(dir)
        }
        for child in children {
            let path = root.join(child.as_ref());
            self.update_file(&path.join("build.rs")).ok();
            if let Ok(dir) = read_dir(path.join("src")) {
                self.update_dir(dir)
            }
        }
    }

    pub fn update_dir(&self, root: ReadDir) {
        root.filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .for_each(|path| {
                if path.is_dir() {
                    self.update_dir(read_dir(path).unwrap());
                } else if self.options.match_extension(&path) {
                    self.update_file(&path).ok();
                };
            });
    }

    pub fn update_file(&self, path: &Path) -> std::io::Result<()> {
        let content = read_to_string(path)?;
        let result = self.update(&content);
        write(path, result)
    }

    pub fn update(&self, raw: &str) -> String {
        let separator = format!(
            "{}{}{}",
            self.options.eol,
            self.options.add_comment(&self.separator),
            self.options.eol,
        );
        format!(
            "{}{}{}{}{}{}{}",
            self.options.add_comment(&self.template),
            self.options.eol,
            self.options.add_comment(&self.comment),
            self.options.eol,
            self.options.add_comment(&self.separator),
            self.options.eol,
            match raw.split_once(&separator) {
                Some((_before, after)) => after.into(),
                None => format!("{}{}", self.options.eol, raw),
            }
        )
    }
}

pub struct LicenseNotationOptions<'a> {
    pub eol: &'a str,
    pub comment_prefix: &'a str,
    pub space_before_comment: bool,
    pub file_extension: &'a str,
}

impl LicenseNotationOptions<'_> {
    pub fn rust() -> Self {
        LicenseNotationOptions {
            eol: "\n",
            comment_prefix: "//",
            space_before_comment: true,
            file_extension: "rs",
        }
    }

    pub fn add_comment(&self, raw: &str) -> String {
        raw.split(self.eol)
            .map(|line| self.add_comment_line(line))
            .collect::<Vec<String>>()
            .join(self.eol)
    }

    pub fn add_comment_line(&self, raw: &str) -> String {
        match raw.is_empty() {
            true => self.comment_prefix.into(),
            false => match self.space_before_comment {
                true => format!("{} {}", self.comment_prefix, raw),
                false => format!("{}{}", self.comment_prefix, raw),
            },
        }
    }

    /// The given path is an existing file and matches the extension.
    pub fn match_extension(&self, path: &Path) -> bool {
        if !path.is_file() {
            return false;
        }
        path.extension()
            .and_then(|ext| ext.to_str())
            .map_or(false, |ext| ext == self.file_extension)
    }
}
