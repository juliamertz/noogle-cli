{
  inputs = {
    # source of documentation, override this input to regenerate docs with updates
    nixpkgs-master.url = "nixpkgs/master";

    # build dependencies
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    noogle = {
      url = "github:juliamertz/noogle"; # fork with darwin support
      inputs.nixpkgs-master.follows = "nixpkgs-master";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      noogle,
      crane,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain (p: p.rust-bin.nightly.latest.default);
        craneArgs = {
          src = craneLib.cleanCargoSource ./.;
          strictDeps = true;
        };

        data-json = noogle.packages.${pkgs.system}.data-json;
      in
      {
        checks = {
          inherit (self.packages.${system}) noogle-cli;
        };

        packages = rec {
          default = noogle-cli;
          noogle-nvim = pkgs.callPackage ./vimPlugin.nix { inherit noogle-cli; };
          noogle-cli = craneLib.buildPackage (
            craneArgs
            // {
              cargoArtifacts = craneLib.buildDepsOnly craneArgs;
              postUnpack = ''
                cp ${data-json} $sourceRoot/data.json
              '';

              meta.mainProgram = "noogle";
            }
          );
        };

        devShells.default = craneLib.devShell {
          checks = self.checks.${system};
          packages = [ ];
        };
      }
    );

  nixConfig = {
    extra-substituters = [ "https://juliamertz.cachix.org" ];
    extra-trusted-public-keys = [
      "juliamertz.cachix.org-1:l9jCGk7vAKU5kS07eulGJiEsZjluCG5HTczsY2IL2aw="
    ];
  };
}
