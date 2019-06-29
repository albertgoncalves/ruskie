{ pkgs ? import <nixpkgs> {} }:
with pkgs; mkShell {
    name = "Rust";
    buildInputs = [
        jq
        pkg-config
        rlwrap
        rustup
        sqlite
    ];
    shellHook = ''
        . .shellhook
    '';
}
