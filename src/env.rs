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
    env::{current_dir, set_var},
    fs::read_to_string,
};

pub mod keys {
    pub const WEBAPP_HOST: &str = "WEBAPP_HOST";
}

pub unsafe fn load_dotenv() -> std::io::Result<()> {
    read_to_string(current_dir()?.join(".env"))?
        .lines()
        .map_while(|line| parse_dotenv_line(line).ok())
        .for_each(|(key, value)| unsafe { set_var(key, value) });
    Ok(())
}

pub fn parse_dotenv_line(line: &str) -> Result<(&str, &str), DotenvLineError> {
    match line.trim() {
        line if line.is_empty() => Err(DotenvLineError::Empty),
        line if line.starts_with('#') => Err(DotenvLineError::Commented),
        line => match line.split_once('=') {
            Some((key, value)) => Ok((key, value)),
            None => Err(DotenvLineError::SeparatorNotFound(line)),
        },
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DotenvLineError<'a> {
    #[error("empty line")]
    Empty,

    #[error("commented line")]
    Commented,

    #[error("separator not found in: {0}")]
    SeparatorNotFound(&'a str),
}
