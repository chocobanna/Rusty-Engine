{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  buildInputs = [
    pkgs.fontconfig
    pkgs.pkg-config
    pkgs.cmake
    pkgs.gcc
    pkgs.xorg.libX11
    pkgs.xorg.libXcursor
    pkgs.xorg.libXrandr
    pkgs.xorg.libXi 
    pkgs.mesa
  ];
}
