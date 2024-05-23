{
  description = "Mensa CLI Project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    {
      devShell = nixpkgs.mkShell {
        buildInputs = [
          nixpkgs.rustc
          nixpkgs.cargo
          nixpkgs.gcc
        ];
      };
    };
}
