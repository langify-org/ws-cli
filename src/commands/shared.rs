use anyhow::{bail, Context, Result};
use rust_i18n::t;
use std::fs;
use std::os::unix::fs as unix_fs;

use crate::cli::{SharedPullCmd, SharedPushCmd, SharedTrackCmd};
use crate::git::worktree_root;
use crate::store::{ensure_store, file_status, read_manifest, require_store, write_manifest, ManifestEntry};

pub(crate) fn cmd_shared_track(cmd: &SharedTrackCmd) -> Result<()> {
    let store = ensure_store()?;
    let wt_root = worktree_root()?;

    if cmd.strategy != "symlink" && cmd.strategy != "copy" {
        bail!("{}", t!("shared.invalid_strategy"));
    }

    let source = wt_root.join(&cmd.file);
    if !source.exists() && source.symlink_metadata().is_err() {
        bail!("{}", t!("shared.file_not_found", file = &cmd.file));
    }

    // manifest を更新
    let mut entries = read_manifest(&store)?;
    let mut found = false;
    for entry in entries.iter_mut() {
        if entry.filepath == cmd.file {
            entry.strategy = cmd.strategy.clone();
            found = true;
            break;
        }
    }
    if !found {
        entries.push(ManifestEntry {
            strategy: cmd.strategy.clone(),
            filepath: cmd.file.clone(),
        });
    }
    write_manifest(&store, &entries)?;

    // store にコピー
    let store_file = store.join(&cmd.file);
    if let Some(parent) = store_file.parent() {
        fs::create_dir_all(parent)?;
    }

    let is_symlink = source
        .symlink_metadata()
        .map(|m| m.file_type().is_symlink())
        .unwrap_or(false);

    if cmd.strategy == "symlink" {
        fs::copy(&source, &store_file).context(t!("shared.copy_to_store_failed").to_string())?;

        if !is_symlink {
            fs::remove_file(&source)?;
            unix_fs::symlink(&store_file, &source)?;
            println!("{}", t!("shared.converted_to_symlink", file = &cmd.file));
        }
    } else {
        fs::copy(&source, &store_file).context(t!("shared.copy_to_store_failed").to_string())?;
    }

    println!("{}", t!("shared.tracking_started", strategy = &cmd.strategy, file = &cmd.file));
    Ok(())
}

pub(crate) fn cmd_shared_status() -> Result<()> {
    let store = require_store()?;
    let wt_root = worktree_root().ok();

    println!("Store: {}", store.display());
    println!();

    let entries = read_manifest(&store)?;
    if entries.is_empty() {
        println!("{}", t!("shared.no_tracked_files"));
        return Ok(());
    }

    println!("{:<8} {:<40} {}", "STRATEGY", "FILE", "STATUS");
    println!(
        "{:<8} {:<40} {}",
        "--------", "----------------------------------------", "----------"
    );

    for entry in &entries {
        let store_file = store.join(&entry.filepath);
        let status = file_status(entry, &store_file, &wt_root);
        println!("{:<8} {:<40} {}", entry.strategy, entry.filepath, status);
    }

    Ok(())
}

pub(crate) fn cmd_shared_push(cmd: &SharedPushCmd) -> Result<()> {
    let store = require_store()?;
    let wt_root = worktree_root()?;
    let entries = read_manifest(&store)?;

    let mut pushed = 0u32;

    for entry in &entries {
        if entry.strategy != "copy" {
            continue;
        }

        if let Some(ref target_file) = cmd.file {
            if entry.filepath != *target_file {
                continue;
            }
        }

        let wt_file = wt_root.join(&entry.filepath);
        if !wt_file.is_file() {
            eprintln!("{}", t!("shared.skip_not_in_worktree", file = &entry.filepath));
            continue;
        }

        let store_file = store.join(&entry.filepath);
        fs::copy(&wt_file, &store_file)?;
        println!("push: {}", entry.filepath);
        pushed += 1;
    }

    if pushed == 0 {
        if let Some(ref target_file) = cmd.file {
            bail!("{}", t!("shared.not_copy_tracked", file = target_file));
        } else {
            println!("{}", t!("shared.no_copy_files_to_push"));
        }
    }

    Ok(())
}

pub(crate) fn cmd_shared_pull(cmd: &SharedPullCmd) -> Result<()> {
    let store = require_store()?;
    let wt_root = worktree_root()?;
    let entries = read_manifest(&store)?;

    let mut pulled = 0u32;

    for entry in &entries {
        if let Some(ref target_file) = cmd.file {
            if entry.filepath != *target_file {
                continue;
            }
        }

        let store_file = store.join(&entry.filepath);
        if !store_file.is_file() {
            eprintln!("{}", t!("shared.skip_not_in_store", file = &entry.filepath));
            continue;
        }

        let wt_file = wt_root.join(&entry.filepath);
        let wt_exists = wt_file.exists() || wt_file.symlink_metadata().is_ok();

        if wt_exists && !cmd.force {
            eprintln!(
                "{}",
                t!("shared.skip_exists_use_force", file = &entry.filepath)
            );
            continue;
        }

        if wt_exists {
            let _ = fs::remove_file(&wt_file);
        }

        if let Some(parent) = wt_file.parent() {
            fs::create_dir_all(parent)?;
        }

        match entry.strategy.as_str() {
            "symlink" => {
                unix_fs::symlink(&store_file, &wt_file)?;
                println!("pull (symlink): {}", entry.filepath);
            }
            "copy" => {
                fs::copy(&store_file, &wt_file)?;
                println!("pull (copy): {}", entry.filepath);
            }
            _ => continue,
        }
        pulled += 1;
    }

    if pulled == 0 {
        if let Some(ref target_file) = cmd.file {
            bail!("{}", t!("shared.not_tracked", file = target_file));
        } else {
            println!("{}", t!("shared.no_files_to_pull"));
        }
    }

    Ok(())
}
