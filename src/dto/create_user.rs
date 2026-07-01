use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUser {
    pub username: String,
}
