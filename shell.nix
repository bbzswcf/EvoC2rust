let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-24.11";
  pkgs = import nixpkgs { config = {}; overlays = []; };
in
pkgs.mkShell {
  packages = [
    pkgs.git
    pkgs.cacert
    pkgs.python311
    pkgs.pdm
    pkgs.gcc14
    pkgs.clang-tools_18
    pkgs.rustup
  ];
}
