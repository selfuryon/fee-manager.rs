{
  description = "Fee Manager";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # fenix = {
    #   url = "github:nix-community/fenix";
    #   inputs.nixpkgs.follows = "nixpkgs";
    #   inputs.rust-analyzer-src.follows = "";
    # };
    # advisory-db = {
    #   url = "github:rustsec/advisory-db";
    #   flake = false;
    # };

    # flake-parts
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    pre-commit-hooks-nix = {
      url = "github:hercules-ci/pre-commit-hooks.nix/flakeModule";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    # Utils
    devshell = {
      url = "github:numtide/devshell";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    statix = {
      url = "github:nerdypepper/statix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;}
    {
      debug = true;
      imports = [./nix];
      systems = ["x86_64-linux"];
    };
}
