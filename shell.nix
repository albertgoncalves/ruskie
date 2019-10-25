with import <nixpkgs> {};
let 
    shared = [
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
in

{
    linux = 
        stdenvNoCC.mkDerivation {
            name = "_";
            buildInputs = [
                cmake
                openssl_1_0_2
                pkg-config
            ] ++ shared;
            shellHook = ''
                . .shellhook
            '';
        };
    darwin = 
        stdenvNoCC.mkDerivation {
            name = "_";
            buildInputs = shared;
            shellHook = ''
                . .shellhook
            '';
        };
    xgboost = 
        gccStdenv.mkDerivation {
            name = "_";
            buildInputs = [];
            shellHook = "";
        };
}
