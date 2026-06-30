use anyhow::Result;
use clap::{Args, Subcommand};
use tabled::{Table, Tabled};
use crate::client::RedmineClient;
use crate::config::Config;
use crate::models::{CreateIssue, CreateIssueRequest, UpdateIssue, UpdateIssueRequest};

#[derive(Args)]
pub struct IssuesArgs {
    #[command(subcommand)]
    pub command: IssuesCommand,
}

#[derive(Subcommand)]
pub enum IssuesCommand {
    /// チケット一覧を表示する
    List {
        #[arg(short, long, help = "プロジェクト識別子")]
        project: Option<String>,
        #[arg(short, long, help = "担当者 ID (me を指定すると自分)")]
        assigned_to: Option<String>,
        #[arg(short, long, default_value = "25", help = "最大件数")]
        limit: u32,
    },
    /// チケットの詳細を表示する
    Show {
        id: u32,
    },
    /// チケットを作成する
    Create {
        #[arg(short, long, help = "プロジェクト識別子")]
        project: String,
        #[arg(short, long, help = "件名")]
        subject: String,
        #[arg(short, long, help = "説明")]
        description: Option<String>,
    },
    /// チケットを更新する
    Update {
        id: u32,
        #[arg(long, help = "ステータス ID")]
        status: Option<u32>,
        #[arg(long, help = "担当者 ID")]
        assigned_to: Option<u32>,
        #[arg(long, help = "コメント")]
        notes: Option<String>,
    },
}

#[derive(Tabled)]
struct IssueRow {
    #[tabled(rename = "ID")]
    id: u32,
    #[tabled(rename = "プロジェクト")]
    project: String,
    #[tabled(rename = "ステータス")]
    status: String,
    #[tabled(rename = "優先度")]
    priority: String,
    #[tabled(rename = "件名")]
    subject: String,
    #[tabled(rename = "担当者")]
    assigned_to: String,
}

pub async fn run(args: IssuesArgs) -> Result<()> {
    let config = Config::load()?;
    let client = RedmineClient::new(config.require_url()?, config.require_api_key()?)?;

    match args.command {
        IssuesCommand::List { project, assigned_to, limit } => {
            let mut query = format!("/issues.json?limit={limit}&status_id=open");
            if let Some(p) = project {
                query.push_str(&format!("&project_id={p}"));
            }
            if let Some(a) = assigned_to {
                query.push_str(&format!("&assigned_to_id={a}"));
            }
            let res: crate::models::IssuesResponse = client.get_json(&query).await?;
            let rows: Vec<IssueRow> = res.issues.into_iter().map(|i| IssueRow {
                id: i.id,
                project: i.project.name,
                status: i.status.name,
                priority: i.priority.name,
                subject: i.subject,
                assigned_to: i.assigned_to.map(|a| a.name).unwrap_or_default(),
            }).collect();
            println!("{}", Table::new(rows));
        }
        IssuesCommand::Show { id } => {
            let res: crate::models::IssueResponse =
                client.get_json(&format!("/issues/{id}.json")).await?;
            let i = res.issue;
            println!("ID       : {}", i.id);
            println!("プロジェクト: {}", i.project.name);
            println!("件名     : {}", i.subject);
            println!("ステータス: {}", i.status.name);
            println!("優先度   : {}", i.priority.name);
            println!("担当者   : {}", i.assigned_to.map(|a| a.name).unwrap_or_default());
            println!("作成者   : {}", i.author.name);
            println!("進捗    : {}%", i.done_ratio);
            println!("作成日   : {}", i.created_on);
            println!("更新日   : {}", i.updated_on);
            if let Some(desc) = i.description {
                println!("\n{desc}");
            }
        }
        IssuesCommand::Create { project, subject, description } => {
            let body = CreateIssueRequest {
                issue: CreateIssue {
                    project_id: project,
                    subject,
                    description,
                    priority_id: None,
                    assigned_to_id: None,
                },
            };
            let res: crate::models::IssueResponse =
                client.post_json("/issues.json", &body).await?;
            println!("チケットを作成しました: #{}", res.issue.id);
        }
        IssuesCommand::Update { id, status, assigned_to, notes } => {
            let body = UpdateIssueRequest {
                issue: UpdateIssue {
                    status_id: status,
                    assigned_to_id: assigned_to,
                    notes,
                    ..Default::default()
                },
            };
            client.put_json(&format!("/issues/{id}.json"), &body).await?;
            println!("チケット #{id} を更新しました");
        }
    }
    Ok(())
}
