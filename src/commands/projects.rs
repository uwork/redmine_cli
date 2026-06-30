use anyhow::Result;
use clap::{Args, Subcommand};
use tabled::{Table, Tabled};
use crate::client::RedmineClient;
use crate::config::Config;

#[derive(Args)]
pub struct ProjectsArgs {
    #[command(subcommand)]
    pub command: ProjectsCommand,
}

#[derive(Subcommand)]
pub enum ProjectsCommand {
    /// プロジェクト一覧を表示する
    List,
    /// プロジェクトの詳細を表示する
    Show { id: String },
}

#[derive(Tabled)]
struct ProjectRow {
    #[tabled(rename = "ID")]
    id: u32,
    #[tabled(rename = "識別子")]
    identifier: String,
    #[tabled(rename = "名前")]
    name: String,
}

pub async fn run(args: ProjectsArgs) -> Result<()> {
    let config = Config::load()?;
    let client = RedmineClient::new(config.require_url()?, config.require_api_key()?)?;

    match args.command {
        ProjectsCommand::List => {
            let res: crate::models::ProjectsResponse =
                client.get_json("/projects.json?limit=100").await?;
            let rows: Vec<ProjectRow> = res.projects.into_iter().map(|p| ProjectRow {
                id: p.id,
                identifier: p.identifier,
                name: p.name,
            }).collect();
            println!("{}", Table::new(rows));
        }
        ProjectsCommand::Show { id } => {
            let res: crate::models::ProjectResponse =
                client.get_json(&format!("/projects/{id}.json")).await?;
            let p = res.project;
            println!("ID       : {}", p.id);
            println!("識別子   : {}", p.identifier);
            println!("名前     : {}", p.name);
            println!("作成日   : {}", p.created_on);
            println!("更新日   : {}", p.updated_on);
            if let Some(desc) = p.description {
                println!("\n{desc}");
            }
        }
    }
    Ok(())
}
