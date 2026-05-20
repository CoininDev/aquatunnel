{
  description = "flake for building aquatunnel on Void-like systems via Nix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11"; # reasonable stable baseline; change if you prefer another channel
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rust = pkgs.rust-bin.stable.latest.default; # stable rust toolchain
      in {
        packages.default = pkgs.stdenv.mkDerivation {
          pname = "aquatunnel";
          src = ./.;
          nativeBuildInputs = [ rust cargo pkgs.pkg-config pkgs.pkgconf ];
          buildInputs = [
            pkgs.gcc
            pkgs.glibc
            pkgs.glibc.dev
            pkgs.alsaLib
            pkgs.openssl
            pkgs.zlib
          ];
          # Use cargo build, relying on rust/cargo from Nix
          buildPhase = ''
export CARGO_HOME=$PWD/.cargo-home
export RUSTUP_HOME=$PWD/.rustup-home
export PATH=${rust}/bin:$PATH
cargo build --release --locked
'';
          installPhase = ''
mkdir -p $out/bin
cp target/release/aquatunnel $out/bin/
'';
          meta = with pkgs.lib; {
            description = "aquatunnel (built via nix flake)";
            license = licenses.mit;
            maintainers = [];
          };
        };

        devShell = pkgs.mkShell {
          buildInputs = [
            rust
            pkgs.cargo
            pkgs.pkg-config
            pkgs.pkgconf
            pkgs.gcc
            pkgs.glibc.dev
            pkgs.alsaLib
            pkgs.openssl
            pkgs.zlib
          ];
          RUST\_BACKTRACE = "1";
          # ensure linker and tools are the Nix-provided ones
          shellHook = ''
        export CC=\${pkgs.gcc}/bin/gcc
        export CXX=\${pkgs.gcc}/bin/g++
        export LD=\${pkgs.binutils}/bin/ld
        export PKG\_CONFIG\_PATH=\${pkgs.alsaLib}/lib/pkgconfig:\${pkgs.openssl}/lib/pkgconfig:\${pkgs.zlib}/lib/pkgconfig
        echo "Nix dev shell: using \${pkgs.gcc}/bin/gcc and \${pkgs.binutils}/bin/ld"
      '';
        };

        # optional: simple default app to build via `nix build`
        apps.\${system}.default = {
          type = "app";
          program = "\${self.packages.\${system}.default}/bin/aquatunnel";
        };
      });
}
