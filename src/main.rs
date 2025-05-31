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
