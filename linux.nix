{ pkgs ? import <nixpkgs> {} }:
with pkgs; mkShell {
    name = "Rust";
    buildInputs = [
        jq
        pkg-config
        rustup
    ];
    shellHook = ''
        . .shellhook
    '';
}
