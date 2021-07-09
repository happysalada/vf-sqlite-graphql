{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        package_overlay = final: prev: {
          vf-graphql-sqlite-backend = import ./package.nix { pkgs = final; inherit self; };
        };
        overlays = [ (import rust-overlay) package_overlay ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        packages = {
          inherit (pkgs) vf-graphql-sqlite-backend;
        };
        defaultPackage = pkgs.vf-graphql-sqlite-backend;
        nixosModule.vf-graphql-sqlite-backend = import ./module.nix;
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            exa
            fd
            rust-bin.stable.latest.default
            sqlite
          ] ++ lib.optionals stdenv.isDarwin [
            libiconv
            darwin.apple_sdk.frameworks.Security
            darwin.apple_sdk.frameworks.CoreFoundation
          ];

          shellHook = ''
            alias ls=exa
            alias find=fd
            export RUST_LOG=info
            export DATABASE_URL=sqlite:db/try.db
            export HTTP_PORT=8080
          '';
        };
      }
    );
}
