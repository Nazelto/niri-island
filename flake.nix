{
  description = "";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      crane,
      ...
    }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };
      rustToolchain = pkgs.rust-bin.stable.latest.default.override {
        extensions = [
          "rust-src"
          "rustfmt"
          "clippy"
        ];
      };
      craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
      commonArgs = {
        pname = "niri-island";
        version = "beta";
        src = craneLib.cleanCargoSource ./.;

        cargoLock = ./Cargo.lock;

        strictDeps = true;

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];
        buildInputs = with pkgs; [
          gtk4
          gtk4-layer-shell
        ];

      };
      cargoArtifacts = craneLib.buildDepsOnly commonArgs;
    in
    {
      devShells.${system} = rec {
        niri-island = pkgs.mkShell {
          packages = with pkgs; [
            rustToolchain
            rust-analyzer
          ];
          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = with pkgs; [
            gtk4
            gtk4-layer-shell
          ];
        };
        default = niri-island;
      };
      packages.${system} = rec {
        niri-island = craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });

        default = niri-island;
      };
    };
}
