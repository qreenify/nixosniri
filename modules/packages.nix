{ config, pkgs, ... }:

{
  # Enable Steam
  programs.steam = {
    enable = true;
    remotePlay.openFirewall = true;
    dedicatedServer.openFirewall = true;
  };

  # Enable gamemode for better gaming performance
  programs.gamemode.enable = true;

  environment.systemPackages = with pkgs; [
    # System utilities
    git
    gh
    sbctl

    # Development
    claude-code
    neovim
    vscodium

    # Browsers
    brave
    chromium

    # Communication
    discord

    # Desktop utilities
    alacritty
    fuzzel
    swayidle
    swaylock
    mako

    # Gaming
    lutris
    prismlauncher
    mangohud
    gamemode
    gamescope
    protontricks
    winetricks
    wine-staging
  ];
}
