# RustBoy

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/mrivnak/rust-boy/Cargo%20Check)
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/mrivnak/rust-boy/Run%20Unit%20Tests?label=tests)
![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/mrivnak/rust-boy?display_name=tag&sort=semver)

Nintendo Game Boy emulator written in Rust

## Getting Started

At the moment RustBoy only supports running from the command line

```sh
# macOS/Linux
./rust_boy <ROM>

# Windows
.\rust_boy.exe <ROM>
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
