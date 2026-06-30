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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn require_url_returns_error_when_not_set() {
        let config = Config {
            url: None,
            api_key: None,
        };
        assert!(config.require_url().is_err());
    }

    #[test]
    fn require_url_returns_url_when_set() {
        let config = Config {
            url: Some("https://redmine.example.com".to_string()),
            api_key: None,
        };
        assert_eq!(config.require_url().unwrap(), "https://redmine.example.com");
    }

    #[test]
    fn require_api_key_returns_error_when_not_set() {
        let config = Config {
            url: None,
            api_key: None,
        };
        assert!(config.require_api_key().is_err());
    }

    #[test]
    fn require_api_key_returns_key_when_set() {
        let config = Config {
            url: None,
            api_key: Some("secret-key-abc".to_string()),
        };
        assert_eq!(config.require_api_key().unwrap(), "secret-key-abc");
    }

    #[test]
    fn config_default_has_no_url_or_key() {
        let config = Config::default();
        assert!(config.url.is_none());
        assert!(config.api_key.is_none());
    }

    #[test]
    fn config_toml_round_trip() {
        let config = Config {
            url: Some("https://redmine.example.com".to_string()),
            api_key: Some("abc123".to_string()),
        };
        let toml_str = toml::to_string_pretty(&config).unwrap();
        let parsed: Config = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.url, config.url);
        assert_eq!(parsed.api_key, config.api_key);
    }

    #[test]
    fn config_toml_round_trip_with_none_fields() {
        let config = Config {
            url: None,
            api_key: None,
        };
        let toml_str = toml::to_string_pretty(&config).unwrap();
        let parsed: Config = toml::from_str(&toml_str).unwrap();
        assert!(parsed.url.is_none());
        assert!(parsed.api_key.is_none());
    }
}
