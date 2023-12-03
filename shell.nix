let
  nix_channel = "23.05";
  rust_overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/refs/tags/${nix_channel}.tar.gz") {
    overlays = [ rust_overlay ];
  };
  rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
in
pkgs.mkShell rec {
  buildInputs = [ rust ] ++ (with pkgs; [
    llvmPackages_16.bintools
    cargo-audit
    cargo-expand
    cargo-generate
    cargo-watch
  ]);

  # Rust
  RUST_BACKTRACE = 1;
}
