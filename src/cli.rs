use clap::{Parser, Subcommand};
use crate::commands::{config, issues, projects, users};

#[derive(Parser)]
#[command(name = "redmine", about = "Redmine API CLI tool", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 接続設定を管理する
    Config(config::ConfigArgs),
    /// チケットを操作する
    Issues(issues::IssuesArgs),
    /// プロジェクトを表示する
    Projects(projects::ProjectsArgs),
    /// ユーザーを表示する
    Users(users::UsersArgs),
}
