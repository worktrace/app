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
    env::{VarError, var},
    fs::read_to_string,
    path::{Path, PathBuf},
};

pub fn crate_root() -> Result<PathBuf, VarError> {
    Ok(PathBuf::from(var("CARGO_MANIFEST_DIR")?))
}

pub fn crate_version(root: &Path) -> Result<String, CargoManifestError> {
    let content = read_to_string(root.join("Cargo.toml"))?;
    let manifest = toml::from_str::<toml::Value>(&content)?;
    let version = manifest
        .get("package")
        .and_then(|package| package.get("version"))
        .and_then(|version| version.as_str())
        .ok_or(CargoManifestError::Config("package version unspecified"))?;
    Ok(version.to_string())
}

#[derive(Debug, thiserror:: Error)]
pub enum CargoManifestError<'a> {
    #[error("cannot read cargo manifest file: {0}")]
    File(#[from] std::io::Error),

    #[error("cannot parse toml: {0}")]
    Syntax(#[from] toml::de::Error),

    #[error("invalid cargo manifest config: {0}")]
    Config(&'a str),
}
