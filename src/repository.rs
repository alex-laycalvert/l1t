use crate::level::{LevelInfo, LevelSource};
use serde::Deserialize;
use std::error::Error;

#[derive(Debug)]
pub struct Repository {
    pub name: String,
    pub url: String,
    pub levels: Vec<LevelInfo>,
}

#[derive(Deserialize, Debug)]
pub struct RepositoryLevelInfo {
    pub source: String,
    pub name: String,
    pub author: String,
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub struct RepositoryResponse {
    pub levels: Vec<RepositoryLevelInfo>,
}

impl Repository {
    pub fn new(name: String, url: String) -> Repository {
        Repository {
            name,
            url,
            levels: vec![],
        }
    }

    pub async fn download_listing(&mut self) -> Result<(), Box<dyn Error>> {
        let response = reqwest::get(&self.url).await?.text().await?;
        let response: RepositoryResponse = match serde_json::from_str(&response) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Error: {e}");
                RepositoryResponse { levels: vec![] }
            }
        };
        self.levels = response
            .levels
            .iter()
            .map(|i| LevelInfo {
                source: LevelSource::Url(self.url.to_string() + "/" + &i.source),
                name: i.name.to_string(),
                author: i.author.to_string(),
                description: i.description.to_string(),
            })
            .collect();
        Ok(())
    }
}