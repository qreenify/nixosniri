{ config, pkgs, ... }:

{
  networking = {
    hostName = "wondernixlandos";
    networkmanager.enable = true;
  };
}
