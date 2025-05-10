use rusqlite::Connection;

pub fn init(conn: &Connection) -> Result<(), rusqlite::Error> {
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
) STRICT;
    ";

    conn.execute(query, [])?;

    Ok(())
}

pub fn insert_bookmark(
    conn: &Connection,
    url: &str,
    // title: &str,
    // description: &str,
    // img_url: &str,
) -> Result<(), rusqlite::Error> {
    let query = "
INSERT INTO bookmarks (created_at, url)
VALUES (strftime('%s', 'now'), ?)
    ";

    conn.execute(query, [url])?;

    Ok(())
}

pub fn get_all_bookmarks(conn: &Connection) -> Result<Vec<(i64, String, f64)>, rusqlite::Error> {
    let query = "
SELECT id, url, created_at FROM bookmarks WHERE deleted_at IS NULL
ORDER BY created_at DESC
    ";

    let mut stmt = conn.prepare(query)?;
    let rows = stmt.query_map([], |row| {
        let id: i64 = row.get(0)?;
        let url: String = row.get(1)?;
        let created_at: f64 = row.get(2)?;

        Ok((id, url, created_at))
    });

    rows?.collect()
}

pub fn delete_bookmark(conn: &Connection, id: i64) -> Result<(), rusqlite::Error> {
    let query = "
UPDATE bookmarks SET deleted_at = strftime('%s', 'now') WHERE id = ?
    ";

    conn.execute(query, [id])?;

    Ok(())
}
