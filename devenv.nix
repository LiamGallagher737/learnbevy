{ pkgs, ... }: {
    languages.rust = {
        enable = true;
        channel = "stable";
    };
    languages.javascript = {
        enable = true;
        npm.enable = true;
    };
    packages = with pkgs; [
        docker
        cargo-watch
        nodePackages.wrangler
        lsof # for wrangler
    ];
}
