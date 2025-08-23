use crate::AppState;
use axum::{
    extract::State,
    http::{header::COOKIE, header, HeaderMap, StatusCode},
    response::IntoResponse,
};
use cookie::{Cookie, CookieJar};
pub async fn logout(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    println!("Logout handler called");

    // Get session_id from the COOKIE header
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

    // Update database
    if sqlx::query("UPDATE users SET loggedin = false WHERE email = ?")
        .bind(&session_email)
        .execute(&*state.pool)
        .await.is_err()
    {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
    }

    // Clear the cookie
    let mut jar = CookieJar::new();
    let cookie = Cookie::build("session_id", "")
        .path("/")
        .max_age(time::Duration::seconds(0)) // expire immediately
        .http_only(true)
        .secure(true)
        .finish();
    jar.add(cookie);

    let mut res = StatusCode::OK.into_response();
    for c in jar.delta() {
        res.headers_mut().append(header::SET_COOKIE, c.to_string().parse().unwrap());
    }

    res
}
