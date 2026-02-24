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

/// config にリポジトリを登録するヘルパー
fn register_repo(repo: &TestRepo, config_path: &std::path::Path, name: &str) {
    let mut cmd = ws_with_config(config_path);
    cmd.current_dir(repo.main_worktree());
    cmd.args(["repos", "add", "--name", name]);
    cmd.assert().success();
}

// ---- エラーケース ----

#[test]
fn open_unregistered_repo_fails() {
    let config_dir = TempDir::new().unwrap();
    let config_path = config_dir.path().join("config.toml");

    let mut cmd = ws_with_config(&config_path);
    cmd.env_remove("VISUAL");
    cmd.env_remove("EDITOR");
    cmd.args(["open", "nonexistent", "main"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("not found in config"));
}

#[test]
fn open_nonexistent_worktree_fails() {
    let repo = TestRepo::new();
    let config_dir = TempDir::new().unwrap();
    let config_path = config_dir.path().join("config.toml");
    register_repo(&repo, &config_path, "test-repo");

    let mut cmd = ws_with_config(&config_path);
    cmd.env("EDITOR", "echo");
    cmd.args(["open", "test-repo", "nonexistent"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

#[test]
fn open_without_editor_fails() {
    let repo = TestRepo::new();
    let config_dir = TempDir::new().unwrap();
    let config_path = config_dir.path().join("config.toml");
    register_repo(&repo, &config_path, "test-repo");

    let mut cmd = ws_with_config(&config_path);
    cmd.env_remove("VISUAL");
    cmd.env_remove("EDITOR");
    cmd.args(["open", "test-repo", "main"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No editor found"));
}

// ---- 正常系 ----

#[test]
fn open_with_editor_env() {
    let repo = TestRepo::new();
    let config_dir = TempDir::new().unwrap();
    let config_path = config_dir.path().join("config.toml");
    register_repo(&repo, &config_path, "test-repo");

    let mut cmd = ws_with_config(&config_path);
    cmd.env_remove("VISUAL");
    cmd.env("EDITOR", "echo");
    cmd.args(["open", "test-repo", "main"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Opening"));
}

#[test]
fn open_with_editor_flag() {
    let repo = TestRepo::new();
    let config_dir = TempDir::new().unwrap();
    let config_path = config_dir.path().join("config.toml");
    register_repo(&repo, &config_path, "test-repo");

    let mut cmd = ws_with_config(&config_path);
    cmd.env_remove("VISUAL");
    cmd.env_remove("EDITOR");
    cmd.args(["open", "test-repo", "main", "--editor", "echo"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Opening"));
}

#[test]
fn open_visual_takes_precedence_over_editor() {
    let repo = TestRepo::new();
    let config_dir = TempDir::new().unwrap();
    let config_path = config_dir.path().join("config.toml");
    register_repo(&repo, &config_path, "test-repo");

    let mut cmd = ws_with_config(&config_path);
    cmd.env("VISUAL", "echo");
    cmd.env("EDITOR", "nonexistent-editor");
    cmd.args(["open", "test-repo", "main"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("echo"));
}

#[test]
fn open_flag_takes_precedence_over_env() {
    let repo = TestRepo::new();
    let config_dir = TempDir::new().unwrap();
    let config_path = config_dir.path().join("config.toml");
    register_repo(&repo, &config_path, "test-repo");

    let mut cmd = ws_with_config(&config_path);
    cmd.env("VISUAL", "nonexistent-editor");
    cmd.env("EDITOR", "nonexistent-editor");
    cmd.args(["open", "test-repo", "main", "--editor", "echo"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("echo"));
}
