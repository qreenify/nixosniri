{ config, pkgs, zen-browser, ... }:

{
  # Enable Steam
  programs.steam = {
    enable = true;
    remotePlay.openFirewall = true;
    dedicatedServer.openFirewall = true;
  };

  # Enable gamemode for better gaming performance
  programs.gamemode.enable = true;

  # Sunshine game streaming server
  services.sunshine = {
    enable = true;
    openFirewall = true;
    capSysAdmin = true;
  };

  environment.systemPackages = with pkgs; [
    # System utilities
    git
    gh
    sbctl
    rclone

    # Development
    claude-code
    neovim
    vscodium

    # Browsers
    brave
    chromium
    zen-browser.packages."${pkgs.system}".default

    # Communication
    discord

    # Desktop utilities
    alacritty
    fuzzel
    swayidle
    swaylock
    mako
    wiremix

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
