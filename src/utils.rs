use std::env;
use std::io;
use std::path::PathBuf;

pub struct Settings {
    pub db_path: PathBuf,
    pub data_path: PathBuf,
}

fn get_arg(args: &Vec<String>, name: &str, default: &str) -> Result<PathBuf, io::Error> {
    let current_path = env::current_dir()?;

    let value = match args.iter().position(|x| x.to_owned() == name) {
        Some(index) => PathBuf::from(&args[index + 1]),
        None => {
            let mut path = current_path.clone().to_path_buf();
            path.push(&default);
            path
        },
    };
    Ok(value)
}

pub fn get_settings() -> Result<Settings, io::Error> {
    let args: Vec<String> = env::args().collect();
    let settings = Settings {
        db_path: get_arg(&args, "--db_path", "db.sqlite")?,
        data_path: get_arg(&args, "--data_path", "data/")?,
    };
    Ok(settings)
}
