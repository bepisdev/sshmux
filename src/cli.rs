use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "sshmux",
          about = "Run a shell command concurrently on multiple SSH hosts defined in a TOML config.",
          author = "Josh Burns <joshyburnss@gmail.com>",
          version = "1",
          long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value = "sshmux.toml")]
    pub config: String,

    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    #[arg(long, default_value_t = false)]
    pub check_config: bool,

    #[arg(long, default_value_t = false)]
    pub force: bool,
}

impl Cli {
    pub fn parse_args() -> Self {
        Cli::parse()
    }
}
