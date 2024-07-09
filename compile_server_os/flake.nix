{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.disko.url = "github:nix-community/disko";
  inputs.disko.inputs.nixpkgs.follows = "nixpkgs";

  inputs.learnbevy = {
      type = "github";
      owner = "LiamGallagher737";
      repo = "learnbevy";
      ref = "nginx-no-cloudflare";
      dir = "compile_api";
  };

  outputs = inputs@{ self, nixpkgs, disko, ... }:
    {
      nixosConfigurations.host-eons-hss1 = nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";
        specialArgs = { inherit inputs; };
        modules = [
          disko.nixosModules.disko
          { disko.devices.disk.disk1.device = "/dev/vda"; }
          ./configuration.nix
        ];
      };
    };
}
