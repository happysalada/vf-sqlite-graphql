{ pkgs, self }:
with pkgs;

rustPlatform.buildRustPackage rec {
  pname = "vf-graphql-sqlite-backend";
  version = "0.0.1";

  src = self;
  cargoLock = {
    lockFile = ./Cargo.lock;
  };
  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ openssl ] ++ lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.Security
    darwin.apple_sdk.frameworks.CoreFoundation
    libiconv
  ];

  postFixup = ''
    cp -r ./migrations $out
  '';
}
