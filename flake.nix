{
  description = "Desktop client for the BIT SRUN gateway";

  nixConfig = {
    extra-substituters = [ "https://so1ve.cachix.org" ];
    extra-trusted-public-keys = [
      "so1ve.cachix.org-1:51jcW4FkJhiLcqPsiUx3nglRP469les8F9zjFxio1nw="
    ];
  };

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { self, nixpkgs }:
    let
      workspaceCargoToml = fromTOML (builtins.readFile ./Cargo.toml);
      packageCargoToml = fromTOML (builtins.readFile ./crates/bitgateway/Cargo.toml);
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
      mkPackage =
        pkgs:
        let
          desktopLibraries = import ./nix/desktop-libraries.nix { inherit pkgs; };
        in
        pkgs.rustPlatform.buildRustPackage {
          pname = packageCargoToml.package.name;
          inherit (workspaceCargoToml.workspace.package) version;

          src = pkgs.lib.fileset.toSource {
            root = ./.;
            fileset = pkgs.lib.fileset.unions [
              ./Cargo.lock
              ./Cargo.toml
              ./README.md
              ./crates
            ];
          };

          cargoLock.lockFile = ./Cargo.lock;
          cargoBuildFlags = [
            "--package"
            packageCargoToml.package.name
          ];

          nativeBuildInputs = with pkgs; [
            makeWrapper
            pkg-config
          ];

          buildInputs = desktopLibraries;

          postInstall = ''
            install -Dm644 crates/bitgateway/assets/icons/icon.png \
              $out/share/icons/hicolor/256x256/apps/bitgateway.png
            install -Dm644 crates/bitgateway/assets/bitgateway.desktop \
              $out/share/applications/bitgateway.desktop
            wrapProgram "$out/bin/bitgateway" \
              --prefix PATH : ${pkgs.lib.makeBinPath [ pkgs.xdg-utils ]} \
              --prefix LD_LIBRARY_PATH : "/run/opengl-driver/lib:${pkgs.lib.makeLibraryPath desktopLibraries}"
          '';

          meta = {
            inherit (packageCargoToml.package) description;
            homepage = "https://github.com/bitoolkit/bitgateway";
            license = pkgs.lib.licenses.mit;
            mainProgram = "bitgateway";
            platforms = pkgs.lib.platforms.linux;
          };
        };
    in
    {
      packages = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          bitgateway = mkPackage pkgs;
        in
        {
          inherit bitgateway;
          default = bitgateway;
        }
      );

      apps = forAllSystems (
        system:
        let
          app = {
            type = "app";
            program = nixpkgs.lib.getExe self.packages.${system}.bitgateway;
            meta.description = packageCargoToml.package.description;
          };
        in
        {
          bitgateway = app;
          default = app;
        }
      );

      formatter = forAllSystems (system: nixpkgs.legacyPackages.${system}.nixfmt-tree);

      homeManagerModules.default = import ./nix/home-manager-module.nix self;
      nixosModules.default = import ./nix/nixos-module.nix self;

      overlays.default = final: _prev: {
        bitgateway = mkPackage final;
      };
    };
}
