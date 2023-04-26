use serde::Deserialize;
use std::error::Error;

pub struct Repository {
    pub name: String,
    pub url: String,
    pub levels: RepositoryLevelListing,
}

#[derive(Deserialize)]
pub struct RepositoryLevelListing {
    levels: Vec<String>,
}

impl Repository {
    pub async fn from_url(url: String) -> Result<Repository, Box<dyn Error>> {
        let resp = reqwest::get(&url).await?.text().await?;
        let levels: RepositoryLevelListing = match serde_json::from_str(&resp) {
            Ok(d) => d,
            Err(_) => RepositoryLevelListing { levels: vec![] },
        };

        Ok(Repository {
            url,
            name: "".to_string(),
            levels,
        })
    }
}
