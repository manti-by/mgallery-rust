use std::fmt::Display;

use image;
use rusqlite;
use std;

#[derive(Debug)]
pub enum ProcessImageError {
    ImageError(image::error::ImageError),
    DBError(rusqlite::Error),
}

impl std::error::Error for ProcessImageError {}

impl Display for ProcessImageError {
    /// Format an appropriate error for a terminal output.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessImageError::ImageError(ref msg) => write!(f, "Image error: {}", msg),
            ProcessImageError::DBError(ref msg) => write!(f, "DB error: {}", msg),
        }
    }
}
