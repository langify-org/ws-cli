use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use rust_i18n::t;

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct Config {
    #[serde(default)]
    pub repos: BTreeMap<String, RepoEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RepoEntry {
    pub path: PathBuf,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// config.toml のパスを返す。
/// 優先順位: `WS_CONFIG_PATH` > `XDG_CONFIG_HOME/ws/config.toml` > `~/.config/ws/config.toml`
pub(crate) fn config_path() -> Result<PathBuf> {
    if let Ok(p) = std::env::var("WS_CONFIG_PATH") {
        return Ok(PathBuf::from(p));
    }
    let config_dir = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| "~".to_string()))
                .join(".config")
        });
    Ok(config_dir.join("ws").join("config.toml"))
}

fn load_config_from(path: &Path) -> Result<Config> {
    if !path.exists() {
        return Ok(Config::default());
    }
    let content = std::fs::read_to_string(path)
        .with_context(|| t!("config.read_failed", path = path.display().to_string()).to_string())?;
    let config: Config = toml::from_str(&content)
        .with_context(|| t!("config.parse_failed", path = path.display().to_string()).to_string())?;
    Ok(config)
}

fn save_config_to(config: &Config, path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| t!("config.mkdir_failed", path = parent.display().to_string()).to_string())?;
    }
    let content = toml::to_string_pretty(config)
        .context(t!("config.serialize_failed").to_string())?;
    std::fs::write(path, content)
        .with_context(|| t!("config.write_failed", path = path.display().to_string()).to_string())?;
    Ok(())
}

/// config.toml を読み込む。ファイルが存在しなければ空の Config を返す。
pub(crate) fn load_config() -> Result<Config> {
    load_config_from(&config_path()?)
}

/// config.toml を保存する。親ディレクトリが存在しなければ作成する。
pub(crate) fn save_config(config: &Config) -> Result<()> {
    save_config_to(config, &config_path()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn load_config_returns_default_for_missing_file() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("nonexistent").join("config.toml");
        let config = load_config_from(&path).unwrap();
        assert!(config.repos.is_empty());
    }

    #[test]
    fn save_then_load_roundtrip() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("ws").join("config.toml");

        let mut config = Config::default();
        config.repos.insert(
            "my-repo".to_string(),
            RepoEntry {
                path: PathBuf::from("/home/user/projects/my-repo"),
                url: Some("git@github.com:user/my-repo.git".to_string()),
            },
        );
        save_config_to(&config, &path).unwrap();

        let loaded = load_config_from(&path).unwrap();

        assert_eq!(loaded.repos.len(), 1);
        let entry = loaded.repos.get("my-repo").unwrap();
        assert_eq!(entry.path, PathBuf::from("/home/user/projects/my-repo"));
        assert_eq!(
            entry.url.as_deref(),
            Some("git@github.com:user/my-repo.git")
        );
    }

    #[test]
    fn load_config_errors_on_malformed_toml() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("config.toml");
        std::fs::write(&path, "this is not valid { toml [[[").unwrap();
        let result = load_config_from(&path);
        assert!(result.is_err());
    }
}
