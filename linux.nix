{ pkgs ? import <nixpkgs> {} }:
with pkgs; stdenvNoCC.mkDerivation {
    name = "ruskie";
    buildInputs = [
        (python37.withPackages(ps: with ps; [
            flake8
            matplotlib
            pandas
        ]))
        cmake
        gcc8Stdenv
        jq
        pkg-config
        openssl
        rlwrap
        rustup
        sqlite
    ] ++ (with python37Packages; [
        (csvkit.overridePythonAttrs (oldAttrs: {
            doCheck = false;
        }))
    ]);
    shellHook = ''
        . .shellhook
    '';
}
