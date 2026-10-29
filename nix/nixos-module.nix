self:
{
  config,
  lib,
  pkgs,
  ...
}:

let
  cfg = config.services.biternet;
in
{
  options.services.biternet = {
    enable = lib.mkEnableOption "BITERNET desktop client";

    package = lib.mkOption {
      type = lib.types.package;
      default = self.packages.${pkgs.stdenv.hostPlatform.system}.default;
      description = "The BITERNET package to use.";
    };

    autoStart = lib.mkOption {
      type = lib.types.bool;
      default = false;
      description = "Start BITERNET automatically with the graphical session.";
    };

    silentStart = lib.mkOption {
      type = lib.types.bool;
      default = false;
      description = "Start BITERNET with its window hidden and only the tray icon visible.";
    };
  };

  config = lib.mkMerge [
    (lib.mkIf cfg.enable {
      environment.systemPackages = [ cfg.package ];
    })

    (lib.mkIf cfg.autoStart {
      assertions = [
        {
          assertion = cfg.enable;
          message = "services.biternet.autoStart requires services.biternet.enable";
        }
      ];

      systemd.user.services.biternet = {
        description = "BIT SRUN gateway client";
        after = [ "graphical-session.target" ];
        partOf = [ "graphical-session.target" ];
        wantedBy = [ "graphical-session.target" ];

        serviceConfig = {
          ExecStart = lib.escapeShellArgs (
            [ (lib.getExe cfg.package) ] ++ lib.optional cfg.silentStart "--silent"
          );
          Restart = "on-failure";
          RestartSec = 3;
        };
      };
    })
  ];
}
