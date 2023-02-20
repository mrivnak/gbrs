# GB.rs

![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/mrivnak/gbrs/build.yml)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/mrivnak/gbrs/test.yml)
![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/mrivnak/gbrs?display_name=tag&sort=semver)

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

- Rust >= 1.56.0

### Building

```sh
cargo build
```

### Running

```sh
cargo run -- <ROM>
```
