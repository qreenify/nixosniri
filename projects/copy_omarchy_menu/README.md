# Omarchy Menu Documentation for NixOS

This project documents how Omarchy's menu and theme system works, and provides a plan for making it compatible with NixOS.

## Files

1. **01-structure.md** - Omarchy directory structure overview
2. **02-theme-system.md** - Detailed explanation of how omarchy themes work
3. **03-menu-and-walker.md** - Menu system and walker integration details
4. **04-nixos-setup-plan.md** - **START HERE** - Step-by-step setup guide

## Quick Start

Read **04-nixos-setup-plan.md** for the complete setup instructions.

## TL;DR - The Problem

Omarchy expects to dynamically switch themes by updating a symlink at `~/.config/omarchy/current/theme`. Home-manager creates read-only symlinks from the nix store, breaking this functionality.

## Solution

**Hybrid approach**:
- NixOS manages omarchy scripts via home-manager
- User manually copies themes to `~/.config/omarchy/themes/`
- Omarchy manages theme switching dynamically

This is documented in detail in **04-nixos-setup-plan.md**.

## Key Insights

1. **Theme symlink**: `~/.config/omarchy/current/theme` → `~/.config/omarchy/themes/<theme-name>`
2. **Walker imports**: Walker CSS imports from this symlink path
3. **NixOS incompatibility**: Read-only nix store symlinks prevent dynamic theme switching
4. **Solution**: Manual theme directory + omarchy theme commands

## Implementation Status

- ✅ Documented theme system
- ✅ Documented menu system
- ✅ Identified NixOS compatibility issues
- ✅ Created setup plan
- ⏳ Awaiting user approval to implement

## Next Steps

1. User reviews documentation
2. Decide on approach (manual themes recommended)
3. Implement changes to `~/.config/nixos/modules/home.nix`
4. Run manual theme installation commands
5. Test omarchy menu with proper theming
