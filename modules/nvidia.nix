{ config, pkgs, ... }:

{
  # Graphics
  hardware.graphics.enable = true;

  # NVIDIA
  services.xserver.videoDrivers = ["nvidia"];

  hardware.nvidia = {
    modesetting.enable = true;
    powerManagement.enable = false;
    nvidiaSettings = true;
    open = false;
    package = config.boot.kernelPackages.nvidiaPackages.stable;
  };
}
