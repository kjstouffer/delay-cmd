#!/usr/bin/env bash
version=$(grep -e '^version' Cargo.toml | sed "s/.*= \"\(.*\)\"/\1/")
cargo build --release --target=x86_64-apple-darwin
tar -czvf ./target/delay-cmd-$version-x86_64-apple-darwin.tar.gz -C ./target/x86_64-apple-darwin/release/build/delay-cmd-*/out/ . -C ../../.. ./delay-cmd
gsha256sum ./target/*.tar.gz > ./target/sha256sums.txt

