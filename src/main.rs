mod db;

use image;
use img_hash;

use rusqlite::Connection;
use std::path::Path;

use db::{create_gallery, create_image};
use image::GenericImageView;
use img_hash::HasherConfig;

const DB_PATH: &str = "/mnt/c/www/mlibrary/db.sqlite";
const IMG_PATH: &str = "/mnt/c/www/mlibrary/data/test.jpg";

fn main() -> Result<(), rusqlite::Error> {
    let mut connection = Connection::open(DB_PATH)?;

    let gallery_id: i64 = create_gallery(&mut connection, IMG_PATH, "data").unwrap();
    println!("Gallery ID: {}", gallery_id);

    let image = image::open(IMG_PATH).unwrap();
    let name: &str = Path::new(IMG_PATH).file_name().unwrap().to_str().unwrap();
    let hash: &str = &HasherConfig::new()
        .to_hasher()
        .hash_image(&image)
        .to_base64();
    let size: u64 = std::fs::metadata(IMG_PATH).unwrap().len();
    let (width, height) = image.dimensions();

    let image_id: i64 = create_image(
        &mut connection,
        gallery_id,
        IMG_PATH,
        name,
        hash,
        size,
        width,
        height,
    )
    .unwrap();
    println!("Image ID: {}", image_id);

    Ok(())
}
