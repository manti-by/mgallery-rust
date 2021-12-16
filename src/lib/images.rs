use image;
use img_hash;

use image::GenericImageView;
use img_hash::HasherConfig;
use std::path::PathBuf;

use crate::lib::database::ImageModel;
use crate::lib::error::ProcessImageError;
use crate::lib::settings::Settings;

/// Compute image dHash, compile other data and save it to the database.
/// ## Arguments
/// * `image_path` - A PathBuf to image for processing.
///
/// ## Errors:
/// * `ProcessImageError::ImageError` if not an image or it's broken.
/// * `ProcessImageError::DBError` if database hasn't been setup correctly.
///
pub fn process_image(image_path: &PathBuf) -> Result<i64, ProcessImageError> {
    let settings = Settings::new();
    let image = match image::open(image_path) {
        Ok(image) => image,
        Err(e) => return Err(ProcessImageError::ImageError(e)),
    };

    let mut path = image_path.to_path_buf();
    path.pop();

    let hasher = HasherConfig::new().to_hasher();
    let mut image_model = ImageModel {
        id: 0,
        path: path
            .strip_prefix(&settings.data_path)
            .unwrap()
            .to_str()
            .unwrap(),
        name: image_path.file_name().unwrap().to_str().unwrap(),
        phash: &hasher.hash_image(&image).to_base64(),
        size: std::fs::metadata(image_path).unwrap().len(),
        width: image.dimensions().0,
        height: image.dimensions().1,
    };

    let image_id = match image_model.create() {
        Ok(image_id) => image_id,
        Err(e) => return Err(ProcessImageError::DBError(e)),
    };
    Ok(image_id)
}
