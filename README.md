# unimarkup-rs [![Unimarkup Status](https://github.com/Unimarkup/unimarkup-rs/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/Unimarkup/unimarkup-rs/actions/workflows/rust.yml)

Compiler implementation for the Unimarkup markup language written in [Rust](https://www.rust-lang.org/).

## Toolchain setup

We use the following tools for development:

1. [rustup](https://rustup.rs/) to get a Rust compiler.
2. [rust-analyzer](https://github.com/rust-analyzer/rust-analyzer) as a Language Server to be used with LSP.
3. `clippy` as linter. i.e. for VS Code this setting is in the `rust-analyzer` extension settings (Change `cargo check` command to `clippy`)
4. Optional: Enable auto-formatting of the code, by using `rustfmt` default settings.

## Repository structure

This repository is split into several crates:

- [unimarkup](/cli/README.md) ... This crate is located under [/cli](/cli/README.md), and is a CLI wrapper over [core](/core/README.md)
- [unimarkup-core](/core/README.md) ... This crate contains the core reference implementation
- [unimarkup-inline](/inline/README.md) ... This crate contains the reference compiler for inline elements
- [unimarkup-render](/render/README.md) ... This crate contains traits needed in **core** and **inline** to render Unimarkup content to supported output formats
- [system-tests](/system-tests/README.md) ... This crate contains tests that are not specific to one of the other crates

## Documentation

The documentation of `unimarkup-rs`, and its crates is not yet published, but it can be built from source:

1. Clone the repository: `git clone https://github.com/unimarkup/unimarkup-rs.git`
2. Change into the directory: `cd ./unimarkup-rs`
3. Build the docs: `cargo doc --no-deps` (**Note:** This command builds the docs ONLY for unimarkup-rs. If you want docs for all dependencies, run `cargo doc` instead)

This builds the documentation into the `unimarkup-rs/target/doc/` folder.
To open docs, locate and open the `unimarkup-rs/target/doc/unimarkup-rs/index.html` file.

# License

MIT Licensed
