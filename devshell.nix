{ pkgs, ... }:

pkgs.devShell.mkShell {
  name = "spoj";
  packages = with pkgs; [
    # Toolchain required for C + Rust binaries building
    binutils
    gcc

    # Rust 1.56 toolchain as Leetcode supports only this version :^(
    bacon
    cargo-flamegraph
    rust-analyzer # rust-analyzer is not available at such old version >:(
    (rust-bin.stable."1.56.0".default.override {
      # Extensions which ease your development process
      #extensions = [ "rust-analyzer" "rust-src" ];
      extensions = [ "rust-src" ];
    })
  ];
}
