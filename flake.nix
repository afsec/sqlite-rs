{
  inputs = {
    nixpkgs.url = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        packages = with pkgs; [
          curl
          wget
          pkg-config
          cmake
          vim
          tmux
          rsync
          cacert
          curl
          git
          tree
          unzip
          zstd
          iproute2
          jq
          clang
          llvm
          lldb
          glibc
          rustup
          musl
          cargo-sort
          cargo-audit
          cargo-deny
          cargo-vet
          cargo-generate
          helix
          cocogitto
          difftastic
          watchexec
          hexyl
          vscodium
          nixd
          nixpkgs-fmt
        ];
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = packages;

          shellHook =
            ''
              rustup default stable
              rustup component add rust-analyzer
              rustup component add clippy
              alias cx='cargo xtask'
            '';
        };
      });
}
