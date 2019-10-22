with import <nixpkgs> {};
stdenvNoCC.mkDerivation {
    name = "_";
    buildInputs = [
        (python37.withPackages(ps: with ps; [
            flake8
            matplotlib
            pandas
        ]))
        csvkit
        gcc8Stdenv
        jq
        rlwrap
        rustup
        shellcheck
        sqlite
    ];
    shellHook = ''
        . .shellhook
    '';
}
