{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs.buildPackages; [
    openssl
    pkg-config
    gcc
    dpkg
  ];
}

