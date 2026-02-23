use anstyle::{AnsiColor, Effects, Style};

use crate::store::FileStatus;

// --- Style constants ---

pub const STYLE_HEADER: Style = Style::new().effects(Effects::BOLD);
pub const STYLE_TABLE_HEADER: Style = Style::new().effects(Effects::BOLD);
pub const STYLE_DIM: Style = Style::new().effects(Effects::DIMMED);
pub const STYLE_OK: Style = Style::new().fg_color(Some(anstyle::Color::Ansi(AnsiColor::Green)));
pub const STYLE_WARN: Style = Style::new().fg_color(Some(anstyle::Color::Ansi(AnsiColor::Yellow)));
pub const STYLE_ERROR: Style = Style::new().fg_color(Some(anstyle::Color::Ansi(AnsiColor::Red)));
pub const STYLE_ERROR_BOLD: Style = Style::new()
    .fg_color(Some(anstyle::Color::Ansi(AnsiColor::Red)))
    .effects(Effects::BOLD);
pub const STYLE_INFO: Style = Style::new().fg_color(Some(anstyle::Color::Ansi(AnsiColor::Cyan)));
pub const STYLE_MARKER: Style = Style::new()
    .fg_color(Some(anstyle::Color::Ansi(AnsiColor::Green)))
    .effects(Effects::BOLD);

/// セクションヘッダーのルーラー全体幅
const SECTION_WIDTH: usize = 50;

// --- Helper functions ---

/// Apply a style to text, returning a string with ANSI escape codes.
pub fn styled(style: Style, text: &str) -> String {
    format!("{style}{text}{style:#}")
}

/// ルーラー付きセクションヘッダーを生成する。
/// `── Title ──────────────────` 形式で、タイトルは Bold、罫線は Dim。
pub fn section_header(title: &str) -> String {
    let prefix = "\u{2500}\u{2500} ";
    let suffix_start = format!("{prefix}{title} ");
    let remaining = SECTION_WIDTH.saturating_sub(suffix_start.len());
    let trail = "\u{2500}".repeat(remaining.max(3));
    format!(
        "{} {} {}",
        styled(STYLE_DIM, "\u{2500}\u{2500}"),
        styled(STYLE_HEADER, title),
        styled(STYLE_DIM, &trail),
    )
}

/// Return the appropriate style for a file status value.
pub fn status_style(status: &FileStatus) -> Style {
    match status {
        FileStatus::Ok => STYLE_OK,
        FileStatus::Missing | FileStatus::MissingStore => STYLE_ERROR,
        FileStatus::Error => STYLE_ERROR_BOLD,
        FileStatus::Modified | FileStatus::NotLink | FileStatus::WrongLink => STYLE_WARN,
        FileStatus::StoreOnly => STYLE_DIM,
    }
}

/// Return the appropriate style for a repository type value.
pub fn repo_type_style(repo_type: &str) -> Style {
    match repo_type {
        "bare" => STYLE_INFO,
        "NOT_FOUND" => STYLE_ERROR,
        _ => Style::new(),
    }
}

// --- StyledCell ---

/// A cell that holds both plain text (for width calculation) and styled text (for display).
pub struct StyledCell {
    pub plain: String,
    pub styled: String,
}

impl StyledCell {
    /// Create a styled cell.
    pub fn new(text: impl Into<String>, style: Style) -> Self {
        let plain = text.into();
        let styled_text = styled(style, &plain);
        StyledCell {
            plain,
            styled: styled_text,
        }
    }

    /// Create a plain (unstyled) cell.
    pub fn plain(text: impl Into<String>) -> Self {
        let plain = text.into();
        StyledCell {
            styled: plain.clone(),
            plain,
        }
    }
}
