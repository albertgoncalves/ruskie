{ pkgs ? import <nixpkgs> {} }:
with pkgs; mkShell {
    name = "Rust";
    buildInputs = [
        jq
        pkg-config
        rustup
    ] ++ (with python37Packages; [
        (csvkit.overridePythonAttrs (oldAttrs: {
            doCheck = false;
        }))
    ]);
    shellHook = ''
        . .shellhook
    '';
}
