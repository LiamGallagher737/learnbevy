{ modulesPath, config, lib, pkgs, inputs, subdomain, ... }: {
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
      allowedTCPPorts = [ 80 443 ];
  };

  services.openssh.settings.PasswordAuthentication = false;

  users.users.root = {
    hashedPassword = "!"; # nothing hashes to "!" so this effectively disables the password
    openssh.authorizedKeys.keys = [
      "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABgQDZJ0j9jx1q5diqgyK0r7j8Zi20ZuZIpp1QKLUmSHeRDycmPZIhdH6P0gkd8Am1Y/VK2K1XTLahApksIXBBox1KSt2GcLLfPBie3tOocxBNtoJdQrHu+2nRL9PsngOZi1wLvv8qoaidp9Rsi4p8SA58WJHsoVqPJNZNPDPvAEXBXy/Wc/JsYy8RSzkAaXbCXo1c2DgctbMjA3+GUl9UFLUf76yd6kUjlTR9Pk7rjYzbLel8yd7i4CjnvNu+tc+WrXzLabKBlS4EP+t9owCD0vb362hyD8Wxg6BrtJnwUABEbZeZlOQjYRmA+zG4Yics7d4C8eOmehufSym616MpEg8zW8qxTnnySbVpKNGv3JM45N7YcXM7mqdFoy/lkI4IgUn3AA6fQ9Jlk+c7vzC75FXarQ5Ln2q71fo/0XH/SOTdgsvmEL3SnAepTCpbfXLjvUs9x4WN2AofVcbybPvU4ytD4vo4r9z1vqaoVpHt/eJb2cbNGHQqEHeUsYRhGXjzWys= liam@desktop"
      "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAICDRuZSMctfFCOR7goQF96iUOWa6sDys0Yoba8n7g6Dh" # github
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

  # Nginx service
  services.nginx = {
    enable = true;
    user = "ferris";
    recommendedProxySettings = true;
    recommendedTlsSettings = true;
    appendHttpConfig = ''
      limit_req_zone $binary_remote_addr zone=ip:10m rate=4r/s;
    '';
    virtualHosts."${subdomain}.compile.learnbevy.com" = {
      addSSL = true;
      enableACME = true;
      locations."/" = {
        proxyPass = "http://127.0.0.1:53740";
        # proxy_set_header will override user
        # set headers so it can be trusted
        extraConfig = ''
          proxy_set_header X-Real-IP $remote_addr;
          limit_req zone=ip burst=4 nodelay;
          limit_req_status 429;
        '';
      };
    };
  };
  security.acme = {
    acceptTerms = true;
    defaults.email = "liam@liamgallagher.dev";
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
    path = [
      pkgs.podman
      pkgs.shadow
    ];
  };

  # Pull the images every 15 minutes
  systemd.services.pull-learnbevy-images = {
    description = "Pull the learnbevy images";
    serviceConfig = {
      User = "ferris";
    };
    script = ''
      ${pkgs.podman}/bin/podman pull ghcr.io/liamgallagher737/learnbevy-main-nightly:main
      ${pkgs.podman}/bin/podman pull ghcr.io/liamgallagher737/learnbevy-main-stable:main
      ${pkgs.podman}/bin/podman pull ghcr.io/liamgallagher737/learnbevy-0.14-nightly:main
      ${pkgs.podman}/bin/podman pull ghcr.io/liamgallagher737/learnbevy-0.14-stable:main
      ${pkgs.podman}/bin/podman image prune -f
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
