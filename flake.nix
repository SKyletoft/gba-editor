{
	description = "A very basic flake";

	inputs = {
		nixpkgs.url     = "github:nixos/nixpkgs/nixpkgs-unstable";
		flake-utils.url = "github:numtide/flake-utils";
	};

	outputs = { self, nixpkgs, flake-utils }:
		flake-utils.lib.eachDefaultSystem(system:
			let
				pkgs = nixpkgs.legacyPackages.${system};
				lib = nixpkgs.lib;

				rust-env = with pkgs; [
					cargo
					rustc
					rustfmt
					clippy
					rust-analyzer
				];
				x11-env = with pkgs.xorg; [
					libX11
					libXcursor
					libXrandr
					libXi
					pkgs.libGL
					pkgs.libxkbcommon
				];
				dbg-env = with pkgs; [
					pkg-config
					nodejs
					gdb
				];

			in rec {
				packages.default = pkgs.rustPlatform.buildRustPackage {
					pname = "gba-hex-editor";
					version = "0.0.1";

					src = ./.;
					cargoLock.lockFile = ./Cargo.lock;

					buildInputs = x11-env;
				};
				devShells.default = pkgs.mkShell rec {
					nativeBuildInputs = rust-env ++ dbg-env;
					buildInputs = x11-env;

					shellHook = ''
						export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${lib.makeLibraryPath buildInputs}
					'';
				};
			}
		);
}
