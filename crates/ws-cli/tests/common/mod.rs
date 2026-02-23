#![allow(dead_code)]

use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

/// テスト用の bare clone + worktree 環境を構築するヘルパー
pub struct TestRepo {
    pub tempdir: TempDir,
    pub root: PathBuf,
    pub config_path: PathBuf,
}

impl TestRepo {
    /// 空の bare リポジトリ + main worktree + 初期コミットを構築
    pub fn new() -> Self {
        let tempdir = TempDir::new().expect("Failed to create tempdir");
        let root = tempdir.path().to_path_buf();

        // git init --bare .bare
        let out = Command::new("git")
            .args(["init", "--bare", ".bare"])
            .current_dir(&root)
            .output()
            .expect("git init --bare failed");
        assert!(out.status.success(), "git init --bare failed");

        // orphan worktree を作成
        let out = Command::new("git")
            .args([
                "--git-dir",
                ".bare",
                "worktree",
                "add",
                "--orphan",
                "-b",
                "main",
                "main",
            ])
            .current_dir(&root)
            .output()
            .expect("worktree add failed");
        assert!(out.status.success(), "worktree add failed");

        // 初期コミットを作成
        let main_dir = root.join("main");
        std::fs::write(main_dir.join("README.md"), "# test\n").unwrap();

        let out = Command::new("git")
            .args(["add", "."])
            .current_dir(&main_dir)
            .output()
            .expect("git add failed");
        assert!(out.status.success(), "git add failed");

        let out = Command::new("git")
            .args([
                "-c",
                "user.name=Test",
                "-c",
                "user.email=test@test.com",
                "commit",
                "-m",
                "initial",
            ])
            .current_dir(&main_dir)
            .output()
            .expect("git commit failed");
        assert!(out.status.success(), "git commit failed");

        let config_path = root.join("ws-config.toml");

        TestRepo {
            tempdir,
            root,
            config_path,
        }
    }

    /// bare root のパス
    pub fn path(&self) -> &Path {
        &self.root
    }

    /// .bare ディレクトリのパス
    pub fn bare_dir(&self) -> PathBuf {
        self.root.join(".bare")
    }

    /// main worktree のパス
    pub fn main_worktree(&self) -> PathBuf {
        self.root.join("main")
    }

    /// store ディレクトリのパス (.bare/worktree-store)
    pub fn store_dir(&self) -> PathBuf {
        // canonicalize して実パスを取得（macOS の /private/var/... 対策）
        std::fs::canonicalize(self.bare_dir())
            .unwrap()
            .join("worktree-store")
    }

    /// store を初期化 (manifest ファイルを作成)
    pub fn init_store(&self) {
        let store = self.store_dir();
        std::fs::create_dir_all(&store).unwrap();
        std::fs::write(store.join("manifest"), "").unwrap();
    }

    /// manifest にエントリを追加
    pub fn add_manifest_entry(&self, strategy: &str, filepath: &str) {
        let manifest = self.store_dir().join("manifest");
        let mut content = std::fs::read_to_string(&manifest).unwrap_or_default();
        content.push_str(&format!("{}:{}\n", strategy, filepath));
        std::fs::write(&manifest, content).unwrap();
    }

    /// store にファイルを配置
    pub fn add_store_file(&self, filepath: &str, content: &str) {
        let path = self.store_dir().join(filepath);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(path, content).unwrap();
    }

    /// ws バイナリを bare root の cwd で実行する Command を生成
    pub fn ws_cmd(&self) -> assert_cmd::Command {
        let mut cmd = assert_cmd::cargo_bin_cmd!("ws");
        cmd.current_dir(&self.root);
        cmd.env("LC_ALL", "en");
        cmd.env("WS_CONFIG_PATH", &self.config_path);
        cmd
    }

    /// ws バイナリを指定 worktree の cwd で実行する Command を生成
    pub fn ws_cmd_in(&self, worktree: &str) -> assert_cmd::Command {
        let mut cmd = assert_cmd::cargo_bin_cmd!("ws");
        cmd.current_dir(self.root.join(worktree));
        cmd.env("LC_ALL", "en");
        cmd.env("WS_CONFIG_PATH", &self.config_path);
        cmd
    }
}
