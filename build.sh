#!/usr/bin/env bash
set -euo pipefail

# Folders
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROTO_PACKET_DIR="$(cd "$ROOT_DIR/../proto-packet" && pwd)"  # todo -- path dependency

# Compile Schema
cargo run --manifest-path "$PROTO_PACKET_DIR/compile/Cargo.toml" -p proto-packet-cli -- \
    compile rust "$ROOT_DIR/schema" "$ROOT_DIR/pkmn-schema/src"

# Format & Test
cargo fmt --manifest-path "$ROOT_DIR/Cargo.toml" --all
cargo test --manifest-path "$ROOT_DIR/Cargo.toml" --workspace --all-features
