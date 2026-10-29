# BITERNET

[简体中文](./README.md) | English

[![CI](https://github.com/bitoolkit/biternet/actions/workflows/ci.yml/badge.svg)](https://github.com/bitoolkit/biternet/actions/workflows/ci.yml)
[![Cachix Cache](https://img.shields.io/badge/cachix-so1ve-blue.svg)](https://so1ve.cachix.org)

A desktop client for the BIT campus network. Log in, log out, and view your IP address, data usage, online time, and account balance. The gateway address is `10.0.0.55`.

Built with Rust and Iced for Windows, macOS, and Linux.

## Installation

### Download a release

Download an installer or portable version for your system from [GitHub Releases](https://github.com/bitoolkit/biternet/releases).

Linux requires a desktop with system tray support (StatusNotifierItem), graphics drivers, and fonts with Chinese glyphs.

### Cargo

With Rust installed, run:

```sh
cargo install biternet
```

On Linux, install the [build dependencies](#linux-dependencies) first.

### Nix

Run directly on Linux:

```sh
nix run github:bitoolkit/biternet
```

See [NixOS and Home Manager](#nixos-and-home-manager) for startup configuration.

## Usage

The interface is in Chinese.

Connect to the BIT campus network, then enter your student ID and unified authentication password to log in. Use Tab to switch between input fields and Enter to submit. Once logged in, you can view your connection details or log out.

Automatic login also enables password saving. The app then attempts to log in whenever it detects that you are logged out. Logging out manually pauses automatic login.

Saved usernames and passwords are stored in plain text in the local `credentials.json` file. Turning off password saving clears the saved credentials.

Open Settings (“设置”) to enable startup at login (“开机启动”). You can then hide the window on startup (“启动时隐藏窗口”) to show only the tray icon. You can also pass `--silent` when launching the app manually.

Use the close button in the top right or press Esc to close the window. The app keeps running in the tray. Choose “显示窗口” from the tray menu to reopen it, or “退出程序” to quit. Quit from the tray before replacing the app with a newer version.

The app checks GitHub Releases on startup. When an update is available, click “有更新” at the bottom of the window to open the download options.

## Upgrading from BITGATEWAY

The app, command, and Cargo package are now named `biternet`, and the protocol library is `biternet-client`. Once the new packages are published, Cargo users can install `biternet`, verify it works, then run `cargo uninstall bitgateway`. Quit the old version from the tray before starting the new one.

For Nix, change the repository URL to `github:bitoolkit/biternet` and rename `inputs.bitgateway` and `services.bitgateway` to `inputs.biternet` and `services.biternet`. The existing configuration directory is retained, so credentials and settings do not need to be moved.

## NixOS and Home Manager

Add this flake input:

```nix
inputs.biternet.url = "github:bitoolkit/biternet";
```

NixOS configuration:

```nix
{ inputs, ... }:

{
  imports = [ inputs.biternet.nixosModules.default ];

  services.biternet = {
    enable = true;
    autoStart = true;
    silentStart = true;
  };
}
```

Home Manager configuration:

```nix
{ inputs, ... }:

{
  imports = [ inputs.biternet.homeManagerModules.default ];

  services.biternet = {
    enable = true;
    autoStart = true;
    silentStart = true;
  };
}
```

`autoStart` starts the app with your graphical session. With `autoStart` enabled, `silentStart` shows only the tray icon. The modules manage startup through a systemd user service. Change these options in your Nix configuration.

Optional Cachix cache configuration:

```nix
nix.settings = {
  extra-substituters = [ "https://so1ve.cachix.org" ];
  extra-trusted-public-keys = [
    "so1ve.cachix.org-1:51jcW4FkJhiLcqPsiUx3nglRP469les8F9zjFxio1nw="
  ];
};
```

## Run from source

Install rustup first. The repository selects Rust nightly through `rust-toolchain.toml`. On Linux, install the [build dependencies](#linux-dependencies) first.

```sh
git clone https://github.com/bitoolkit/biternet.git
cd biternet
rustup toolchain install nightly
cargo run -p biternet
```

### Linux dependencies

On Debian / Ubuntu:

```sh
sudo apt-get update
sudo apt-get install -y \
  pkg-config \
  libwayland-dev \
  libxkbcommon-dev \
  libx11-dev \
  libxcursor-dev \
  libxi-dev \
  libxrandr-dev \
  libegl1-mesa-dev \
  libvulkan-dev \
  fontconfig \
  fonts-noto-cjk \
  xdg-utils
```

## Packaging

Run these commands from the repository root. Install the packager and build a release binary:

```sh
cargo install cargo-packager --version 0.11.8 --locked
cargo build --release -p biternet --locked
```

Choose the command for your current system:

| System | Format | Command |
| --- | --- | --- |
| Windows | NSIS installer | `cargo packager --release -p biternet --formats nsis` |
| macOS | .app, DMG | `cargo packager --release -p biternet --formats app,dmg` |
| Linux | AppImage, deb | `cargo packager --release -p biternet --formats appimage,deb` |

Linux packaging also requires:

```sh
sudo apt-get install -y librsvg2-dev patchelf libfuse2
```

To create an RPM package, install the additional tool:

```sh
cargo install cargo-generate-rpm --locked
cargo generate-rpm -p crates/biternet -o dist
```

Packages are written to `dist/`. The Windows portable executable is at `target/release/biternet.exe`.

## Acknowledgements

The protocol implementation in `biternet-client` is based on [bitsrun-rs](https://github.com/spencerwooo/bitsrun-rs).

## LICENSE

[MIT](./LICENSE). Made with ❤️ by [Ray](https://github.com/so1ve)
