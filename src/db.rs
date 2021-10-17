use rusqlite::{params, Connection, Result};

const DB_PATH: &str = "/mnt/c/www/mlibrary/db.sqlite";


pub fn create_gallery(path: &str, name: &str) -> Result<i64, rusqlite::Error> {
    let conn = Connection::open(DB_PATH)?;
    conn.execute("INSERT INTO gallery (path, name) VALUES (?1, ?2);", params![path, name])?;
    Ok(conn.last_insert_rowid())
}

pub fn create_image(
    gallery_id: i64,
    path: &str,
    name: &str,
    phash: &str,
    size: u64,
    width: u16,
    height: u16,
) -> Result<i64, rusqlite::Error> {
    let conn = Connection::open(DB_PATH)?;
    conn.execute(
        "INSERT INTO image (gallery_id, path, name, phash, size, width, height) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7);",
        params![gallery_id, path, name, phash, size, width, height],
    )?;
    Ok(conn.last_insert_rowid())
}
