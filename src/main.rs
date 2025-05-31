mod cli;
mod config;
mod output;
mod runner;

use anyhow::Result;
use cli::Cli;
use config::{load_config, validate_config};
use runner::run_all;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse_args();
    let config = load_config(&cli.config)?;

    if cli.check_config {
        validate_config(&config, cli.force)?;
        println!("Config is valid.");
        return Ok(());
    }

    if cli.verbose {
        println!(
            "Loaded config: Command='{}', Hosts={:#?}",
            config.command, config.hosts
        );
    }

    validate_config(&config, cli.force)?;
    run_all(&config, cli.verbose).await;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use config::Config;
    use config::Host;

    #[test]
    fn test_valid_config_no_duplicates() {
        let config = Config {
            command: "uptime".into(),
            hosts: vec![
                Host {
                    host: "host1".into(),
                    user: Some("user1".into()),
                    port: Some(22),
                    identity_file: None,
                },
                Host {
                    host: "host2".into(),
                    user: None,
                    port: Some(2222),
                    identity_file: None,
                },
            ],
        };

        let result = validate_config(&config, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_config_missing_host() {
        let config = Config {
            command: "uptime".into(),
            hosts: vec![Host {
                host: "".into(),
                user: None,
                port: None,
                identity_file: None,
            }],
        };

        let result = validate_config(&config, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_duplicate_hosts_error() {
        let config = Config {
            command: "uptime".into(),
            hosts: vec![
                Host {
                    host: "host1".into(),
                    user: Some("user1".into()),
                    port: None,
                    identity_file: None,
                },
                Host {
                    host: "host1".into(),
                    user: Some("user1".into()),
                    port: None,
                    identity_file: None,
                },
            ],
        };

        let result = validate_config(&config, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_duplicate_hosts_with_force_ok() {
        let config = Config {
            command: "uptime".into(),
            hosts: vec![
                Host {
                    host: "host1".into(),
                    user: Some("user1".into()),
                    port: None,
                    identity_file: None,
                },
                Host {
                    host: "host1".into(),
                    user: Some("user1".into()),
                    port: None,
                    identity_file: None,
                },
            ],
        };

        let result = validate_config(&config, true);
        assert!(result.is_ok());
    }
}
