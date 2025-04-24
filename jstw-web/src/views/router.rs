use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::AppState;

use super::homepage::{self};

pub fn new() -> Router<AppState> {
    Router::new()
        .route("/", get(homepage::index))
        .route("/add-bookmark", post(homepage::add_bookmark))
        .route("/delete-bookmark/{id}", delete(homepage::delete_bookmark))
}
