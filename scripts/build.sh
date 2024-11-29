#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2024 Ali Sajid Imami
#
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

set -euo pipefail
set -x

# Set the necessary variables

# Set the prefix for the binary
PREFIX="gh-bofh"

# Declare the associative array mapping Rust triple to Go arch
declare -A GOARCH_MAP=(
  ["aarch64-apple-darwin"]="darwin-arm64"
  ["x86_64-apple-darwin"]="darwin-amd64"
  ["i686-pc-windows-gnu"]="windows-386"
  ["x86_64-pc-windows-gnu"]="windows-amd64"
  ["i686-unknown-linux-gnu"]="linux-386"
  ["x86_64-unknown-linux-gnu"]="linux-amd64"
  ["aarch64-unknown-linux-gnu"]="linux-arm64"
)

# Check the files in the source folder
tree artifacts

# Check the files in the dist folder
mkdir -p dist

# Copy binaries with the rust triple and go arch names to the dist folder
for arch in artifacts/*/; do
  if [ -d "$arch" ]; then
    arch=$(basename "$arch")
    filename=$PREFIX-$arch
    if [ -f "artifacts/$arch/$filename" ]; then
      cp -v "artifacts/$arch/$filename" "dist/$filename"
      cp -v "artifacts/$arch/$filename" "dist/$PREFIX-${GOARCH_MAP[$arch]}"
    fi
  fi
done

# Create the checksums
shasum -a 256 dist/* | sed 's/dist\///' | tee dist/SHA256SUMS.txt

# Sign the files
for file in dist/*; do
  gpg --armor --output "$file.asc" --detach-sig "$file"
done

tree -a dist/

set +x
