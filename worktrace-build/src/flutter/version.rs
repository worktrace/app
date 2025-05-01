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

use lazy_regex::regex_captures;
use semver::Version;
use std::process::Command;

pub fn platform_flutter_version()
-> Result<(Version, String), FlutterVersionError> {
    let result = Command::new("flutter").arg("--version").output()?;
    if !result.status.success() {
        let stderr = String::from_utf8_lossy(&result.stderr).to_string();
        return Err(FlutterVersionError::DetectFailed(stderr));
    }
    let output = String::from_utf8_lossy(&result.stdout).to_string();
    let version = parse_flutter_version(&output)
        .ok_or_else(|| FlutterVersionError::Format(output))?;
    if !is_flutter_channel(&version.1) {
        return Err(FlutterVersionError::Channel(version.1));
    }
    Ok((Version::parse(&version.0)?, version.1))
}

pub fn parse_flutter_version(output: &str) -> Option<(String, String)> {
    let (_, version, channel) = regex_captures!(
        r"^Flutter (\d+\.\d+\.\d+(?:-\w+)?) • channel (\w+)",
        &output
    )?;
    Some((version.to_string(), channel.to_string()))
}

pub fn is_flutter_channel(raw: &str) -> bool {
    raw == "stable" || raw == "beta" || raw == "dev" || raw == "master"
}

#[derive(Debug, thiserror::Error)]
pub enum FlutterVersionError {
    #[error("cannot detect flutter version: {0}")]
    DetectFailed(String),

    #[error("cannot find flutter in PATH: {0}")]
    NotFound(#[from] std::io::Error),

    #[error("unsupported format: {0}")]
    Format(String),

    #[error("invalid semver format: {0}")]
    Semver(#[from] semver::Error),

    #[error("not a flutter channel: {0}")]
    Channel(String),
}

#[cfg(test)]
mod test {
    use super::parse_flutter_version;

    #[test]
    fn test_parse_flutter_version() {
        let output = [
            "Flutter 3.29.3 • channel stable • git@github.com:flutter/flutter",
            "Framework • revision ea121f8859 (3 weeks ago) • 2025-04-11",
            "Engine • revision cf56914b32",
            "Tools • Dart 3.7.2 • DevTools 2.42.3",
        ]
        .iter()
        .collect::<Vec<String>>()
        .join("\n");
        let (version, channel) = parse_flutter_version(&output);
        assert_eq!(version, "3.29.3");
        assert_eq!(channel, "stable");
    }

    #[test]
    fn test_parse_flutter_version_beta() {
        let output = "Flutter 3.29.3-beta.123 • channel beta • git@...";
        let (version, channel) = parse_flutter_version(&output);
        assert_eq!(version, "3.29.3-beta.123");
        assert_eq!(channel, "beta");
    }
}
