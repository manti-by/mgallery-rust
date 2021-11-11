mod database;
mod error;
mod services;

use std::env;
use std::io;

use services::process_dir;

fn main() -> Result<(), io::Error> {
    let current_path = env::current_dir()?;
    let mut db_path = current_path.clone().to_path_buf();
    let mut data_path = current_path.parent().unwrap().to_path_buf();

    data_path.push("data");
    db_path.push("db.sqlite");
    process_dir(&data_path, &db_path)?;

    Ok(())
}
