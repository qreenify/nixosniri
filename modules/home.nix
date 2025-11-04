{ config, pkgs, ... }:

{
  home.username = "qreenify";
  home.homeDirectory = "/home/qreenify";
  home.stateVersion = "25.05";

  programs.home-manager.enable = true;
  xdg.enable = true;

  # === Nushell Configuration ===
  programs.nushell = {
    enable = true;
    configFile.text = ''
      $env.config = {
        show_banner: false
        edit_mode: vi
      }
    '';
    shellAliases = {
      n = "nvim";
      rebuild = "sudo nixos-rebuild switch --flake /etc/nixos#nixos";
      deploy = "~/claude/deploy.sh";
      gdrive-start = "systemctl --user start rclone-gdrive";
      gdrive-stop = "systemctl --user stop rclone-gdrive";
      gdrive-status = "systemctl --user status rclone-gdrive";
    };
  };

  # === Git Configuration ===
  programs.git = {
    enable = true;
    settings = {
      user.name = "qreenify";
      user.email = "qreenify@users.noreply.github.com";
      init.defaultBranch = "main";
      pull.rebase = false;
      core.editor = "nvim";
      credential."https://github.com".helper = "!${pkgs.gh}/bin/gh auth git-credential";
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
      # Learning aid - shows keybindings as you type
      which-key-nvim

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
      vim-fugitive

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

      # Surround text objects (helps learn vim motions)
      nvim-surround

      # Better quickfix window
      trouble-nvim

      # Undo tree visualization
      undotree
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

      -- Which-key: Shows available keybindings in a popup
      -- This helps you learn shortcuts as you type!
      local wk = require("which-key")
      wk.setup({
        plugins = {
          marks = true,
          registers = true,
          spelling = { enabled = true },
        },
        window = {
          border = "rounded",
          padding = { 1, 2, 1, 2 },
        },
      })

      -- Register key groups with descriptions
      wk.register({
        ["<leader>f"] = { name = "Find (Telescope)" },
        ["<leader>g"] = { name = "Git" },
        ["<leader>c"] = { name = "Code" },
        ["<leader>t"] = { name = "Trouble/Diagnostics" },
        ["<leader>u"] = { name = "Undo Tree" },
      })

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

      -- Nvim-surround: Easily surround text with quotes, brackets, etc.
      -- Usage: ys + motion + character (e.g., ysiw" to surround word with quotes)
      --        ds + character to delete surroundings
      --        cs + old + new to change surroundings
      require('nvim-surround').setup()

      -- Trouble: Better diagnostics and quickfix list
      require('trouble').setup()
      vim.keymap.set('n', '<leader>xx', '<cmd>Trouble diagnostics toggle<cr>', { desc = 'Toggle Trouble' })
      vim.keymap.set('n', '<leader>xd', '<cmd>Trouble diagnostics toggle filter.buf=0<cr>', { desc = 'Document Diagnostics' })

      -- Undo tree: Visualize your undo history
      vim.keymap.set('n', '<leader>u', vim.cmd.UndotreeToggle, { desc = 'Toggle Undo Tree' })

      -- Git shortcuts (vim-fugitive + gitsigns)
      vim.keymap.set('n', '<leader>gs', vim.cmd.Git, { desc = 'Git Status' })
      vim.keymap.set('n', '<leader>gc', ':Git commit<CR>', { desc = 'Git Commit' })
      vim.keymap.set('n', '<leader>gp', ':Git push<CR>', { desc = 'Git Push' })
      vim.keymap.set('n', '<leader>gl', ':Git pull<CR>', { desc = 'Git Pull' })
      vim.keymap.set('n', '<leader>gb', ':Git blame<CR>', { desc = 'Git Blame' })

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

      -- Keymaps for LSP (when attached to a buffer)
      vim.api.nvim_create_autocmd('LspAttach', {
        callback = function(args)
          local opts = { buffer = args.buf }
          vim.keymap.set('n', 'gd', vim.lsp.buf.definition, vim.tbl_extend('force', opts, { desc = 'Go to Definition' }))
          vim.keymap.set('n', 'K', vim.lsp.buf.hover, vim.tbl_extend('force', opts, { desc = 'Hover Documentation' }))
          vim.keymap.set('n', '<leader>rn', vim.lsp.buf.rename, vim.tbl_extend('force', opts, { desc = 'Rename Symbol' }))
          vim.keymap.set('n', '<leader>ca', vim.lsp.buf.code_action, vim.tbl_extend('force', opts, { desc = 'Code Action' }))
          vim.keymap.set('n', 'gr', vim.lsp.buf.references, vim.tbl_extend('force', opts, { desc = 'Find References' }))
        end,
      })

      -- Basic file operations
      vim.keymap.set('n', '<leader>w', ':w<CR>', { desc = 'Save File' })
      vim.keymap.set('n', '<leader>q', ':q<CR>', { desc = 'Quit' })
      vim.keymap.set('n', '<leader>wq', ':wq<CR>', { desc = 'Save and Quit' })

      -- Window navigation (Ctrl+hjkl to move between splits)
      vim.keymap.set('n', '<C-h>', '<C-w>h', { desc = 'Move to Left Window' })
      vim.keymap.set('n', '<C-j>', '<C-w>j', { desc = 'Move to Lower Window' })
      vim.keymap.set('n', '<C-k>', '<C-w>k', { desc = 'Move to Upper Window' })
      vim.keymap.set('n', '<C-l>', '<C-w>l', { desc = 'Move to Right Window' })

      -- Better indenting (stay in visual mode)
      vim.keymap.set('v', '<', '<gv', { desc = 'Indent Left' })
      vim.keymap.set('v', '>', '>gv', { desc = 'Indent Right' })

      -- Move selected lines up/down
      vim.keymap.set('v', 'J', ":m '>+1<CR>gv=gv", { desc = 'Move Line Down' })
      vim.keymap.set('v', 'K', ":m '<-2<CR>gv=gv", { desc = 'Move Line Up' })

      -- Keep cursor centered when scrolling
      vim.keymap.set('n', '<C-d>', '<C-d>zz', { desc = 'Scroll Down (Centered)' })
      vim.keymap.set('n', '<C-u>', '<C-u>zz', { desc = 'Scroll Up (Centered)' })

      -- Learning tip: Press Space and wait to see all available shortcuts!
      -- The which-key popup will show you what you can do.
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

  # === Font Configuration ===
  fonts.fontconfig.enable = true;

  # === Packages ===
  home.packages = with pkgs; [
    # Fonts
    jetbrains-mono
    nerd-fonts.jetbrains-mono
    font-awesome

    # Niri utilities
    swaybg
    grim
    slurp
    wl-clipboard
    xwayland-satellite # Required for XWayland apps (Steam, etc.) to work with Niri

    # File managers
    nautilus           # GUI file manager for file picker dialogs
    yazi               # Terminal file manager

    # Yazi enhancements
    ffmpegthumbnailer  # Video thumbnails
    poppler-utils      # PDF preview
    fd                 # Better file search
    ripgrep            # Better content search
    fzf                # Fuzzy finding
    zoxide             # Smart directory jumping
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

  # === Rclone Google Drive Mount ===
   systemd.user.services.rclone-gdrive = {
    Unit = {
      Description = "Rclone mount for Google Team Drive (Wonderland)";
      After = [ "network-online.target" ];
      Wants = [ "network-online.target" ];
    };
    Service = {
      Type = "notify";
      ExecStartPre = "/run/current-system/sw/bin/mkdir -p /mnt/gdrive-wonderland";
      ExecStart = "${pkgs.rclone}/bin/rclone mount wonderland: /mnt/gdrive-wonderland --vfs-cache-mode writes --log-level INFO";
      ExecStop = "/run/current-system/sw/bin/fusermount -u /mnt/gdrive-wonderland";
      Restart = "on-failure";
      RestartSec = "10s";
      Environment = [ "PATH=/run/wrappers/bin/:$PATH" ];
    };
    Install = {
      WantedBy = [ "default.target" ];
    };
  };

}
