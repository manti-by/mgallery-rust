use rusqlite::{params, Connection, Result};

pub fn create_gallery(
    connection: &mut Connection,
    path: &str,
    name: &str,
) -> Result<i64, rusqlite::Error> {
    connection.execute(
        "INSERT INTO gallery (path, name) VALUES (?1, ?2);",
        params![path, name],
    )?;
    Ok(connection.last_insert_rowid())
}

pub fn create_image(
    connection: &mut Connection,
    gallery_id: i64,
    path: &str,
    name: &str,
    phash: &str,
    size: u64,
    width: u32,
    height: u32,
) -> Result<i64, rusqlite::Error> {
    connection.execute(
        "INSERT INTO image (gallery_id, path, name, phash, size, width, height) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7);",
        params![gallery_id, path, name, phash, size, width, height],
    )?;
    Ok(connection.last_insert_rowid())
}
