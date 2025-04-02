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
      "x86_64-darwin"
      "aarch64-linux"
      "aarch64-darwin"
    ];
    forEachSystem = nixpkgs.lib.genAttrs systems;
    pkgsForEach = forEachSystem (system:
      import nixpkgs {
        inherit system;
        overlays = [rust-overlay.overlays.default];
      });
  in {
    devShells = forEachSystem (system: let
      pkgs = pkgsForEach.${system};
      libPath = with pkgs;
        lib.makeLibraryPath [
          libGL
          libxkbcommon
          wayland
        ];
    in {
      default = pkgs.mkShell {
        buildInputs = [
          pkgs.rust-bin.stable.latest.default
          pkgs.alejandra
        ];

        LD_LIBRARY_PATH = libPath;
      };
    });
    packages = forEachSystem (system: let
      pkgs = pkgsForEach.${system};
    in {
      mdrop = let
        cargoToml = builtins.fromTOML (builtins.readFile ./mdrop-cli/Cargo.toml);
      in
        pkgs.rustPlatform.buildRustPackage {
          inherit (cargoToml.package) name version;

          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          cargoFlags = [
            "--bin"
            "mdrop"
          ];

          meta = with pkgs.lib; {
            description = "Linux CLI tool for controlling Moondrop USB audio dongles.";
            homepage = "https://github.com/frahz/mdrop";
            license = licenses.mit;
          };
        };
      gui = let
        cargoToml = builtins.fromTOML (builtins.readFile ./mdrop-gui/Cargo.toml);
        libPath = with pkgs;
          lib.makeLibraryPath [
            libGL
            libxkbcommon
            wayland
          ];
      in
        pkgs.rustPlatform.buildRustPackage {
          inherit (cargoToml.package) name version;

          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
            outputHashes = {
              "cryoglyph-0.1.0" = "sha256-8QCTD5OLO05RmwaugblYbKPPN7hfckctQlaBydPNDPE=";
              "dpi-0.1.1" = "sha256-hlVhlQ8MmIbNFNr6BM4edKdZbe+ixnPpKm819zauFLQ=";
              "iced-0.14.0-dev" = "sha256-YC74qowoW9VJonluX/FuiQc+TvvBytskhvgCLpmknQg=";
            };
          };
          cargoFlags = [
            "--bin"
            "mdrop-gui"
          ];

          nativeBuildInputs = [pkgs.makeWrapper];

          postInstall = ''
            wrapProgram $out/bin/mdrop-gui --prefix LD_LIBRARY_PATH : ${libPath}
          '';

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
