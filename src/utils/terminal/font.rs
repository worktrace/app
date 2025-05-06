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

pub struct TerminalFont<'a> {
    pub code: &'a str,
    pub reset_code: &'a str,
}

impl TerminalFont<'_> {
    #[inline]
    pub fn wrap(&self, raw: impl AsRef<str>) -> String {
        format!("{}{}{}", self.code, raw.as_ref(), self.reset_code)
    }

    pub fn render(&self, raw: impl AsRef<str>) -> String {
        let raw = raw.as_ref();
        match raw.split_once(&self.reset_code) {
            None => self.wrap(raw),
            Some((before, after)) => format!(
                "{}{}{}{}{}",
                self.code, before, self.code, after, self.reset_code
            ),
        }
    }
}

pub const BOLD: TerminalFont = TerminalFont {
    code: "\x1b[1m",
    reset_code: "\x1b[22m",
};

pub const FAINT: TerminalFont = TerminalFont {
    code: "\x1b[2m",
    reset_code: "\x1b[22m",
};

pub const ITALIC: TerminalFont = TerminalFont {
    code: "\x1b[3m",
    reset_code: "\x1b[23m",
};

pub const UNDERLINE: TerminalFont = TerminalFont {
    code: "\x1b[4m",
    reset_code: "\x1b[24m",
};

pub const BLINK: TerminalFont = TerminalFont {
    code: "\x1b[5m",
    reset_code: "\x1b[25m",
};

pub const BLINK_RAPID: TerminalFont = TerminalFont {
    code: "\x1b[6m",
    reset_code: "\x1b[25m",
};

pub const NEGATIVE: TerminalFont = TerminalFont {
    code: "\x1b[7m",
    reset_code: "\x1b[27m",
};

pub const CONCEAL: TerminalFont = TerminalFont {
    code: "\x1b[8m",
    reset_code: "\x1b[28m",
};

pub const STRIKETHROUGH: TerminalFont = TerminalFont {
    code: "\x1b[9m",
    reset_code: "\x1b[29m",
};

pub const DOUBLE_UNDERLINE: TerminalFont = TerminalFont {
    code: "\x1b[21m",
    reset_code: "\x1b[24m",
};

pub trait WrapTerminalFont: AsRef<str> {
    fn bold(&self) -> String {
        BOLD.wrap(self)
    }

    fn faint(&self) -> String {
        FAINT.wrap(self)
    }

    fn italic(&self) -> String {
        ITALIC.wrap(self)
    }

    fn underline(&self) -> String {
        UNDERLINE.wrap(self)
    }

    fn blink(&self) -> String {
        BLINK.wrap(self)
    }

    fn blink_rapid(&self) -> String {
        BLINK_RAPID.wrap(self)
    }

    fn negative(&self) -> String {
        NEGATIVE.wrap(self)
    }

    fn conceal(&self) -> String {
        CONCEAL.wrap(self)
    }

    fn strikethrough(&self) -> String {
        STRIKETHROUGH.wrap(self)
    }

    fn double_underline(&self) -> String {
        DOUBLE_UNDERLINE.wrap(self)
    }
}

pub trait RenderTerminalFont: AsRef<str> {
    fn render_bold(&self) -> String {
        BOLD.render(self)
    }

    fn render_faint(&self) -> String {
        FAINT.render(self)
    }

    fn render_italic(&self) -> String {
        ITALIC.render(self)
    }

    fn render_underline(&self) -> String {
        UNDERLINE.render(self)
    }

    fn render_blink(&self) -> String {
        BLINK.render(self)
    }

    fn render_blink_rapid(&self) -> String {
        BLINK_RAPID.render(self)
    }

    fn render_negative(&self) -> String {
        NEGATIVE.render(self)
    }

    fn render_conceal(&self) -> String {
        CONCEAL.render(self)
    }

    fn render_strikethrough(&self) -> String {
        STRIKETHROUGH.render(self)
    }

    fn render_double_underline(&self) -> String {
        DOUBLE_UNDERLINE.render(self)
    }
}
