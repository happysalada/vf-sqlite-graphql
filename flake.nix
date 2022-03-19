{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
    devshell.url = "github:numtide/devshell";
  };

  outputs = { self, nixpkgs, rust-overlay, crane, devshell }:
    let
      package_overlay = final: prev:
        let system = final.system;
        in
        {
          vf-backend = crane.lib.${system}.buildPackage {
            src = ./.;

            buildInputs = with final; lib.optionals stdenv.isDarwin [
              libiconv
              darwin.apple_sdk.frameworks.Security
              darwin.apple_sdk.frameworks.CoreFoundation
            ];
          };
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
      overlays = [ rust-overlay.overlay package_overlay devshell.overlay ];
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
          pkgs.devshell.mkShell {
            packages = with pkgs; [
              rust-bin.stable.latest.default
              sqlite
            ] ++ lib.optionals stdenv.isDarwin [

              libiconv
              darwin.apple_sdk.frameworks.Security
              darwin.apple_sdk.frameworks.CoreFoundation
            ];
            env = [
              {
                name = "RUST_LOG";
                value = "info";
              }
              {
                name = "DATABASE_URL";
                value = "sqlite:db/try.db";

              }
              {
                name = "HTTP_PORT";
                value = "8080";
              }
            ];
          }
        );

    };
}
