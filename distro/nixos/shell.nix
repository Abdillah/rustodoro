with import <nixos> {}; {
  # Run the command for development,
  # nix-shell --add-root /nix/var/nix/gcroots/per-user/user/timer-rs ./distro/nixos/shell.nix
  self = callPackage ./build.nix {};
}

