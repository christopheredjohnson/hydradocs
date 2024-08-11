use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentConfig {
    pub title: String,
    pub author: String,
    pub pages: Vec<PageConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageConfig {
    pub elements: Vec<Element>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Element {
    Text {
        content: String,
        font_size: f32,
        position: [f32; 2],
    },
}

pub fn load_config(path: &str) -> Result<DocumentConfig, std::io::Error> {
    let config_data = fs::read_to_string(path)?;
    let config: DocumentConfig = serde_json::from_str(&config_data)?;
    Ok(config)
}
