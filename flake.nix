{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
          config = {
            allowUnfree = true;
            android_sdk.accept_license = true;
          };
        };

        rust-toolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [
            "x86_64-unknown-linux-gnu"
            "x86_64-linux-android"
            "aarch64-linux-android"
          ];
        };

        androidComposition = pkgs.androidenv.composeAndroidPackages {
          cmdLineToolsVersion = "13.0";
          toolsVersion = "26.1.1";
          platformToolsVersion = "35.0.1";
          buildToolsVersions = [ "30.0.3" "34.0.0" ];
          includeEmulator = false;
          platformVersions = [ "28" "29" "30" "33" "34" ];
          includeSources = false;
          includeSystemImages = false;
          systemImageTypes = [ "google_apis_playstore" ];
          abiVersions = [ "armeabi-v7a" "arm64-v8a" ];
          cmakeVersions = [ "3.10.2" ];
          includeNDK = true;
          ndkVersions = [ "26.3.11579264" ];
          useGoogleAPIs = false;
          useGoogleTVAddOns = false;
          includeExtras = [ "extras;google;gcm" ];
        };

      in {
        devShell = pkgs.mkShell rec {
          nativeBuildInputs = with pkgs; [
            pkg-config
            cargo-tauri
            nodejs
            nodePackages.pnpm
            rust-toolchain
            jdk17
            gradle
          ];

          buildInputs = (with pkgs; [ libsoup_3 webkitgtk_4_1 openssl ])
            ++ [ androidComposition.androidsdk ];

          ANDROID_HOME = "${androidComposition.androidsdk}/libexec/android-sdk";
          NDK_HOME = "${ANDROID_HOME}/ndk-bundle";
          GRADLE_OPTS =
            "-Dorg.gradle.project.android.aapt2FromMavenOverride=${androidComposition.androidsdk}/libexec/android-sdk/build-tools/34.0.0/aapt2";
          XDG_DATA_DIRS =
            "${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}";
        };
      });
}
