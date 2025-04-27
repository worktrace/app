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
    env::var,
    fs::{copy, read_to_string},
    path::{Path, PathBuf},
    slice::Iter,
};
use worktrace_build::{fmt::eol, license::LicenseNotationGenerator};

fn main() -> std::io::Result<()> {
    let root = PathBuf::from(var("CARGO_MANIFEST_DIR").unwrap());
    update_cargo_license(&root)?;
    update_assets(&root)?;
    Ok(())
}

const WORKTRACE_BUILD: &str = "worktrace-build";

fn update_cargo_license(root: &PathBuf) -> std::io::Result<()> {
    let generator = LicenseNotationGenerator {
        template: read_to_string(root.join(".license.txt"))?,
        comment: "上述开源协议注释乃程序自动生成，请勿编辑".into(),
        separator: "=== Auto generated, DO NOT EDIT ABOVE ===".into(),
        eol: eol::LF,
    };
    generator.update_dir(&root.join(WORKTRACE_BUILD).join("src"))?;
    generator.update_dir(&root.join("src"))?;
    generator.update_file(&root.join("build.rs"))
}

fn update_assets(root: &PathBuf) -> std::io::Result<()> {
    let files = ["LICENSE", "CONTRIBUTORS.yaml"].iter();
    copy_assets(&files, &root, &root.join(WORKTRACE_BUILD))
}

fn copy_assets(
    files: &Iter<impl AsRef<Path>>,
    src: &PathBuf,
    out: &PathBuf,
) -> std::io::Result<()> {
    for name in files.as_slice() {
        copy(src.join(name), out.join(name))?;
    }
    Ok(())
}
