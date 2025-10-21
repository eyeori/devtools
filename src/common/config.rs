use config_file::FromConfigFile;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

#[allow(dead_code)]
#[derive(Default, Debug, Clone, Deserialize)]
pub struct Config(HashMap<String, String>);

#[allow(dead_code)]
impl Config {
    pub fn load_from_file(path: &Path) -> Self {
        if path.exists() {
            Self::from_config_file(path).unwrap_or_default()
        } else {
            Default::default()
        }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(String::as_str)
    }

    pub fn keys(&self) -> Vec<String> {
        self.0.keys().cloned().collect()
    }
}
