{inputs, ...}: {
  imports = [
    inputs.devshell.flakeModule
  ];

  perSystem = {
    pkgs,
    config,
    ...
  }: let
    inherit (pkgs) statix;
  in {
    devshells.default = {
      name = "fee-manager";
      packages = with pkgs; [
        clang
        cargo
        clippy
        rustc
        rustfmt
        rust-analyzer
        statix
        cargo-flamegraph
        postgresql
        cargo-tarpaulin
        cargo-audit
        cargo-watch
        sqlx-cli
      ];
      devshell.startup = {
        pre-commit.text = config.pre-commit.installationScript;
      };
    };
  };
}
