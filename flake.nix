{
  description = "infra-notify binary build";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    crane = {
      url = "github:ipetkov/crane";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    crane,
    rust-overlay,
    advisory-db,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };

      inherit (pkgs) lib;
      craneLib = crane.mkLib nixpkgs.legacyPackages.${system};
      src = craneLib.cleanCargoSource (craneLib.path ./.);

      commonArgs = {
        inherit src;
        strictDeps = true;
        buildInputs = [] ++ lib.optionals pkgs.stdenv.isDarwin [];
      };

      cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      infra-notify = craneLib.buildPackage (commonArgs // {inherit cargoArtifacts;});

      infra-notify-clippy = craneLib.cargoClippy (commonArgs
        // {
          inherit cargoArtifacts;
          cargoClippyExtraArgs = "--all-targets -- --deny warnings";
        });

      infra-notify-doc = craneLib.cargoDoc (commonArgs
        // {
          inherit cargoArtifacts;
        });

      # Check formatting
      infra-notify-fmt = craneLib.cargoFmt {
        inherit src;
      };

      # Audit dependencies
      infra-notify-audit = craneLib.cargoAudit {
        inherit src advisory-db;
      };
    in {
      checks = {
        inherit infra-notify-doc;
        inherit infra-notify-clippy;
        inherit infra-notify-fmt;
        inherit infra-notify-audit;
      };

      packages = {
        inherit infra-notify;
        default = infra-notify;
      };

      apps.default = flake-utils.lib.mkApp {
        drv = infra-notify;
      };

      devShells.default = craneLib.devShell {
        checks = self.checks.${system};
        packages = with pkgs; [
          cargo-edit
          cargo-machete
        ];
      };
    });
}
