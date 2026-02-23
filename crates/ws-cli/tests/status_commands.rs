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

/// worktree 内から status を実行すると Current workspace セクションが表示される
#[test]
fn status_shows_current_workspace_from_worktree() {
    let repo = TestRepo::new();
    repo.ws_cmd_in("main")
        .arg("status")
        .assert()
        .success()
        .stdout(predicate::str::contains("Current workspace:"));
}

/// store が初期化されていて tracked ファイルがある場合、Shared files セクションが表示される
#[test]
fn status_shows_shared_files_when_store_exists() {
    let repo = TestRepo::new();
    repo.init_store();
    repo.add_manifest_entry("symlink", ".envrc");
    repo.add_store_file(".envrc", "use flake");

    repo.ws_cmd_in("main")
        .arg("status")
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Shared files:")
                .and(predicate::str::contains(".envrc"))
                .and(predicate::str::contains("symlink")),
        );
}

/// store がない場合は Shared files セクションが表示されない
#[test]
fn status_without_store_omits_shared_files() {
    let repo = TestRepo::new();
    let output = repo.ws_cmd_in("main").arg("status").output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("STRATEGY"),
        "Expected no STRATEGY header without store, got: {}",
        stdout
    );
    assert!(
        !stdout.contains("Shared files:"),
        "Expected no Shared files section without store, got: {}",
        stdout
    );
}

/// repos が登録されている場合、status に Repositories セクションが表示される
#[test]
fn status_shows_repositories_when_registered() {
    let repo = TestRepo::new();

    // リポジトリを登録
    repo.ws_cmd_in("main")
        .args(["repos", "add", "--name", "test-repo"])
        .assert()
        .success();

    // status 表示
    repo.ws_cmd_in("main")
        .arg("status")
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Repositories:")
                .and(predicate::str::contains("test-repo"))
                .and(predicate::str::contains("GIT_DIR: .bare"))
                .and(predicate::str::contains("Worktrees:")),
        );
}

/// repos が空の場合、リポジトリ外から status を実行すると no_repos メッセージが表示される
#[test]
fn status_outside_repo_without_repos_shows_no_repos() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("ws-config.toml");

    let mut cmd = ws_with_config(&config_path);
    cmd.current_dir(tmp.path());
    cmd.arg("status");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No registered repositories"));
}

/// repos が登録済みでパスが存在しない場合、NOT_FOUND が表示される
#[test]
fn status_shows_not_found_for_missing_repo() {
    let config_dir = TempDir::new().unwrap();
    let config_path = config_dir.path().join("config.toml");

    let config_content = r#"
[repos.broken-repo]
path = "/tmp/nonexistent-ws-test-repo-path"
"#;
    std::fs::write(&config_path, config_content).unwrap();

    let mut cmd = ws_with_config(&config_path);
    cmd.arg("status");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("broken-repo").and(predicate::str::contains("NOT_FOUND")));
}
