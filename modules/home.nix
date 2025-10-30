{ config, pkgs, ... }:

{
  home.username = "qreenify";
  home.homeDirectory = "/home/qreenify";
  home.stateVersion = "25.05";

  programs.home-manager.enable = true;
  xdg.enable = true;

  # === Git Configuration ===
  programs.git = {
    enable = true;
    userName = "qreenify";
    userEmail = "qreenify@users.noreply.github.com";
    extraConfig = {
      init.defaultBranch = "main";
      pull.rebase = false;
      core.editor = "nvim";
    };
  };

  # === Neovim Configuration ===
  programs.neovim = {
    enable = true;
    defaultEditor = true;
    viAlias = true;
    vimAlias = true;
    vimdiffAlias = true;

    plugins = with pkgs.vimPlugins; [
      # File explorer
      nvim-tree-lua
      nvim-web-devicons

      # Fuzzy finder
      telescope-nvim
      telescope-fzf-native-nvim
      plenary-nvim

      # Statusline
      lualine-nvim

      # Syntax highlighting
      nvim-treesitter.withAllGrammars

      # Color scheme
      catppuccin-nvim

      # Auto pairs
      nvim-autopairs

      # Comment toggle
      comment-nvim

      # Git integration
      gitsigns-nvim

      # LSP
      nvim-lspconfig

      # Autocompletion
      nvim-cmp
      cmp-nvim-lsp
      cmp-buffer
      cmp-path
      luasnip
      cmp_luasnip

      # Indent guides
      indent-blankline-nvim
    ];

    extraLuaConfig = ''
      -- Basic settings
      vim.opt.number = true
      vim.opt.relativenumber = true
      vim.opt.mouse = 'a'
      vim.opt.ignorecase = true
      vim.opt.smartcase = true
      vim.opt.hlsearch = false
      vim.opt.wrap = false
      vim.opt.tabstop = 2
      vim.opt.shiftwidth = 2
      vim.opt.expandtab = true
      vim.opt.termguicolors = true
      vim.opt.signcolumn = 'yes'
      vim.opt.updatetime = 300
      vim.opt.completeopt = 'menuone,noselect'

      -- Set leader key
      vim.g.mapleader = ' '

      -- Color scheme
      require("catppuccin").setup({
        flavour = "mocha",
        transparent_background = false,
      })
      vim.cmd.colorscheme "catppuccin"

      -- Nvim-tree
      require("nvim-tree").setup({
        view = {
          width = 30,
        },
      })
      vim.keymap.set('n', '<leader>e', ':NvimTreeToggle<CR>', { silent = true })

      -- Telescope
      require('telescope').setup({
        extensions = {
          fzf = {
            fuzzy = true,
            override_generic_sorter = true,
            override_file_sorter = true,
          },
        },
      })
      require('telescope').load_extension('fzf')
      vim.keymap.set('n', '<leader>ff', '<cmd>Telescope find_files<cr>')
      vim.keymap.set('n', '<leader>fg', '<cmd>Telescope live_grep<cr>')
      vim.keymap.set('n', '<leader>fb', '<cmd>Telescope buffers<cr>')

      -- Lualine
      require('lualine').setup({
        options = {
          theme = 'catppuccin',
          icons_enabled = true,
        },
      })

      -- Treesitter
      require('nvim-treesitter.configs').setup({
        highlight = {
          enable = true,
        },
        indent = {
          enable = true,
        },
      })

      -- Autopairs
      require('nvim-autopairs').setup({})

      -- Comment
      require('Comment').setup()

      -- Gitsigns
      require('gitsigns').setup()

      -- Indent blankline
      require('ibl').setup()

      -- LSP and completion
      local cmp = require('cmp')
      local luasnip = require('luasnip')

      cmp.setup({
        snippet = {
          expand = function(args)
            luasnip.lsp_expand(args.body)
          end,
        },
        mapping = cmp.mapping.preset.insert({
          ['<C-b>'] = cmp.mapping.scroll_docs(-4),
          ['<C-f>'] = cmp.mapping.scroll_docs(4),
          ['<C-Space>'] = cmp.mapping.complete(),
          ['<C-e>'] = cmp.mapping.abort(),
          ['<CR>'] = cmp.mapping.confirm({ select = true }),
          ['<Tab>'] = cmp.mapping(function(fallback)
            if cmp.visible() then
              cmp.select_next_item()
            else
              fallback()
            end
          end, { 'i', 's' }),
          ['<S-Tab>'] = cmp.mapping(function(fallback)
            if cmp.visible() then
              cmp.select_prev_item()
            else
              fallback()
            end
          end, { 'i', 's' }),
        }),
        sources = cmp.config.sources({
          { name = 'nvim_lsp' },
          { name = 'luasnip' },
          { name = 'buffer' },
          { name = 'path' },
        }),
      })

      -- Basic LSP setup for common languages
      local lspconfig = require('lspconfig')
      local capabilities = require('cmp_nvim_lsp').default_capabilities()

      -- Keymaps for LSP
      vim.api.nvim_create_autocmd('LspAttach', {
        callback = function(args)
          local opts = { buffer = args.buf }
          vim.keymap.set('n', 'gd', vim.lsp.buf.definition, opts)
          vim.keymap.set('n', 'K', vim.lsp.buf.hover, opts)
          vim.keymap.set('n', '<leader>rn', vim.lsp.buf.rename, opts)
          vim.keymap.set('n', '<leader>ca', vim.lsp.buf.code_action, opts)
        end,
      })

      -- Additional keymaps
      vim.keymap.set('n', '<leader>w', ':w<CR>')
      vim.keymap.set('n', '<leader>q', ':q<CR>')
      vim.keymap.set('n', '<C-h>', '<C-w>h')
      vim.keymap.set('n', '<C-j>', '<C-w>j')
      vim.keymap.set('n', '<C-k>', '<C-w>k')
      vim.keymap.set('n', '<C-l>', '<C-w>l')
    '';
  };

  # === Niri Configuration ===
  xdg.configFile."niri/config.kdl".source = ../config/niri/config.kdl;

  # === Waybar Configuration ===
  programs.waybar = {
    enable = true;
    systemd.enable = true;
  };
  xdg.configFile."waybar/config.jsonc".source = ../config/waybar/config.jsonc;
  xdg.configFile."waybar/style.css".source = ../config/waybar/style.css;

  # === Fuzzel Configuration ===
  xdg.configFile."fuzzel/fuzzel.ini".source = ../config/fuzzel/fuzzel.ini;

  # === Alacritty Terminal ===
  programs.alacritty = {
    enable = true;
    settings = {
      font = {
        size = 14.0;
        normal.family = "JetBrainsMono Nerd Font";
        bold.family = "JetBrainsMono Nerd Font";
        italic.family = "JetBrainsMono Nerd Font";
        bold_italic.family = "JetBrainsMono Nerd Font";
      };

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

  # === Dark Mode Theme ===
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
    gtk3.extraConfig.gtk-application-prefer-dark-theme = true;
    gtk4.extraConfig.gtk-application-prefer-dark-theme = true;
  };

  qt = {
    enable = true;
    platformTheme.name = "adwaita";
    style.name = "adwaita-dark";
  };

  dconf.settings."org/gnome/desktop/interface".color-scheme = "prefer-dark";

  home.sessionVariables = {
    GTK_THEME = "Adwaita:dark";
    QT_STYLE_OVERRIDE = "adwaita-dark";
  };

  # === Packages ===
  home.packages = with pkgs; [
    # Fonts
    jetbrains-mono
    nerd-fonts.jetbrains-mono

    # Niri utilities
    swaybg
    grim
    slurp
    wl-clipboard
  ];

  # === Auto-backup NixOS config to GitHub ===
  systemd.user.services.nixos-config-backup = {
    Unit = {
      Description = "Backup NixOS configuration to GitHub";
    };
    Service = {
      Type = "oneshot";
      WorkingDirectory = "/home/qreenify/claude";
      ExecStart = pkgs.writeShellScript "nixos-config-backup" ''
        set -e
        ${pkgs.git}/bin/git add -A
        if ! ${pkgs.git}/bin/git diff-index --quiet HEAD --; then
          ${pkgs.git}/bin/git commit -m "Auto-backup: $(date '+%Y-%m-%d %H:%M:%S')"
          ${pkgs.git}/bin/git push origin main
        fi
      '';
    };
  };

  systemd.user.timers.nixos-config-backup = {
    Unit = {
      Description = "Timer for NixOS configuration backup";
    };
    Timer = {
      OnBootSec = "5m";
      OnUnitActiveSec = "1h";
      Persistent = true;
    };
    Install = {
      WantedBy = [ "timers.target" ];
    };
  };
}
