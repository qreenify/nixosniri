{ config, pkgs, ... }:

{
  home.username = "qreenify";
  home.homeDirectory = "/home/qreenify";

  # This value determines the Home Manager release that your configuration is
  # compatible with. This helps avoid breakage when a new Home Manager release
  # introduces backwards incompatible changes.
  home.stateVersion = "25.05";

  # Let Home Manager install and manage itself.
  programs.home-manager.enable = true;

  # XDG configuration directories
  xdg.enable = true;

  # Configure niri
  xdg.configFile."niri/config.kdl".source = ./config/niri/config.kdl;

  # Configure Waybar
  programs.waybar = {
    enable = true;
    systemd.enable = true;
  };
  xdg.configFile."waybar/config.jsonc".source = ./config/waybar/config.jsonc;
  xdg.configFile."waybar/style.css".source = ./config/waybar/style.css;

  # Configure Alacritty
  programs.alacritty = {
    enable = true;
    settings = {
      font = {
        size = 14.0;
        normal = {
          family = "JetBrainsMono Nerd Font";
          style = "Regular";
        };
        bold = {
          family = "JetBrainsMono Nerd Font";
          style = "Bold";
        };
        italic = {
          family = "JetBrainsMono Nerd Font";
          style = "Italic";
        };
        bold_italic = {
          family = "JetBrainsMono Nerd Font";
          style = "Bold Italic";
        };
      };
      # Dark theme colors
      colors = {
        primary = {
          background = "#1e1e1e";
          foreground = "#d4d4d4";
        };
        cursor = {
          text = "#1e1e1e";
          cursor = "#d4d4d4";
        };
        normal = {
          black = "#1e1e1e";
          red = "#f48771";
          green = "#a9dc76";
          yellow = "#ffd866";
          blue = "#78dce8";
          magenta = "#ab9df2";
          cyan = "#78dce8";
          white = "#fcfcfa";
        };
        bright = {
          black = "#5b5858";
          red = "#f48771";
          green = "#a9dc76";
          yellow = "#ffd866";
          blue = "#78dce8";
          magenta = "#ab9df2";
          cyan = "#78dce8";
          white = "#fcfcfa";
        };
      };
    };
  };

  # Configure fuzzel
  xdg.configFile."fuzzel/fuzzel.ini".source = ./config/fuzzel/fuzzel.ini;

  # GTK theme for dark mode
  gtk = {
    enable = true;
    theme = {
      name = "Adwaita-dark";
      package = pkgs.gnome-themes-extra;
    };
    iconTheme = {
      name = "Adwaita";
      package = pkgs.adwaita-icon-theme;
    };
    gtk3.extraConfig = {
      gtk-application-prefer-dark-theme = true;
    };
    gtk4.extraConfig = {
      gtk-application-prefer-dark-theme = true;
    };
  };

  # Qt theme for dark mode
  qt = {
    enable = true;
    platformTheme.name = "adwaita";
    style.name = "adwaita-dark";
  };

  # Set dark theme preference for applications
  dconf.settings = {
    "org/gnome/desktop/interface" = {
      color-scheme = "prefer-dark";
    };
  };

  # Environment variables for dark mode
  home.sessionVariables = {
    GTK_THEME = "Adwaita:dark";
    QT_STYLE_OVERRIDE = "adwaita-dark";
  };

  # Additional packages for your home environment
  home.packages = with pkgs; [
    # Fonts
    jetbrains-mono
    nerd-fonts.jetbrains-mono

    # Niri utilities
    swaybg
    grim
    slurp
    wl-clipboard

    # Audio
    noisetorch
  ];

  # NoiseTorch systemd service for automatic noise suppression
  systemd.user.services.noisetorch = {
    Unit = {
      Description = "NoiseTorch Noise Suppression";
      After = [ "pipewire.service" ];
      Requires = [ "pipewire.service" ];
    };
    Service = {
      Type = "oneshot";
      RemainAfterExit = true;
      ExecStart = "${pkgs.noisetorch}/bin/noisetorch -i -s alsa_input.usb-Beyerdynamic_FOX-00.mono-fallback";
      ExecStop = "${pkgs.noisetorch}/bin/noisetorch -u";
      Restart = "on-failure";
    };
    Install = {
      WantedBy = [ "default.target" ];
    };
  };
}
