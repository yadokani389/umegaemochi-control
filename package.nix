{ lib, stdenv, cargo-tauri, glib-networking, nodejs, openssl, pkg-config, pnpm_9
, rustPlatform, webkitgtk_4_1, wrapGAppsHook4, }:

stdenv.mkDerivation (finalAttrs: {
  pname = "umegaemochi-control";
  version = "0.4.1";

  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.difference ./. (lib.fileset.unions [
      (lib.fileset.maybeMissing ./result)
      ./package.nix
      ./flake.nix
      ./flake.lock
    ]);
  };

  pnpmDeps = pnpm_9.fetchDeps {
    inherit (finalAttrs) src pname version;
    hash = "sha256-m4Lq7px56zfQGa+uuApXnzd8P6LxjVVnxTJNfhKCDmM=";
  };

  cargoRoot = "src-tauri";
  buildAndTestSubdir = finalAttrs.cargoRoot;

  cargoDeps =
    rustPlatform.importCargoLock { lockFile = ./src-tauri/Cargo.lock; };

  nativeBuildInputs = [
    cargo-tauri.hook
    nodejs
    pkg-config
    pnpm_9.configHook
    rustPlatform.cargoSetupHook
    wrapGAppsHook4
  ];

  buildInputs = [ glib-networking openssl webkitgtk_4_1 ];

  meta = {
    license = lib.licenses.mit;
    mainProgram = "umegaemochi-control";
    platforms = lib.platforms.linux;
  };
})
