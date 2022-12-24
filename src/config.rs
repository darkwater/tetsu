use std::{env, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub db_path: PathBuf,
}

impl Config {
    pub fn read() -> Self {
        let config_path = PathBuf::from(
            env::var("HOME").expect("$HOME is not set") + "/.config/tetsu/config.toml",
        );

        if !PathBuf::from(&config_path).exists() {
            let defaults = include_bytes!("../default-config.toml");
            fs::create_dir_all(config_path.parent().unwrap()).unwrap();
            fs::write(&config_path, defaults).unwrap();
        }

        let config = fs::read_to_string(&config_path).unwrap();

        toml::from_str(&config).unwrap()
    }
}
