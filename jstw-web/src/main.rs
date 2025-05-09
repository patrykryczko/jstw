mod api;
mod app_error;
mod db;
mod models;
mod services;
mod views;

use std::{env, sync::Arc};

use anyhow::Context;
use app_error::AppError;
use axum::Router;
use models::app_state::AppState;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();

    let port = env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3000);

    let bind_addr = format!("127.0.0.1:{}", port);

    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .context("Failed to bind to 127.0.0.1:3000")?;

    let addr = listener
        .local_addr()
        .context("Failed to get local address")?;

    tracing::debug!("listening on {}", addr);

    let database_name = env::var("DATABASE_NAME").unwrap_or_else(|_| "jstw.db".to_string());
    let app = app(&database_name).context("Failed to initialize application")?;

    axum::serve(listener, app).await.context("Server error")?;

    Ok(())
}

fn app(db_name: &str) -> Result<Router, AppError> {
    let manager = SqliteConnectionManager::file(db_name);
    let pool = Pool::new(manager)?;

    let app_state = AppState {
        db_pool: Arc::new(pool),
    };

    let conn = app_state.db_pool.get()?;

    db::init(&conn)?;

    let api_router = api::router::new();
    let views_router = views::router::new();

    let router = Router::new()
        .nest("/api", api_router)
        .merge(views_router)
        .with_state(app_state);

    Ok(router)
}
