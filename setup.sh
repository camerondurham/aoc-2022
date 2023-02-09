#!/bin/bash
set -x

setup() {
  if [ $# -lt 1 ]; then
    echo "usage setup <DAY>"
    return
  fi
  day=$1
  mkdir "$day"
  cp ./starter.rs "$day/main.rs"
  echo -e "\n[[bin]]\nname=\"$day\"\npath=\"$day/main.rs\"" >> Cargo.toml
}
setup "$@"
