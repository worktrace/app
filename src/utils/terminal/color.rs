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

pub mod reset_code {
    pub const FOREGROUND: &str = "\x1b[39m";
    pub const BACKGROUND: &str = "\x1b[49m";
}

pub struct TerminalColor<'a> {
    pub foreground: &'a str,
    pub background: &'a str,
}

impl TerminalColor<'_> {
    #[inline]
    pub fn wrap_foreground(&self, raw: impl AsRef<str>) -> String {
        format!(
            "{}{}{}",
            self.foreground,
            raw.as_ref(),
            reset_code::FOREGROUND
        )
    }

    #[inline]
    pub fn wrap_background(&self, raw: impl AsRef<str>) -> String {
        format!(
            "{}{}{}",
            self.background,
            raw.as_ref(),
            reset_code::BACKGROUND
        )
    }

    pub fn render_foreground(&self, raw: impl AsRef<str>) -> String {
        match raw.as_ref().split_once(reset_code::FOREGROUND) {
            None => self.wrap_foreground(raw),
            Some((before, after)) => format!(
                "{}{}{}{}{}",
                self.foreground,
                before,
                self.foreground,
                after,
                reset_code::FOREGROUND
            ),
        }
    }

    pub fn render_background(&self, raw: impl AsRef<str>) -> String {
        match raw.as_ref().split_once(reset_code::BACKGROUND) {
            None => self.wrap_background(raw),
            Some((before, after)) => format!(
                "{}{}{}{}{}",
                self.background,
                before,
                self.background,
                after,
                reset_code::BACKGROUND
            ),
        }
    }
}
