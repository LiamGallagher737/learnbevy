{ pkgs, ... }: {
  languages.rust.enable = true;
  languages.javascript = {
      enable = true;
      npm.enable = true;
  };
  packages = with pkgs; [
    podman
    cargo-watch
    nodePackages.wrangler
    lsof # for wrangler
  ];
}
