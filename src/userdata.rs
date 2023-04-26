use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{self, Path, PathBuf},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletedLevel {
    pub name: String,
    pub author: String,
    pub description: String,
    pub completed_at: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    #[serde(skip)]
    file: PathBuf,
    pub completed_core_levels: Vec<usize>,
    pub completed_levels: Vec<CompletedLevel>,
}

impl UserData {
    pub fn read(home_dir: &Path) -> Result<UserData, String> {
        let filename = home_dir.join("/.l1t/data.json");
        if !path::Path::new(&filename).exists() {
            if let Err(e) = fs::create_dir(home_dir.join("/.l1t")) {
                return Err(e.to_string());
            };
            let data = UserData {
                file: filename.clone(),
                completed_core_levels: vec![],
                completed_levels: vec![],
            };
            let content = match serde_json::to_string(&data) {
                Ok(c) => c,
                Err(e) => return Err(e.to_string()),
            };
            if let Err(e) = fs::write(&filename, content) {
                return Err(e.to_string());
            };
        }
        let file_content = fs::read_to_string(&filename).unwrap_or("".to_string());
        match serde_json::from_str::<UserData>(&file_content) {
            Ok(d) => Ok(d),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn complete_core(&mut self, level: usize) -> Result<(), String> {
        if self.completed_core_levels.iter().any(|i| *i == level) {
            return Ok(());
        }
        self.completed_core_levels.push(level);
        let content = match serde_json::to_string(self) {
            Ok(c) => c,
            Err(e) => return Err(e.to_string()),
        };
        if let Err(e) = fs::write(&self.file, content) {
            return Err(e.to_string());
        };
        Ok(())
    }
}
