{
  description = "A js development environment";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [];
        };
      in
      {
        devShell = pkgs.mkShell {
          CHROME_PATH = "${pkgs.chromium}/bin/chromium";
          shellHook = ''
          loop() {
            while true; do                                                                                                      
             change=$(, inotifywait -e close_write,moved_to,create ./src/**/*)    yarn run format && yarn run prod && sleep 1
            done
          }
          '';
          buildInputs = [
            pkgs.nodejs_21
            pkgs.yarn
          ];
        };
      });
}
