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
    fs::{File, write},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use crate::packages::{CargoManifestError, crate_version};

pub struct ChangelogGenerator {
    pub version: String,
    pub source: PathBuf,
    pub target: PathBuf,
}

impl ChangelogGenerator {
    pub fn default(root: &Path, version: String) -> Self {
        Self {
            version,
            source: root.join("CHANGELOG.md"),
            target: root.join(".changelog.md"),
        }
    }

    pub fn named(root: &Path, name: &str, version: String) -> Self {
        Self {
            version,
            source: root.join(name),
            target: root.join(format!(".{}", name.to_lowercase())),
        }
    }

    pub fn cargo(root: &Path) -> Result<Self, CargoManifestError> {
        let version = crate_version(root)?;
        Ok(Self::default(root, version))
    }

    pub fn cargo_named(root: &Path) -> Result<Self, CargoManifestError> {
        let version = crate_version(root)?;
        Ok(Self::named(root, "changelog.rs.md", version))
    }

    pub fn update(&self) -> std::io::Result<()> {
        let reader = BufReader::new(File::open(&self.source)?);
        let title = format!("## {}", self.version);
        let mut inside = false;
        let mut handler = String::from(
            "<!-- Auto generated changelog of current version. -->\n",
        );
        for line in reader.lines() {
            let line = line?;
            match inside {
                false => inside = line == title,
                true => match line {
                    line if line.starts_with("## ") => break,
                    line => {
                        handler.push_str(&line);
                        handler.push('\n');
                    }
                },
            }
        }
        handler = handler.trim().to_string();
        handler.push('\n');
        write(&self.target, handler)?;
        Ok(())
    }
}
