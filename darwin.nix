{ pkgs ? import <nixpkgs> {} }:
with pkgs; mkShell {
    name = "Rust";
    buildInputs = [
        (python37.withPackages(ps: with ps; [
            flake8
            matplotlib
            pandas
        ]))
        gtk2
        jq
        rlwrap
        rustup
        sqlite
    ];
    shellHook = ''
        . .shellhook
    '';
}
