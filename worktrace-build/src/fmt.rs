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

use std::path::Path;

pub mod eol {
    pub const LF: &str = "\n";
    pub const CRLF: &str = "\r\n";
}

pub fn add_line_before(raw: impl AsRef<str>, eol: &str) -> String {
    let mut buffer = String::from(eol);
    buffer.push_str(raw.as_ref());
    buffer
}

pub fn rust_line_comment(raw: impl AsRef<str>) -> String {
    let mut buffer = String::new();
    raw.as_ref()
        .split('\n')
        .for_each(|line| match line.trim().is_empty() {
            true => buffer.push_str("//\n"),
            false => {
                buffer.push_str("// ");
                buffer.push_str(line);
                buffer.push('\n');
            }
        });
    buffer
}

pub fn is_rust_file(path: &Path) -> bool {
    path.is_file()
        && path
            .extension()
            .and_then(|ext| ext.to_str())
            .map_or(false, |ext| ext == "rs")
}
