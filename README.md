# Libgen TUI - Download books over the terminal
![](./assets/showcase.png)

Download any book over the internet through the terminal, quick and easy. Without dealing with annoying popups, ads, navigation, etc.

## Features
- asynchronous downloads allowing multiple books at the same time
- configurable download directory
- automatic failover between mirrors when one stops responding

## Installation
1. `git clone https://github.com/Houdiee/libgen-tui`
2. `cd libgen-tui` && `cargo build --release`
3. Ensure the binary exists in system $PATH

### NixOS users
If you are struggling to build on nixos, run `nix-shell` within the project directory first, before running `cargo build --release`.

## Configuration
By default, running `libgen-tui` will create a configuration file at `$XDG_CONFIG_HOME/libgen-tui/config.toml` if there isn't one. Below is the defaut configuration:
```toml
mirrors = ["libgen.li", "libgen.vg", "libgen.la", "libgen.bz"] # Which libgen domains to use
download_directory = "/home/{user}/libgen-tui" # The default download path, where all books will be downloaded to 
max_results = 50 # How many results to display. Accepted values are 25, 50 or 100
```

If an existing config still lists older mirrors such as `libgen.is` or `libgen.rs`, those domains are gone and searching will fail until the list is updated to the mirrors above.

## Troubleshooting
Libgen changes its domains and page layout from time to time, which breaks searching or downloading. To find out which stage broke, run the live tests:
```sh
cargo test
```
They check mirror reachability, result parsing, download-link resolution and an actual download, and report which one fails.

Run with `RUST_LOG=debug libgen-tui 2>log.txt` to record what each request did. Logging is off by default because it writes to stderr, which would otherwise draw over the interface.
