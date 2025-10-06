use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

/// Configuration structure for the cross-poster tool
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub dev_to: DevToConfig,
    pub medium: MediumConfig,
}

/// Dev.to platform configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DevToConfig {
    pub api_key: String,
}

/// Medium platform configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MediumConfig {
    pub access_token: String,
}

impl Config {
    /// Get the path to the config file
    pub fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("Failed to determine config directory")?
            .join("article-cross-poster");

        Ok(config_dir.join("config.toml"))
    }

    /// Initialize config directory and create example config if it doesn't exist
    pub fn init() -> Result<()> {
        let config_path = Self::config_path()?;
        let config_dir = config_path
            .parent()
            .context("Failed to get config directory")?;

        // Create config directory if it doesn't exist
        if !config_dir.exists() {
            fs::create_dir_all(config_dir).context("Failed to create config directory")?;
        }

        // Create config file from example if it doesn't exist
        // Use OpenOptions with create_new for atomic file creation (prevents race conditions)
        use std::fs::OpenOptions;
        use std::io::Write;

        let file_result = OpenOptions::new()
            .write(true)
            .create_new(true) // Fails if file already exists (atomic)
            .open(&config_path);

        match file_result {
            Ok(mut file) => {
                let example_config = Self::example_config();
                let toml_string = toml::to_string_pretty(&example_config)
                    .context("Failed to serialize example config")?;

                file.write_all(toml_string.as_bytes())
                    .context("Failed to write config file")?;

                // Set restrictive permissions (Unix only: 0600 = user read/write only)
                #[cfg(unix)]
                {
                    let mut perms = file.metadata()?.permissions();
                    perms.set_mode(0o600);
                    file.set_permissions(perms)
                        .context("Failed to set config file permissions")?;
                }

                println!("Created config file at: {}", config_path.display());
                println!("\n⚠️  SECURITY WARNING:");
                println!("API keys and tokens are stored in PLAIN TEXT in this file.");
                println!("Ensure file permissions are set correctly to protect your credentials.");
                println!("This file should only be readable by your user account.\n");
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                println!("Config file already exists at: {}", config_path.display());
            }
            Err(e) => return Err(e).context("Failed to create config file"),
        }

        Ok(())
    }

    /// Load config from file
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        let content = fs::read_to_string(&config_path).context(format!(
            "Failed to read config file at {}",
            config_path.display()
        ))?;

        let config: Config = toml::from_str(&content).context("Failed to parse config file")?;

        // Validate that placeholder values haven't been used
        if config.dev_to.api_key.contains("your_dev_to_api_key")
            || config.dev_to.api_key.is_empty()
            || config.dev_to.api_key.contains("INSERT")
        {
            anyhow::bail!(
                "dev.to API key is not configured. Please edit {} and add your API key.\n\
                Get your API key from: https://dev.to/settings/extensions",
                config_path.display()
            );
        }

        if config
            .medium
            .access_token
            .contains("your_medium_access_token")
            || config.medium.access_token.is_empty()
            || config.medium.access_token.contains("INSERT")
        {
            anyhow::bail!(
                "Medium access token is not configured. Please edit {} and add your access token.\n\
                Get your token from: https://medium.com/me/settings/security",
                config_path.display()
            );
        }

        Ok(config)
    }

    /// Display the current config (with sensitive data masked)
    pub fn show() -> Result<()> {
        let _config = Self::load()?;

        println!("Current configuration:");
        println!("  dev.to:");
        println!("    api_key: ********");
        println!("  medium:");
        println!("    access_token: ********");

        Ok(())
    }

    /// Display the config file path
    pub fn show_path() -> Result<()> {
        let config_path = Self::config_path()?;
        println!("{}", config_path.display());
        Ok(())
    }

    /// Generate an example config structure
    fn example_config() -> Self {
        Config {
            dev_to: DevToConfig {
                api_key: "your_dev_to_api_key_here".to_string(),
            },
            medium: MediumConfig {
                access_token: "your_medium_access_token_here".to_string(),
            },
        }
    }
}
