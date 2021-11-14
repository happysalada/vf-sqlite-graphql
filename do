#!/run/current-system/sw/bin/oil

proc dev() {
  cargo run
}

proc update_deps() {
  cargo update
  cargo upgrade
  crate2nix generate
}
proc reset_db {
  sqlx database reset -y
}

proc import_db(name) {
  cat seeds/$name.sql | sqlite3 db/try.db
}


@ARGV