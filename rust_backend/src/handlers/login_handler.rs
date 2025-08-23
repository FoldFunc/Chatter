use crate::AppState;
use axum::{
    extract::State,
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use cookie::{Cookie, CookieJar};
use serde::Deserialize;
use sqlx::Row;
use time::Duration;
use crate::handlers::register_handler::Validate;

#[derive(Debug, Deserialize)]
pub struct LoginRequestStruct {
    email: String,
    password: String,
}

impl Validate for LoginRequestStruct {
    fn has_empty_fields(&self) -> bool {
        self.email.is_empty() || self.password.is_empty()
    }
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequestStruct>,
) -> impl IntoResponse {
    println!("Login handler called");

    if payload.has_empty_fields() {
        return (StatusCode::BAD_REQUEST, "Email or password cannot be empty").into_response();
    }

    // Query the database
    let row = match sqlx::query("SELECT email, password FROM users WHERE email = ?")
        .bind(&payload.email)
        .fetch_optional(&*state.pool)
        .await
    {
        Ok(opt) => opt,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    };

    // Check if user exists
    let row = match row {
        Some(r) => r,
        None => return (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response(),
    };
    if sqlx::query("UPDATE users SET loggedin = true WHERE email = ?")
        .bind(payload.email.clone())
        .execute(&*state.pool)
        .await.is_err()
    {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
    }
    let stored_password: String = row.try_get("password").unwrap();

    // Check password
    if payload.password != stored_password {
        return (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response();
    }

    // Create session cookie
    let mut jar = CookieJar::new();
    let cookie = Cookie::build("session_id", payload.email.clone())
        .path("/")
        .max_age(Duration::days(1))
        .http_only(true)
        .secure(true)
        .finish();
    jar.add(cookie);

    let mut res = (StatusCode::OK, "Logged in").into_response();
    for c in jar.delta() {
        res.headers_mut().append(header::SET_COOKIE, c.to_string().parse().unwrap());
    }
    res
}
