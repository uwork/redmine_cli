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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn issue_json_deserialization() {
        let json = r#"{
            "id": 1,
            "subject": "Test issue",
            "status": {"id": 1, "name": "New"},
            "priority": {"id": 2, "name": "Normal"},
            "author": {"id": 10, "name": "Alice"},
            "assigned_to": null,
            "project": {"id": 3, "name": "My Project"},
            "description": "Some description",
            "done_ratio": 50,
            "created_on": "2024-01-01T00:00:00Z",
            "updated_on": "2024-01-02T00:00:00Z"
        }"#;
        let issue: Issue = serde_json::from_str(json).unwrap();
        assert_eq!(issue.id, 1);
        assert_eq!(issue.subject, "Test issue");
        assert_eq!(issue.status.name, "New");
        assert_eq!(issue.priority.name, "Normal");
        assert_eq!(issue.author.name, "Alice");
        assert!(issue.assigned_to.is_none());
        assert_eq!(issue.done_ratio, 50);
    }

    #[test]
    fn issue_json_deserialization_with_assigned_to() {
        let json = r#"{
            "id": 5,
            "subject": "Assigned issue",
            "status": {"id": 2, "name": "In Progress"},
            "priority": {"id": 1, "name": "High"},
            "author": {"id": 1, "name": "Admin"},
            "assigned_to": {"id": 3, "name": "Bob"},
            "project": {"id": 1, "name": "Project A"},
            "description": null,
            "done_ratio": 25,
            "created_on": "2024-01-01T00:00:00Z",
            "updated_on": "2024-01-05T00:00:00Z"
        }"#;
        let issue: Issue = serde_json::from_str(json).unwrap();
        assert_eq!(issue.assigned_to.as_ref().unwrap().name, "Bob");
        assert!(issue.description.is_none());
    }

    #[test]
    fn create_issue_omits_none_fields() {
        let req = CreateIssueRequest {
            issue: CreateIssue {
                project_id: "my-project".to_string(),
                subject: "Bug found".to_string(),
                description: None,
                priority_id: None,
                assigned_to_id: None,
            },
        };
        let json = serde_json::to_value(&req).unwrap();
        let obj = json["issue"].as_object().unwrap();
        assert!(obj.contains_key("project_id"));
        assert!(obj.contains_key("subject"));
        assert!(!obj.contains_key("description"));
        assert!(!obj.contains_key("priority_id"));
        assert!(!obj.contains_key("assigned_to_id"));
    }

    #[test]
    fn create_issue_includes_set_optional_fields() {
        let req = CreateIssueRequest {
            issue: CreateIssue {
                project_id: "proj".to_string(),
                subject: "Task".to_string(),
                description: Some("Details here".to_string()),
                priority_id: Some(2),
                assigned_to_id: Some(5),
            },
        };
        let json = serde_json::to_value(&req).unwrap();
        let obj = json["issue"].as_object().unwrap();
        assert_eq!(obj["description"], "Details here");
        assert_eq!(obj["priority_id"], 2);
        assert_eq!(obj["assigned_to_id"], 5);
    }

    #[test]
    fn update_issue_default_serializes_to_empty_object() {
        let req = UpdateIssueRequest {
            issue: UpdateIssue::default(),
        };
        let json = serde_json::to_value(&req).unwrap();
        let obj = json["issue"].as_object().unwrap();
        assert!(obj.is_empty());
    }

    #[test]
    fn update_issue_includes_only_set_fields() {
        let req = UpdateIssueRequest {
            issue: UpdateIssue {
                status_id: Some(3),
                notes: Some("Fixed".to_string()),
                ..Default::default()
            },
        };
        let json = serde_json::to_value(&req).unwrap();
        let obj = json["issue"].as_object().unwrap();
        assert_eq!(obj["status_id"], 3);
        assert_eq!(obj["notes"], "Fixed");
        assert!(!obj.contains_key("subject"));
        assert!(!obj.contains_key("priority_id"));
        assert!(!obj.contains_key("assigned_to_id"));
    }
}
