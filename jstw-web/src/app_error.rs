use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use maud::html;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),

    #[error("Pool error: {0}")]
    PoolError(#[from] r2d2::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("Application error: {:?}", self);

        let (status, error_message) =
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error occurred");

        let markup = html! {
            html {
                head {
                    title { "Error - " (status.as_u16()) }
                }
                body {
                    h1 { "Something went wrong" }
                    div class="error-container" {
                        h2 class="error-title" { (status.as_u16()) " - " (status.as_str()) }
                        p { (error_message) }
                        @if cfg!(debug_assertions) {
                            p { "Debug details: " (self) }
                        }
                    }
                    div class="back-link" {
                        a href="/" { "Back to home" }
                    }
                }
            }
        };

        (status, markup).into_response()
    }
}
