{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        formatter = pkgs.nixfmt-tree;
        devShell = pkgs.mkShell {
          packages = with pkgs; [
            cargo
            clippy
            taplo
            rustfmt
            rust-analyzer
            rustc
          ];
          shellHook = ''
            export SHELL=/run/current-system/sw/bin/bash
          '';
        };
      }
    );
}
