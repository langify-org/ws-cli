use anyhow::{Context, Result, bail};
use rust_i18n::t;
use std::fs;
use std::io::Write;
use std::os::unix::fs as unix_fs;
use std::path::{Path, PathBuf};

use crate::git;
use crate::ui;

#[derive(Debug, Clone, PartialEq, clap::ValueEnum)]
pub enum Strategy {
    Symlink,
    Copy,
}

impl Strategy {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Symlink => "symlink",
            Self::Copy => "copy",
        }
    }
}

impl std::fmt::Display for Strategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for Strategy {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "symlink" => Ok(Self::Symlink),
            "copy" => Ok(Self::Copy),
            _ => Err(anyhow::anyhow!("{}", t!("store.invalid_strategy"))),
        }
    }
}

pub struct ManifestEntry {
    pub strategy: Strategy,
    pub filepath: String,
}

pub fn store_dir() -> Result<PathBuf> {
    // まず git rev-parse --git-common-dir を試す
    if let Ok(common_dir) = git::git_output(&["rev-parse", "--git-common-dir"]) {
        let canonical = fs::canonicalize(&common_dir).with_context(|| {
            t!("store.path_canonicalize_failed", path = &common_dir).to_string()
        })?;
        return Ok(canonical.join("worktree-store"));
    }

    // フォールバック: .bare ディレクトリを探す
    if let Some(bare_dir) = git::find_bare_dir() {
        let canonical = fs::canonicalize(&bare_dir).with_context(|| {
            t!(
                "store.path_canonicalize_failed",
                path = bare_dir.display().to_string()
            )
            .to_string()
        })?;
        return Ok(canonical.join("worktree-store"));
    }

    bail!("{}", t!("store.run_inside_repo"))
}

pub fn require_store() -> Result<PathBuf> {
    let store = store_dir()?;
    if !store.is_dir() || !store.join("manifest").is_file() {
        bail!("{}", t!("store.store_not_initialized"));
    }
    Ok(store)
}

pub fn ensure_store() -> Result<PathBuf> {
    let store = store_dir()?;
    fs::create_dir_all(&store)?;
    let manifest = store.join("manifest");
    if !manifest.is_file() {
        fs::File::create(&manifest)?;
    }
    Ok(store)
}

pub fn read_manifest(store: &Path) -> Result<Vec<ManifestEntry>> {
    let manifest_path = store.join("manifest");
    let content = fs::read_to_string(&manifest_path).with_context(|| {
        t!(
            "store.manifest_read_failed",
            path = manifest_path.display().to_string()
        )
        .to_string()
    })?;

    let mut entries = Vec::new();
    for line in content.lines() {
        if line.is_empty() {
            continue;
        }
        if let Some((strategy_str, filepath)) = line.split_once(':')
            && let Ok(strategy) = strategy_str.parse::<Strategy>()
        {
            entries.push(ManifestEntry {
                strategy,
                filepath: filepath.to_string(),
            });
        }
    }
    Ok(entries)
}

pub fn write_manifest(store: &Path, entries: &[ManifestEntry]) -> Result<()> {
    let manifest_path = store.join("manifest");
    let mut file = fs::File::create(&manifest_path).with_context(|| {
        t!(
            "store.manifest_write_failed",
            path = manifest_path.display().to_string()
        )
        .to_string()
    })?;

    for entry in entries {
        writeln!(file, "{}:{}", entry.strategy.as_str(), entry.filepath)?;
    }
    Ok(())
}

/// ファイルまたはシンボリックリンク（リンク切れ含む）が存在するか判定。
/// `Path::exists()` はリンク切れ symlink で false を返すため、
/// symlink 自体の存在も `symlink_metadata()` で確認する。
pub fn path_or_symlink_exists(path: &Path) -> bool {
    path.exists() || path.symlink_metadata().is_ok()
}

