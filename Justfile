set dotenv-load

# show available recipes.
list:
  @just --list

# running dev server.
run:
  cargo watch -x run

# building production.
build:
  #!/bin/sh
  if [ -z $(which mold) ]; then
    RUSTFLAGS=-Clink-arg=-fuse-ld=lld cargo build --release
  else
    RUSTFLAGS=-Clink-arg=-fuse-ld=mold cargo build --release
  fi

buildx:
  just _cross build --release --target x86_64-unknown-linux-musl
  just _cross build --release --target aarch64-unknown-linux-gnu
  just _cross build --release --target aarch64-unknown-linux-musl

# detect before running sea-orm-cli and install it if it doesn't exist.
_cross *args:
  #!/bin/sh
  if [ -z $(which cross) ]; then
    cargo install cross
  fi
  cross {{args}}

# format code. (args example: just fmt --check)
fmt *args='':
  cargo fmt --all {{args}}

# check code. (args example: just check --quiet)
check *args='':
  cargo check --all {{args}}

# lint code. (args example: just lint --fix)
lint *args='':
  cargo clippy {{args}} -- -W clippy::pedantic -W clippy::nursery -A clippy::missing-errors-doc -A clippy::module_name_repetitions

# docker-build version='nightly':
#   docker build . \
#   --tag "importantimport/hatsu:{{version}}"

# docker-buildx version='nightly':
#   docker buildx build . \
#   --platform "linux/amd64,linux/arm64" \
#   --tag "importantimport/hatsu:{{version}}"

# create and remove account (method: create/remove) (name: example.com)
account method name:
  #!/bin/sh
  if [ -z ${HATSU_ACCESS_TOKEN+x} ]; then
    echo "env HATSU_ACCESS_TOKEN must be set"
  else
    just _account {{method}} {{name}}
  fi

_account method name:
  curl -X POST "http://localhost:${HATSU_LISTEN_PORT}/api/v0/admin/{{method}}-account?token=${HATSU_ACCESS_TOKEN}" \
  -H "Content-Type: application/json" \
  -d "{\"name\": \"{{name}}\"}"

# use db_* without underscores.
db *args='migration up':
  just db_{{args}}

# apply migrations to database.
db_migration *args='up':
  just _sea-orm-cli migrate {{args}} -d crates/db_migration

# generate entities from database.
db_schema: (db_migration 'fresh')
  just _sea-orm-cli generate entity -l -o crates/db_schema/src

# detect before running sea-orm-cli and install it if it doesn't exist.
_sea-orm-cli *args:
  #!/bin/sh
  if [ -z $(which sea-orm-cli) ]; then
    cargo install sea-orm-cli
  fi
  sea-orm-cli {{args}}

# setup dev environment for arch linux (target-arch: amd64/arm64)
setup-arch target-arch='amd64':
  sudo pacman -S mold rustup
  just _setup-rustup {{target-arch}}
  just _setup-cargo arch

# setup dev environment for debian sid (target-arch: amd64/arm64)
setup-debian target-arch='amd64':
  sudo apt install mold rustup
  just _setup-rustup {{target-arch}}
  just _setup-cargo debian

# setup dev environment for docker (target-arch: amd64/arm64)
setup-docker target-arch='amd64':
  just setup-debian
  cargo install cargo-chef

# TODO: cargo-pgo
# cargo install cargo-pgo
# (distro: undefined/arch/debian)
_setup-cargo distro='undefined':
  {{ if distro == 'arch' { "sudo pacman -S cargo-watch" } else { "cargo install cargo-watch" } }}