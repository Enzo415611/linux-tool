use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    #[serde(rename = "AppTheme")]
    pub app_theme: bool
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            app_theme: true
        }
    }
}


pub fn load_config() -> Result<AppConfig> {
    let cfg: AppConfig = confy::load("linux-tool", Some("config"))?;
    Ok(cfg)
}