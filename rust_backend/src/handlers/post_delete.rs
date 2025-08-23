use crate::AppState;
use sqlx::Row;
use axum::{
    extract::State,
    http::{header::COOKIE, HeaderMap, StatusCode},
    response::IntoResponse,
};
use cookie::Cookie;
pub async fn post_delete(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    println!("Delete post handler called");
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
    if sqlx::query("DELETE FROM posts WHERE post_owner = ?")
        .bind(user_name)
        .execute(&*state.pool)
        .await.is_err()
    {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
    }
    (StatusCode::OK, "Post deleted").into_response()
}
