{ config, pkgs, ... }:

{
  users.users.qreenify = {
    isNormalUser = true;
    description = "qreenify";
    extraGroups = [ "networkmanager" "wheel" "fuse" "video" "render" "input" ];
    shell = pkgs.nushell;
  };
}
