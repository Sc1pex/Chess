use router::AppState;
use sqlx::mysql::MySqlPoolOptions;

mod auth;
mod components;
mod models;
mod router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    // Ignore errors if .env file is not found
    let _ = dotenvy::dotenv();

    let db = MySqlPoolOptions::new()
        .connect(&std::env::var("DATABASE_URL")?)
        .await?;
    sqlx::migrate!().run(&db).await?;

    let app = router::app(AppState { pool: db });

    let listener = tokio::net::TcpListener::bind("[::]:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
