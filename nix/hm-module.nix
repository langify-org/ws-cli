{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.programs.ws;

  repoEntryType = lib.types.submodule {
    options = {
      path = lib.mkOption {
        type = lib.types.str;
        description = "Path to the repository.";
      };
      url = lib.mkOption {
        type = lib.types.nullOr lib.types.str;
        default = null;
        description = "Remote URL of the repository.";
      };
    };
  };

  tomlValue =
    {
      repos = lib.mapAttrs (
        _: entry:
        { inherit (entry) path; }
        // lib.optionalAttrs (entry.url != null) { inherit (entry) url; }
      ) cfg.repos;
    };
in
{
  options.programs.ws = {
    enable = lib.mkEnableOption "ws workspace manager";

    package = lib.mkPackageOption pkgs "ws" { };

    repos = lib.mkOption {
      type = lib.types.attrsOf repoEntryType;
      default = { };
      description = "Repositories to register with ws.";
      example = lib.literalExpression ''
        {
          my-repo = {
            path = "~/Projects/my-repo";
            url = "git@github.com:user/my-repo.git";
          };
        }
      '';
    };
  };

  config = lib.mkIf cfg.enable {
    home.packages = [ cfg.package ];

    xdg.configFile."ws/config.toml" = lib.mkIf (cfg.repos != { }) {
      source = (pkgs.formats.toml { }).generate "ws-config.toml" tomlValue;
    };
  };
}
