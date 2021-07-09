{ pkgs, config, lib, ... }:
with lib;
let
  serviceConfig = config.services.vf-backend;
in
{
  options.services.vf-backend = {
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
      type = types.port;
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

  config = mkIf serviceConfig.enable {
    users.groups.vf = { };
    users.users.vf = {
      description = "vf user";
      group = "vf";
      isSystemUser = true;
    };

    systemd.services.vf-backend = {
      wantedBy = [ "multi-user.target" ];
      description = "A backend with graphql and sqlite for valueflows";
      serviceConfig = {
        Type = "exec";
        Restart = "on-failure";
        RestartSec = 5;

        ExecStartPre = pkgs.writeShellScript "db_create_and_migrade" ''
          ${pkgs.sqlx-cli}/bin/sqlx db create
          ${pkgs.sqlx-cli}/bin/sqlx migrate run
        '';
        ExecStart = "${serviceConfig.package}/bin/backend";
        ExecStop = "${serviceConfig.package}/bin/backend";

        User = "vf";
        Group = "vf";

        StateDirectory = "vf";

        PrivateTmp = true;
        ProtectSystem = "full";
        NoNewPrivileges = true;
        ReadWritePaths = "${serviceConfig.stateDir}";
      };
      environment = {
        DATABASE_URL = "sqlite:${serviceConfig.stateDir}/${serviceConfig.dbName}.db";
        HTTP_PORT = toString serviceConfig.port;
      };
    };
  };
}
