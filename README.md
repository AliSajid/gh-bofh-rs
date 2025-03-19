<!--
SPDX-FileCopyrightText: 2023 - 2025 Ali Sajid Imami

SPDX-License-Identifier: Apache-2.0
SPDX-License-Identifier: MIT
-->

# A `gh` plugin to generate BOFH excuses

[![GitHub release (latest by date)](https://img.shields.io/github/v/release/AliSajid/gh-bofh-rs)](https://github.com/AliSajid/gh-bofh-rs/releases/latest)
[![GitHub tag (latest SemVer)](https://img.shields.io/github/v/tag/AliSajid/gh-bofh-rs)](https://github.com/AliSajid/gh-bofh-rs/releases/latest)
[![Crates.io Version](https://img.shields.io/crates/v/gh-bofh-rs)](https://crates.io/crates/gh-bofh-rs)
[![Crates.io Size](https://img.shields.io/crates/size/gh-bofh-rs)](https://crates.io/crates/gh-bofh-rs)
![Maintenance](https://img.shields.io/maintenance/yes/2025)
![OSS Lifecycle](https://img.shields.io/osslifecycle?file_url=https%3A%2F%2Fraw.githubusercontent.com%2FAliSajid%2Fgh-bofh-rs%2Fmain%2FOSSMETADATA)

[![Continuous integration](https://github.com/AliSajid/gh-bofh-rs/actions/workflows/ci.yaml/badge.svg?branch=main&event=push)](https://github.com/AliSajid/gh-bofh-rs/actions/workflows/ci.yaml)
[![codecov](https://codecov.io/gh/AliSajid/gh-bofh-rs/graph/badge.svg?token=rrsVYywjlx)](https://codecov.io/gh/AliSajid/gh-bofh-rs)
[![Security Audit](https://github.com/AliSajid/gh-bofh-rs/actions/workflows/audit.yaml/badge.svg?branch=main)](https://github.com/AliSajid/gh-bofh-rs/actions/workflows/audit.yaml)
[![GitHub issues](https://img.shields.io/github/issues/AliSajid/gh-bofh-rs)](https://github.com/AliSajid/gh-bofh-rs/issues)

[![Crates.io License](https://img.shields.io/crates/l/gh-bofh-rs)](https://crates.io/crates/gh-bofh-rs)
[![REUSE Compliance](https://img.shields.io/reuse/compliance/github.com%2FAliSajid%2Fgh-bofh-rs)](https://api.reuse.software/info/github.com/AliSajid/gh-bofh-rs)
[![OpenSSF Best Practices](https://www.bestpractices.dev/projects/9466/badge)](https://www.bestpractices.dev/projects/9466)

[![Libraries.io SourceRank](https://img.shields.io/librariesio/sourcerank/cargo/gh-bofh-rs)](https://libraries.io/cargo/gh-bofh-rs)
[![ClearlyDefined Score](https://img.shields.io/clearlydefined/score/crate/cratesio/-/gh-bofh-rs/1.2.0-next.1)](https://clearlydefined.io/definitions/crate/cratesio/-/gh-bofh-rs/1.2.0-next.1)

Generate a BOFH Excuse for github-cli from the commandline as a `gh` extension.

## Builds

|         | Stable                                                                                                                                                             | Beta                                                                                                                                                           | Nightly                                                                                                                                                              | MSRV (1.74.1)                                                                                                                                                  |
| ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Linux   | ![Ubuntu x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/ubuntu-stable.json)   | ![Ubuntu x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/ubuntu-beta.json)   | ![Ubuntu x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/ubuntu-nightly.json)   | ![Ubuntu x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/ubuntu-msrv.json)   |
| Windows | ![Windows x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/windows-stable.json) | ![Windows x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/windows-beta.json) | ![Windows x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/windows-nightly.json) | ![Windows x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/windows-msrv.json) |
| macos   | ![macos x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/macos-stable.json)     | ![macos x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/macos-beta.json)     | ![macos x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/macos-nightly.json)     | ![macos x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/macos-msrv.json)     |

## Installation

### From Source

```bash

cargo install gh-bofh-rs

```

### From Release

Download the latest release from [here](https://github.com/AliSajid/gh-bofh-rs/releases/latest) and install it using `gh extension install <path-to-release>`

### From The GitHub CLI

```bash
gh extension install AliSajid/gh-bofh-rs
```

## Usage

```bash
gh bofh-rs
```

You can switch between the different types of excuses using the `--type` flag.

```bash

gh bofh-rs --type [modern|classic]

```

You can also use them as standalone flags.

```bash

gh bofh-rs --modern
gh bofh-rs --classic

```

## Current Status

This project is currently being maintained. We are happy to include more modern excuses. Please see [CONTRIBUTING.md](CONTRIBUTING.md) for more information.

## Contributing

If you want to contribute to this project, please read the [CONTRIBUTING.md](CONTRIBUTING.md) file.

## Detailed documentation

### High Level Overview

The architecture documentation can be found in the project repository under the following URL:  
[Architecture Documentation](ARCHITECTURE.md)

### Security Requirements

The security requirements and expectations documentation can be found in the project repository under the following URL:  
[Security Requirements Documentation](SECURITY_REQUIREMENTS.md)

## License

This project is dual-licensed under the [MIT](LICENSE-MIT) and [Apache 2.0](LICENSE-APACHE) licenses.
