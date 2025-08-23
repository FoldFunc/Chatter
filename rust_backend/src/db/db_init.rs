use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
pub async fn db_init() -> SqlitePool{
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://db/chadder.db")
        .await
        .unwrap();
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            email TEXT NOT NULL,
            password TEXT NOT NULL,
            loggedin TEXT NOT NULL
        )",
    )
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS posts(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            post_owner TEXT NOT NULL,
            post_name TEXT NOT NULL,
            post_body TEXT NOT NULL
        )",
    )
        .execute(&pool)
        .await
        .unwrap();
    pool
}
