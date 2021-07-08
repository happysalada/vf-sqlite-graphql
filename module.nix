{ pkgs, config, lib, ... }:
with lib;
let
  serviceConfig = config.services.vf-graphql-sqlite-backend;
in
{
  options.services.vf-graphql-sqlite-backend = {
    enable = mkEnableOption "Valueflows graphql sqlite backend";
    stateDir = mkOption {
      type = types.str;
      default = "/var/lib/vf";
      example = "/var/lib/vf";
      description = ''
        location of folder for the application dynamic changes
      '';
    };
    dbName = mkOption {
      type = types.str;
      default = "temp";
      example = "temp";
      description = ''
        location of the sqlite db you want to use
      '';
    };
    port = mkOption {
      type = type.port;
      default = 8080;
      example = 8080;
      description = ''
        local port on which to run the server
      '';
    };
    package = mkOption {
      type = types.package;
      description = "package to run the instance with";
    };
  };

  config = with serviceConfig; mkIf enable {
    users.groups.vf = { };
    users.users.vf = {
      description = "vf user";
      group = "vf";
      isSystemUser = true;
    };

    systemd.services.vf-graphql-sqlite-backend = {
      wantedBy = [ "multi-user.target" ];
      description = "A backend with graphql and sqlite for valueflows";
      serviceConfig = {
        Type = "exec";
        Restart = "on-failure";
        RestartSec = 5;

        ExecStartPre = "${package}/bin/backend eval \"Union.Release.migrate\"";
        ExecStart = "${package}/bin/backend start";
        ExecStop = "${package}/bin/backend stop";

        User = "vf";
        Group = "vf";

        StateDirectory = "vf";

        PrivateTmp = true;
        ProtectSystem = "full";
        NoNewPrivileges = true;
        ReadWritePaths = "${StateDir}";
      };
      environment = {
        DATABASE_URL = "sqlite:${StateDir}/${dbName}.db";
        HTTP_PORT = toString port;
      };
    };
  };
}
