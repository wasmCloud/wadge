{
  nixConfig.extra-substituters = [
    "https://wasmcloud.cachix.org"
    "https://nixify.cachix.org"
    "https://crane.cachix.org"
    "https://bytecodealliance.cachix.org"
    "https://nix-community.cachix.org"
    "https://cache.garnix.io"
  ];
  nixConfig.extra-trusted-public-keys = [
    "wasmcloud.cachix.org-1:9gRBzsKh+x2HbVVspreFg/6iFRiD4aOcUQfXVDl3hiM="
    "nixify.cachix.org-1:95SiUQuf8Ij0hwDweALJsLtnMyv/otZamWNRp1Q1pXw="
    "crane.cachix.org-1:8Scfpmn9w+hGdXH/Q9tTLiYAE/2dnJYRJP7kl80GuRk="
    "bytecodealliance.cachix.org-1:0SBgh//n2n0heh0sDFhTm+ZKBRy2sInakzFGfzN531Y="
    "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
    "cache.garnix.io:CTFPyKSLcx5RMJKfLo5EEPUObbA78b0YQ2DTCJXqr9g="
  ];

  inputs.nixify.inputs.nixlib.follows = "nixlib";
  inputs.nixify.url = "github:rvolosatovs/nixify";
  inputs.nixlib.url = "github:nix-community/nixpkgs.lib";
  inputs.nixpkgs-unstable.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.wit-deps.inputs.nixify.follows = "nixify";
  inputs.wit-deps.inputs.nixlib.follows = "nixlib";
  inputs.wit-deps.url = "github:bytecodealliance/wit-deps/v0.5.0";

  outputs =
    {
      nixify,
      nixlib,
      nixpkgs-unstable,
      wit-deps,
      ...
    }:
    with builtins;
    with nixlib.lib;
    with nixify.lib;
    rust.mkFlake {
      src = ./.;
      name = "wadge";

      overlays = [
        wit-deps.overlays.fenix
        wit-deps.overlays.default
        (final: prev: {
          pkgsUnstable = import nixpkgs-unstable {
            inherit (final.stdenv.hostPlatform)
              system
              ;

            inherit (final)
              config
              ;
          };
        })
      ];

      excludePaths = [
        ".envrc"
        ".github"
        ".gitignore"
        "ADOPTERS.md"
        "CODE_OF_CONDUCT.md"
        "CONTRIBUTING.md"
        "flake.nix"
        "LICENSE"
        "README.md"
        "SECURITY.md"
      ];

      doCheck = false; # testing is performed in checks via `nextest`

      build.packages = [
        "wadge-sys"
      ];

      targets.arm-unknown-linux-gnueabihf = false;
      targets.arm-unknown-linux-musleabihf = false;
      targets.armv7-unknown-linux-gnueabihf = false;
      targets.armv7-unknown-linux-musleabihf = false;
      targets.powerpc64le-unknown-linux-gnu = false;
      targets.s390x-unknown-linux-gnu = false;
      targets.wasm32-unknown-unknown = false;
      targets.wasm32-wasip1 = false;
      targets.wasm32-wasip2 = false;

      clippy.deny = [ "warnings" ];
      clippy.workspace = true;

      test.allTargets = true;
      test.workspace = true;

      buildOverrides =
        {
          pkgs,
          pkgsCross ? pkgs,
          ...
        }:
        {
          nativeCheckInputs ? [ ],
          preCheck ? "",
          ...
        }@args:
        optionalAttrs (args ? cargoArtifacts) {
          preCheck =
            ''
              export GOCACHE=$TMPDIR/gocache
              export GOMODCACHE=$TMPDIR/gomod
              export GOPATH=$TMPDIR/go
              export HOME=$TMPDIR/home
            ''
            + preCheck;

          nativeCheckInputs = nativeCheckInputs ++ [
            pkgs.pkgsUnstable.go
          ];
        };

      withPackages =
        {
          hostRustToolchain,
          packages,
          ...
        }:
        packages
        // {
          rust = hostRustToolchain;
        };

      withDevShells =
        {
          devShells,
          pkgs,
          ...
        }:
        extendDerivations {
          buildInputs = [
            pkgs.wit-deps

            pkgs.pkgsUnstable.binaryen
            pkgs.pkgsUnstable.cargo-audit
            pkgs.pkgsUnstable.cargo-nextest
            pkgs.pkgsUnstable.gh
            pkgs.pkgsUnstable.go
            pkgs.pkgsUnstable.wasm-tools
            pkgs.pkgsUnstable.wasmtime
          ];
        } devShells;
    };
}
