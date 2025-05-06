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

use super::TerminalColorUsages;
use std::fmt::Display;

pub struct TerminalColor {
    pub foreground: String,
    pub background: String,
}

impl TerminalColor {
    pub fn from_code(code: u8) -> Self {
        Self {
            foreground: format!("\x1b[38;5;{}m", code),
            background: format!("\x1b[48;5;{}m", code),
        }
    }

    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self {
            foreground: format!("\x1b[38;2;{};{};{}m", red, green, blue),
            background: format!("\x1b[48;2;{};{};{}m", red, green, blue),
        }
    }
}

impl TerminalColorUsages for TerminalColor {
    #[inline]
    fn foreground(&self) -> impl Display {
        &self.foreground
    }

    #[inline]
    fn background(&self) -> impl Display {
        &self.background
    }
}
