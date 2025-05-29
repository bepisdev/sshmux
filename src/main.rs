use colored::*;
use serde::Deserialize;
use std::fs;
use std::process::Stdio;
use std::collections::HashMap;
use tokio::{io::{AsyncBufReadExt, BufReader}, process::Command};
use clap::{Parser, Arg};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "sshmux")]
#[command(author = "Josh Burns <joshyburnss@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Run a shell command concurrently on multiple SSH hosts defined in a TOML config.", long_about = None)]
struct Cli {
    /// Path to the TOML config file
    #[arg(short, long, default_value = "sshmux.toml")]
    config: String,

    /// Enable verbose output
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

#[derive(Deserialize)]
struct Config {
    command: String,
    hosts: Vec<String>,
}

fn get_colored_prefix(host: &str, color_index: usize) -> ColoredString {
    let colors = ["red", "green", "yellow", "blue", "magenta", "cyan", "white"];
    let color = colors[color_index % colors.len()];
    format!("[{}]", host).color(color)
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let toml_str = fs::read_to_string(cli.config)?;
    let config: Config = toml::from_str(&toml_str)?;

    if cli.verbose {
        println!("Loaded config: Command='{}', Hosts={:?}", config.command, config.hosts);
    }

    let mut tasks = vec![];

    for (i, host) in config.hosts.iter().enumerate() {
        let command = config.command.clone();
        let host_clone = host.clone();
        let prefix = get_colored_prefix(&host_clone, i);
        let verbose = cli.verbose;

        let task = tokio::spawn(async move {
            if verbose {
                println!("{} Connecting...", prefix);
            }

            let mut child = Command::new("ssh")
                .arg(&host_clone)
                .arg(&command)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("failed to execute ssh command");

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

            let _ = child.await;
        });

        tasks.push(task);
    }

    for task in tasks {
        let _ = task.await;
    }

    Ok(())
}
