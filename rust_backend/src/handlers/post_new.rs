use crate::AppState;
use sqlx::Row;
use axum::{
    extract::State,
    http::{header::COOKIE, HeaderMap, StatusCode},
    response::IntoResponse, Json,
};
use cookie::Cookie;
use serde::Deserialize;

use super::register_handler::Validate;
#[derive(Debug, Deserialize)]
pub struct PostNewRequest {
    post_name: String,
    post_body: String,
}
impl Validate for PostNewRequest {
    fn has_empty_fields(&self) -> bool {
        self.post_body.is_empty() || self.post_name.is_empty()
    } 
}
pub async fn post_new(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<PostNewRequest>,
) -> impl IntoResponse {
    println!("Post new handler called");

    // Extract session cookie
    let session_email = headers
        .get(COOKIE)
        .and_then(|cookie_header| cookie_header.to_str().ok())
        .and_then(|cookie_str| {
            cookie_str
                .split(';')
                .filter_map(|c| Cookie::parse(c.trim()).ok())
                .find(|c| c.name() == "session_id")
                .map(|c| c.value().to_string())
        });

    let session_email = match session_email {
        Some(email) => email,
        None => return (StatusCode::BAD_REQUEST, "No session found").into_response(),
    };

    // Validate payload
    if payload.has_empty_fields() {
        return (StatusCode::BAD_REQUEST, "Invalid request body").into_response();
    }

    // Fetch user
    let user_row = match sqlx::query("SELECT username FROM users WHERE email = ?")
        .bind(&session_email)
        .fetch_optional(&*state.pool)
        .await
    {
        Ok(Some(row)) => row,
        Ok(None) => return (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    };

    let user_name: String = user_row.try_get("username").unwrap();

    // Insert post
    if sqlx::query("INSERT INTO posts (post_owner, post_name, post_body) VALUES (?, ?, ?)")
        .bind(user_name)
        .bind(payload.post_name)
        .bind(payload.post_body)
        .execute(&*state.pool)
        .await.is_err()
    {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
    }

    (StatusCode::OK, "Post created").into_response()
}
