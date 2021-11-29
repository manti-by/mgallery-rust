use image;

use image::{DynamicImage, GenericImageView};
use std::path::PathBuf;

use crate::lib::database::{create_image, DBImage};
use crate::lib::error::ProcessImageError;
use crate::lib::settings::Settings;

fn get_image_phash(_image: &DynamicImage) -> Result<&str, ProcessImageError> {
    Ok("TODO: Implement hash function")
}

pub fn process_image(image_path: &PathBuf, settings: &Settings) -> Result<i64, ProcessImageError> {
    let image = match image::open(image_path) {
        Ok(image) => image,
        Err(e) => return Err(ProcessImageError::ImageError(e)),
    };

    let mut path = image_path.to_path_buf();
    path.pop();

    let db_image = DBImage {
        path: path
            .strip_prefix(&settings.data_path)
            .unwrap()
            .to_str()
            .unwrap(),
        name: image_path.file_name().unwrap().to_str().unwrap(),
        phash: get_image_phash(&image).unwrap(),
        size: std::fs::metadata(image_path).unwrap().len(),
        width: image.dimensions().0,
        height: image.dimensions().1,
    };

    let image_id = match create_image(&db_image, &settings) {
        Ok(image_id) => image_id,
        Err(e) => return Err(ProcessImageError::DBError(e)),
    };
    Ok(image_id)
}
