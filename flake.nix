{
  description = "Groth16 implementation development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = { nixpkgs.follows = "nixpkgs"; };
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];

        pkgs = import nixpkgs { inherit system overlays; };

        rustToolchainFile = ./rust-toolchain.toml;
      in with pkgs; {
        devShells.default = mkShell {
          nativeBuildInputs = [
            llvm
            rustup
            (rust-bin.fromRustupToolchainFile rustToolchainFile)
          ];

          buildInputs = [ ];
        };
      });
}
