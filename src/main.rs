use image;
use img_hash;

use img_hash::{HasherConfig};

fn main() {
    let image = image::open("/mnt/c/www/mlibrary/data/test.jpg").unwrap();
    let hasher = HasherConfig::new().to_hasher();

    let hash = hasher.hash_image(&image);

    println!("Image hash: {}", hash.to_base64());
}