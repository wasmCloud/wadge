#!/usr/bin/env sh

set -xe

id=$(gh run list -b main -w wadge -L 1 --json databaseId -q '.[].databaseId')
git clean -xf lib target/artifacts
gh run download -D lib -n 'passthrough' "$id"
gh run download -D target/artifacts -p 'wadge-*' "$id"
mv -v target/artifacts/wadge-aarch64-apple-darwin/lib/libwadge_sys.a ./lib/aarch64-darwin/libwadge.a
mv -v target/artifacts/wadge-aarch64-linux-android/lib/libwadge_sys.a ./lib/aarch64-android/libwadge.a
mv -v target/artifacts/wadge-aarch64-unknown-linux-musl/lib/libwadge_sys.a ./lib/aarch64-linux/libwadge.a
mv -v target/artifacts/wadge-riscv64gc-unknown-linux-gnu/lib/libwadge_sys.a ./lib/riscv64-linux/libwadge.a
mv -v target/artifacts/wadge-x86_64-apple-darwin/lib/libwadge_sys.a ./lib/x86_64-darwin/libwadge.a
mv -v target/artifacts/wadge-x86_64-pc-windows-gnu/lib/libwadge_sys.a ./lib/x86_64-windows/libwadge.a
mv -v target/artifacts/wadge-x86_64-unknown-linux-musl/lib/libwadge_sys.a ./lib/x86_64-linux/libwadge.a
git reset
git add -f lib
git commit -Ss -m 'build: add build artifacts'
