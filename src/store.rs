use anyhow::{bail, Context, Result};
use rust_i18n::t;
use std::fs;
use std::io::Write;
use std::os::unix::fs as unix_fs;
use std::path::{Path, PathBuf};

use crate::git;

pub(crate) struct ManifestEntry {
    pub strategy: String,
    pub filepath: String,
}

pub(crate) fn store_dir() -> Result<PathBuf> {
    // まず git rev-parse --git-common-dir を試す
    if let Ok(common_dir) = git::git_output(&["rev-parse", "--git-common-dir"]) {
        let canonical = fs::canonicalize(&common_dir)
            .with_context(|| t!("store.path_canonicalize_failed", path = &common_dir).to_string())?;
        return Ok(canonical.join("worktree-store"));
    }

    // フォールバック: .bare ディレクトリを探す
    if let Some(bare_dir) = git::find_bare_dir() {
        let canonical = fs::canonicalize(&bare_dir)
            .with_context(|| t!("store.path_canonicalize_failed", path = bare_dir.display().to_string()).to_string())?;
        return Ok(canonical.join("worktree-store"));
    }

    bail!("{}", t!("store.run_inside_repo"))
}

pub(crate) fn require_store() -> Result<PathBuf> {
    let store = store_dir()?;
    if !store.is_dir() || !store.join("manifest").is_file() {
        bail!("{}", t!("store.store_not_initialized"));
    }
    Ok(store)
}

pub(crate) fn ensure_store() -> Result<PathBuf> {
    let store = store_dir()?;
    fs::create_dir_all(&store)?;
    let manifest = store.join("manifest");
    if !manifest.is_file() {
        fs::File::create(&manifest)?;
    }
    Ok(store)
}

pub(crate) fn read_manifest(store: &Path) -> Result<Vec<ManifestEntry>> {
    let manifest_path = store.join("manifest");
    let content = fs::read_to_string(&manifest_path)
        .with_context(|| t!("store.manifest_read_failed", path = manifest_path.display().to_string()).to_string())?;

    let mut entries = Vec::new();
    for line in content.lines() {
        if line.is_empty() {
            continue;
        }
        if let Some((strategy, filepath)) = line.split_once(':') {
            if !strategy.is_empty() {
                entries.push(ManifestEntry {
                    strategy: strategy.to_string(),
                    filepath: filepath.to_string(),
                });
            }
        }
    }
    Ok(entries)
}

pub(crate) fn write_manifest(store: &Path, entries: &[ManifestEntry]) -> Result<()> {
    let manifest_path = store.join("manifest");
    let mut file = fs::File::create(&manifest_path)
        .with_context(|| t!("store.manifest_write_failed", path = manifest_path.display().to_string()).to_string())?;

    for entry in entries {
        writeln!(file, "{}:{}", entry.strategy, entry.filepath)?;
    }
    Ok(())
}

pub(crate) fn apply_file(strategy: &str, filepath: &str, store: &Path, target_root: &Path) -> Result<()> {
    let target = target_root.join(filepath);
    let source = store.join(filepath);

    if target.exists() || target.symlink_metadata().is_ok() {
        eprintln!("{}", t!("store.skip_exists", file = filepath));
        return Ok(());
    }

    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)?;
    }

    match strategy {
        "symlink" => {
            unix_fs::symlink(&source, &target)?;
            println!("  symlink: {}", filepath);
        }
        "copy" => {
            fs::copy(&source, &target)?;
            println!("  copy: {}", filepath);
        }
        _ => {}
    }

    Ok(())
}

pub(crate) fn file_status(
    entry: &ManifestEntry,
    store_file: &Path,
    wt_root: &Option<PathBuf>,
) -> &'static str {
    if !store_file.is_file() {
        return "MISSING(store)";
    }

    let Some(ref root) = wt_root else {
        return "(store only)";
    };

    let wt_file = root.join(&entry.filepath);
    let wt_exists = wt_file.exists() || wt_file.symlink_metadata().is_ok();

    if !wt_exists {
        return "MISSING";
    }

    if entry.strategy == "symlink" {
        let is_link = wt_file
            .symlink_metadata()
            .map(|m| m.file_type().is_symlink())
            .unwrap_or(false);

        if !is_link {
            return "NOT_LINK";
        }

        let link_target = match fs::read_link(&wt_file) {
            Ok(t) => t,
            Err(_) => return "ERROR",
        };
        if link_target != *store_file {
            "WRONG_LINK"
        } else {
            "OK"
        }
    } else if entry.strategy == "copy" {
        let store_content = fs::read(store_file).ok();
        let wt_content = fs::read(&wt_file).ok();
        if store_content != wt_content {
            "MODIFIED"
        } else {
            "OK"
        }
    } else {
        "OK"
    }
}
