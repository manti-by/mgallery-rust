mod db;

use image;
use img_hash;

use db::{create_gallery, create_image};
use img_hash::HasherConfig;

fn main() {
    let image = image::open("/mnt/c/www/mlibrary/data/test.jpg").unwrap();
    let hasher = HasherConfig::new().to_hasher();

    let hash = hasher.hash_image(&image);

    println!("Image hash: {}", hash.to_base64());

    let gallery_id: i64 = create_gallery("/mnt/c/www/mlibrary/data/test.jpg", "data").unwrap();
    println!("Gallery ID: {}", gallery_id);

    let image_id: i64 = create_image(
        gallery_id,
        "/mnt/c/www/mlibrary/data/test.jpg",
        "test.jpg",
        &hash.to_base64(),
        1000000,
        1920,
        1080,
    ).unwrap();
    println!("Image ID: {}", image_id);
}
