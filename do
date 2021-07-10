#!/run/current-system/sw/bin/oil

proc dev() {
  cargo run
}

proc update_deps() {
  cargo update
  cargo upgrade
}

@ARGV