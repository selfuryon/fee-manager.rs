{inputs, ...}: {
  perSystem = {
    self',
    system,
    ...
  }: let
    craneLib = inputs.crane.lib.${system};
  in {
    packages.default = craneLib.buildPackage {
      src = craneLib.cleanCargoSource (craneLib.path ../.);
    };
    apps.default = {
      type = "app";
      program = "${self'.packages.default}/bin/fee-manager";
    };
  };
}
