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
    fs::copy,
    path::{Path, PathBuf},
    slice::Iter,
};
use worktrace_build::{
    changelog::ChangelogGenerator, license::LicenseNotationGenerator,
    proto::update_proto_dir,
};

fn main() -> std::io::Result<()> {
    let root = PathBuf::from(var("CARGO_MANIFEST_DIR").unwrap());
    let children = ["worktrace-build"];
    update_changelog(&root, children.iter());
    update_license_notation(&root, children.iter())?;

    update_assets(&root).ok(); // Children crates won't included in publish.
    update_proto_dir(&root.join("proto"))?;
    Ok(())
}

const WORKTRACE_BUILD: &str = "worktrace-build";

fn update_changelog(root: &Path, children: Iter<impl AsRef<str>>) {
    for child in children {
        let path = root.join(child.as_ref());
        if let Ok(generator) = ChangelogGenerator::rust(&path) {
            generator.update().ok();
        }
    }
}

fn update_license_notation(
    root: &Path,
    children: Iter<impl AsRef<str>>,
) -> std::io::Result<()> {
    const COMMENT: &str = "上述开源协议注释乃程序自动生成，请勿编辑";
    let generator = LicenseNotationGenerator::cargo(root, COMMENT)?;
    generator.update_cargo(root, children);
    Ok(())
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
