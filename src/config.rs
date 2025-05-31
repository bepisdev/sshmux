use anyhow::Result;
use serde::Deserialize;
use std::{collections::HashSet, fs};

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Host {
    pub host: String,
    pub user: Option<String>,
    pub port: Option<u16>,
    pub identity_file: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub command: String,
    pub hosts: Vec<Host>,
}

pub fn load_config(path: &str) -> Result<Config> {
    let toml_str = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&toml_str)?;
    Ok(config)
}

pub fn validate_config(config: &Config, force: bool) -> Result<()> {
    let mut seen = HashSet::new();

    for (i, host) in config.hosts.iter().enumerate() {
        if host.host.trim().is_empty() {
            anyhow::bail!("Host entry at index {} is missing a hostname.", i);
        }

        let key = if let Some(user) = &host.user {
            format!("{}@{}", user, host.host)
        } else {
            host.host.clone()
        };

        if !seen.insert(key.clone()) && !force {
            anyhow::bail!("Duplicate host entry found: '{}'. Use --force to override.", key);
        }
    }

    Ok(())
}
