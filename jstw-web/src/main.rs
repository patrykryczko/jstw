mod api;
mod db;
mod models;
mod services;
mod views;

use std::sync::Arc;

use axum::Router;
use models::app_state::AppState;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    let app = app().unwrap();

    axum::serve(listener, app).await.unwrap();
}

fn app() -> Result<Router, rusqlite::Error> {
    let manager = SqliteConnectionManager::file("jstw.db");
    let pool = Pool::new(manager).expect("Failed to create pool");

    let app_state = AppState {
        db_pool: Arc::new(pool),
    };

    let conn = app_state.db_pool.get().unwrap();

    db::init(&conn);

    let api_router = api::router::new();
    let views_router = views::router::new();

    let router = Router::new()
        .nest("/api", api_router)
        .merge(views_router)
        .with_state(app_state);

    Ok(router)
}