/// store 内にエントリ（ファイルまたはディレクトリ）が存在するか判定。
pub fn store_entry_exists(path: &Path) -> bool {
    path.is_file() || path.is_dir()
}

/// ディレクトリを再帰的にコピーする。
pub fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

/// 2つのディレクトリの内容を再帰的に比較する（copy strategy の MODIFIED 判定用）。
fn dirs_equal_recursive(a: &Path, b: &Path) -> bool {
    let Ok(a_entries) = fs::read_dir(a) else {
        return false;
    };
    let Ok(b_entries) = fs::read_dir(b) else {
        return false;
    };

    let mut a_names: Vec<_> = a_entries
        .filter_map(|e| e.ok())
        .map(|e| e.file_name())
        .collect();
    let mut b_names: Vec<_> = b_entries
        .filter_map(|e| e.ok())
        .map(|e| e.file_name())
        .collect();

    a_names.sort();
    b_names.sort();

    if a_names != b_names {
        return false;
    }

    for name in &a_names {
        let a_path = a.join(name);
        let b_path = b.join(name);

        if a_path.is_dir() && b_path.is_dir() {
            if !dirs_equal_recursive(&a_path, &b_path) {
                return false;
            }
        } else if a_path.is_file() && b_path.is_file() {
            let a_content = fs::read(&a_path).ok();
            let b_content = fs::read(&b_path).ok();
            if a_content != b_content {
                return false;
            }
        } else {
            // 一方がファイルで他方がディレクトリ → 不一致
            return false;
        }
    }

    true
}

pub fn apply_file(
    strategy: &Strategy,
    filepath: &str,
    store: &Path,
    target_root: &Path,
) -> Result<()> {
    let target = target_root.join(filepath);
    let source = store.join(filepath);

    if path_or_symlink_exists(&target) {
        anstream::eprintln!(
            "{}",
            ui::styled(ui::STYLE_WARN, &t!("store.skip_exists", file = filepath))
        );
        return Ok(());
    }

    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)?;
    }

    match strategy {
        Strategy::Symlink => {
            unix_fs::symlink(&source, &target)?;
            anstream::println!(
                "  {}",
                ui::styled(ui::STYLE_OK, &format!("symlink: {}", filepath))
            );
        }
        Strategy::Copy => {
            if source.is_dir() {
                copy_dir_recursive(&source, &target)?;
            } else {
                fs::copy(&source, &target)?;
            }
            anstream::println!(
                "  {}",
                ui::styled(ui::STYLE_OK, &format!("copy: {}", filepath))
            );
        }
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
pub enum FileStatus {
    Ok,
    Missing,
    MissingStore,
    Modified,
    NotLink,
    WrongLink,
    Error,
    StoreOnly,
}

impl std::fmt::Display for FileStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok => write!(f, "OK"),
            Self::Missing => write!(f, "MISSING"),
            Self::MissingStore => write!(f, "MISSING(store)"),
            Self::Modified => write!(f, "MODIFIED"),
            Self::NotLink => write!(f, "NOT_LINK"),
            Self::WrongLink => write!(f, "WRONG_LINK"),
            Self::Error => write!(f, "ERROR"),
            Self::StoreOnly => write!(f, "(store only)"),
        }
    }
}

