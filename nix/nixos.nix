{ lib, config, pkgs, ... }:
let
  cfg = config.services.protontweaks;

  inherit (lib) mkIf mkEnableOption;
in
{
  options.services.protontweaks = {
    enable = mkEnableOption "protontweaks";

    watch = {
      enable = mkEnableOption "protontweaks watch service" // {
        default = true;
      };
    };
  };

  config = mkIf (cfg.enable) {
    environment.systemPackages = with pkgs; [
      protontweaks
      protontricks # Install this for now until we figure out why nix-shell isn't working
    ];

    systemd.services.protontweaks = mkIf (cfg.watch.enable) {
      enable = true;
      description = "Protontweaks Watch Service";
      serviceConfig = {
        Type = "simple";
        ExecStart = "${pkgs.protontweaks}/bin/protontweaks watch";
      };
      wantedBy = [ "multi-user.target" ];
    };
  };
}
