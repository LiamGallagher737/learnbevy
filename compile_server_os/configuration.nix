{ modulesPath, config, lib, pkgs, inputs, ... }: {
  imports = [
    (modulesPath + "/installer/scan/not-detected.nix")
    (modulesPath + "/profiles/qemu-guest.nix")
    ./disk-config.nix
  ];

  boot.loader.grub = {
    # no need to set devices, disko will add all devices that have a EF02 partition to the list already
    # devices = [ ];
    efiSupport = true;
    efiInstallAsRemovable = true;
  };
  services.openssh.enable = true;

  environment.systemPackages = map lib.lowPrio [
    pkgs.curl
    pkgs.gitMinimal
  ];

  virtualisation.containers.enable = true;
  virtualisation.podman.enable = true;

  networking.firewall = {
      enable = true;
      allowedTCPPorts = [ 53740 ];
  };

  services.openssh.settings.PasswordAuthentication = false;

  users.users.root = {
    hashedPassword = "!"; # nothing hashes to "!" so this effectively disables the password
    openssh.authorizedKeys.keys = [
      "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABgQDZJ0j9jx1q5diqgyK0r7j8Zi20ZuZIpp1QKLUmSHeRDycmPZIhdH6P0gkd8Am1Y/VK2K1XTLahApksIXBBox1KSt2GcLLfPBie3tOocxBNtoJdQrHu+2nRL9PsngOZi1wLvv8qoaidp9Rsi4p8SA58WJHsoVqPJNZNPDPvAEXBXy/Wc/JsYy8RSzkAaXbCXo1c2DgctbMjA3+GUl9UFLUf76yd6kUjlTR9Pk7rjYzbLel8yd7i4CjnvNu+tc+WrXzLabKBlS4EP+t9owCD0vb362hyD8Wxg6BrtJnwUABEbZeZlOQjYRmA+zG4Yics7d4C8eOmehufSym616MpEg8zW8qxTnnySbVpKNGv3JM45N7YcXM7mqdFoy/lkI4IgUn3AA6fQ9Jlk+c7vzC75FXarQ5Ln2q71fo/0XH/SOTdgsvmEL3SnAepTCpbfXLjvUs9x4WN2AofVcbybPvU4ytD4vo4r9z1vqaoVpHt/eJb2cbNGHQqEHeUsYRhGXjzWys= liam@desktop"
    ];
    linger = true; # for podman
  };

  users.users.ferris = {
    isNormalUser = true;
    home = "/home/ferris";
    description = "The user that runs the server";
    linger = true; # for podman
    packages = [
      pkgs.podman
      pkgs.shadow # podman needs this
      inputs.learnbevy.packages."${pkgs.system}".default
    ];
  };

# Run the compile api as a service
  systemd.services.compile_api = {
    description = "Compile API Service";
    wantedBy = [ "multi-user.target" ];
    serviceConfig = {
      ExecStart = "${inputs.learnbevy.packages."${pkgs.system}".default}/bin/compile_api";
      User = "ferris";
      Restart = "always";
      WorkingDirectory = "/home/ferris";
    };
  };

  # Pull the images every 15 minutes
  systemd.services.pull-learnbevy-images = {
    description = "Pull the learnbevy images on startup";
    serviceConfig = {
      User = "ferris";
    };
    script = ''
      ${pkgs.podman}/bin/podman pull ghcr.io/liamgallagher737/learnbevy-main-nightly:main
      ${pkgs.podman}/bin/podman pull ghcr.io/liamgallagher737/learnbevy-main-stable:main
      ${pkgs.podman}/bin/podman pull ghcr.io/liamgallagher737/learnbevy-0.14-nightly:main
      ${pkgs.podman}/bin/podman pull ghcr.io/liamgallagher737/learnbevy-0.14-stable:main
    '';
    wantedBy = [ "default.target" ];
    path = [
      pkgs.shadow
    ];
  };

  systemd.timers.pull-learnbevy-images = {
    description = "Podman Pull Images Timer";
    timerConfig = {
      OnCalendar = "*:0/15";
      Persistent = true;
    };
    wantedBy = [ "timers.target" ];
  };

  system.stateVersion = "24.05";
}
