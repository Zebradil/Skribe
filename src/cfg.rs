use failure::Error;
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const CONFIG_FILENAME: &str = "config.toml";

#[derive(Debug, Default)]
pub struct Config {
    pub path: PathBuf,
    pub data: Data,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Data {
    database_path: String,
}

impl Config {
    fn open(file_path: PathBuf) -> Result<Self, Error> {
        let content = std::fs::read_to_string(&file_path)?;
        Ok(Self {
            path: file_path,
            data: Self::parse(&content)?,
        })
    }

    fn parse(content: &str) -> Result<Data, Error> {
        Ok(toml::from_str(content)?)
    }

    fn save(&self) -> Result<(), Error> {
        fs::write(&self.path, toml::to_string_pretty(&self.data)?)?;
        Ok(())
    }

    pub fn init(path: PathBuf) -> Result<Self, Error> {
        let config_file_path = path.join(CONFIG_FILENAME);
        if config_file_path.exists() {
            // TODO Return a custom error
            panic!("Config file already exists!");
        }
        fs::create_dir_all(path)?;
        let mut config = Self::default();
        config.path = config_file_path;
        config.save()?;
        Ok(config)
    }
}
