args@{ stdenv, lib, fetchurl, fetchFromGitHub, fetchgit, rustPlatform,
  pkgconfig, cargo, rustc, ncurses,
  # Plugins
  ... }:

let
  self = rustPlatform.buildRustPackage rec {
    name = "rustodoro-${version}-builder0";
    version = "rev-4c34dd";

    # Haven't upload yet, but usable for nix-shell.
    src = fetchgit {
      url = "/home/fazbdillah/Project/Rustodoro/rustodoro/";
      rev = "4c34dd67f62be546feb7d2d30991dea080e9e952";
      sha256 = "0iqi2a3dc57nvr8bc8m70pp7niwb5aqnpji6h80y8im76yxn1637";
    };

    nativeBuildInputs = [ pkgconfig rustc cargo ncurses ncurses ];
    buildInputs = [ ];

    cargoSha256 = "13hh596gl8v2scfqzi3x61zadnb8sizpgrhri85g9fjpjzxfb601";

    doCheck = false;
  };
in self

