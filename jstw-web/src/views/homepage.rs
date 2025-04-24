use crate::{models::app_state::AppState, services::bookmarks};
use axum::{
    Form,
    extract::{Path, State},
};
use maud::{Markup, html};
use rusqlite::Connection;

pub async fn index(State(state): State<AppState>) -> Markup {
    let conn = state
        .db_pool
        .get()
        .map_err(|e| format!("Failed to get DB connection: {}", e))
        .unwrap();

    html! {
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

                (bookmarks_list(&conn))
            }
        }
    }
}

#[derive(serde::Deserialize)]
pub struct AddBookmarkForm {
    url: String,
}

pub async fn add_bookmark(State(state): State<AppState>, form: Form<AddBookmarkForm>) -> Markup {
    let conn = state
        .db_pool
        .get()
        .map_err(|e| format!("Failed to get DB connection: {}", e))
        .unwrap();

    bookmarks::create(&conn, form.url.clone());

    bookmarks_list(&conn)
}

pub async fn delete_bookmark(State(state): State<AppState>, Path(id): Path<i64>) -> Markup {
    let conn = state
        .db_pool
        .get()
        .map_err(|e| format!("Failed to get DB connection: {}", e))
        .unwrap();

    bookmarks::delete(&conn, id);

    bookmarks_list(&conn)
}

fn bookmarks_list(conn: &Connection) -> Markup {
    let bookmarks = bookmarks::get_all(&conn);

    html! {
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
    }
}
