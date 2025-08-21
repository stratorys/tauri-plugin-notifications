{
  description = "tauri-plugin-push-notification";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      fenix,
      ...
    }:
    let
      inherit (nixpkgs) lib;

      supportedSystems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];

      forAllSystems =
        systems: f:
        lib.genAttrs systems (
          system:
          f (
            import nixpkgs {
              inherit system;
              overlays = [ fenix.overlays.default ];
            }
          )
        );
    in
    {
      formatter = forAllSystems supportedSystems (pkgs: pkgs.nixfmt-rfc-style);

      devShells = forAllSystems supportedSystems (pkgs: {
        default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            nodejs
            typescript

            pkg-config

            pkgs.fenix.stable.rustc
            pkgs.fenix.stable.cargo
            pkgs.fenix.stable.rust-std
            pkgs.fenix.stable.clippy
            pkgs.fenix.stable.rust-src
            pkgs.fenix.stable.rust-docs
            pkgs.fenix.latest.rustfmt
            rust-analyzer
          ];

          buildInputs = with pkgs; [
            at-spi2-atk
            atkmm
            cairo
            gdk-pixbuf
            glib
            gtk3
            harfbuzz
            librsvg
            libsoup_3
            pango
            webkitgtk_4_1
          ];
        };
      });
    };
}
