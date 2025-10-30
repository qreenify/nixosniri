{ config, pkgs, ... }:

{
  users.users.qreenify = {
    isNormalUser = true;
    description = "qreenify";
    extraGroups = [ "networkmanager" "wheel" ];
  };
}
