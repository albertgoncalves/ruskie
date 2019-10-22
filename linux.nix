with import <nixpkgs> {};
stdenvNoCC.mkDerivation {
    name = "_";
    buildInputs = [
        (python37.withPackages(ps: with ps; [
            flake8
            matplotlib
            pandas
        ]))
        cmake
        csvkit
        gcc8Stdenv
        jq
        pkg-config
        openssl
        rlwrap
        rustup
        shellcheck
        sqlite
    ];
    shellHook = ''
        . .shellhook
    '';
}
