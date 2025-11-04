{ config, pkgs, ... }:

{
  # Niri compositor
  programs.niri.enable = true;

  # Hyprland - Stable tiling Wayland compositor
  programs.hyprland.enable = true;

  # COSMIC Desktop Environment (experimental)
  services.desktopManager.cosmic.enable = true;

  # Enable XWayland for compatibility (Steam, etc.)
  programs.xwayland.enable = true;

  # Waybar - started by niri, not systemd
  # programs.waybar.enable = true;  # Disabled to avoid duplicate instances

  # Display manager
  # Using COSMIC's native display manager
  services.displayManager.cosmic-greeter.enable = true;

  # Flatpak support (for Discord with Krisp)
  services.flatpak.enable = true;

  # Security
  security.polkit.enable = true;
  security.pam.services.swaylock = {};

  # Keyring
  services.gnome.gnome-keyring.enable = true;
}
