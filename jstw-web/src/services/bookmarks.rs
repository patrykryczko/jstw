use std::vec;

use rusqlite::Connection;

use crate::db;
use crate::models::bookmark::Bookmark;

pub fn get_all(conn: &Connection) -> Vec<Bookmark> {
    let data = db::get_all_bookmarks(conn);
    let mut bookmarks: Vec<Bookmark> = vec![];
    for d in data {
        let bookmark = Bookmark {
            id: d.0,
            url: d.1,
            created_at: d.2,
            deleted_at: None,
            edited_at: None,
            title: None,
            description: None,
            img_url: None,
        };
        bookmarks.push(bookmark);
    }

    bookmarks
}

pub fn create(conn: &Connection, url: String) {
    db::insert_bookmark(conn, &url);
}

pub fn delete(conn: &Connection, id: i64) {
    db::delete_bookmark(conn, id);
}
