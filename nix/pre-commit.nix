{inputs, ...}: {
  imports = [
    inputs.pre-commit-hooks-nix.flakeModule
  ];

  perSystem = {
    pre-commit.settings = {
      hooks = {
        alejandra.enable = true;
        rustfmt.enable = true;
        clippy.enable = true;
        statix.enable = true;
        deadnix.enable = true;
      };
    };
  };
}
