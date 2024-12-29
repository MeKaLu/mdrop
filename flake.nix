{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    ...
  }: let
    systems = [
      "x86_64-linux"
      "aarch64-linux"
    ];
    overlays = [(import rust-overlay)];
    forEachSystem = nixpkgs.lib.genAttrs systems;
    pkgsForEach = forEachSystem (system: import nixpkgs {inherit system overlays;});
  in {
    devShells = forEachSystem (system: let
      pkgs = pkgsForEach.${system};
    in {
      default = pkgs.mkShell {
        buildInputs = [
          pkgs.rust-bin.stable.latest.default
          pkgs.alejandra
        ];
      };
    });
    packages = forEachSystem (system: let
      pkgs = pkgsForEach.${system};
      cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
    in {
      mdrop = pkgs.rustPlatform.buildRustPackage {
        inherit (cargoToml.package) name version;

        src = ./.;
        cargoLock = {
          lockFile = ./Cargo.lock;
        };

        meta = with pkgs.lib; {
          description = "Linux CLI tool for controlling Moondrop USB audio dongles.";
          homepage = "https://github.com/frahz/mdrop";
          license = licenses.mit;
        };
      };
      default = self.packages.${system}.mdrop;
    });
  };
}
