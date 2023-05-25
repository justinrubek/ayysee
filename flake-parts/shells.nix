{inputs, ...}: {
  perSystem = {
    config,
    pkgs,
    system,
    inputs',
    self',
    ...
  }: let
    devTools = [
      # rust tooling
      self'.packages.rust-toolchain
      pkgs.cargo-audit
      pkgs.cargo-udeps
      pkgs.bacon
      pkgs.wasm-bindgen-cli
      # version control
      pkgs.cocogitto
      # inputs'.bomper.packages.cli
      # formatting
      self'.packages.treefmt
      # misc
    ];

    inherit (self'.legacyPackages) cargoExtraPackages;
  in rec {
    devShells = {
      default = pkgs.mkShell rec {
        packages = devTools ++ cargoExtraPackages;
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath packages;

        shellHook = ''
          ${config.pre-commit.installationScript}
        '';
      };
    };
  };
}
