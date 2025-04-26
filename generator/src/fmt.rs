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
