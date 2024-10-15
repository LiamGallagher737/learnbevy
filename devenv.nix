{ pkgs, inputs, ... }:
let
    pkgs-unstable = import inputs.nixpkgs-unstable { system = pkgs.stdenv.system; };
in {
    languages.rust = {
        enable = true;
        channel = "stable";
    };
    languages.javascript = {
        enable = true;
        npm.enable = true;
    };
    packages = [
        pkgs.podman
        pkgs.cargo-watch
        pkgs.nodePackages_latest.svelte-language-server
        pkgs.nodePackages_latest.typescript-language-server
        pkgs-unstable.deno
    ];
}
