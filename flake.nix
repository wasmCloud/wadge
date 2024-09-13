{
  nixConfig.extra-substituters = [
    "https://west.cachix.org"
    "https://nixify.cachix.org"
    "https://crane.cachix.org"
    "https://wasmcloud.cachix.org"
    "https://bytecodealliance.cachix.org"
    "https://nix-community.cachix.org"
    "https://cache.garnix.io"
  ];
  nixConfig.extra-trusted-public-keys = [
    "west.cachix.org-1:F8ZwKSRWiSCh+rMyZAP7xhgUP6ZW88AGXE7KOR30Fg0="
    "nixify.cachix.org-1:95SiUQuf8Ij0hwDweALJsLtnMyv/otZamWNRp1Q1pXw="
    "crane.cachix.org-1:8Scfpmn9w+hGdXH/Q9tTLiYAE/2dnJYRJP7kl80GuRk="
    "wasmcloud.cachix.org-1:9gRBzsKh+x2HbVVspreFg/6iFRiD4aOcUQfXVDl3hiM="
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
  inputs.wit-deps.url = "github:bytecodealliance/wit-deps/v0.3.5";

  outputs = {
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
        name = "west";

        overlays = [
          wit-deps.overlays.fenix
          wit-deps.overlays.default
          (
            final: prev: {
              pkgsUnstable = import nixpkgs-unstable {
                inherit
                  (final.stdenv.hostPlatform)
                  system
                  ;

                inherit
                  (final)
                  config
                  ;
              };
            }
          )
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

        targets.arm-unknown-linux-gnueabihf = false;
        targets.arm-unknown-linux-musleabihf = false;
        targets.armv7-unknown-linux-gnueabihf = false;
        targets.armv7-unknown-linux-musleabihf = false;
        targets.powerpc64le-unknown-linux-gnu = false;
        targets.s390x-unknown-linux-gnu = false;
        targets.wasm32-unknown-unknown = false;
        targets.wasm32-wasi = false;

        clippy.deny = ["warnings"];
        clippy.workspace = true;

        test.allTargets = true;
        test.workspace = true;

        buildOverrides = {
          pkgs,
          pkgsCross ? pkgs,
          ...
        }: {
          buildInputs ? [],
          depsBuildBuild ? [],
          nativeBuildInputs ? [],
          nativeCheckInputs ? [],
          preCheck ? "",
          ...
        } @ args:
          with pkgs.lib; let
            darwin2darwin = pkgs.stdenv.hostPlatform.isDarwin && pkgsCross.stdenv.hostPlatform.isDarwin;

            depsBuildBuild' =
              depsBuildBuild
              ++ optional pkgs.stdenv.hostPlatform.isDarwin pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
              ++ optional darwin2darwin pkgs.xcbuild.xcrun;
          in
            {
              buildInputs =
                buildInputs
                ++ optional pkgs.stdenv.hostPlatform.isDarwin pkgs.libiconv;

              depsBuildBuild = depsBuildBuild';
            }
            // optionalAttrs (args ? cargoArtifacts) {
              preCheck =
                ''
                  export GOCACHE=$TMPDIR/gocache
                  export GOMODCACHE=$TMPDIR/gomod
                  export GOPATH=$TMPDIR/go
                  export HOME=$TMPDIR/home
                ''
                + preCheck;

              depsBuildBuild =
                depsBuildBuild'
                ++ optionals darwin2darwin [
                  pkgs.darwin.apple_sdk.frameworks.CoreFoundation
                  pkgs.darwin.apple_sdk.frameworks.CoreServices
                ];

              nativeCheckInputs =
                nativeCheckInputs
                ++ [
                  pkgs.pkgsUnstable.go
                ];
            };

        withDevShells = {
          devShells,
          pkgs,
          ...
        }:
          extendDerivations {
            buildInputs = [
              pkgs.wit-deps

              pkgs.pkgsUnstable.go_1_23
              pkgs.pkgsUnstable.wasm-tools
              pkgs.pkgsUnstable.wasmtime
            ];
          }
          devShells;
      };
}
