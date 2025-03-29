{
  lib,
  stdenv,
  vimUtils,
  writeText,
  noogle-cli,
  ...
}:
let
  initLua = writeText "init.lua" (prependLua + builtins.readFile ./lua/noogle/init.lua);
  prependLua =
    # lua
    ''
      local nix_store_bin_path = '${lib.getExe noogle-cli}'
    '';
in
vimUtils.buildVimPlugin {
  pname = "noogle.nvim";
  version = "0.1.0";
  src = stdenv.mkDerivation {
    name = "noogle-nvim-source";
    src = initLua;

    unpackPhase = ''
      mkdir -p $out/lua/noogle
      ln -sf $src $out/lua/noogle/init.lua
    '';
  };
}
