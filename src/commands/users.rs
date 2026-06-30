use crate::client::RedmineClient;
use crate::config::Config;
use anyhow::Result;
use clap::{Args, Subcommand};
use tabled::{Table, Tabled};

#[derive(Args)]
pub struct UsersArgs {
    #[command(subcommand)]
    pub command: UsersCommand,
}

#[derive(Subcommand)]
pub enum UsersCommand {
    /// ユーザー一覧を表示する (管理者権限が必要)
    List,
    /// 自分自身の情報を表示する
    Me,
}

#[derive(Tabled)]
struct UserRow {
    #[tabled(rename = "ID")]
    id: u32,
    #[tabled(rename = "ログイン名")]
    login: String,
    #[tabled(rename = "氏名")]
    name: String,
    #[tabled(rename = "メール")]
    mail: String,
}

pub async fn run(args: UsersArgs) -> Result<()> {
    let config = Config::load()?;
    let client = RedmineClient::new(config.require_url()?, config.require_api_key()?)?;

    match args.command {
        UsersCommand::List => {
            let res: crate::models::UsersResponse =
                client.get_json("/users.json?limit=100").await?;
            let rows: Vec<UserRow> = res
                .users
                .into_iter()
                .map(|u| {
                    let name = u.full_name();
                    UserRow {
                        id: u.id,
                        login: u.login,
                        name,
                        mail: u.mail.unwrap_or_default(),
                    }
                })
                .collect();
            println!("{}", Table::new(rows));
        }
        UsersCommand::Me => {
            let res: crate::models::UserResponse = client.get_json("/users/current.json").await?;
            let u = res.user;
            println!("ID       : {}", u.id);
            println!("ログイン名: {}", u.login);
            println!("氏名     : {}", u.full_name());
            println!("メール   : {}", u.mail.unwrap_or_default());
            println!("作成日   : {}", u.created_on);
        }
    }
    Ok(())
}
