{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.disko.url = "github:nix-community/disko";
  inputs.disko.inputs.nixpkgs.follows = "nixpkgs";

  inputs.learnbevy.url = "path:./../compile_api";

  outputs = inputs@{ self, nixpkgs, disko, ... }:
    {
      nixosConfigurations.host-eons-slc = nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";
        specialArgs = { inherit inputs; subdomain = "slc"; };
        modules = [
          disko.nixosModules.disko
          { disko.devices.disk.disk1.device = "/dev/vda"; }
          ./configuration.nix
        ];
      };
    };
}
