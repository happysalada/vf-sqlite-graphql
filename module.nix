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
    package = mkOption {
      type = types.package;
      description = "package to run the instance with";
    };
    logLevel = mkOption {
      type = types.str;
      default = "info";
      example = "info";
      description = ''
        log level you want for the web server
      '';
    };
    instances = mkOption {
      description = "instances of the union service to run. Typically one for each client";
      default = { };
      type = types.attrsOf (types.submodule {
        options = {
          dbName = mkOption {
            type = types.str;
            default = "temp";
            example = "temp";
            description = ''
              name of the sqlite db you want to use
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
        };
      });
    };
  };

  config = mkIf serviceConfig.enable {
    users.groups.vf = { };
    users.users.vf = {
      description = "vf user";
      group = "vf";
      isSystemUser = true;
    };

    systemd.services = lib.mapAttrs'
      (name: instanceConfig: lib.nameValuePair "vf-backend-${name}"
        {
          wantedBy = [ "multi-user.target" ];
          description = "A backend with graphql and sqlite for valueflows";
          serviceConfig = {
            Type = "exec";
            Restart = "on-failure";
            RestartSec = 5;

            ExecStartPre = pkgs.writeShellScript "db_create_and_migrate" ''
              # go into the directory where the migrations are
              cd ${serviceConfig.package}
              ${pkgs.sqlx-cli}/bin/sqlx db create
              ${pkgs.sqlx-cli}/bin/sqlx migrate run
              records=$(${pkgs.sqlite}/bin/sqlite3 "/var/lib/vf/${instanceConfig.dbName}.db" "SELECT COUNT(*) FROM agents")
              if [[ $records == 0 ]]; then
                  echo "Initializing db for service ${instanceConfig.dbName}"
                  cat ${serviceConfig.package}/seeds/${instanceConfig.dbName}.sql | ${pkgs.sqlite}/bin/sqlite3 "/var/lib/vf/${instanceConfig.dbName}.db"
              fi
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
            DATABASE_URL = "sqlite:${serviceConfig.stateDir}/${instanceConfig.dbName}.db";
            HTTP_PORT = toString instanceConfig.port;
            RUST_LOG = "${serviceConfig.logLevel}";
          };
        })
      serviceConfig.instances;
  };
}
