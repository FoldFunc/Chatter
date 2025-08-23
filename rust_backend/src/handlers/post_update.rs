use crate::AppState;
use axum::{
    extract::State,
    http::{header::COOKIE, HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use cookie::Cookie;
use serde::Deserialize;
use sqlx::Row;
use crate::handlers::register_handler::Validate;

#[derive(Debug, Deserialize)]
pub struct PostUpdateRequestStruct{
    post_name: String,
    post_body: String,
}

impl Validate for PostUpdateRequestStruct{
    fn has_empty_fields(&self) -> bool {
        self.post_body.is_empty() || self.post_name.is_empty()
    }
}

pub async fn post_update(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<PostUpdateRequestStruct>,
) -> impl IntoResponse {
    println!("Update post handler called");

    if payload.has_empty_fields() {
        return (StatusCode::BAD_REQUEST, "Email or password cannot be empty").into_response();
    }
    let session_email = if let Some(cookie_header) = headers.get(COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            cookie_str
                .split(';')
                .filter_map(|c| Cookie::parse(c.trim()).ok())
                .find(|c| c.name() == "session_id")
                .map(|c| c.value().to_string())
        } else {
            None
        }
    } else {
        None
    };

    let session_email = match session_email {
        Some(email) => email,
        None => return (StatusCode::BAD_REQUEST, "No session found").into_response(),
    };
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
    // Update database
    if sqlx::query("UPDATE posts SET post_name = ?, post_body = ? WHERE post_owner = ?")
        .bind(payload.post_name)
        .bind(payload.post_body)
        .bind(user_name)
        .execute(&*state.pool)
        .await.is_err()
    {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
    }
    (StatusCode::OK, "Post updated").into_response()
}
