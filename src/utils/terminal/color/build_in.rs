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

pub struct BasicTerminalColor<'a> {
    pub foreground: &'a str,
    pub background: &'a str,
}

impl TerminalColorUsages for BasicTerminalColor<'_> {
    #[inline]
    fn foreground(&self) -> impl Display {
        self.foreground
    }

    #[inline]
    fn background(&self) -> impl Display {
        self.background
    }
}

pub const BLACK: BasicTerminalColor = BasicTerminalColor {
    foreground: "\x1b[30m",
    background: "\x1b[40m",
};

pub const RED: BasicTerminalColor = BasicTerminalColor {
    foreground: "\x1b[31m",
    background: "\x1b[41m",
};

pub const GREEN: BasicTerminalColor = BasicTerminalColor {
    foreground: "\x1b[32m",
    background: "\x1b[42m",
};

pub const YELLOW: BasicTerminalColor = BasicTerminalColor {
    foreground: "\x1b[33m",
    background: "\x1b[43m",
};

pub const BLUE: BasicTerminalColor = BasicTerminalColor {
    foreground: "\x1b[34m",
    background: "\x1b[44m",
};

pub const MAGENTA: BasicTerminalColor = BasicTerminalColor {
    foreground: "\x1b[35m",
    background: "\x1b[45m",
};

pub const CYAN: BasicTerminalColor = BasicTerminalColor {
    foreground: "\x1b[36m",
    background: "\x1b[46m",
};

pub const WHITE: BasicTerminalColor = BasicTerminalColor {
    foreground: "\x1b[37m",
    background: "\x1b[47m",
};

pub const HI_BLACK: BasicTerminalColor = BasicTerminalColor {
    foreground: "\x1b[90m",
    background: "\x1b[100m",
};

pub const HI_RED: BasicTerminalColor = BasicTerminalColor {
    foreground: "\x1b[91m",
    background: "\x1b[101m",
};

pub const HI_GREEN: BasicTerminalColor = BasicTerminalColor {
    foreground: "\x1b[92m",
    background: "\x1b[102m",
};

pub const HI_YELLOW: BasicTerminalColor = BasicTerminalColor {
    foreground: "\x1b[93m",
    background: "\x1b[103m",
};

pub const HI_BLUE: BasicTerminalColor = BasicTerminalColor {
    foreground: "\x1b[94m",
    background: "\x1b[104m",
};

pub const HI_MAGENTA: BasicTerminalColor = BasicTerminalColor {
    foreground: "\x1b[95m",
    background: "\x1b[105m",
};

pub const HI_CYAN: BasicTerminalColor = BasicTerminalColor {
    foreground: "\x1b[96m",
    background: "\x1b[106m",
};

pub const HI_WHITE: BasicTerminalColor = BasicTerminalColor {
    foreground: "\x1b[97m",
    background: "\x1b[107m",
};
