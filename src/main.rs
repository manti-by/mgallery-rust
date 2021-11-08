mod database;
mod error;
mod services;

use std::env;
use std::fs::read_dir;
use std::io;
use std::path::PathBuf;

use services::process_image;

fn process_dir(data_path: &PathBuf, db_path: &PathBuf) -> io::Result<()> {
    for resource in read_dir(data_path).unwrap() {
        let entry = resource.unwrap();
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

fn main() -> Result<(), io::Error> {
    let current_path = match env::current_dir() {
        Ok(v) => v,
        Err(e) => panic!("Error: {}", e),
    };

    let mut db_path = current_path.clone().to_path_buf();
    let mut data_path = current_path.parent().unwrap().to_path_buf();

    data_path.push("data");
    db_path.push("db.sqlite");
    process_dir(&data_path, &db_path)?;

    Ok(())
}
