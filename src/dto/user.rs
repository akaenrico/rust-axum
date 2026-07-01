use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct User {
    pub id: u64,
    pub username: String,
}
