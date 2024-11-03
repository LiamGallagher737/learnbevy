{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.disko.url = "github:nix-community/disko";
  inputs.disko.inputs.nixpkgs.follows = "nixpkgs";

  inputs.learnbevy.url = "path:./../server";

  outputs = inputs@{ self, nixpkgs, disko, ... }:
    {
      nixosConfigurations.mi = nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";
        specialArgs = { inherit inputs; subdomain = "mi"; };
        modules = [
          disko.nixosModules.disko
          {
            disko.devices.disk.disk1.device = "/dev/vda";
            networking.interfaces.ens3 = {
              useDHCP = false;
              ipv4.addresses = [
                { address = "185.165.44.18"; prefixLength = 24; }
              ];
            };
            networking.defaultGateway = "185.165.44.1";
          }
          ./configuration.nix
        ];
      };
    };
}
