{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay, ... }:
    let
      package_overlay = final: prev: {
        vf-backend = import ./package.nix { pkgs = final; };
      };
      # taken from https://github.com/ngi-nix/project-template/blob/master/flake.nix
      # System types to support.
      supportedSystems = [ "x86_64-linux" "x86_64-darwin" ];

      # Helper function to generate an attrset '{ x86_64-linux = f "x86_64-linux"; ... }'.
      forAllSystems = f: nixpkgs.lib.genAttrs supportedSystems (system: f system);

      # Nixpkgs instantiated for supported system types.
      nixpkgsFor = forAllSystems (system: import nixpkgs { inherit system; overlays = self.overlays; });
    in
    {
      overlays = [ rust-overlay.overlay package_overlay ];
      packages = forAllSystems (system:
        {
          inherit (nixpkgsFor.${system}) vf-backend;
        });
      defaultPackage = forAllSystems (system: self.packages.${system}.vf-backend);
      nixosModules.backend = import ./module.nix;
      devShell = forAllSystems
        (system:
          let
            pkgs = nixpkgsFor.${system};
          in
          pkgs.mkShell {
            buildInputs = with pkgs; [
              rust-bin.stable.latest.default
              sqlite
              crate2nix
            ] ++ lib.optionals stdenv.isDarwin [
              libiconv
              darwin.apple_sdk.frameworks.Security
              darwin.apple_sdk.frameworks.CoreFoundation
            ];
            shellHook = ''
              export RUST_LOG=info
              export DATABASE_URL=sqlite:db/try.db
              export HTTP_PORT=8080
            '';
          }
        );

    };
}
