use colored::*;
use serde::Deserialize;
use std::fs;
use std::process::Stdio;
use tokio::{io::{AsyncBufReadExt, BufReader}, process::Command};
use clap::Parser;
use anyhow::Result;

#[derive(Parser, Debug)]
#[command(name = "sshmux",
          about = "Run a shell command concurrently on multiple SSH hosts defined in a TOML config.",
          author = "Josh Burns <joshyburnss@gmail.com>",
          version = "1",
          long_about = None)]
struct Cli {
    /// Path to the TOML config file
    #[arg(short, long, default_value = "sshmux.toml")]
    config: String,

    /// Enable verbose output
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Only check the config for validity and exit
    #[arg(long, default_value_t = false)]
    check_config: bool,

    /// Allow duplicate hosts in the config
    #[arg(long, default_value_t = false)]
    force: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
struct Host {
    host: String,
    user: Option<String>,
    port: Option<u16>,
    identity_file: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    command: String,
    hosts: Vec<Host>,
}

fn get_colored_prefix(host: &str, color_index: usize) -> ColoredString {
    let colors = ["red", "green", "yellow", "blue", "magenta", "cyan", "white"];
    let color = colors[color_index % colors.len()];
    format!("[{}]", host).color(color)
}

fn validate_config(config: &Config, force: bool) -> Result<()> {
    use std::collections::HashSet;
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


#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let toml_str = fs::read_to_string(cli.config)?;
    let config: Config = toml::from_str(&toml_str)?;

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

    let mut tasks = vec![];

    for (i, host_config) in config.hosts.iter().enumerate() {
        let command = config.command.clone();
        let host_clone = host_config.clone();
        let prefix = get_colored_prefix(&host_clone.host, i);
        let verbose = cli.verbose;

        let task = tokio::spawn(async move {
            let host = host_clone.host;
            let port = host_clone.port.unwrap_or(22);
            let user = host_clone.user;
            let identity_file = host_clone.identity_file;

            if verbose {
                println!("{} Connecting to {}:{}...", prefix, host, port);
            }

            let mut ssh_args = vec!["-p".to_string(), port.to_string()];

            if let Some(identity) = identity_file {
                ssh_args.push("-i".to_string());
                ssh_args.push(identity);
            }

            let destination = if let Some(user) = user {
                format!("{}@{}", user, host)
            } else {
                host
            };

            ssh_args.push(destination);
            ssh_args.push(command);

            let mut child = match Command::new("ssh")
                .args(&ssh_args)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
            {
                Ok(child) => child,
                Err(e) => {
                    eprintln!("{} Failed to spawn ssh command: {}", prefix, e);
                    return;
                }
            };

            if let Some(stdout) = child.stdout.take() {
                let reader = BufReader::new(stdout);
                let mut lines = reader.lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    println!("{} {}", prefix, line);
                }
            }

            if let Some(stderr) = child.stderr.take() {
                let reader = BufReader::new(stderr);
                let mut lines = reader.lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    eprintln!("{} {}", prefix, line);
                }
            }
        });

        tasks.push(task);
    }

    for task in tasks {
        let _ = task.await;
    }

    Ok(())
}
