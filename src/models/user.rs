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
