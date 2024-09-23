#!/usr/bin/env sh

set -xe

id=$(gh run list -b main -w west -L 1 --json databaseId -q '.[].databaseId')
git clean -xf lib target/artifacts
gh run download -D lib -n 'passthrough' "$id"
gh run download -D target/artifacts -p 'west-*' "$id"
mv -v target/artifacts/west-aarch64-apple-darwin/lib/libwest_sys.a ./lib/aarch64-darwin/libwest.a
mv -v target/artifacts/west-aarch64-linux-android/lib/libwest_sys.a ./lib/aarch64-android/libwest.a
mv -v target/artifacts/west-aarch64-unknown-linux-musl/lib/libwest_sys.a ./lib/aarch64-linux/libwest.a
mv -v target/artifacts/west-riscv64gc-unknown-linux-gnu/lib/libwest_sys.a ./lib/riscv64-linux/libwest.a
mv -v target/artifacts/west-x86_64-apple-darwin/lib/libwest_sys.a ./lib/x86_64-darwin/libwest.a
mv -v target/artifacts/west-x86_64-pc-windows-gnu/lib/libwest_sys.a ./lib/x86_64-windows/libwest.a
mv -v target/artifacts/west-x86_64-unknown-linux-musl/lib/libwest_sys.a ./lib/x86_64-linux/libwest.a
git reset
git add -f lib
git commit -Ss -m 'build: add build artifacts'
