use axum::{Json, http::StatusCode};
use uuid::Uuid;

use crate::dto::create_user::CreateUser;
use crate::dto::user::User;

pub async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: Uuid::new_v4(),
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}
