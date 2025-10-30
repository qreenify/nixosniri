{
  description = "NixOS configuration with Lanzaboote, home-manager and niri";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    lanzaboote = {
      url = "github:nix-community/lanzaboote/v0.4.2";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    home-manager = {
      url = "github:nix-community/home-manager";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    zen-browser = {
      url = "github:MarceColl/zen-browser-flake";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, lanzaboote, home-manager, zen-browser, ... }: {
    nixosConfigurations.nixos = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      specialArgs = { inherit zen-browser; };
      modules = [
        # Hardware
        ./hardware-configuration.nix

        # System modules
        ./modules/boot.nix
        ./modules/networking.nix
        ./modules/locale.nix
        ./modules/nvidia.nix
        ./modules/packages.nix
        ./modules/users.nix
        ./modules/desktop.nix

        # Lanzaboote
        lanzaboote.nixosModules.lanzaboote

        # Home Manager
        home-manager.nixosModules.home-manager
        {
          home-manager.useGlobalPkgs = true;
          home-manager.useUserPackages = true;
          home-manager.users.qreenify = import ./modules/home.nix;
        }
      ];
    };
  };
}
