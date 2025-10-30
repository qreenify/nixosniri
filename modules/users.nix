{ config, pkgs, ... }:

{
  # Enable nushell system-wide
  programs.nushell.enable = true;

  users.users.qreenify = {
    isNormalUser = true;
    description = "qreenify";
    extraGroups = [ "networkmanager" "wheel" ];
    shell = pkgs.nushell;
  };
}
