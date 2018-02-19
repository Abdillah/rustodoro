args@{ stdenv, lib, fetchurl, fetchFromGitHub, rustPlatform,
  pkgconfig, cargo, rustc, ncurses,
  # Plugins
  ... }:

let
  self = rustPlatform.buildRustPackage rec {
    name = "zoomy-${version}";
    version = "0.0.1";

    # Haven't upload yet, but usable for nix-shell.
    src = null;

    nativeBuildInputs = [ pkgconfig rustc cargo ];
    buildInputs = [ ncurses ];

    cargoSha256 = "18mfkzdm8wkkf171d3gxsbwnp6m8l30fj4yi6vfi3ys276yv116m";

    doCheck = false;
  };
in self

