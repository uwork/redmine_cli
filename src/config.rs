use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub url: Option<String>,
    pub api_key: Option<String>,
}

impl Config {
    pub fn path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("redmine_cli")
            .join("config.toml")
    }

    pub fn load() -> Result<Self> {
        let path = Self::path();
        let mut config: Self = if path.exists() {
            let content = std::fs::read_to_string(&path).with_context(|| {
                format!("設定ファイルの読み込みに失敗しました: {}", path.display())
            })?;
            toml::from_str(&content).context("設定ファイルのパースに失敗しました")?
        } else {
            Self::default()
        };

        if let Ok(url) = std::env::var("REDMINE_CLI_URL") {
            config.url = Some(url);
        }
        if let Ok(api_key) = std::env::var("REDMINE_CLI_API_KEY") {
            config.api_key = Some(api_key);
        }

        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::path();
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let content = toml::to_string_pretty(self)?;
        std::fs::write(&path, content)
            .with_context(|| format!("設定ファイルの保存に失敗しました: {}", path.display()))
    }

    pub fn require_url(&self) -> Result<&str> {
        self.url.as_deref().context(
            "Redmine の URL が設定されていません。`redmine config set --url <URL>` で設定してください",
        )
    }

    pub fn require_api_key(&self) -> Result<&str> {
        self.api_key.as_deref().context(
            "API キーが設定されていません。`redmine config set --api-key <KEY>` で設定してください",
        )
    }
}
