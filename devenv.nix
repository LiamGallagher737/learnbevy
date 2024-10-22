{ pkgs, inputs, ... }:
let
    pkgs-unstable = import inputs.nixpkgs-unstable { system = pkgs.stdenv.system; };
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
        pkgs.flyctl
        pkgs.openssl
        pkgs.cargo-watch
        pkgs.tailwindcss
        pkgs-unstable.dioxus-cli
        pkgs-unstable.wasm-bindgen-cli
    ];
}
