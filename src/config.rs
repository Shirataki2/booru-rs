use std::{path::{Path, PathBuf}, fs::{self, File}, io::Write, env};

use crate::prelude::*;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub account: AccountConfig
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountConfig {
    pub username: String,
    pub api_key: String,
}

impl Config {
    pub fn read_from<P: AsRef<Path>>(path: P) -> Result<Config, Error> {
        let path = path.as_ref();
        let body = fs::read_to_string(path)?;
        let config: Self = toml::from_str(&body)?;
        Ok(config)
    }

    pub fn save_to<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let path = path.as_ref();
        let mut file = File::create(path)?;
        let body = toml::to_string_pretty(self).with_context(|| "Failed to parse config file")?;
        write!(file, "{}", body)?;
        file.flush()?;
        Ok(())
    }

    fn config_path() -> Result<PathBuf, Error> {
        let config_path = match env::var_os("CONFIG_PATH") {
            Some(path) => path.into(),
            None => dirs::config_dir().with_context(|| "Failed to get config directory")?,
        };
        let config_path = config_path.join("booru-config.toml");
        if !config_path.exists() {
            fs::create_dir_all(config_path.parent().unwrap())?;
        }
        Ok(config_path)
    }

    pub fn save_to_config_file(&self) -> Result<(), Error> {
        self.save_to(Self::config_path()?)
    }

    pub fn read_from_config_file() -> Result<Config, Error> {
        Self::read_from(Self::config_path()?)
    }
}
