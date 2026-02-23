mod common;

use common::TestRepo;
use std::fs;
use tempfile::TempDir;

// ---- ws clone ----

#[test]
fn clone_creates_bare_repo() {
    let tmp = TempDir::new().unwrap();

    let mut cmd = assert_cmd::cargo_bin_cmd!("ws");
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

    let mut cmd = assert_cmd::cargo_bin_cmd!("ws");
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

    repo.ws_cmd().args(["new", "feat-x"]).assert().success();

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
    repo.ws_cmd().args(["new", "feat-y"]).assert().success();

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
    assert_eq!(
        parts.len(),
        3,
        "Expected 3 hyphen-separated words, got: {}",
        name
    );
}

// ---- ws rm ----

#[test]
fn rm_removes_worktree() {
    let repo = TestRepo::new();

    // worktree を作成
    repo.ws_cmd().args(["new", "to-remove"]).assert().success();

    assert!(repo.path().join("to-remove").is_dir());

    // worktree を削除 (cmd_rm は --git-dir を付与しないので worktree 内から実行)
    let abs_path = repo.path().join("to-remove").to_string_lossy().to_string();
    repo.ws_cmd_in("main")
        .args(["rm", &abs_path])
        .assert()
        .success();

    assert!(!repo.path().join("to-remove").is_dir());
}

// ---- config registration ----

#[test]
fn clone_registers_in_config() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("ws-config.toml");

    let mut cmd = assert_cmd::cargo_bin_cmd!("ws");
    cmd.arg("clone")
        .current_dir(tmp.path())
        .env("LC_ALL", "en")
        .env("WS_CONFIG_PATH", &config_path)
        .assert()
        .success();

    // config.toml が作成されている
    assert!(config_path.exists(), "config.toml should be created");

    // config の中身を検証
    let content = fs::read_to_string(&config_path).unwrap();
    let config: toml::Value = toml::from_str(&content).unwrap();
    let repos = config.get("repos").expect("repos section should exist");
    let repos_table = repos.as_table().expect("repos should be a table");
    assert_eq!(repos_table.len(), 1, "Expected exactly one repo entry");

    // エントリの path が実際のディレクトリを指している
    let (_name, entry) = repos_table.iter().next().unwrap();
    let path_str = entry
        .get("path")
        .expect("path field should exist")
        .as_str()
        .expect("path should be a string");
    assert!(
        std::path::Path::new(path_str).exists(),
        "Registered path should exist: {}",
        path_str
    );
}
