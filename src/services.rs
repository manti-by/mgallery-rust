use image;
use img_hash;

use image::GenericImageView;
use img_hash::HasherConfig;
use std::path::{Path, PathBuf};

use crate::database::create_image;
use crate::error::ProcessImageError;

pub fn process_image(db_path: &str, image_path: &Path) -> Result<i64, ProcessImageError> {
    let image = match image::open(image_path) {
        Ok(image) => image,
        Err(e) => return Err(ProcessImageError::ImageError(e)),
    };

    let name: &str = image_path.file_name().unwrap().to_str().unwrap();
    let hash: &str = &HasherConfig::new()
        .to_hasher()
        .hash_image(&image)
        .to_base64();
    let size: u64 = std::fs::metadata(image_path).unwrap().len();
    let (width, height) = image.dimensions();

    let mut path: PathBuf = image_path.to_path_buf();
    path.pop();

    let image_id = match create_image(
        db_path,
        path.to_str().unwrap(),
        name,
        hash,
        size,
        width,
        height,
    ) {
        Ok(image_id) => image_id,
        Err(e) => return Err(ProcessImageError::DBError(e)),
    };
    Ok(image_id)
}
