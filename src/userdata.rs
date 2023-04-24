use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use serde_json::{Result, Value};
use std::path;

const USER_DATA_DIR: &str = ".l1t";
const USER_DATA_FILE: &str = "data.json";

#[derive()]
pub struct CompletedLevel {
    name: String,
    author: String,
    description: String,
    completed_on: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct UserData {
    completed_levels: Vec<CompletedLevel>,
}

impl UserData {
    pub fn read(home_dir: String) -> Result<UserData, String> {
        let full_path = path::Path::new(&(home_dir + USER_DATA_DIR + USER_DATA_FILE));
    }
}
