{
  description = "This library displays the difference between 2 strings using the Levenshtein distance";
  nixConfig.bash-prompt = "[flox] \\[\\033[38;5;172m\\]λ \\[\\033[0m\\]";

  inputs = {
    floxpkgs.url = "github:flox/floxpkgs";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };

  };

  outputs = args @ { self, floxpkgs, nixpkgs, crane, flake-utils, advisory-db, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        flox = floxpkgs.project args (_: {});
        pkgs = import nixpkgs {
          inherit system;
        };

        inherit (pkgs) lib;

        craneLib = crane.lib.${system};
        src = craneLib.cleanCargoSource ./.;

        # Build *just* the cargo dependencies, so we can reuse
        # all of that work (e.g. via cachix) when running in CI
        cargoArtifacts = craneLib.buildDepsOnly {
          inherit src;
          buildInputs = with pkgs; lib.optional stdenv.isDarwin libiconv;
        };

        # Build the actual crate itself, reusing the dependency
        # artifacts from above.
        edits-crate = craneLib.buildPackage {
          inherit cargoArtifacts src;
          buildInputs = with pkgs; lib.optional stdenv.isDarwin libiconv;
        };
      in
      {
        checks = {
          # Build the crate as part of `nix flake check` for convenience
          inherit edits-crate;

          # Run clippy (and deny all warnings) on the crate source,
          # again, resuing the dependency artifacts from above.
          #
          # Note that this is done as a separate derivation so that
          # we can block the CI if there are issues here, but not
          # prevent downstream consumers from building our crate by itself.
          edits-crate-clippy = craneLib.cargoClippy {
            inherit cargoArtifacts src;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          };

          edits-crate-doc = craneLib.cargoDoc {
            inherit cargoArtifacts src;
          };

          # Check formatting
          edits-crate-fmt = craneLib.cargoFmt {
            inherit src;
          };

          # Audit dependencies
          edits-crate-audit = craneLib.cargoAudit {
            inherit src advisory-db;
          };

          # Run tests with cargo-nextest
          # Consider setting `doCheck = false` on `edits-crate` if you do not want
          # the tests to run twice
          edits-crate-nextest = craneLib.cargoNextest {
            inherit cargoArtifacts src;
            partitions = 1;
            partitionType = "count";
          };
        } // lib.optionalAttrs (system == "x86_64-linux") {
          # NB: cargo-tarpaulin only supports x86_64 systems
          # Check code coverage (note: this will not upload coverage anywhere)
          edits-crate-coverage = craneLib.cargoTarpaulin {
            inherit cargoArtifacts src;
          };
        };

        packages.default = edits-crate;

        apps.default = flake-utils.lib.mkApp {
          drv = edits-crate;
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.checks;

          # Extra inputs can be added here
          nativeBuildInputs = with pkgs; [
            cargo
            rustc
          ];
        };
      });
}
