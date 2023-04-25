use serde::{Deserialize, Serialize};
use std::{fs, path};

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletedLevel {
    name: String,
    author: String,
    description: String,
    completed_at: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    completed_levels: Vec<CompletedLevel>,
}

impl UserData {
    pub fn read(home_dir: String) -> Result<UserData, String> {
        let filename = home_dir.to_string() + "/.l1t/data.json";
        if !path::Path::new(&filename).exists() {
            match fs::create_dir(home_dir + "/.l1t") {
                Err(e) => return Err(e.to_string()),
                _ => (),
            };
            let data = UserData {
                completed_levels: vec![],
            };
            let content = match serde_json::to_string(&data) {
                Ok(c) => c,
                Err(e) => return Err(e.to_string()),
            };
            match fs::write(&filename, content) {
                Err(e) => return Err(e.to_string()),
                _ => (),
            };
        }
        let file_content = fs::read_to_string(filename).unwrap_or("".to_string());
        match serde_json::from_str::<UserData>(&file_content) {
            Ok(d) => Ok(d),
            Err(e) => Err(e.to_string()),
        }
    }
}
