{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    rust-analyzer
    pkg-config
    openssl
  ];

  OPENSSL_DIR = "${pkgs.openssl.dev}";
  OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

  shellHook = ''
    echo "Dashbook development shell"
    echo "Rust: $(rustc --version)"
    echo "Cargo: $(cargo --version)"
  '';
}
