use predicates::prelude::*;

fn ws() -> assert_cmd::Command {
    let mut cmd = assert_cmd::cargo_bin_cmd!("ws");
    cmd.env("LC_ALL", "en");
    cmd
}

#[test]
fn help_shows_all_subcommands() {
    ws().arg("--help")
        .assert()
        .success()
        .stdout(
            predicate::str::contains("clone")
                .and(predicate::str::contains("new"))
                .and(predicate::str::contains("rm"))
                .and(predicate::str::contains("list"))
                .and(predicate::str::contains("status"))
                .and(predicate::str::contains("store"))
                .and(predicate::str::contains("repos")),
        );
}

#[test]
fn store_help_shows_subcommands() {
    ws().args(["store", "--help"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("track")
                .and(predicate::str::contains("status"))
                .and(predicate::str::contains("push"))
                .and(predicate::str::contains("pull"))
                .and(predicate::str::contains("untrack")),
        );
}

#[test]
fn no_subcommand_shows_error() {
    ws().assert().failure();
}

#[test]
fn unknown_subcommand_fails() {
    ws().arg("foobar").assert().failure();
}

#[test]
fn repos_help_shows_subcommands() {
    ws().args(["repos", "--help"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("add")
                .and(predicate::str::contains("list"))
                .and(predicate::str::contains("rm")),
        );
}

#[test]
fn repos_no_subcommand_fails() {
    ws().arg("repos").assert().failure();
}

#[test]
fn repos_rm_missing_name_fails() {
    ws().args(["repos", "rm"]).assert().failure();
}

#[test]
fn store_track_missing_args_fails() {
    // -s なし
    ws().args(["store", "track", ".envrc"])
        .assert()
        .failure();

    // file なし
    ws().args(["store", "track", "-s", "symlink"])
        .assert()
        .failure();
}
