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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_json_deserialization() {
        let json = r#"{
            "id": 1,
            "name": "My Project",
            "identifier": "my-project",
            "description": "A test project",
            "status": 1,
            "created_on": "2024-01-01T00:00:00Z",
            "updated_on": "2024-01-02T00:00:00Z"
        }"#;
        let project: Project = serde_json::from_str(json).unwrap();
        assert_eq!(project.id, 1);
        assert_eq!(project.name, "My Project");
        assert_eq!(project.identifier, "my-project");
        assert_eq!(project.description, Some("A test project".to_string()));
        assert_eq!(project.status, 1);
    }

    #[test]
    fn project_json_deserialization_without_description() {
        let json = r#"{
            "id": 2,
            "name": "No Desc Project",
            "identifier": "no-desc",
            "description": null,
            "status": 1,
            "created_on": "2024-03-01T00:00:00Z",
            "updated_on": "2024-03-01T00:00:00Z"
        }"#;
        let project: Project = serde_json::from_str(json).unwrap();
        assert!(project.description.is_none());
        assert_eq!(project.identifier, "no-desc");
    }

    #[test]
    fn projects_response_deserialization() {
        let json = r#"{
            "projects": [
                {
                    "id": 1,
                    "name": "Alpha",
                    "identifier": "alpha",
                    "description": null,
                    "status": 1,
                    "created_on": "2024-01-01T00:00:00Z",
                    "updated_on": "2024-01-01T00:00:00Z"
                }
            ],
            "total_count": 1
        }"#;
        let res: ProjectsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(res.total_count, 1);
        assert_eq!(res.projects.len(), 1);
        assert_eq!(res.projects[0].name, "Alpha");
    }
}
