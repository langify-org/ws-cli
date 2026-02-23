use anyhow::Result;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::commands::repos::{WorktreeEntry, parse_worktree_list};
use crate::config::{Config, load_config};
use crate::git::worktree_root;
use crate::store::{ManifestEntry, read_manifest, store_dir};

pub struct AppContext {
    pub config: Config,
    pub current_repo: Option<CurrentRepo>,
    pub current_workspace: Option<CurrentWorkspace>,
}

pub struct CurrentRepo {
    pub name: Option<String>,
    pub root: PathBuf,
    pub is_bare: bool,
    pub worktrees: Vec<WorktreeEntry>,
}

pub struct CurrentWorkspace {
    pub root: PathBuf,
    pub branch: String,
    pub store_dir: Option<PathBuf>,
    pub manifest: Vec<ManifestEntry>,
}

impl AppContext {
    pub fn build() -> Result<Self> {
        let config = load_config()?;
        let current_repo = Self::resolve_current_repo(&config);
        let current_workspace = Self::resolve_current_workspace();

        Ok(AppContext {
            config,
            current_repo,
            current_workspace,
        })
    }

    fn resolve_current_repo(config: &Config) -> Option<CurrentRepo> {
        let root = resolve_repo_root_from_cwd()?;
        let is_bare = root.join(".bare").is_dir();

        let name = config.repos.iter().find_map(|(name, entry)| {
            entry
                .path
                .canonicalize()
                .ok()
                .filter(|p| *p == root)
                .map(|_| name.clone())
        });

        let wt_output = if is_bare {
            Command::new("git")
                .args(["--git-dir", ".bare", "worktree", "list"])
                .current_dir(&root)
                .output()
                .ok()
        } else {
            Command::new("git")
                .args(["worktree", "list"])
                .current_dir(&root)
                .output()
                .ok()
        };

        let worktrees = wt_output
            .filter(|o| o.status.success())
            .map(|o| parse_worktree_list(&String::from_utf8_lossy(&o.stdout), &root))
            .unwrap_or_default();

        Some(CurrentRepo {
            name,
            root,
            is_bare,
            worktrees,
        })
    }

    fn resolve_current_workspace() -> Option<CurrentWorkspace> {
        let root = worktree_root().ok()?;
        let branch = crate::git::git_output(&["rev-parse", "--abbrev-ref", "HEAD"])
            .unwrap_or_else(|_| "???".to_string());

        let store = store_dir()
            .ok()
            .filter(|s| s.is_dir() && s.join("manifest").is_file());
        let manifest = store
            .as_ref()
            .and_then(|s| read_manifest(s).ok())
            .unwrap_or_default();

        Some(CurrentWorkspace {
            root,
            branch,
            store_dir: store,
            manifest,
        })
    }
}

/// カレントディレクトリが属するリポジトリのルートパス（canonical）を解決する。
fn resolve_repo_root_from_cwd() -> Option<PathBuf> {
    let common_dir = Command::new("git")
        .args(["rev-parse", "--git-common-dir"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .ok()?;

    if common_dir.status.success() {
        let common = String::from_utf8_lossy(&common_dir.stdout)
            .trim()
            .to_string();
        let common_path = if Path::new(&common).is_absolute() {
            PathBuf::from(&common)
        } else {
            std::env::current_dir().ok()?.join(&common)
        };
        if let Ok(canonical) = common_path.canonicalize() {
            // bare worktree パターン: .bare がリポジトリルート
            if canonical.file_name().and_then(|n| n.to_str()) == Some(".bare") {
                return canonical.parent().map(|p| p.to_path_buf());
            }
            // 通常の clone: .git の親がリポジトリルート
            if canonical.file_name().and_then(|n| n.to_str()) == Some(".git") {
                return canonical.parent().map(|p| p.to_path_buf());
            }
        }
    }

    // フォールバック: bare root にいる場合
    if PathBuf::from(".bare").is_dir() {
        return std::fs::canonicalize(".").ok();
    }

    None
}

/// ホームディレクトリを `~` に短縮して表示する。
pub fn abbreviate_home(path: &Path) -> String {
    if let Ok(home) = std::env::var("HOME") {
        let home_path = Path::new(&home);
        if let Ok(suffix) = path.strip_prefix(home_path) {
            return format!("~/{}", suffix.display());
        }
    }
    path.display().to_string()
}

/// カラム幅を動的に計算してテーブルを表示する。
///
/// - `headers`: ヘッダー文字列のスライス（英語固定）
/// - `rows`: 各行のセルデータ
/// - `indent`: 全行に付与するインデント（スペース数）
pub fn print_table(headers: &[&str], rows: &[Vec<String>], indent: usize) {
    if headers.is_empty() {
        return;
    }

    let num_cols = headers.len();
    let mut widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();
    for row in rows {
        for (i, cell) in row.iter().enumerate().take(num_cols) {
            widths[i] = widths[i].max(cell.len());
        }
    }

    let prefix = " ".repeat(indent);

    // Header
    let header_line: String = headers
        .iter()
        .enumerate()
        .map(|(i, h)| {
            if i == num_cols - 1 {
                h.to_string()
            } else {
                format!("{:<width$}", h, width = widths[i])
            }
        })
        .collect::<Vec<_>>()
        .join("  ");
    println!("{prefix}{header_line}");

    // Separator (─ をヘッダー文字数分繰り返し、カラム幅までスペースで埋める)
    let sep_line: String = headers
        .iter()
        .enumerate()
        .map(|(i, h)| {
            let sep = "\u{2500}".repeat(h.len());
            if i == num_cols - 1 {
                sep
            } else {
                let padding = widths[i].saturating_sub(h.len());
                format!("{sep}{}", " ".repeat(padding))
            }
        })
        .collect::<Vec<_>>()
        .join("  ");
    println!("{prefix}{sep_line}");

    // Data rows
    for row in rows {
        let row_line: String = (0..num_cols)
            .map(|i| {
                let cell = row.get(i).map(|s| s.as_str()).unwrap_or("");
                if i == num_cols - 1 {
                    cell.to_string()
                } else {
                    format!("{:<width$}", cell, width = widths[i])
                }
            })
            .collect::<Vec<_>>()
            .join("  ");
        println!("{prefix}{row_line}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abbreviate_home_replaces_home_prefix() {
        let home = std::env::var("HOME").unwrap();
        let path = PathBuf::from(&home).join("Projects/test");
        assert_eq!(abbreviate_home(&path), "~/Projects/test");
    }

    #[test]
    fn abbreviate_home_leaves_non_home_paths() {
        let path = PathBuf::from("/tmp/something");
        assert_eq!(abbreviate_home(&path), "/tmp/something");
    }
}
