use anyhow::Result;
use clap::{Args, Subcommand};
use crate::config::Config;

#[derive(Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommand,
}

#[derive(Subcommand)]
pub enum ConfigCommand {
    /// URL と API キーを設定する
    Set {
        #[arg(long, help = "Redmine の URL")]
        url: Option<String>,
        #[arg(long, help = "API キー")]
        api_key: Option<String>,
    },
    /// 現在の設定を表示する
    Show,
}

pub async fn run(args: ConfigArgs) -> Result<()> {
    match args.command {
        ConfigCommand::Set { url, api_key } => {
            let mut config = Config::load()?;
            if let Some(u) = url {
                config.url = Some(u);
            }
            if let Some(k) = api_key {
                config.api_key = Some(k);
            }
            config.save()?;
            println!("設定を保存しました: {}", Config::path().display());
        }
        ConfigCommand::Show => {
            let config = Config::load()?;
            println!("url     : {}", config.url.as_deref().unwrap_or("(未設定)"));
            println!("api_key : {}", config.api_key.as_deref().unwrap_or("(未設定)"));
        }
    }
    Ok(())
}
