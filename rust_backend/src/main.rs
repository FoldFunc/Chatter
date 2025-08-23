use std::sync::Arc;
use axum::routing::delete;
use axum::{routing::post, routing::get, Router};
mod handlers;
mod db;
use handlers::register_handler::register;
use handlers::login_handler::login;
use handlers::logout_handler::logout;
use handlers::delete_user::user_delete;
use handlers::post_new::post_new;
use handlers::post_delete::post_delete;
use handlers::post_update::post_update;
#[derive(Clone)]
struct AppState {
    pool: Arc<sqlx::SqlitePool>,
}
#[tokio::main]
async fn main() {
    let pool = db::db_init::db_init().await;
    let state = AppState {
        pool: Arc::new(pool),
    };
    let server = Router::new()
        .route("/user/register", post(register))
        .route("/user/login", post(login))
        .route("/user/logout", get(logout))
        .route("/user/delete", delete(user_delete))
        .route("/post/new", post(post_new))
        .route("/post/delete", delete(post_delete))
        .route("/post/update", post(post_update))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:42069").await.unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, server.into_make_service()).await.unwrap();
}
