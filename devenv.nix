{ lib, pkgs, ... }:

let
  desktopLibraries = import ./nix/desktop-libraries.nix { inherit pkgs; };
in
{
  languages.rust = {
    enable = true;
    toolchainFile = ./rust-toolchain.toml;
  };

  packages =
    with pkgs;
    [
      actionlint
      nixfmt-tree
      pkg-config
      tombi
    ]
    ++ desktopLibraries;

  env.LD_LIBRARY_PATH = "/run/opengl-driver/lib:${lib.makeLibraryPath desktopLibraries}";
}
