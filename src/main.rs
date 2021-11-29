mod lib;

use std::io;

use lib::files::process_dir;
use lib::settings::get_settings;

fn main() -> Result<(), io::Error> {
    let settings = get_settings()?;
    process_dir(&settings.data_path, &settings)?;
    Ok(())
}
