# Libgen TUI - Download books over the terminal
![](./assets/showcase.png)

Download any book over the internet through the terminal, quick and easy. Without dealing with annoying popups, ads, navigation, etc.

## Features
- asynchronous downloads allowing multiple books at the same time
- configurable download directory
- automatic failover between mirrors when one stops responding

## Installation

### Prebuilt binaries
Grab the archive for your platform from the [latest release](https://github.com/Houdiee/libgen-tui/releases/latest). Each one holds the binary, the README and the LICENSE, and is published next to a matching `.sha256` file.

| Platform | Asset |
| --- | --- |
| Linux (x86_64) | `libgen-tui-<version>-x86_64-unknown-linux-gnu.tar.gz` |
| macOS (Apple Silicon) | `libgen-tui-<version>-aarch64-apple-darwin.tar.gz` |
| macOS (Intel) | `libgen-tui-<version>-x86_64-apple-darwin.tar.gz` |
| Windows (x86_64) | `libgen-tui-<version>-x86_64-pc-windows-msvc.zip` |

#### Linux
```sh
tar -xzf libgen-tui-*-x86_64-unknown-linux-gnu.tar.gz
cd libgen-tui-*-x86_64-unknown-linux-gnu
install -Dm755 libgen-tui ~/.local/bin/libgen-tui
```
Make sure `~/.local/bin` is on your `$PATH`, or use `sudo install -m755 libgen-tui /usr/local/bin/` instead.

#### macOS
```sh
tar -xzf libgen-tui-*-apple-darwin.tar.gz
cd libgen-tui-*-apple-darwin
sudo install -m755 libgen-tui /usr/local/bin/
```
The binaries are not signed, so macOS quarantines anything downloaded from a browser. If you get *"cannot be opened because the developer cannot be verified"*, clear the flag:
```sh
xattr -d com.apple.quarantine /usr/local/bin/libgen-tui
```

#### Windows
In PowerShell, extract the `.zip`, move `libgen-tui.exe` somewhere permanent and put that folder on your `PATH`:
```powershell
Expand-Archive .\libgen-tui-*-x86_64-pc-windows-msvc.zip -DestinationPath .
$dest = "$env:LOCALAPPDATA\Programs\libgen-tui"
New-Item -ItemType Directory -Force -Path $dest
Move-Item .\libgen-tui-*-x86_64-pc-windows-msvc\libgen-tui.exe $dest
[Environment]::SetEnvironmentVariable("Path", "$([Environment]::GetEnvironmentVariable('Path','User'));$dest", "User")
```
Reopen your terminal afterwards. Use Windows Terminal rather than the legacy console host, which renders the interface poorly.

### From source
Needs a [Rust toolchain](https://rustup.rs). There is no OpenSSL or other system dependency to install first.
```sh
cargo install --git https://github.com/Houdiee/libgen-tui
```
This drops the binary in `~/.cargo/bin`. To build a checkout instead:
```sh
git clone https://github.com/Houdiee/libgen-tui
cd libgen-tui
cargo build --release
```
The binary is then at `target/release/libgen-tui`; copy it somewhere on your `PATH`.

### NixOS users
If you are struggling to build on nixos, run `nix-shell` within the project directory first, before running `cargo build --release`.

## Configuration
On first run `libgen-tui` writes a configuration file if there isn't one already:

| Platform | Config file | Default download directory |
| --- | --- | --- |
| Linux | `$XDG_CONFIG_HOME/libgen-tui/config.toml` (usually `~/.config/...`) | `~/libgen-tui` |
| macOS | `~/Library/Application Support/libgen-tui/config.toml` | `~/libgen-tui` |
| Windows | `%APPDATA%\libgen-tui\config.toml` | `C:\Users\<you>\libgen-tui` |

Below is the default configuration:
```toml
mirrors = ["libgen.li", "libgen.vg", "libgen.la", "libgen.bz"] # Which libgen domains to use
download_directory = "/home/{user}/libgen-tui" # The default download path, where all books will be downloaded to 
max_results = 50 # How many results to display. Accepted values are 25, 50 or 100
```

On Windows, write paths with either forward slashes or escaped backslashes, since TOML treats a single backslash as an escape: `download_directory = "C:/Users/you/Books"`.

If an existing config still lists older mirrors such as `libgen.is` or `libgen.rs`, those domains are gone and searching will fail until the list is updated to the mirrors above.

## Troubleshooting
Libgen changes its domains and page layout from time to time, which breaks searching or downloading. To find out which stage broke, run the live tests:
```sh
cargo test
```
They check mirror reachability, result parsing, download-link resolution and an actual download, and report which one fails.

Run with `RUST_LOG=debug libgen-tui 2>log.txt` to record what each request did. Logging is off by default because it writes to stderr, which would otherwise draw over the interface.
