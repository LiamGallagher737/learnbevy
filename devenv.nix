{ pkgs, inputs, ... }:
let
    pkgs-unstable = import inputs.nixpkgs-unstable { system = pkgs.stdenv.system; };
    pkgs-local = import inputs.nixpkgs-local { system = pkgs.stdenv.system; };
in {
    languages.rust = {
        enable = true;
        channel = "stable";
        targets = [
            "wasm32-unknown-unknown"
        ];
    };
    languages.javascript = {
        enable = true;
        npm.enable = true;
    };
    packages = [
        pkgs.podman
        pkgs.cargo-watch
        pkgs.tailwindcss
        pkgs-local.dioxus-cli
        pkgs-local.wasm-bindgen-cli
    ];
}
