{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/master";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };
        oskd = import ./package.nix { pkgs = pkgs; };
      in {
        packages.default = oskd;

        devShells.default = pkgs.mkShell {
          inputsFrom = [ oskd ];

          buildInputs = with pkgs; [
            rustup
            rust-analyzer
          ];

          shellHook = ''
            export LD_LIBRARY_PATH=${pkgs.xorg.libX11}/lib:$LD_LIBRARY_PATH
            export LD_LIBRARY_PATH=${pkgs.xorg.libXtst}/lib:$LD_LIBRARY_PATH
          '';
        };
      }
    );
}
