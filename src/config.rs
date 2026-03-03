use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct JourneyConfig {
    pub version: String,
    pub default_profile: String,
    pub dry_run_default: bool,
}

impl Default for JourneyConfig {
    fn default() -> Self {
        Self {
            version: "0.1.0".to_string(),
            default_profile: "base".to_string(),
            dry_run_default: true,
        }
    }
}

pub fn config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("journey_terminal")
}

pub fn config_file_path() -> PathBuf {
    config_dir().join("config.toml")
}

pub fn init_config() -> Result<(), Box<dyn std::error::Error>> {
    let dir = config_dir();
    let file_path = config_file_path();

    if file_path.exists() {
        println!("Config already exists at {}", file_path.display());
        return Ok(());
    }

    fs::create_dir_all(&dir)?;

    let config = JourneyConfig::default();
    let toml_string = toml::to_string_pretty(&config)?;

    let mut file = fs::File::create(&file_path)?;
    file.write_all(toml_string.as_bytes())?;

    println!("Created config at {}", file_path.display());
    Ok(())
}

pub fn load_config() -> Result<JourneyConfig, Box<dyn std::error::Error>> {
    let file_path = config_file_path();

    if !file_path.exists() {
        return Err("Config does not exist. Run `journey_terminal config init` first.".into());
    }

    let contents = std::fs::read_to_string(&file_path)?;
    let cfg: JourneyConfig = toml::from_str(&contents)?;
    Ok(cfg)
}

impl JourneyConfig {
    pub fn to_pretty_text(&self, file_path: &std::path::Path) -> String {
        format!(
            "Journey Terminal config\n\
             - Version: {}\n\
             - Default profile: {}\n\
             - Dry-run by default: {}\n\
             - File: {}",
            self.version,
            self.default_profile,
            self.dry_run_default,
            file_path.display()
        )
    }
}
