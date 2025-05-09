use axum::{Json, extract::State, http::StatusCode};

use crate::{
    app_error::AppError,
    models::{app_state::AppState, bookmark::Bookmark},
    services,
};

pub async fn get_bookmarks(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<Bookmark>>), AppError> {
    let conn = state.db_pool.get()?;
    let bookmarks = services::bookmarks::get_all(&conn)?;

    Ok((StatusCode::OK, Json(bookmarks)))
}
