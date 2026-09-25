{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  nativeBuildInputs = with pkgs.buildPackages; [
    cargo
    rustc
    openssl
    pkg-config
    gcc
    dpkg
  ];
}
