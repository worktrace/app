// Copyright (c) 2025-present All contributors of this project
// All contributors are listed in CONTRIBUTORS.yaml at repository root.
//
// AutoStories is licensed under Mulan PSL v2.
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
    path::PathBuf,
};
use worktrace_generator::{fmt::eol, license::LicenseNotationGenerator};

fn main() -> std::io::Result<()> {
    let root = PathBuf::from(var("CARGO_MANIFEST_DIR").unwrap());
    update_cargo_license(&root).ok();
    update_child_repo_license_files(&root);
    Ok(())
}

const GENERATOR_CHILD_REPO_PATH: &str = "gen";

fn update_child_repo_license_files(root: &PathBuf) {
    let generator = root.join(GENERATOR_CHILD_REPO_PATH);
    ["LICENSE", "CONTRIBUTORS.yaml"].iter().for_each(|name| {
        copy(root.join(name), generator.join(name)).ok();
    });
}

fn update_cargo_license(root: &PathBuf) -> std::io::Result<()> {
    let generator = LicenseNotationGenerator {
        template: read_to_string(root.join(".license.txt"))?,
        comment: "上述开源协议注释乃程序自动生成，请勿编辑".into(),
        separator: "=== Auto generated, DO NOT EDIT ABOVE ===".into(),
        eol: eol::LF,
    };
    generator.update_dir(&root.join(GENERATOR_CHILD_REPO_PATH).join("src"))?;
    generator.update_dir(&root.join("src"))?;
    generator.update_file(&root.join("build.rs"))
}
