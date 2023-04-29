use crate::{
    level::{LevelInfo, LevelSource},
    repository::Repository,
};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, path};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CompletedRepoLevel {
    pub url: String,
    pub name: String,
    pub author: String,
    pub description: String,
    pub completed_at: u64,
}

pub struct UserData {
    file: String,
    pub completed_core_levels: Vec<usize>,
    pub completed_levels: Vec<CompletedRepoLevel>,
    pub repositories: Vec<Repository>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedUserData {
    file: String,
    pub completed_core_levels: Vec<usize>,
    pub completed_levels: Vec<CompletedRepoLevel>,
}

impl UserData {
    fn read_repositories(home_dir: String) -> Result<Vec<Repository>, String> {
        let file = home_dir.to_string() + "/.l1t/repositories.l1t_conf";
        if !path::Path::new(&file).exists() {
            fs::create_dir(home_dir + "/.l1t").ok();
            if let Err(e) = fs::write(&file, "") {
                return Err(e.to_string());
            }
        }
        let file_content = fs::read_to_string(&file).unwrap_or_default();
        let mut repositories: Vec<Repository> = vec![];
        for line in file_content.trim().split('\n') {
            let parts: Vec<&str> = line.trim().split('=').collect();
            if parts.len() < 2 {
                continue;
            }
            repositories.push(Repository::new(
                parts[0].trim().to_string(),
                parts[1].trim().to_string(),
            ));
        }
        Ok(repositories)
    }

    pub fn read(home_dir: String) -> Result<UserData, String> {
        let file = home_dir.to_string() + "/.l1t/data.json";
        if !path::Path::new(&file).exists() {
            fs::create_dir(home_dir.clone() + "/.l1t").ok();
            let data = SerializedUserData {
                file: file.clone(),
                completed_core_levels: vec![],
                completed_levels: vec![],
            };
            let content = match serde_json::to_string(&data) {
                Ok(c) => c,
                Err(e) => return Err(e.to_string()),
            };
            if let Err(e) = fs::write(&file, content) {
                return Err(e.to_string());
            };
        }
        let file_content = fs::read_to_string(&file).unwrap_or_default();
        let data = match serde_json::from_str::<SerializedUserData>(&file_content) {
            Ok(d) => d,
            Err(e) => return Err(e.to_string()),
        };
        let repositories = UserData::read_repositories(home_dir)?;

        Ok(UserData {
            repositories,
            file,
            completed_core_levels: data.completed_core_levels,
            completed_levels: data.completed_levels,
        })
    }

    fn complete_core(&mut self, level: usize) -> Result<(), String> {
        if self.completed_core_levels.iter().any(|i| *i == level) {
            return Ok(());
        }
        self.completed_core_levels.push(level);
        let content = match serde_json::to_string(&SerializedUserData {
            file: self.file.clone(),
            completed_core_levels: self.completed_core_levels.clone(),
            completed_levels: self.completed_levels.clone(),
        }) {
            Ok(c) => c,
            Err(e) => return Err(e.to_string()),
        };
        if let Err(e) = fs::write(&self.file, content) {
            return Err(e.to_string());
        };
        Ok(())
    }

    fn complete_repo(&mut self, level_info: LevelInfo) -> Result<(), String> {
        if let LevelSource::Url(url) = level_info.source {
            if self.completed_levels.iter().any(|l| {
                l.url == url || (l.name == level_info.name && l.author == level_info.author)
            }) {
                return Ok(());
            }
            let completed_at = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            self.completed_levels.push(CompletedRepoLevel {
                url,
                completed_at,
                name: level_info.name,
                author: level_info.author,
                description: level_info.description,
            });
            let content = match serde_json::to_string(&SerializedUserData {
                file: self.file.clone(),
                completed_core_levels: self.completed_core_levels.clone(),
                completed_levels: self.completed_levels.clone(),
            }) {
                Ok(c) => c,
                Err(e) => return Err(e.to_string()),
            };
            if let Err(e) = fs::write(&self.file, content) {
                return Err(e.to_string());
            };
        }
        Ok(())
    }

    pub fn complete(&mut self, level_info: LevelInfo) -> Result<(), String> {
        match level_info.source {
            LevelSource::Core(level) => self.complete_core(level),
            LevelSource::Url(_) => self.complete_repo(level_info),
            _ => Err("".to_string()),
        }
    }
}
