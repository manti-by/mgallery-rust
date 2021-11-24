use image;

use image::{DynamicImage, GenericImageView};
use std::fs::read_dir;
use std::io;
use std::path::PathBuf;

use crate::database::{create_image, DBImage};
use crate::error::ProcessImageError;
use crate::utils::Settings;

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
        path: path.strip_prefix(&settings.data_path).unwrap().to_str().unwrap(),
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

pub fn process_dir(current_dir: &PathBuf, settings: &Settings) -> io::Result<()> {
    for resource in read_dir(current_dir)? {
        let entry = resource?;
        let file_path = entry.path();

        if file_path.is_dir() {
            process_dir(&file_path, &settings)?;
        } else {
            let file_name_buf = entry.file_name();
            let file_name = file_name_buf.to_str().unwrap();
            if !file_name.starts_with(".")
                && (file_name.ends_with(".jpg")
                    || file_name.ends_with(".jpeg")
                    || file_name.ends_with(".png")
                    || file_name.ends_with(".gif"))
            {
                let _ = match process_image(&file_path, &settings) {
                    Ok(n) => n,
                    Err(e) => {
                        println!("Error processing image {} [{}]", file_path.display(), e);
                        continue;
                    }
                };
            }
        }
    }
    Ok(())
}
