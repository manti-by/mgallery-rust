use image;
use img_hash;

use image::GenericImageView;
use img_hash::{HashAlg, HasherConfig};
use std::fs::read_dir;
use std::io;
use std::path::{Path, PathBuf};

use crate::database::{create_image, DBImage};
use crate::error::ProcessImageError;

pub fn process_image(db_path: &str, image_path: &Path) -> Result<i64, ProcessImageError> {
    let image = match image::open(image_path) {
        Ok(image) => image,
        Err(e) => return Err(ProcessImageError::ImageError(e)),
    };

    let mut path: PathBuf = image_path.to_path_buf();
    path.pop();

    let mut hasher = HasherConfig::new();
    hasher = hasher.hash_alg(HashAlg::Mean);

    let db_image = DBImage {
        path: path.to_str().unwrap(),
        name: image_path.file_name().unwrap().to_str().unwrap(),
        phash: &hasher.to_hasher().hash_image(&image).to_base64(),
        size: std::fs::metadata(image_path).unwrap().len(),
        width: image.dimensions().0,
        height: image.dimensions().1,
    };

    let image_id = match create_image(db_path, &db_image) {
        Ok(image_id) => image_id,
        Err(e) => return Err(ProcessImageError::DBError(e)),
    };
    Ok(image_id)
}

pub fn process_dir(data_path: &PathBuf, db_path: &PathBuf) -> io::Result<()> {
    for resource in read_dir(data_path)? {
        let entry = resource?;
        let file_path = entry.path();

        if file_path.is_dir() {
            process_dir(&file_path, db_path)?;
        } else {
            let file_name_buf = entry.file_name();
            let file_name = file_name_buf.to_str().unwrap();
            if !file_name.starts_with(".")
                && (file_name.ends_with(".jpg")
                    || file_name.ends_with(".jpeg")
                    || file_name.ends_with(".png")
                    || file_name.ends_with(".gif"))
            {
                let _ = match process_image(db_path.to_str().unwrap(), &file_path) {
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
