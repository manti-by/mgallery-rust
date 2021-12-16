use lazy_static::lazy_static;
use std::env;
use std::path::PathBuf;

// https://github.com/PhilipDaniels/rust-config-example/blob/master/src/configuration.rs
lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new();
}

pub struct Settings {
    pub db_path: PathBuf,
    pub data_path: PathBuf,
}

impl Default for Settings {
    fn default() -> Self {
        let current_path = env::current_dir().unwrap();

        let mut db_path = current_path.clone().to_path_buf();
        db_path.push("db.sqlite");

        let mut data_path = current_path.clone().to_path_buf();
        data_path.push("data/");

        Settings { db_path, data_path }
    }
}

impl Settings {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut settings = Settings::default();

        if let Some(index) = args.iter().position(|x| x.to_owned() == "db_path") {
            settings.db_path = PathBuf::from(&args[index + 1])
        }

        if let Some(index) = args.iter().position(|x| x.to_owned() == "data_path") {
            settings.data_path = PathBuf::from(&args[index + 1])
        }

        settings
    }
}
