use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;

use crate::AppState;
#[derive(Debug, Deserialize)]
pub struct RegisterRequestStruct {
    username: String,
    email: String,
    password: String,
}
pub trait Validate {
    fn has_empty_fields(&self) -> bool;
}
impl Validate for RegisterRequestStruct {
    fn has_empty_fields(&self) -> bool {
        self.email.is_empty() || self.password.is_empty() || self.username.is_empty()
    }
}
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequestStruct>,
) -> impl IntoResponse {
    println!("Register handler called");
    if payload.has_empty_fields() {
        return (StatusCode::BAD_REQUEST, "Bad request fields").into_response();
    }
    let result = sqlx::query("INSERT INTO users (username, email, password, loggedin) VALUES (?, ?, ?, false)")
        .bind(payload.username)
        .bind(payload.email)
        .bind(payload.password)
        .execute(&*state.pool)
        .await;
    match result {
        Ok(_) => (StatusCode::CREATED, "User created").into_response(),
        Err(e) => {
            eprintln!("database error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create a user").into_response()
        }
    }
}
