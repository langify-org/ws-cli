mod common;

use common::TestRepo;
use predicates::prelude::*;
use std::fs;

// ---- ws store track ----

#[test]
fn track_symlink_registers_and_converts() {
    let repo = TestRepo::new();
    let wt = repo.main_worktree();

    // worktree にファイルを作成
    fs::write(wt.join(".envrc"), "use flake").unwrap();

    repo.ws_cmd_in("main")
        .args(["store", "track", "-s", "symlink", ".envrc"])
        .assert()
        .success();

    // manifest に登録されている
    let manifest = fs::read_to_string(repo.store_dir().join("manifest")).unwrap();
    assert!(manifest.contains("symlink:.envrc"));

    // store にマスターコピーが存在
    assert!(repo.store_dir().join(".envrc").is_file());

    // worktree 内のファイルが symlink に変換されている
    let meta = wt.join(".envrc").symlink_metadata().unwrap();
    assert!(meta.file_type().is_symlink());
}

#[test]
fn track_copy_registers() {
    let repo = TestRepo::new();
    let wt = repo.main_worktree();

    fs::write(wt.join(".mcp.json"), r#"{"key":"value"}"#).unwrap();

    repo.ws_cmd_in("main")
        .args(["store", "track", "-s", "copy", ".mcp.json"])
        .assert()
        .success();

    // manifest に登録されている
    let manifest = fs::read_to_string(repo.store_dir().join("manifest")).unwrap();
    assert!(manifest.contains("copy:.mcp.json"));

    // store にコピーが存在
    assert!(repo.store_dir().join(".mcp.json").is_file());

    // worktree 内のファイルは通常ファイルのまま
    let meta = wt.join(".mcp.json").symlink_metadata().unwrap();
    assert!(!meta.file_type().is_symlink());
}

#[test]
fn track_invalid_strategy_fails() {
    let repo = TestRepo::new();
    let wt = repo.main_worktree();
    fs::write(wt.join(".envrc"), "content").unwrap();

    repo.ws_cmd_in("main")
        .args(["store", "track", "-s", "invalid", ".envrc"])
        .assert()
        .failure();
}

#[test]
fn track_nonexistent_file_fails() {
    let repo = TestRepo::new();

    repo.ws_cmd_in("main")
        .args(["store", "track", "-s", "symlink", "nonexistent"])
        .assert()
        .failure();
}

// ---- ws store status ----

#[test]
fn status_shows_entries() {
    let repo = TestRepo::new();
    let wt = repo.main_worktree();

    fs::write(wt.join(".envrc"), "use flake").unwrap();

    repo.ws_cmd_in("main")
        .args(["store", "track", "-s", "symlink", ".envrc"])
        .assert()
        .success();

    repo.ws_cmd_in("main")
        .args(["store", "status"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("STRATEGY")
                .and(predicate::str::contains(".envrc"))
                .and(predicate::str::contains("symlink")),
        );
}

#[test]
fn status_without_store_fails() {
    let repo = TestRepo::new();

    // store 未初期化の状態で status → エラー
    repo.ws_cmd_in("main")
        .args(["store", "status"])
        .assert()
        .failure();
}

// ---- ws store push ----

#[test]
fn push_updates_store_copy() {
    let repo = TestRepo::new();
    let wt = repo.main_worktree();

    // copy で track
    fs::write(wt.join(".mcp.json"), "original").unwrap();
    repo.ws_cmd_in("main")
        .args(["store", "track", "-s", "copy", ".mcp.json"])
        .assert()
        .success();

    // worktree 側を変更
    fs::write(wt.join(".mcp.json"), "modified").unwrap();

    // push
    repo.ws_cmd_in("main")
        .args(["store", "push"])
        .assert()
        .success();

    // store が更新されている
    let store_content = fs::read_to_string(repo.store_dir().join(".mcp.json")).unwrap();
    assert_eq!(store_content, "modified");
}

#[test]
fn push_ignores_symlink() {
    let repo = TestRepo::new();
    let wt = repo.main_worktree();

    // symlink で track
    fs::write(wt.join(".envrc"), "use flake").unwrap();
    repo.ws_cmd_in("main")
        .args(["store", "track", "-s", "symlink", ".envrc"])
        .assert()
        .success();

    // push しても symlink ファイルは対象外（エラーにはならない、"no copy files" メッセージ）
    repo.ws_cmd_in("main")
        .args(["store", "push"])
        .assert()
        .success();
}

// ---- ws store pull ----

#[test]
fn pull_restores_missing_symlink() {
    let repo = TestRepo::new();
    let wt = repo.main_worktree();

    // symlink で track
    fs::write(wt.join(".envrc"), "use flake").unwrap();
    repo.ws_cmd_in("main")
        .args(["store", "track", "-s", "symlink", ".envrc"])
        .assert()
        .success();

    // worktree から symlink を削除
    fs::remove_file(wt.join(".envrc")).unwrap();
    assert!(!wt.join(".envrc").exists());

    // pull で復元
    repo.ws_cmd_in("main")
        .args(["store", "pull"])
        .assert()
        .success();

    // symlink が復元されている
    let meta = wt.join(".envrc").symlink_metadata().unwrap();
    assert!(meta.file_type().is_symlink());
}

#[test]
fn pull_skips_existing_without_force() {
    let repo = TestRepo::new();
    let wt = repo.main_worktree();

    // copy で track
    fs::write(wt.join(".mcp.json"), "original").unwrap();
    repo.ws_cmd_in("main")
        .args(["store", "track", "-s", "copy", ".mcp.json"])
        .assert()
        .success();

    // worktree 側を変更
    fs::write(wt.join(".mcp.json"), "local changes").unwrap();

    // force なし pull → スキップ（ファイルは変わらない）
    repo.ws_cmd_in("main")
        .args(["store", "pull"])
        .assert()
        .success();

    let content = fs::read_to_string(wt.join(".mcp.json")).unwrap();
    assert_eq!(content, "local changes");
}

#[test]
fn pull_overwrites_with_force() {
    let repo = TestRepo::new();
    let wt = repo.main_worktree();

    // copy で track
    fs::write(wt.join(".mcp.json"), "original").unwrap();
    repo.ws_cmd_in("main")
        .args(["store", "track", "-s", "copy", ".mcp.json"])
        .assert()
        .success();

    // worktree 側を変更
    fs::write(wt.join(".mcp.json"), "local changes").unwrap();

    // force 付き pull → 上書き
    repo.ws_cmd_in("main")
        .args(["store", "pull", "-f"])
        .assert()
        .success();

    let content = fs::read_to_string(wt.join(".mcp.json")).unwrap();
    assert_eq!(content, "original");
}

// ---- ws store untrack ----

#[test]
fn untrack_removes_and_restores() {
    let repo = TestRepo::new();
    let wt = repo.main_worktree();

    // symlink で track
    fs::write(wt.join(".envrc"), "use flake").unwrap();
    repo.ws_cmd_in("main")
        .args(["store", "track", "-s", "symlink", ".envrc"])
        .assert()
        .success();

    // untrack
    repo.ws_cmd_in("main")
        .args(["store", "untrack", ".envrc"])
        .assert()
        .success();

    // manifest からエントリが消えている
    let manifest = fs::read_to_string(repo.store_dir().join("manifest")).unwrap();
    assert!(!manifest.contains(".envrc"));

    // store 内のマスターコピーが削除されている
    assert!(!repo.store_dir().join(".envrc").exists());

    // worktree 内のファイルが通常ファイルに復元されている
    assert!(wt.join(".envrc").exists());
    let meta = wt.join(".envrc").symlink_metadata().unwrap();
    assert!(!meta.file_type().is_symlink());
}
