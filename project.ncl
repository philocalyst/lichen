let inputs = import "./nickel.lock.ncl" in
let organist = inputs.organist in

organist.OrganistExpression
& {
  Schema,
  config | Schema
    = {
      # Here we pull in the classical Rust dependencies
      shells = organist.shells.Rust,

      shells.build = {
        packages = {
          # For the packaging process
          coreutils = organist.import_nix "nixpkgs#coreutils",
          b3sum = organist.import_nix "nixpkgs#b3sum", # Blake3
        },
      },

      shells.dev = {
        packages.just = organist.import_nix "nixpkgs#just", # Command runner
        packages.jujutsu = organist.import_nix "nixpkgs#jujutsu", # VCS of choice
        packages.git = organist.import_nix "nixpkgs#git", # For pasky git operations
        packages.xh = organist.import_nix "nixpkgs#xh", # For HTTP requests
      },
    },
}
  | organist.modules.T
