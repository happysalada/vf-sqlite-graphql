#!/run/current-system/sw/bin/oil

proc dev() {
  cargo run
}

proc update_deps() {
  cargo update
  cargo upgrade
}
proc reset_db {
  sqlx database reset -y
}


@ARGV