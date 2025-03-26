use std::{collections::HashMap, fs};
use std::path::Path;
use std::sync::Mutex;
use crate::models::ShortenedLink;

pub struct AppState {
    pub links: Mutex<HashMap<String, ShortenedLink>>,
    pub file_path: String,
}

impl AppState {
    pub fn load(&self) -> Result<HashMap<String, ShortenedLink>, std::io::Error> {
        if Path::new(&self.file_path).exists() {
            let data = fs::read_to_string(&self.file_path)?;
            let links: HashMap<String, ShortenedLink> = serde_json::from_str(&data)?;
            Ok(links)
        } else {
            Ok(HashMap::new())
        }
    }

    pub fn save(&self, links: &HashMap<String, ShortenedLink>) -> Result<(), std::io::Error> {
        let data = serde_json::to_string_pretty(links)?;
        fs::write(&self.file_path, data)
    }
}
