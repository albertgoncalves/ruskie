{ pkgs ? import <nixpkgs> {} }:
with pkgs; mkShell {
    name = "Rust";
    buildInputs = [
        gtk2
        jq
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
