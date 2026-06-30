use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ProjectsResponse {
    pub projects: Vec<Project>,
    pub total_count: u32,
}

#[derive(Debug, Deserialize)]
pub struct ProjectResponse {
    pub project: Project,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    pub id: u32,
    pub name: String,
    pub identifier: String,
    pub description: Option<String>,
    pub status: u32,
    pub created_on: String,
    pub updated_on: String,
}
