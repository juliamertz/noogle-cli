{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    noogle.url = "github:nix-community/noogle";
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      noogle,
    }:
    let
      forEachSystem =
        f:
        nixpkgs.lib.genAttrs (nixpkgs.lib.systems.flakeExposed) (
          system: f nixpkgs.legacyPackages.${system}
        );
    in
    {
      packages = forEachSystem (
        pkgs:
        let
          craneLib = crane.mkLib pkgs;
          commonArgs = {
            src = craneLib.cleanCargoSource ./.;
            strictDeps = true;
            buildInputs = [ ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [ pkgs.libiconv ];
          };

          data-json = noogle.packages.${pkgs.system}.data-json;
        in
        rec {
          default = noogle-cli;

          noogle-nvim = pkgs.callPackage ./vimPlugin.nix { inherit noogle-cli; };
          noogle-cli = craneLib.buildPackage (
            commonArgs
            // {
              cargoArtifacts = craneLib.buildDepsOnly commonArgs;
              postUnpack = ''
                cp ${data-json} $sourceRoot/data.json
              '';

              meta.mainProgram = "noogle";
            }
          );
        }
      );

      checks = forEachSystem (pkgs: {
        inherit (self.packages.${pkgs.system}) default;
      });

      devShells = forEachSystem (pkgs: {
        default = (crane.mkLib pkgs).devShell {
          checks = self.checks.${pkgs.system};
          packages = [ ];
        };
      });
    };
}
