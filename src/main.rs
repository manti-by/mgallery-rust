mod lib;

use std::io;

use crate::lib::settings::SETTINGS;
use lib::files::process_dir;

fn main() -> Result<(), io::Error> {
    Ok(process_dir(&SETTINGS.data_path)?)
}
