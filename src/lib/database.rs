use crate::lib::settings::Settings;
use rusqlite::{params, Connection, Result};

pub struct DBImage<'a> {
    pub path: &'a str,
    pub name: &'a str,
    pub phash: &'a str,
    pub size: u64,
    pub width: u32,
    pub height: u32,
}

pub fn create_image(image: &DBImage, settings: &Settings) -> Result<i64, rusqlite::Error> {
    let connection = Connection::open(&settings.db_path)?;
    connection.execute(
        "INSERT INTO image (path, name, phash, size, width, height) VALUES (?1, ?2, ?3, ?4, ?5, ?6);",
        params![image.path, image.name, image.phash, image.size, image.width, image.height],
    )?;

    let image_id: i64 = connection.last_insert_rowid();
    drop(connection);

    Ok(image_id)
}
