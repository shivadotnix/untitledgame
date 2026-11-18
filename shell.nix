{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell rec {
  nativeBuildInputs = with pkgs; [
    gnumake
    cmake
    pkg-config
    clang
    llvm
  ];

  buildInputs = with pkgs; [
    libX11
    libXrandr
    libXinerama
    libXcursor
    libXi
    xorgproto
    libGL
    alsa-lib
    libpulseaudio
    pipewire
  ];

  LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages.libclang.lib ];

  PKG_CONFIG_PATH = pkgs.lib.makeSearchPath "lib/pkgconfig" (with pkgs; [
    libX11
    libXrandr
    libXinerama
    libXcursor
    libXi
  ]);

  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
}
