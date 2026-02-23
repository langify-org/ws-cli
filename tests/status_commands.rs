mod common;

use common::TestRepo;
use predicates::prelude::*;

/// bare root から status を実行すると Workspaces セクションが表示される
#[test]
fn status_shows_workspaces_from_bare_root() {
    let repo = TestRepo::new();
    repo.ws_cmd()
        .arg("status")
        .assert()
        .success()
        .stdout(predicate::str::contains("Workspaces:"));
}

/// worktree 内から status を実行すると Workspaces セクションが表示される
#[test]
fn status_shows_workspaces_from_worktree() {
    let repo = TestRepo::new();
    repo.ws_cmd_in("main")
        .arg("status")
        .assert()
        .success()
        .stdout(predicate::str::contains("Workspaces:"));
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
