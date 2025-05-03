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

use std::{fs::copy, path::Path, slice::Iter};
use worktrace_build::{
    changelog::ChangelogGenerator, license::LicenseNotationGenerator,
    packages::crate_root, proto::update_proto_dir,
};

fn main() -> std::io::Result<()> {
    let root = crate_root().unwrap();
    let children = ["worktrace-build"];
    update_changelog(&root, children.iter());
    update_assets(&root, children.iter())?;
    update_proto_dir(&root.join("proto"))?;
    update_license_notation(&root, children.iter())?;
    Ok(())
}

fn update_changelog(root: &Path, children: Iter<impl AsRef<str>>) {
    if let Ok(generator) = ChangelogGenerator::cargo(&root) {
        generator.update().ok();
    }
    for child in children {
        let path = root.join(child.as_ref());
        if let Ok(generator) = ChangelogGenerator::cargo(&path) {
            generator.update().ok();
        }
    }
}

fn update_assets(
    root: &Path,
    children: Iter<impl AsRef<str>>,
) -> std::io::Result<()> {
    let files = ["LICENSE", "CONTRIBUTORS.yaml"];
    let children = children
        .map(|child| root.join(child.as_ref()))
        .filter(|child| child.exists());
    for child in children {
        for file in files.iter() {
            copy(root.join(file), child.join(file))?;
        }
    }
    Ok(())
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
