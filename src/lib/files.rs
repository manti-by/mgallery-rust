use std::fs::read_dir;
use std::io;
use std::path::PathBuf;

use crate::lib::images::process_image;

/// Recursively scan directory for images (jpg, jpeg, png, gif).
/// ## Arguments
///
/// * `current_dir` - A PathBuf for directory to scan.
///
pub fn process_dir(current_dir: &PathBuf) -> io::Result<()> {
    for resource in read_dir(current_dir)? {
        let entry = resource?;
        let file_path = entry.path();

        if file_path.is_dir() {
            process_dir(&file_path)?;
        } else {
            let file_name_buf = entry.file_name();
            let file_name = file_name_buf.to_str().unwrap();
            if !file_name.starts_with(".")
                && (file_name.ends_with(".jpg")
                    || file_name.ends_with(".jpeg")
                    || file_name.ends_with(".png")
                    || file_name.ends_with(".gif"))
            {
                match process_image(&file_path) {
                    Ok(_) => (),
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
