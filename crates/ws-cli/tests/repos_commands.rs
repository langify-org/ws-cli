mod common;

use common::TestRepo;
use predicates::prelude::*;
use tempfile::TempDir;

fn ws_with_config(config_path: &std::path::Path) -> assert_cmd::Command {
    let mut cmd = assert_cmd::cargo_bin_cmd!("ws");
    cmd.env("LC_ALL", "en");
    cmd.env("WS_CONFIG_PATH", config_path);
    cmd
}

// ---- ws repos add ----

#[test]
fn repos_add_registers_current_dir() {
    let repo = TestRepo::new();
    let config_dir = TempDir::new().unwrap();
    let config_path = config_dir.path().join("config.toml");

    let mut cmd = ws_with_config(&config_path);
    cmd.current_dir(repo.main_worktree());
    cmd.args(["repos", "add"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Registered:"));

    // config に登録されている（bare worktree パターンなので bare root が登録される）
    let content = std::fs::read_to_string(&config_path).unwrap();
    let config: toml::Value = toml::from_str(&content).unwrap();
    let repos = config.get("repos").unwrap().as_table().unwrap();
    assert_eq!(repos.len(), 1);
    let (_name, entry) = repos.iter().next().unwrap();
    let registered_path = entry.get("path").unwrap().as_str().unwrap();
    let bare_root = repo.path().canonicalize().unwrap();
    assert_eq!(
        std::path::Path::new(registered_path),
        bare_root.as_path(),
        "Should register bare root, not worktree"
    );
}

#[test]
fn repos_add_with_explicit_path() {
    let repo = TestRepo::new();
    let config_dir = TempDir::new().unwrap();
    let config_path = config_dir.path().join("config.toml");

    let wt_path = repo.main_worktree();
    let mut cmd = ws_with_config(&config_path);
    cmd.args(["repos", "add", wt_path.to_str().unwrap()]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Registered:"));

    let content = std::fs::read_to_string(&config_path).unwrap();
    let config: toml::Value = toml::from_str(&content).unwrap();
    let repos = config.get("repos").unwrap().as_table().unwrap();
    assert_eq!(repos.len(), 1);
}

#[test]
fn repos_add_with_custom_name() {
    let repo = TestRepo::new();
    let config_dir = TempDir::new().unwrap();
    let config_path = config_dir.path().join("config.toml");

    let mut cmd = ws_with_config(&config_path);
    cmd.current_dir(repo.main_worktree());
    cmd.args(["repos", "add", "--name", "my-repo"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("my-repo"));

    let content = std::fs::read_to_string(&config_path).unwrap();
    let config: toml::Value = toml::from_str(&content).unwrap();
    let repos = config.get("repos").unwrap().as_table().unwrap();
    assert!(repos.contains_key("my-repo"));
}

#[test]
fn repos_add_rejects_non_git_dir() {
    let tmp = TempDir::new().unwrap();
    let config_dir = TempDir::new().unwrap();
    let config_path = config_dir.path().join("config.toml");

    let mut cmd = ws_with_config(&config_path);
    cmd.current_dir(tmp.path());
    cmd.args(["repos", "add"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Not a git repository"));
}

#[test]
fn repos_add_rejects_duplicate_name() {
    let repo = TestRepo::new();
    let config_dir = TempDir::new().unwrap();
    let config_path = config_dir.path().join("config.toml");

    // 1 回目: 成功
    let mut cmd = ws_with_config(&config_path);
    cmd.current_dir(repo.main_worktree());
    cmd.args(["repos", "add", "--name", "dup"]);
    cmd.assert().success();

    // 2 回目: 重複エラー
    let mut cmd = ws_with_config(&config_path);
    cmd.current_dir(repo.main_worktree());
    cmd.args(["repos", "add", "--name", "dup"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("already registered"));
}

// ---- ws repos list ----

#[test]
fn repos_list_shows_registered() {
    let repo = TestRepo::new();
    let config_dir = TempDir::new().unwrap();
    let config_path = config_dir.path().join("config.toml");

    // 登録
    let mut cmd = ws_with_config(&config_path);
    cmd.current_dir(repo.main_worktree());
    cmd.args(["repos", "add", "--name", "test-repo"]);
    cmd.assert().success();

    // リスト表示
    let mut cmd = ws_with_config(&config_path);
    cmd.args(["repos", "list"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test-repo"));
}

#[test]
fn repos_list_empty() {
    let config_dir = TempDir::new().unwrap();
    let config_path = config_dir.path().join("config.toml");

    let mut cmd = ws_with_config(&config_path);
    cmd.args(["repos", "list"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No registered repositories"));
}

// ---- ws repos rm ----

#[test]
fn repos_rm_removes_entry() {
    let repo = TestRepo::new();
    let config_dir = TempDir::new().unwrap();
    let config_path = config_dir.path().join("config.toml");

    // 登録
    let mut cmd = ws_with_config(&config_path);
    cmd.current_dir(repo.main_worktree());
    cmd.args(["repos", "add", "--name", "to-remove"]);
    cmd.assert().success();

    // 削除
    let mut cmd = ws_with_config(&config_path);
    cmd.args(["repos", "rm", "to-remove"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Unregistered: to-remove"));

    // config から消えている
    let content = std::fs::read_to_string(&config_path).unwrap();
    let config: toml::Value = toml::from_str(&content).unwrap();
    let repos = config.get("repos").unwrap().as_table().unwrap();
    assert!(repos.is_empty());
}

#[test]
fn repos_rm_nonexistent_fails() {
    let config_dir = TempDir::new().unwrap();
    let config_path = config_dir.path().join("config.toml");

    let mut cmd = ws_with_config(&config_path);
    cmd.args(["repos", "rm", "nonexistent"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

// ---- ws repos status (旧コマンド) が削除されていることを確認 ----

#[test]
fn repos_status_command_removed() {
    let config_dir = TempDir::new().unwrap();
    let config_path = config_dir.path().join("config.toml");

    let mut cmd = ws_with_config(&config_path);
    cmd.args(["repos", "status"]);
    cmd.assert().failure();
}
