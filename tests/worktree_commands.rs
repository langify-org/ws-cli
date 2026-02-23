#![allow(deprecated)]

mod common;

use common::TestRepo;
use std::fs;
use tempfile::TempDir;

// ---- ws clone ----

#[test]
fn clone_creates_bare_repo() {
    let tmp = TempDir::new().unwrap();

    let mut cmd = assert_cmd::Command::cargo_bin("ws").unwrap();
    cmd.arg("clone")
        .current_dir(tmp.path())
        .env("LC_ALL", "en")
        .assert()
        .success();

    assert!(tmp.path().join(".bare").join("HEAD").exists());
}

#[test]
fn clone_fails_if_bare_exists() {
    let tmp = TempDir::new().unwrap();
    fs::create_dir(tmp.path().join(".bare")).unwrap();
    // .bare/HEAD を作成して bare repo に見せかける必要はない
    // ws は .bare ディレクトリの存在だけでエラーにする

    let mut cmd = assert_cmd::Command::cargo_bin("ws").unwrap();
    cmd.arg("clone")
        .current_dir(tmp.path())
        .env("LC_ALL", "en")
        .assert()
        .failure();
}

// ---- ws new ----

#[test]
fn new_creates_worktree() {
    let repo = TestRepo::new();

    repo.ws_cmd()
        .args(["new", "feat-x"])
        .assert()
        .success();

    assert!(repo.path().join("feat-x").is_dir());
}

#[test]
fn new_applies_store_files() {
    let repo = TestRepo::new();

    // store を手動でセットアップ
    repo.init_store();
    repo.add_manifest_entry("copy", ".mcp.json");
    repo.add_store_file(".mcp.json", r#"{"key":"value"}"#);

    // 新しい worktree を作成
    repo.ws_cmd()
        .args(["new", "feat-y"])
        .assert()
        .success();

    // store ファイルが自動配布されている
    let wt_file = repo.path().join("feat-y").join(".mcp.json");
    assert!(wt_file.is_file());
    assert_eq!(fs::read_to_string(&wt_file).unwrap(), r#"{"key":"value"}"#);
}

#[test]
fn new_auto_name() {
    let repo = TestRepo::new();

    let output = repo
        .ws_cmd()
        .arg("new")
        .output()
        .expect("failed to run ws new");
    assert!(output.status.success());

    // bare root 直下に petname 形式のディレクトリが作成されているはず
    let entries: Vec<_> = fs::read_dir(repo.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
        .filter(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            name != ".bare" && name != "main"
        })
        .collect();

    assert_eq!(entries.len(), 1, "Expected exactly one auto-named worktree");
    let name = entries[0].file_name().to_string_lossy().to_string();
    let parts: Vec<&str> = name.split('-').collect();
    assert_eq!(parts.len(), 3, "Expected 3 hyphen-separated words, got: {}", name);
}

// ---- ws rm ----

#[test]
fn rm_removes_worktree() {
    let repo = TestRepo::new();

    // worktree を作成
    repo.ws_cmd()
        .args(["new", "to-remove"])
        .assert()
        .success();

    assert!(repo.path().join("to-remove").is_dir());

    // worktree を削除 (cmd_rm は --git-dir を付与しないので worktree 内から実行)
    let abs_path = repo.path().join("to-remove").to_string_lossy().to_string();
    repo.ws_cmd_in("main")
        .args(["rm", &abs_path])
        .assert()
        .success();

    assert!(!repo.path().join("to-remove").is_dir());
}
