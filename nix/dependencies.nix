# This is a file we'll write ourselves to make it easier to ensure that configurations stay the same between each environment, both build and dev.

let
  sources = import ./sources.nix;
  pkgs = import sources.nixpkgs { };
in
{
  # This will import our pinned instance of the nixpkgs upstream repository.
  inherit pkgs;
  # This will contain all of the CLI tools that our shell uses.
  devDeps = [
    (import sources.crate2nix {inherit pkgs;})
    pkgs.llvmPackages.lld
    pkgs.yasm
    pkgs.binutils
    pkgs.cargo
    pkgs.llvm
    pkgs.autoconf213
    pkgs.python3
    pkgs.python2
    pkgs.which
    pkgs.perl
  ];
  # Pinned versions of other tools.
  libclang = pkgs.llvmPackages.libclang;
}
