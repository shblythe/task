{
    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    };
    outputs = {self, nixpkgs}:
        let
            system = "x86_64-linux";
            pkgs = nixpkgs.legacyPackages.${system};
        in {
            devShells.${system}.default = pkgs.mkShell {
                name = "task-dev";
                buildInputs = with pkgs; [
                    cargo
                    clippy
                    rustc
                ];
            };
        };
}
