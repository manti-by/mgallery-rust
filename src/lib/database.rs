use crate::lib::settings::SETTINGS;
use rusqlite::{params, Connection, Result};

/// Image data structure for database operations.
pub struct ImageModel<'a> {
    pub id: i64,
    pub path: &'a str,
    pub name: &'a str,
    pub phash: &'a str,
    pub size: u64,
    pub width: u32,
    pub height: u32,
}

impl ImageModel<'_> {
    /// Save current image data to the database.
    ///
    /// ## Errors:
    /// * `rusqlite::Error` error if database hasn't been setup correctly.
    ///
    pub fn create(&mut self) -> Result<i64, rusqlite::Error> {
        let connection = Connection::open(&SETTINGS.db_path)?;
        connection.execute(
            "INSERT INTO image (path, name, phash, size, width, height) VALUES (?1, ?2, ?3, ?4, ?5, ?6);",
            params![self.path, self.name, self.phash, self.size, self.width, self.height],
        )?;

        self.id = connection.last_insert_rowid();
        drop(connection);

        Ok(self.id)
    }
}
