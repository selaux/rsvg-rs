let
    host_pkgs = import <nixpkgs> {};
    nixpkgs = host_pkgs.fetchFromGitHub {
        owner = "NixOS";
        repo = "nixpkgs-channels";
        rev = "13e74a838db27825c88be99b1a21fbee33aa6803";
        sha256 = "02kmj8cvxhhhalx14hbwwrzdnmpp072wgl5drlk6asn0zg68cgmy";
    };
in
with import nixpkgs {};
stdenv.mkDerivation {
  name = "the-renderer";
  version = "0.1.0";
  src = ./.;
  buildInputs = [ cargo rustc librsvg gnome3.gtk ];
}
