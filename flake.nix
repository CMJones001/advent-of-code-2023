{
  description = "An example of a development flake using rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable"; 
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }: 
    flake-utils.lib.eachDefaultSystem

    (system:
     let
       overlays = [ (import rust-overlay) ];
       nativeBuildInputs = with pkgs; [ rustup rustc ];
       pkgs = import nixpkgs {
          inherit system overlays;
          config.allowUnfree = true;
       };
     in
     with pkgs; {
      devShells.default = mkShell {
        inherit nativeBuildInputs;
        buildInputs = [ 
          gnuplot
          rust-bin.stable.latest.default 
          eza fd bacon
          cargo
          (jetbrains.plugins.addPlugins jetbrains.pycharm-professional ["github-copilot" "ideavim"])
          (jetbrains.plugins.addPlugins jetbrains.rust-rover [ "github-copilot" "ideavim" ])
          (python311.withPackages(ps: with ps; [wheel numpy pandas matplotlib seaborn]))
          black
          clippy
        ];
        inputsFrom = [ pkgs.neovim pkgs.zsh ];
        shellHook = ''
          alias vim='nvim';
        '';
       };
    }
    );
}
