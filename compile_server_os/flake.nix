{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.disko.url = "github:nix-community/disko";
  inputs.disko.inputs.nixpkgs.follows = "nixpkgs";

  inputs.learnbevy.url = "path:./../server";

  outputs = inputs@{ self, nixpkgs, disko, ... }:
    {
      nixosConfigurations.slc = nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";
        specialArgs = { inherit inputs; subdomain = "slc"; };
        modules = [
          disko.nixosModules.disko
          { disko.devices.disk.disk1.device = "/dev/vda"; }
          ./configuration.nix
        ];
      };
      nixosConfigurations.deu = nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";
        specialArgs = { inherit inputs; subdomain = "deu"; };
        modules = [
          disko.nixosModules.disko
          {
            disko.devices.disk.disk1.device = "/dev/vda";
            networking.interfaces.ens3 = {
              useDHCP = false;
              ipv4.addresses = [
                { address = "5.83.147.226"; prefixLength = 24; }
              ];
            };
            networking.defaultGateway = "5.83.147.1";
          }
          ./configuration.nix
        ];
      };
    };
}
