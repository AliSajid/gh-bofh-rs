#!/usr/bin/env bash

ls dist/
rm dist/SHASUMS256.txt

# Rename the binaries
mv -v dist/gh-bofh-aarch64-apple-darwin dist/gh-bofh_${RELEASE_VERSION}_darwin-arm64
mv -v dist/gh-bofh-aarch64-unknown-linux-gnu dist/gh-bofh_${RELEASE_VERSION}_linux-arm64
mv -v dist/gh-bofh-i686-unknown-linux-gnu dist/gh-bofh_${RELEASE_VERSION}_linux-386
mv -v dist/gh-bofh-x86_64-apple-darwin dist/gh-bofh_${RELEASE_VERSION}_darwin-amd64
mv -v dist/gh-bofh-x86_64-unknown-linux-gnu dist/gh-bofh_${RELEASE_VERSION}_linux-amd64
mv -v dist/gh-bofh-x86_64-pc-windows-gnu dist/gh-bofh_${RELEASE_VERSION}_windows-amd64.exe
mv -v dist/gh-bofh-i686-pc-windows-gnu dist/gh-bofh_${RELEASE_VERSION}_windows-386.exe

# Create the checksums
shasum -a 256 dist/* > dist/SHASUMS256.txt

tree -a dist/
