{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = inputs:
    inputs.flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import inputs.rust-overlay) ];
        pkgs = import inputs.nixpkgs { inherit system overlays; };
      in with pkgs; {
        devShells.default = mkShell {
          packages = [
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" ];
            })
            jq
            just
            nixfmt-classic
          ];
        };
      });
}
