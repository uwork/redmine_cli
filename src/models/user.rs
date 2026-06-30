use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UsersResponse {
    pub users: Vec<User>,
    pub total_count: u32,
}

#[derive(Debug, Deserialize)]
pub struct UserResponse {
    pub user: User,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: u32,
    pub login: String,
    pub firstname: String,
    pub lastname: String,
    pub mail: Option<String>,
    pub created_on: String,
    pub last_login_on: Option<String>,
}

impl User {
    pub fn full_name(&self) -> String {
        format!("{} {}", self.firstname, self.lastname)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_user() -> User {
        User {
            id: 1,
            login: "jdoe".to_string(),
            firstname: "John".to_string(),
            lastname: "Doe".to_string(),
            mail: Some("jdoe@example.com".to_string()),
            created_on: "2024-01-01T00:00:00Z".to_string(),
            last_login_on: None,
        }
    }

    #[test]
    fn full_name_concatenates_first_and_last() {
        let user = make_user();
        assert_eq!(user.full_name(), "John Doe");
    }

    #[test]
    fn user_json_deserialization() {
        let json = r#"{
            "id": 42,
            "login": "alice",
            "firstname": "Alice",
            "lastname": "Smith",
            "mail": "alice@example.com",
            "created_on": "2024-01-15T10:00:00Z",
            "last_login_on": null
        }"#;
        let user: User = serde_json::from_str(json).unwrap();
        assert_eq!(user.id, 42);
        assert_eq!(user.login, "alice");
        assert_eq!(user.full_name(), "Alice Smith");
        assert_eq!(user.mail, Some("alice@example.com".to_string()));
        assert!(user.last_login_on.is_none());
    }

    #[test]
    fn user_json_deserialization_with_last_login() {
        let json = r#"{
            "id": 7,
            "login": "bob",
            "firstname": "Bob",
            "lastname": "Jones",
            "mail": null,
            "created_on": "2023-06-01T00:00:00Z",
            "last_login_on": "2024-06-30T12:00:00Z"
        }"#;
        let user: User = serde_json::from_str(json).unwrap();
        assert_eq!(user.full_name(), "Bob Jones");
        assert!(user.mail.is_none());
        assert_eq!(user.last_login_on.as_deref(), Some("2024-06-30T12:00:00Z"));
    }
}
