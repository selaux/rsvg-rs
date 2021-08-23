let
    nixpkgs = import (builtins.fetchTarball
      https://github.com/NixOS/nixpkgs/archive/0a5f5bab0e08e968ef25cff393312aa51a3512cf.tar.gz);
in
with nixpkgs {};
stdenv.mkDerivation {
  name = "rsvg-rs";
  version = "0.1.0";
  src = lib.sourceByRegex ./. [
    "^examples.*$"
    "^gir.*$"
    "^gir-files$"
    "^rsvg-sys$"
    "^src$"
    "^test-fixtures$"
    "^Cargo.toml$"
    "^Cargo.lock$"
    "^Gir.toml$"
    "^Makefile$"
  ];
  buildInputs = [ cargo rustc rustfmt librsvg gnome3.gtk http-parser pkgconfig ];
}
