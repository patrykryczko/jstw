use rusqlite::Connection;

pub fn init(conn: &Connection) {
    let query = "
CREATE TABLE IF NOT EXISTS bookmarks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at REAL NOT NULL,
    edited_at REAL,
    deleted_at REAL,
    url TEXT NOT NULL,
    title TEXT,
    description TEXT,
    img_url TEXT
);
    ";

    conn.execute(query, []).unwrap();
}

pub fn insert_bookmark(
    conn: &Connection,
    url: &str,
    // title: &str,
    // description: &str,
    // img_url: &str,
) {
    let query = "
INSERT INTO bookmarks (created_at, url)
VALUES (strftime('%s', 'now'), ?)
    ";

    conn.execute(query, [url]).unwrap();
}

pub fn get_all_bookmarks(conn: &Connection) -> Vec<(i64, String, f64)> {
    let query = "
SELECT id, url, created_at FROM bookmarks WHERE deleted_at IS NULL
ORDER BY created_at DESC
    ";

    let mut stmt = conn.prepare(query).unwrap();
    let rows = stmt.query_map([], |row| {
        let id: i64 = row.get(0)?;
        let url: String = row.get(1)?;
        let created_at: f64 = row.get(2)?;

        Ok((id, url, created_at))
    });

    rows.unwrap().map(|x| x.unwrap()).collect()
}

pub fn delete_bookmark(conn: &Connection, id: i64) {
    let query = "
UPDATE bookmarks SET deleted_at = strftime('%s', 'now') WHERE id = ?
    ";

    conn.execute(query, [id]).unwrap();
}
