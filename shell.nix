let
  # Import dependencies (this preamble will be very common)
  dependencies = import ./nix/dependencies.nix;
  pkgs = dependencies.pkgs;
in
  # A "shell", in nix, defines the environment variables and other dependencies
  # for doing development. The "clangStdenv" line makes it use LLVM's C compiler
  # instead of the default GNU C Compiler, because that's what SpiderMonkey
  # needs.
  pkgs.mkShell.override { stdenv = pkgs.clangStdenv; } {
    # This name is pretty much arbitrary.
    name = "mozjs-example";
    # This makes it pull in crate2nix and other tools.
    buildInputs = dependencies.devDeps;
    # This is needed to build SpiderMonkey.
    LIBCLANG_PATH = "${dependencies.libclang.lib}/lib";
  }
