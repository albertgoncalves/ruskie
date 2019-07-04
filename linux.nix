{ pkgs ? import <nixpkgs> {} }:
with pkgs; mkShell {
    name = "Rust";
    buildInputs = [
        (python37.withPackages(ps: with ps; [
            flake8
            matplotlib
            pandas
        ]))
        jq
        pkg-config
        openssl
        rlwrap
        rustup
        sqlite
    ];
    shellHook = ''
        . .shellhook
    '';
}
