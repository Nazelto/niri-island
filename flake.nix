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
      lib = pkgs.lib;
      sourceFilter =
        path: type:
        let
          pathStr = toString path;
        in
        (craneLib.filterCargoSources path type)
        || (lib.hasInfix "/src/css/" pathStr)
        || (lib.hasSuffix "/src/css" pathStr);
      commonArgs = {
        pname = "niri-island";
        version = "beta";
        src = lib.cleanSourceWith {
          src = craneLib.path ./.;
          filter = sourceFilter;
        };

        cargoLock = ./Cargo.lock;

        strictDeps = true;

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];
        buildInputs = with pkgs; [
          gtk4
          gtk4-layer-shell
          gst_all_1.gstreamer
          gst_all_1.gst-plugins-base
          gst_all_1.gst-plugins-good
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
            pulseaudio
          ];
          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = with pkgs; [
            gtk4
            gtk4-layer-shell
            gst_all_1.gstreamer
            gst_all_1.gst-plugins-base
            gst_all_1.gst-plugins-good
          ];
          shellHook = ''
            export NIRI_ISLAND_AUDIO_SOURCE=alsa_output.pci-0000_00_1f.3-platform-skl_hda_dsp_generic.HiFi__Speaker__sink.monitor
          '';
        };
        default = niri-island;
      };
      packages.${system} = rec {
        niri-island = craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });

        default = niri-island;
      };
    };
}
