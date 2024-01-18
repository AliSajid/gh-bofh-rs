#!/usr/bin/env bash

ls dist/
rm -rfv dist/*

TAG=${RELEASE_VERSION:-canary}

# Rename the binaries
cp -vr artifacts/aarch64-apple-darwin/gh-bofh-aarch64-apple-darwin dist/gh-bofh_${TAG}_darwin-arm64
cp -vr artifacts/aarch64-unknown-linux-gnu/gh-bofh-aarch64-unknown-linux-gnu dist/gh-bofh_${TAG}_linux-arm64
cp -vr artifacts/i686-unknown-linux-gnu/gh-bofh-i686-unknown-linux-gnu dist/gh-bofh_${TAG}_linux-386
cp -vr artifacts/x86_64-apple-darwin/gh-bofh-x86_64-apple-darwin dist/gh-bofh_${TAG}_darwin-amd64
cp -vr artifacts/x86_64-unknown-linux-gnu/gh-bofh-x86_64-unknown-linux-gnu dist/gh-bofh_${TAG}_linux-amd64
cp -vr artifacts/x86_64-pc-windows-gnu/gh-bofh-x86_64-pc-windows-gnu dist/gh-bofh_${TAG}_windows-amd64.exe
cp -vr artifacts/i686-pc-windows-gnu/gh-bofh-i686-pc-windows-gnu dist/gh-bofh_${TAG}_windows-386.exe

# Create the checksums
shasum -a 256 dist/* > dist/SHA256SUMS.txt

tree -a dist/
