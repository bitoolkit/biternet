# BITGATEWAY

简体中文 | [English](./README_EN.md)

[![CI](https://github.com/bitoolkit/bitgateway/actions/workflows/ci.yml/badge.svg)](https://github.com/bitoolkit/bitgateway/actions/workflows/ci.yml)
[![Cachix Cache](https://img.shields.io/badge/cachix-so1ve-blue.svg)](https://so1ve.cachix.org)

BIT 校园网桌面客户端，可登录、注销，查看 IP、已用流量、已用时长和账户余额。网关地址为 `10.0.0.55`。

使用 Rust 和 Iced 开发，支持 Windows、macOS 和 Linux。

## 安装

### 下载安装包

从 [GitHub Releases](https://github.com/bitoolkit/bitgateway/releases) 下载对应系统的安装包或便携版。

Linux 桌面需支持系统托盘（StatusNotifierItem），并安装图形驱动和中文字体。

### Cargo

安装 Rust 后运行：

```sh
cargo install bitgateway
```

Linux 用户需先安装[编译依赖](#linux-依赖)。

### Nix

在 Linux 上直接运行：

```sh
nix run github:bitoolkit/bitgateway
```

自动启动配置见 [NixOS 与 Home Manager](#nixos-与-home-manager)。

## 使用

连接 BIT 校园网后，输入学号和统一认证密码登录。可用 Tab 切换输入框，按 Enter 提交。登录后可查看上网信息或注销。

“自动登录”会同时启用“记住密码”，并在检测到未登录时尝试登录。手动注销会暂停自动登录。

“记住密码”会将用户名和密码以明文保存在本机的 `credentials.json` 中。取消勾选会清除已保存的凭据。

在“设置”中开启“开机启动”后，可选“启动时隐藏窗口”，让下次启动只显示托盘。手动启动时也可传入 `--silent`。

点击右上角的关闭按钮或按 Esc 可关闭窗口，应用仍在托盘运行。通过托盘菜单的“显示窗口”重新打开，选择“退出程序”结束运行。更新程序前也需先从托盘退出。

应用启动时会检查 GitHub Releases，发现新版本后在底部显示“有更新”。

## NixOS 与 Home Manager

在 flake inputs 中添加：

```nix
inputs.bitgateway.url = "github:bitoolkit/bitgateway";
```

NixOS 配置：

```nix
{ inputs, ... }:

{
  imports = [ inputs.bitgateway.nixosModules.default ];

  services.bitgateway = {
    enable = true;
    autoStart = true;
    silentStart = true;
  };
}
```

Home Manager 配置：

```nix
{ inputs, ... }:

{
  imports = [ inputs.bitgateway.homeManagerModules.default ];

  services.bitgateway = {
    enable = true;
    autoStart = true;
    silentStart = true;
  };
}
```

`autoStart` 随图形会话启动应用。启用后，`silentStart` 可让应用只显示托盘。模块通过 systemd 用户服务管理启动，请在 Nix 配置中修改这些选项。

可选的 Cachix 缓存配置：

```nix
nix.settings = {
  extra-substituters = [ "https://so1ve.cachix.org" ];
  extra-trusted-public-keys = [
    "so1ve.cachix.org-1:51jcW4FkJhiLcqPsiUx3nglRP469les8F9zjFxio1nw="
  ];
};
```

## 从源码运行

需安装 rustup。仓库通过 `rust-toolchain.toml` 选择 Rust nightly。Linux 用户需先安装[编译依赖](#linux-依赖)。

```sh
git clone https://github.com/bitoolkit/bitgateway.git
cd bitgateway
rustup toolchain install nightly
cargo run -p bitgateway
```

### Linux 依赖

在 Debian / Ubuntu 上安装：

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

## 打包

以下命令均在仓库根目录执行。先安装打包工具并构建发布版本：

```sh
cargo install cargo-packager --version 0.11.8 --locked
cargo build --release -p bitgateway --locked
```

按当前系统选择打包命令：

| 系统 | 格式 | 命令 |
| --- | --- | --- |
| Windows | NSIS 安装包 | `cargo packager --release -p bitgateway --formats nsis` |
| macOS | .app、DMG | `cargo packager --release -p bitgateway --formats app,dmg` |
| Linux | AppImage、deb | `cargo packager --release -p bitgateway --formats appimage,deb` |

Linux 打包还需安装：

```sh
sudo apt-get install -y librsvg2-dev patchelf libfuse2
```

生成 RPM 包需另装工具：

```sh
cargo install cargo-generate-rpm --locked
cargo generate-rpm -p crates/bitgateway -o dist
```

安装包输出到 `dist/`。Windows 便携程序位于 `target/release/bitgateway.exe`。

## 致谢

`bitgateway-client` 的协议实现参考了 [bitsrun-rs](https://github.com/spencerwooo/bitsrun-rs)。

## LICENSE

[MIT](./LICENSE). Made with ❤️ by [Ray](https://github.com/so1ve)
