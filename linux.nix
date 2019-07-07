{ pkgs ? import <nixpkgs> {} }:
with pkgs; mkShell {
    name = "ruskie";
    buildInputs = [
        (python37.withPackages(ps: with ps; [
            flake8
            matplotlib
            pandas
        ]))
        gcc8
        jq
        llvmPackages.openmp
        llvmPackages.libcxxClang
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
