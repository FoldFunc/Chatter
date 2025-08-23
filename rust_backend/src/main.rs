use std::sync::Arc;
use axum::{routing::post, Router};
mod handlers;
mod db;
use handlers::register_handler::register;
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
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:42069").await.unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, server.into_make_service()).await.unwrap();
}
