with import <nixos> {};
{ pkgs ? import <nixpkgs> {} }:
let
  stdenv = pkgs.stdenv;
in
stdenv.mkDerivation {
  name = "cathode-tube-raycing";
  buildInputs = with pkgs; [
    cmake
    pkgconfig
  ];

  RUST_BACKTRACE=1;
}

