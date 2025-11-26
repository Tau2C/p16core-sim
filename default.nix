{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  packages = with pkgs; [
    nixfmt-rfc-style

    rustup
    vscode-extensions.vadimcn.vscode-lldb

    nil

    nodejs
  ];
}
