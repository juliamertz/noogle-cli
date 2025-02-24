{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    noogle = {
      url = "github:nix-community/noogle";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { nixpkgs, noogle, ... }:
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
          inherit (pkgs)
            lib
            system
            vimUtils
            rustPlatform
            openssl
            pkg-config
            runCommandNoCC
            ;

          manifest = lib.importTOML ./Cargo.toml;
          data-json = noogle.packages.${system}.data-json;
        in
        rec {
          default = noogle-cli;

          noogle-cli = rustPlatform.buildRustPackage {
            inherit (manifest.package) name version;

            buildInputs = [ openssl ];
            nativeBuildInputs = [ pkg-config ];

            src = ./.;

            postUnpack = ''
              cp ${data-json} $sourceRoot/data.json
            '';

            cargoLock = {
              lockFile = ./Cargo.lock;
              allowBuiltinFetchGit = true;
            };
            meta.mainProgram = "noogle";
          };

          noogle-nvim = vimUtils.buildVimPlugin {
            name = "noogle-nvim";
            src =
              let
                prependLua = # lua
                  "local nix_store_bin_path = '${lib.getExe noogle-cli}'";
              in
              runCommandNoCC "noogle-nvim-source" { } ''
                mkdir -p $out/lua/noogle
                echo "${prependLua}" >> $out/lua/noogle/init.lua
                cat ${./lua/noogle/init.lua} >> $out/lua/noogle/init.lua
              '';
          };
        }
      );

      devShells = forEachSystem (pkgs: {
        default =
          with pkgs;
          mkShell {
            nativeBuildInputs = [ pkg-config ];
            buildInputs = [
              openssl
              cargo
              clippy
              rustfmt
              rust-analyzer
            ];
          };
      });
    };
}
