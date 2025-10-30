{ config, pkgs, ... }:

{
  # Niri compositor
  programs.niri.enable = true;

  # Waybar - started by niri, not systemd
  # programs.waybar.enable = true;  # Disabled to avoid duplicate instances

  # Display manager
  services.displayManager.ly.enable = true;

  # Security
  security.polkit.enable = true;
  security.pam.services.swaylock = {};

  # Keyring
  services.gnome.gnome-keyring.enable = true;
}
