use crate::{app_error::AppError, models::app_state::AppState, services::bookmarks};
use axum::{
    Form,
    extract::{Path, State},
};
use maud::{Markup, html};
use rusqlite::Connection;

pub async fn index(State(state): State<AppState>) -> Result<Markup, AppError> {
    let conn = state.db_pool.get()?;

    let markup = html! {
        html {
            head {
                script src="https://unpkg.com/htmx.org@2.0.4" {}
            }
            body {
                h2 { "Bookmarks" }

                form
                    id="bookmark-form"
                    hx-post="/add-bookmark"
                    hx-target="#bookmarks-list"
                    hx-swap="outerHTML"
                    hx-on:htmx:after-on-load="document.getElementById('bookmark-form').reset()" {
                        input type="url" name="url" placeholder="Enter URL to bookmark" required;
                        button type="submit" { "Add Bookmark" }
                    }

                (bookmarks_list(&conn)?)
            }
        }
    };

    Ok(markup)
}

#[derive(serde::Deserialize)]
pub struct AddBookmarkForm {
    url: String,
}

pub async fn add_bookmark(
    State(state): State<AppState>,
    form: Form<AddBookmarkForm>,
) -> Result<Markup, AppError> {
    let conn = state.db_pool.get()?;

    bookmarks::create(&conn, form.url.clone())?;

    Ok(bookmarks_list(&conn)?)
}

pub async fn delete_bookmark(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Markup, AppError> {
    let conn = state.db_pool.get()?;

    bookmarks::delete(&conn, id)?;

    Ok(bookmarks_list(&conn)?)
}

fn bookmarks_list(conn: &Connection) -> Result<Markup, AppError> {
    let bookmarks = bookmarks::get_all(&conn)?;

    let markup = html! {
        ul id="bookmarks-list" {
            @for bookmark in bookmarks {
                li id=(bookmark.id) {
                    a href=(bookmark.url) { (bookmark.url) }
                    button
                        hx-delete={ "/delete-bookmark/" (bookmark.id)}
                        hx-target="#bookmarks-list"
                        hx-swap="outerHTML" {
                            "X"
                        }
                }
            }
        }
    };

    Ok(markup)
}
