# GB.rs

![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/mrivnak/gbrs/check.yml)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/mrivnak/gbrs/test.yml?label=tests)
[![Coverage Status](https://coveralls.io/repos/github/mrivnak/gbrs/badge.svg?branch=main)](https://coveralls.io/github/mrivnak/gbrs?branch=main)
![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/mrivnak/gbrs?display_name=tag&sort=semver)

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![GitHub Actions](https://img.shields.io/badge/github%20actions-%232671E5.svg?style=for-the-badge&logo=githubactions&logoColor=white)
![Coveralls](https://img.shields.io/badge/coveralls-%23b94947.svg?style=for-the-badge&logo=coveralls&logoColor=white)
![Renovate](https://img.shields.io/badge/renovate-%230281a1?style=for-the-badge&logo=renovatebot&logoColor=white)

Nintendo Game Boy emulator written in Rust

## Getting Started

At the moment GB.rs only supports running from the command line

```sh
# Linux/macOS
./gbrs <ROM>

# Windows
.\gbrs.exe <ROM>
```

## Development

### Dependencies

- Rust >= 1.62.1

### Building

```sh
cargo build
```

### Running

```sh
cargo run -- <ROM>
```
