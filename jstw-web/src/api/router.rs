use axum::{Router, routing::get};

use crate::AppState;

use super::bookmarks;

pub fn new() -> Router<AppState> {
    Router::new().route("/bookmarks", get(bookmarks::get_bookmarks))
}
