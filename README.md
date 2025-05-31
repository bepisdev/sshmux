# sshmux

Run a shell command concurrently on multiple SSH hosts defined in a TOML config.

## Features

* 🧹 Run any shell command over SSH on multiple hosts concurrently.
* 🎨 Color-coded output for clarity.
* ✅ Config validation (with unknown key rejection and duplicate host detection).
* 💪 Supports per-host `user`, `port`, and `identity_file` settings.
* 🛠️ `--check-config` mode to validate configs before running.
* 🛡️ Optional `--force` flag to allow duplicate hosts.

---

## Installation

### Requirements

* Rust ([https://rustup.rs](https://rustup.rs))
* OpenSSH installed and available on your system

### Build & Install

```bash
git clone https://github.com/yourusername/sshmux.git
cd sshmux
make
sudo make install
```

This will install `sshmux` to `/usr/local/bin/`.

---

## Configuration

Create a `sshmux.toml` file in the same directory or specify a path via `--config`.

```toml
command = "uptime"

[[hosts]]
host = "192.168.1.10"
user = "josh"
port = 22
identity_file = "~/.ssh/id_rsa"

[[hosts]]
host = "192.168.1.11"

[[hosts]]
host = "192.168.1.10"
user = "admin"
```

> ⚠️ By default, duplicate hosts will trigger a validation error. Use `--force` to allow duplicates.

---

## Usage

```bash
sshmux --config sshmux.toml
```

### Options

```bash
-c, --config <PATH>       Path to the TOML config file (default: sshmux.toml)
-v, --verbose             Enable verbose output
    --check-config        Only check the config for validity and exit
    --force               Allow duplicate hosts in the config
```

---

## Example

```bash
sshmux -c ./sshmux.toml -v
```

Or just to validate the config:

```bash
sshmux --check-config
```

---

## License

MIT © Josh Burns
