{ pkgs ? import <nixpkgs> {} }:
with pkgs; mkShell {
    name = "Rust";
    buildInputs = [
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
