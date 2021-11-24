mod database;
mod error;
mod services;
mod utils;

use std::io;

use services::process_dir;
use utils::get_settings;

fn main() -> Result<(), io::Error> {
    let settings = get_settings()?;
    process_dir(&settings.data_path, &settings)?;
    Ok(())
}
