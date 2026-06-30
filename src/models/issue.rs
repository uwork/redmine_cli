use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct IssuesResponse {
    pub issues: Vec<Issue>,
    pub total_count: u32,
    pub offset: u32,
    pub limit: u32,
}

#[derive(Debug, Deserialize)]
pub struct IssueResponse {
    pub issue: Issue,
}

#[derive(Debug, Deserialize)]
pub struct Issue {
    pub id: u32,
    pub subject: String,
    pub status: IdName,
    pub priority: IdName,
    pub author: IdName,
    pub assigned_to: Option<IdName>,
    pub project: IdName,
    pub description: Option<String>,
    pub done_ratio: u32,
    pub created_on: String,
    pub updated_on: String,
}

#[derive(Debug, Deserialize)]
pub struct IdName {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateIssueRequest {
    pub issue: CreateIssue,
}

#[derive(Debug, Serialize)]
pub struct CreateIssue {
    pub project_id: String,
    pub subject: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to_id: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct UpdateIssueRequest {
    pub issue: UpdateIssue,
}

#[derive(Debug, Serialize, Default)]
pub struct UpdateIssue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}
