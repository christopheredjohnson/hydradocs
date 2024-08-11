use serde::{Deserialize, Serialize};
use std::fs;


#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentConfig {
    pub title: String,
    pub author: String,
}

pub fn load_config(path: &str) -> Result<DocumentConfig, std::io::Error> {
    let config_data = fs::read_to_string(path)?;
    let config: DocumentConfig = serde_json::from_str(&config_data)?;
    Ok(config)
}
