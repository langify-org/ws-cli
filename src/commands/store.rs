use anyhow::{bail, Context, Result};
use rust_i18n::t;
use std::fs;
use std::os::unix::fs as unix_fs;

use crate::cli::{StorePullCmd, StorePushCmd, StoreTrackCmd, StoreUntrackCmd};
use crate::git::{git_output, worktree_root};
use crate::store::{ensure_store, file_status, read_manifest, require_store, write_manifest, ManifestEntry};

pub(crate) fn cmd_store_track(cmd: &StoreTrackCmd) -> Result<()> {
    let store = ensure_store()?;
    let wt_root = worktree_root()?;

    if cmd.strategy != "symlink" && cmd.strategy != "copy" {
        bail!("{}", t!("store.invalid_strategy"));
    }

    let source = wt_root.join(&cmd.file);
    if !source.exists() && source.symlink_metadata().is_err() {
        bail!("{}", t!("store.file_not_found", file = &cmd.file));
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
        fs::copy(&source, &store_file).context(t!("store.copy_to_store_failed").to_string())?;

        if !is_symlink {
            fs::remove_file(&source)?;
            unix_fs::symlink(&store_file, &source)?;
            println!("{}", t!("store.converted_to_symlink", file = &cmd.file));
        }
    } else {
        fs::copy(&source, &store_file).context(t!("store.copy_to_store_failed").to_string())?;
    }

    println!("{}", t!("store.tracking_started", strategy = &cmd.strategy, file = &cmd.file));
    Ok(())
}

pub(crate) fn cmd_store_status() -> Result<()> {
    let store = require_store()?;
    let wt_root = worktree_root().ok();

    println!("Store: {}", store.display());
    println!();

    let entries = read_manifest(&store)?;
    if entries.is_empty() {
        println!("{}", t!("store.no_tracked_files"));
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

pub(crate) fn cmd_store_push(cmd: &StorePushCmd) -> Result<()> {
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
            eprintln!("{}", t!("store.skip_not_in_worktree", file = &entry.filepath));
            continue;
        }

        let store_file = store.join(&entry.filepath);
        fs::copy(&wt_file, &store_file)?;
        println!("push: {}", entry.filepath);
        pushed += 1;
    }

    if pushed == 0 {
        if let Some(ref target_file) = cmd.file {
            bail!("{}", t!("store.not_copy_tracked", file = target_file));
        } else {
            println!("{}", t!("store.no_copy_files_to_push"));
        }
    }

    Ok(())
}

pub(crate) fn cmd_store_pull(cmd: &StorePullCmd) -> Result<()> {
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
            eprintln!("{}", t!("store.skip_not_in_store", file = &entry.filepath));
            continue;
        }

        let wt_file = wt_root.join(&entry.filepath);
        let wt_exists = wt_file.exists() || wt_file.symlink_metadata().is_ok();

        if wt_exists && !cmd.force {
            eprintln!(
                "{}",
                t!("store.skip_exists_use_force", file = &entry.filepath)
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
            bail!("{}", t!("store.not_tracked", file = target_file));
        } else {
            println!("{}", t!("store.no_files_to_pull"));
        }
    }

    Ok(())
}

pub(crate) fn cmd_store_untrack(cmd: &StoreUntrackCmd) -> Result<()> {
    let store = require_store()?;
    let mut entries = read_manifest(&store)?;

    // 指定ファイルのエントリを検索
    let pos = entries.iter().position(|e| e.filepath == cmd.file);
    let pos = match pos {
        Some(p) => p,
        None => bail!("{}", t!("store.not_tracked", file = &cmd.file)),
    };

    let entry = &entries[pos];

    // symlink strategy の場合: 全 worktree の symlink を実ファイルに復元
    if entry.strategy == "symlink" {
        let store_file = store.join(&entry.filepath);
        if store_file.is_file() {
            let wt_list = git_output(&["worktree", "list"])?;
            for line in wt_list.lines() {
                if line.contains("(bare)") {
                    continue;
                }
                let wt_path = match line.split_whitespace().next() {
                    Some(p) => p,
                    None => continue,
                };
                let target = std::path::Path::new(wt_path).join(&entry.filepath);
                let is_link = target
                    .symlink_metadata()
                    .map(|m| m.file_type().is_symlink())
                    .unwrap_or(false);
                if is_link {
                    let _ = fs::remove_file(&target);
                    if let Some(parent) = target.parent() {
                        let _ = fs::create_dir_all(parent);
                    }
                    match fs::copy(&store_file, &target) {
                        Ok(_) => println!(
                            "{}",
                            t!("store.symlink_restored", file = &entry.filepath, path = wt_path)
                        ),
                        Err(_) => eprintln!(
                            "{}",
                            t!("store.restore_copy_failed", file = &entry.filepath, path = wt_path)
                        ),
                    }
                }
            }
        }
    }

    // manifest からエントリ削除
    entries.remove(pos);
    write_manifest(&store, &entries)?;

    // store 内のマスターコピーを削除
    let store_file = store.join(&cmd.file);
    if store_file.exists() {
        fs::remove_file(&store_file)?;
    }

    // 空になった親ディレクトリを削除（store ルートは残す）
    let mut dir = store_file.parent().map(|p| p.to_path_buf());
    while let Some(ref d) = dir {
        if *d == store {
            break;
        }
        if d.read_dir().map(|mut rd| rd.next().is_none()).unwrap_or(true) {
            let _ = fs::remove_dir(d);
            dir = d.parent().map(|p| p.to_path_buf());
        } else {
            break;
        }
    }

    println!("{}", t!("store.untrack_success", file = &cmd.file));
    Ok(())
}