pub fn file_status(
    entry: &ManifestEntry,
    store_file: &Path,
    wt_root: &Option<PathBuf>,
) -> FileStatus {
    if !store_entry_exists(store_file) {
        return FileStatus::MissingStore;
    }

    let Some(root) = wt_root else {
        return FileStatus::StoreOnly;
    };

    let wt_file = root.join(&entry.filepath);

    if !path_or_symlink_exists(&wt_file) {
        return FileStatus::Missing;
    }

    match entry.strategy {
        Strategy::Symlink => {
            let is_link = wt_file
                .symlink_metadata()
                .map(|m| m.file_type().is_symlink())
                .unwrap_or(false);

            if !is_link {
                return FileStatus::NotLink;
            }

            let link_target = match fs::read_link(&wt_file) {
                Ok(t) => t,
                Err(_) => return FileStatus::Error,
            };
            if link_target != *store_file {
                FileStatus::WrongLink
            } else {
                FileStatus::Ok
            }
        }
        Strategy::Copy => {
            if store_file.is_dir() {
                if wt_file.is_dir() {
                    if dirs_equal_recursive(store_file, &wt_file) {
                        FileStatus::Ok
                    } else {
                        FileStatus::Modified
                    }
                } else {
                    FileStatus::Modified
                }
            } else {
                let store_content = fs::read(store_file).ok();
                let wt_content = fs::read(&wt_file).ok();
                if store_content != wt_content {
                    FileStatus::Modified
                } else {
                    FileStatus::Ok
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs as unix_fs;
    use tempfile::TempDir;

    /// store ディレクトリと manifest を構築するヘルパー
    fn setup_store() -> (TempDir, PathBuf) {
        let tmp = TempDir::new().unwrap();
        let store = tmp.path().join("worktree-store");
        fs::create_dir_all(&store).unwrap();
        fs::write(store.join("manifest"), "").unwrap();
        (tmp, store)
    }

    // ---- read_manifest ----

    #[test]
    fn read_manifest_parses_entries() {
        let (_tmp, store) = setup_store();
        fs::write(store.join("manifest"), "symlink:.envrc\ncopy:.mcp.json\n").unwrap();

        let entries = read_manifest(&store).unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].strategy, Strategy::Symlink);
        assert_eq!(entries[0].filepath, ".envrc");
        assert_eq!(entries[1].strategy, Strategy::Copy);
        assert_eq!(entries[1].filepath, ".mcp.json");
    }

    #[test]
    fn read_manifest_skips_empty_and_malformed() {
        let (_tmp, store) = setup_store();
        fs::write(
            store.join("manifest"),
            "\nsymlink:.envrc\n\nno_colon_here\n:empty_strategy\n",
        )
        .unwrap();

        let entries = read_manifest(&store).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].filepath, ".envrc");
    }

    #[test]
    fn read_manifest_filepath_with_colon() {
        let (_tmp, store) = setup_store();
        fs::write(store.join("manifest"), "symlink:path:with:colon\n").unwrap();

        let entries = read_manifest(&store).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].strategy, Strategy::Symlink);
        assert_eq!(entries[0].filepath, "path:with:colon");
    }

    #[test]
    fn write_then_read_roundtrip() {
        let (_tmp, store) = setup_store();
        let original = vec![
            ManifestEntry {
                strategy: Strategy::Symlink,
                filepath: ".envrc".into(),
            },
            ManifestEntry {
                strategy: Strategy::Copy,
                filepath: ".mcp.json".into(),
            },
        ];

        write_manifest(&store, &original).unwrap();
        let read_back = read_manifest(&store).unwrap();

        assert_eq!(read_back.len(), 2);
        assert_eq!(read_back[0].strategy, Strategy::Symlink);
        assert_eq!(read_back[0].filepath, ".envrc");
        assert_eq!(read_back[1].strategy, Strategy::Copy);
        assert_eq!(read_back[1].filepath, ".mcp.json");
    }

    // ---- file_status ----

    #[test]
    fn file_status_missing_store() {
        let tmp = TempDir::new().unwrap();
        let store_file = tmp.path().join("nonexistent");
        let entry = ManifestEntry {
            strategy: Strategy::Symlink,
            filepath: "test".into(),
        };
        assert_eq!(
            file_status(&entry, &store_file, &None),
            FileStatus::MissingStore
        );
    }

    #[test]
    fn file_status_store_only() {
        let tmp = TempDir::new().unwrap();
        let store_file = tmp.path().join("test_file");
        fs::write(&store_file, "content").unwrap();

        let entry = ManifestEntry {
            strategy: Strategy::Symlink,
            filepath: "test_file".into(),
        };
        assert_eq!(
            file_status(&entry, &store_file, &None),
            FileStatus::StoreOnly
        );
    }

    #[test]
    fn file_status_missing_in_worktree() {
        let tmp = TempDir::new().unwrap();
        let store_file = tmp.path().join("store_copy");
        fs::write(&store_file, "content").unwrap();

        let wt_root = tmp.path().join("worktree");
        fs::create_dir_all(&wt_root).unwrap();

        let entry = ManifestEntry {
            strategy: Strategy::Symlink,
            filepath: "missing_file".into(),
        };
        assert_eq!(
            file_status(&entry, &store_file, &Some(wt_root)),
            FileStatus::Missing
        );
    }

    #[test]
    fn file_status_symlink_not_link() {
        let tmp = TempDir::new().unwrap();
        let store_file = tmp.path().join("store_copy");
        fs::write(&store_file, "content").unwrap();

        let wt_root = tmp.path().join("worktree");
        fs::create_dir_all(&wt_root).unwrap();
        fs::write(wt_root.join(".envrc"), "regular file").unwrap();

        let entry = ManifestEntry {
            strategy: Strategy::Symlink,
            filepath: ".envrc".into(),
        };
        assert_eq!(
            file_status(&entry, &store_file, &Some(wt_root)),
            FileStatus::NotLink
        );
    }

    #[test]
    fn file_status_symlink_wrong_target() {
        let tmp = TempDir::new().unwrap();
        let store_file = tmp.path().join("store_copy");
        fs::write(&store_file, "content").unwrap();

        let wt_root = tmp.path().join("worktree");
        fs::create_dir_all(&wt_root).unwrap();

        let wrong_target = tmp.path().join("wrong_target");
        fs::write(&wrong_target, "wrong").unwrap();
        unix_fs::symlink(&wrong_target, wt_root.join(".envrc")).unwrap();

        let entry = ManifestEntry {
            strategy: Strategy::Symlink,
            filepath: ".envrc".into(),
        };
        assert_eq!(
            file_status(&entry, &store_file, &Some(wt_root)),
            FileStatus::WrongLink
        );
    }

    #[test]
    fn file_status_symlink_ok() {
        let tmp = TempDir::new().unwrap();
        let store_file = tmp.path().join("store_copy");
        fs::write(&store_file, "content").unwrap();

        let wt_root = tmp.path().join("worktree");
        fs::create_dir_all(&wt_root).unwrap();
        unix_fs::symlink(&store_file, wt_root.join(".envrc")).unwrap();

        let entry = ManifestEntry {
            strategy: Strategy::Symlink,
            filepath: ".envrc".into(),
        };
        assert_eq!(
            file_status(&entry, &store_file, &Some(wt_root)),
            FileStatus::Ok
        );
    }

    #[test]
    fn file_status_copy_modified() {
        let tmp = TempDir::new().unwrap();
        let store_file = tmp.path().join("store_copy");
        fs::write(&store_file, "original").unwrap();

        let wt_root = tmp.path().join("worktree");
        fs::create_dir_all(&wt_root).unwrap();
        fs::write(wt_root.join(".mcp.json"), "modified").unwrap();

        let entry = ManifestEntry {
            strategy: Strategy::Copy,
            filepath: ".mcp.json".into(),
        };
        assert_eq!(
            file_status(&entry, &store_file, &Some(wt_root)),
            FileStatus::Modified
        );
    }

    #[test]
    fn file_status_copy_ok() {
        let tmp = TempDir::new().unwrap();
        let store_file = tmp.path().join("store_copy");
        fs::write(&store_file, "same content").unwrap();

        let wt_root = tmp.path().join("worktree");
        fs::create_dir_all(&wt_root).unwrap();
        fs::write(wt_root.join(".mcp.json"), "same content").unwrap();

        let entry = ManifestEntry {
            strategy: Strategy::Copy,
            filepath: ".mcp.json".into(),
        };
        assert_eq!(
            file_status(&entry, &store_file, &Some(wt_root)),
            FileStatus::Ok
        );
    }

    // ---- apply_file ----

    #[test]
    fn apply_file_symlink_creates_symlink() {
        let tmp = TempDir::new().unwrap();
        let store = tmp.path().join("store");
        fs::create_dir_all(&store).unwrap();
        fs::write(store.join(".envrc"), "content").unwrap();

        let target_root = tmp.path().join("target");
        fs::create_dir_all(&target_root).unwrap();

        apply_file(&Strategy::Symlink, ".envrc", &store, &target_root).unwrap();

        let target = target_root.join(".envrc");
        assert!(target.symlink_metadata().unwrap().file_type().is_symlink());
    }

    #[test]
    fn apply_file_copy_creates_regular_file() {
        let tmp = TempDir::new().unwrap();
        let store = tmp.path().join("store");
        fs::create_dir_all(&store).unwrap();
        fs::write(store.join(".mcp.json"), "content").unwrap();

        let target_root = tmp.path().join("target");
        fs::create_dir_all(&target_root).unwrap();

        apply_file(&Strategy::Copy, ".mcp.json", &store, &target_root).unwrap();

        let target = target_root.join(".mcp.json");
        assert!(target.is_file());
        assert_eq!(fs::read_to_string(&target).unwrap(), "content");
    }

    #[test]
    fn apply_file_skips_existing() {
        let tmp = TempDir::new().unwrap();
        let store = tmp.path().join("store");
        fs::create_dir_all(&store).unwrap();
        fs::write(store.join(".envrc"), "new content").unwrap();

        let target_root = tmp.path().join("target");
        fs::create_dir_all(&target_root).unwrap();
        fs::write(target_root.join(".envrc"), "existing").unwrap();

        apply_file(&Strategy::Symlink, ".envrc", &store, &target_root).unwrap();

        // 既存ファイルが変更されていないこと
        assert_eq!(
            fs::read_to_string(target_root.join(".envrc")).unwrap(),
            "existing"
        );
    }

    #[test]
    fn apply_file_creates_parent_dirs() {
        let tmp = TempDir::new().unwrap();
        let store = tmp.path().join("store");
        fs::create_dir_all(store.join("sub/dir")).unwrap();
        fs::write(store.join("sub/dir/file"), "content").unwrap();

        let target_root = tmp.path().join("target");
        fs::create_dir_all(&target_root).unwrap();

        apply_file(&Strategy::Copy, "sub/dir/file", &store, &target_root).unwrap();

        assert!(target_root.join("sub/dir/file").is_file());
    }

    // ---- directory: apply_file ----

    #[test]
    fn apply_file_symlink_directory() {
        let tmp = TempDir::new().unwrap();
        let store = tmp.path().join("store");
        let store_dir = store.join("nix/secrets");
        fs::create_dir_all(&store_dir).unwrap();
        fs::write(store_dir.join("key"), "secret").unwrap();

        let target_root = tmp.path().join("target");
        fs::create_dir_all(&target_root).unwrap();

        apply_file(&Strategy::Symlink, "nix/secrets", &store, &target_root).unwrap();

        let target = target_root.join("nix/secrets");
        assert!(target.symlink_metadata().unwrap().file_type().is_symlink());
        // symlink 先のディレクトリが読める
        assert_eq!(fs::read_to_string(target.join("key")).unwrap(), "secret");
    }

    #[test]
    fn apply_file_copy_directory() {
        let tmp = TempDir::new().unwrap();
        let store = tmp.path().join("store");
        let store_dir = store.join("nix/secrets");
        fs::create_dir_all(store_dir.join("sub")).unwrap();
        fs::write(store_dir.join("a"), "val_a").unwrap();
        fs::write(store_dir.join("sub/b"), "val_b").unwrap();

        let target_root = tmp.path().join("target");
        fs::create_dir_all(&target_root).unwrap();

        apply_file(&Strategy::Copy, "nix/secrets", &store, &target_root).unwrap();

        let target = target_root.join("nix/secrets");
        assert!(target.is_dir());
        assert_eq!(fs::read_to_string(target.join("a")).unwrap(), "val_a");
        assert_eq!(fs::read_to_string(target.join("sub/b")).unwrap(), "val_b");
    }

    // ---- directory: file_status ----

    #[test]
    fn file_status_dir_symlink_ok() {
        let tmp = TempDir::new().unwrap();
        let store_dir = tmp.path().join("store_secrets");
        fs::create_dir_all(&store_dir).unwrap();
        fs::write(store_dir.join("key"), "secret").unwrap();

        let wt_root = tmp.path().join("worktree");
        fs::create_dir_all(&wt_root).unwrap();
        unix_fs::symlink(&store_dir, wt_root.join("secrets")).unwrap();

        let entry = ManifestEntry {
            strategy: Strategy::Symlink,
            filepath: "secrets".into(),
        };
        assert_eq!(
            file_status(&entry, &store_dir, &Some(wt_root)),
            FileStatus::Ok
        );
    }

    #[test]
    fn file_status_dir_copy_ok() {
        let tmp = TempDir::new().unwrap();
        let store_dir = tmp.path().join("store_secrets");
        fs::create_dir_all(&store_dir).unwrap();
        fs::write(store_dir.join("key"), "secret").unwrap();

        let wt_root = tmp.path().join("worktree");
        let wt_dir = wt_root.join("secrets");
        fs::create_dir_all(&wt_dir).unwrap();
        fs::write(wt_dir.join("key"), "secret").unwrap();

        let entry = ManifestEntry {
            strategy: Strategy::Copy,
            filepath: "secrets".into(),
        };
        assert_eq!(
            file_status(&entry, &store_dir, &Some(wt_root)),
            FileStatus::Ok
        );
    }

    #[test]
    fn file_status_dir_copy_modified() {
        let tmp = TempDir::new().unwrap();
        let store_dir = tmp.path().join("store_secrets");
        fs::create_dir_all(&store_dir).unwrap();
        fs::write(store_dir.join("key"), "secret").unwrap();

        let wt_root = tmp.path().join("worktree");
        let wt_dir = wt_root.join("secrets");
        fs::create_dir_all(&wt_dir).unwrap();
        fs::write(wt_dir.join("key"), "modified_secret").unwrap();

        let entry = ManifestEntry {
            strategy: Strategy::Copy,
            filepath: "secrets".into(),
        };
        assert_eq!(
            file_status(&entry, &store_dir, &Some(wt_root)),
            FileStatus::Modified
        );
    }

    #[test]
    fn file_status_dir_missing_store() {
        let tmp = TempDir::new().unwrap();
        let store_dir = tmp.path().join("nonexistent_dir");

        let entry = ManifestEntry {
            strategy: Strategy::Copy,
            filepath: "secrets".into(),
        };
        assert_eq!(
            file_status(&entry, &store_dir, &None),
            FileStatus::MissingStore
        );
    }

    // ---- copy_dir_recursive ----

    #[test]
    fn copy_dir_recursive_nested() {
        let tmp = TempDir::new().unwrap();
        let src = tmp.path().join("src");
        fs::create_dir_all(src.join("a/b")).unwrap();
        fs::write(src.join("a/file1"), "content1").unwrap();
        fs::write(src.join("a/b/file2"), "content2").unwrap();

        let dst = tmp.path().join("dst");
        copy_dir_recursive(&src, &dst).unwrap();

        assert!(dst.join("a/b").is_dir());
        assert_eq!(fs::read_to_string(dst.join("a/file1")).unwrap(), "content1");
        assert_eq!(
            fs::read_to_string(dst.join("a/b/file2")).unwrap(),
            "content2"
        );
    }
}
