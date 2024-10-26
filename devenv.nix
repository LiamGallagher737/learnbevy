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
    packages = [
        pkgs.docker
        pkgs.flyctl
        pkgs.openssl
        pkgs.cargo-watch
        pkgs.tailwindcss
        pkgs-unstable.dioxus-cli
        pkgs-unstable.wasm-bindgen-cli
    ];
}
