# SSHMux

sshmux is a CLI tool to run a shell command concurrently across multiple SSH hosts as defined in a TOML config file. Each host's output is displayed in real time and prefixed with a uniquely colored `[HOSTNAME]` label.

## Installation

``` bash
git clone https://github.com/yourusername/ssh-runner.git
cd ssh-runner
cargo build --release
```

## Usage

``` bash
./target/release/sshmux --config ./sshmux.toml --verbose
```

### CLI Options

- `-c, --config <FILE>` - TOML config file (default: `sshmux.toml`)
- `-v, --verbose` - Enables verbose output

## Configuration

Create a `sshmux.toml` file 

``` bash
command = "uptime"
hosts = [
    "host1.example.com",
    "host2.example.com"
]
```

