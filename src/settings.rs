use serde::{Serialize, Deserialize};
use tokio::fs;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub theme: String,
    pub language: String,
}

impl Settings {
    pub async fn load(filename: &str) -> Result<Self, Box<dyn Error>> {
        let data = fs::read_to_string(filename).await?;
        let settings: Settings = toml::from_str(&data)?;
        Ok(settings)
    }

    pub async fn save(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let toml_str = toml::to_string(self)?;
        fs::write(filename, toml_str).await?;
        Ok(())
    }
}
