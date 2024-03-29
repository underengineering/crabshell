{
  inputs = {
    nixpkgs = {
      url = "github:nixos/nixpkgs/nixpkgs-unstable";
    };
    utils = {
      url = "github:numtide/flake-utils";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crate2nix = {
      url = "github:kolloch/crate2nix";
      flake = false;
    };
  };
  outputs = {
    self,
    nixpkgs,
    utils,
    rust-overlay,
    crate2nix,
    ...
  }: let
    name = "crabshell";
  in
    utils.lib.eachDefaultSystem
    (
      system: let
        # Imports
        pkgs = import nixpkgs {inherit system;};
        cargoNixPkgs = import nixpkgs {
          inherit system;
          overlays = [
            rust-overlay.overlays.default
            (self: super: let
              rust = self.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
            in {
              # Because rust-overlay bundles multiple rust packages into one
              # derivation, specify that mega-bundle here, so that crate2nix
              # will use them automatically.
              rustc = rust;
              cargo = rust;
            })
          ];
        };

        # Mold wrapper from https://gitlab.com/roosemberth/mtt/-/commit/4cd5ad13f851b0a9daf81c9d6cff2ce5e0c0d827
        lib = pkgs.lib;
        bintools-wrapper = "${nixpkgs}/pkgs/build-support/bintools-wrapper";
        mold' = pkgs.symlinkJoin {
          name = "mold";
          wrapperName = "mold";
          paths = [pkgs.mold];
          nativeBuildInputs = [pkgs.makeWrapper];
          suffixSalt = lib.replaceStrings ["-" "."] ["_" "_"] system;

          coreutils_bin = lib.getBin pkgs.coreutils;
          postBuild = ''
            for bin in ${pkgs.mold}/bin/*; do
              rm $out/bin/"$(basename "$bin")"

              export prog="$bin"
              substituteAll "${bintools-wrapper}/ld-wrapper.sh" $out/bin/"$(basename "$bin")"
              chmod +x $out/bin/"$(basename "$bin")"

              mkdir -p $out/nix-support
              substituteAll "${bintools-wrapper}/add-flags.sh" $out/nix-support/add-flags.sh
              substituteAll "${bintools-wrapper}/add-hardening.sh" $out/nix-support/add-hardening.sh
              substituteAll "${bintools-wrapper}/../wrapper-common/utils.bash" $out/nix-support/utils.bash
            done
          '';
        };

        inherit
          (import "${crate2nix}/tools.nix" {inherit pkgs;})
          generatedCargoNix
          ;

        # Create the cargo2nix project
        project =
          cargoNixPkgs.callPackage
          (generatedCargoNix {
            inherit name;
            src = ./.;
          })
          {
            # Individual crate overrides go here
            # Example: https://github.com/balsoft/simple-osd-daemons/blob/6f85144934c0c1382c7a4d3a2bbb80106776e270/flake.nix#L28-L50
            defaultCrateOverrides =
              pkgs.defaultCrateOverrides
              // {
                # The app crate itself is overriden here. Typically we
                # configure non-Rust dependencies (see below) here.
                ${name} = oldAttrs:
                  {
                    inherit buildInputs nativeBuildInputs;
                  }
                  // buildEnvVars;
                mlua-sys = oldAttrs: {
                  buildInputs = [pkgs.luajit];
                  nativeBuildInputs = [pkgs.pkg-config];
                };
                gobject-sys = oldAttrs: {
                  buildInputs = [pkgs.gtk4];
                  nativeBuildInputs = [pkgs.pkg-config];
                };
                gio-sys = oldAttrs: {
                  buildInputs = [pkgs.gtk4];
                  nativeBuildInputs = [pkgs.pkg-config];
                };
                gdk-pixbuf-sys = oldAttrs: {
                  buildInputs = [pkgs.gtk4];
                  nativeBuildInputs = [pkgs.pkg-config];
                };
                gtk4-layer-shell-sys = oldAttrs: {
                  buildInputs = [
                    pkgs.gtk4-layer-shell
                  ];
                  nativeBuildInputs = [pkgs.pkg-config];
                };
                libpulse-sys = oldAttrs: {
                  buildInputs = [
                    pkgs.libpulseaudio
                  ];
                  nativeBuildInputs = [pkgs.pkg-config];
                };
                libpulse-mainloop-glib-sys = oldAttrs: {
                  buildInputs = [
                    pkgs.libpulseaudio
                  ];
                  nativeBuildInputs = [pkgs.pkg-config];
                };
              };
          };

        # Packages used by the flake at run-time
        buildInputs = with pkgs; [
          gtk4
          libpulseaudio
          librsvg
          luajit
        ];
        # Build dependencies
        nativeBuildInputs = with pkgs; [
          cargoNixPkgs.cargo
          cargoNixPkgs.rustc
          clang_15
          gtk4-layer-shell
          mold'
          pkg-config
        ];
        buildEnvVars = {};
      in rec {
        packages.${name} = project.rootCrate.build;

        # `nix build`
        defaultPackage = packages.${name};

        # `nix run`
        apps.${name} = utils.lib.mkApp {
          inherit name;
          drv = packages.${name};
        };

        defaultApp = apps.${name};

        # `nix develop`
        devShell =
          pkgs.mkShell
          {
            inherit buildInputs nativeBuildInputs;
            RUST_SRC_PATH = "${cargoNixPkgs.rustc}/lib/rustlib/src/rust/library";
          }
          // buildEnvVars;
      }
    );
}
