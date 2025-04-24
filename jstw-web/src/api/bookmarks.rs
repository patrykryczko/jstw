use axum::{Json, extract::State, http::StatusCode};

use crate::{
    models::{app_state::AppState, bookmark::Bookmark},
    services,
};

pub async fn get_bookmarks(State(state): State<AppState>) -> (StatusCode, Json<Vec<Bookmark>>) {
    let conn = state
        .db_pool
        .get()
        .map_err(|e| format!("Failed to get DB connection: {}", e))
        .unwrap();

    let bookmarks = services::bookmarks::get_all(&conn);
    (StatusCode::OK, Json(bookmarks))
}
