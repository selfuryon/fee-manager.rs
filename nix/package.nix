{inputs, ...}: {
  perSystem = {
    self',
    system,
    ...
  }: let
    craneLib = inputs.crane.lib.${system};
    commonArgs = {
      src = craneLib.cleanCargoSource (craneLib.path ../.);
      # buildInputs = with pkgs; [ ];
      # nativeBuildInputs = with pkgs; [ ];
    };

    cargoArtifacts = craneLib.buildDepsOnly (commonArgs
      // {
        pname = "fee-manager";
      });

    feeManagerClippy = craneLib.cargoClippy (commonArgs
      // {
        inherit cargoArtifacts;
        cargoClippyExtraArgs = "--all-targets -- --deny warnings";
      });

    feeManagerCrate = craneLib.buildPackage (commonArgs
      // {
        inherit cargoArtifacts;
      });

    feeManagerCoverage = craneLib.cargoTarpaulin (commonArgs
      // {
        inherit cargoArtifacts;
      });
  in {
    packages.default = feeManagerCrate;
    apps.default = {
      type = "app";
      program = "${self'.packages.default}/bin/fee-manager";
    };
    checks = {
      inherit feeManagerCrate feeManagerClippy feeManagerCoverage;
    };
  };
}
