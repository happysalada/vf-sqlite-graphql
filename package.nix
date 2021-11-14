{ pkgs }:
with pkgs;
let
cargo_nix = import ./Cargo.nix { inherit pkgs;
  buildRustCrateForPkgs = pkgs: pkgs.buildRustCrate.override {
    rustc = pkgs.rust-bin.stable.latest.default;
    defaultCrateOverrides = pkgs.defaultCrateOverrides // {
      sqlx-macros = attrs: {
        buildInputs = lib.optionals stdenv.isDarwin [ darwin.apple_sdk.frameworks.Security ];
      };
      backend = attrs: {
        buildInputs = lib.optionals stdenv.isDarwin [ darwin.apple_sdk.frameworks.Security ];
      };
    };
  };
};
crate = cargo_nix.rootCrate.build;
in
stdenv.mkDerivation {
  pname = "vf-graphql-sqlite-backend";
  version = "0.0.1";

  src = ./.;
  dontUnpack = true;
  dontBuild = true;
  installPhase = ''
    mkdir -p $out/{bin,migrations,seeds}
    cp ${crate}/bin/backend $out/bin
    cp -r $src/migrations $out
    cp -r $src/seeds $out
  '';
  dontCheck = true;
  dontFixup = true;
}
