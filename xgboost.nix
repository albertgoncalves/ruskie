with import <nixpkgs> {};
gccStdenv.mkDerivation {
    name = "_";
    buildInputs = [
        (python37.withPackages(ps: with ps; [
            flake8
            matplotlib
            pandas
        ]))
        csvkit
        jq
        git
        rlwrap
        rustup
        shellcheck
        sqlite
    ];
    shellHook = ''
    '';
}
