let
    host_pkgs = import <nixpkgs> {};
    nixpkgs = host_pkgs.fetchFromGitHub {
        owner = "NixOS";
        repo = "nixpkgs-channels";
        rev = "19eedaf867da3155eec62721e0c8a02895aed74b";
        sha256 = "06k0hmdn8l1wiirfjcym86pn9rdi8xyfh1any6vgb5nbx87al515";
    };
in
with import nixpkgs {};
stdenv.mkDerivation {
  name = "the-renderer";
  version = "0.1.0";
  src = ./.;
  buildInputs = [ cargo rustc librsvg gnome3.gtk ];
}
