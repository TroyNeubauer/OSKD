{ pkgs }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "oskd";
  version = "0.1.1";
  src = ./.;

  cargoHash = "sha256-IRJp2JkC4p0lDPONnsbaSEwCTkIk33k16/a7ZLqZViw=";

  nativeBuildInputs = [
    pkgs.pkg-config
  ];

  buildInputs = [
    pkgs.xorg.libX11
    pkgs.xorg.libXi
    pkgs.xorg.libXtst
  ];

  preFixup = let
    libPath = pkgs.lib.makeLibraryPath [
      pkgs.xorg.libX11
      pkgs.xorg.libXtst
    ];
  in ''
    patchelf \
      --set-interpreter "$(cat $NIX_CC/nix-support/dynamic-linker)" \
      --set-rpath "${libPath}" \
      $out/bin/oskd
  '';

  meta = {
    description = "on-screen keyboard display for your coding streams";
    homepage = "https://github.com/TroyNeubauer/OSKD";
    license = pkgs.lib.licenses.mit;
  };
}
