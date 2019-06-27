{ pkgs ? import <nixpkgs> {} }:
with pkgs; mkShell {
    name = "Rust";
    buildInputs = [
        gtk2
        jq
        rustup
    ];
    shellHook = ''
        . .shellhook
    '';
}
