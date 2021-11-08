use rusqlite::{params, Connection, Result};

pub fn create_image(
    db_path: &str,
    path: &str,
    name: &str,
    phash: &str,
    size: u64,
    width: u32,
    height: u32,
) -> Result<i64, rusqlite::Error> {
    let connection = Connection::open(db_path)?;
    connection.execute(
        "INSERT INTO image (path, name, phash, size, width, height) VALUES (?1, ?2, ?3, ?4, ?5, ?6);",
        params![path, name, phash, size, width, height],
    )?;

    let image_id: i64 = connection.last_insert_rowid();
    drop(connection);

    Ok(image_id)
}
