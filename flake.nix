{
  description = "Development environment for comprehensive rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
    nixpkgs-unstable.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, nixpkgs-unstable, ... }:
    let
      mkOverlay = input: name: (final: prev: {
        "${name}" = import input {
          system = final.system;
          config = final.config;
        };
      });

      forAllSystems = function:
        nixpkgs.lib.genAttrs [
          "x86_64-linux"
          "aarch64-linux"
          "x86_64-darwin"
          "aarch64-darwin"
        ]
          (system:
            function (import nixpkgs {
              inherit system;
              config.allowUnfree = true;
              overlays = [
                (mkOverlay nixpkgs-unstable "unstable")
              ];
            }));
    in
    {
      devShells = forAllSystems (pkgs: {
        default =
          pkgs.mkShell
            {
              name = "comprehensive-rust";
              packages = with pkgs; [
                cargo
                cargo-edit
                rustc
              ] ++
              (lib.optional pkgs.stdenvNoCC.isDarwin [
                darwin.apple_sdk.frameworks.Security
                darwin.apple_sdk.frameworks.SystemConfiguration
                libiconv
              ]);
            };
      });
      formatter = forAllSystems (pkgs: pkgs.nixpkgs-fmt);
    };
}
