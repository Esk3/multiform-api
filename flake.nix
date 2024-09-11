{
  description = "Rust development template";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    ...
  }:
    utils.lib.eachDefaultSystem
    (
      system: let
        pkgs = import nixpkgs{inherit system;};
      in rec
      {
        # Used by `nix develop`
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
	    cargo
            rustc
            clippy
            rustfmt
            pkg-config
	    bacon
	    sqlx-cli
	    rust-analyzer
          ];

	  shellHook = ''
	     exec tmux new "nvim ; fish" \; \
	       new-window "bacon ; fish" \; \
	       new-window "nix run ./services/flake.nix ; fish"
	  '';

	  env = {
            # Required by rust-analyzer
            #RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
          };
        };
      }
    );
}
