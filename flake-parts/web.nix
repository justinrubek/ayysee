{inputs, ...}: {
  perSystem = {
    config,
    pkgs,
    system,
    inputs',
    self',
    ...
  }: let
    static-files = pkgs.runCommand "static-files" {} ''
      mkdir -p $out

      mkdir -p $out/public/wasm

      cp -r ${../public}/* $out/public

      cp -r ${self'.packages.wasm}/* $out/public/wasm
    '';
  in rec {
    packages = {
      inherit static-files;
      serve = pkgs.writeShellApplication {
        name = "serve-ayysee";
        runtimeInputs = [pkgs.miniserve];
        text = ''
          miniserve ${static-files}/public "$@"
        '';
      };
    };

    apps = {
      serve = {
        type = "app";
        program = packages.serve;
      };
    };
  };
}
