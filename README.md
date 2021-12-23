# unimarkup-rs [![UniMarkup Status](https://github.com/Unimarkup/unimarkup-rs/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/Unimarkup/unimarkup-rs/actions/workflows/rust.yml)

Compiler implementation for the UniMarkup markup language written in [Rust](https://www.rust-lang.org/).

## Toolchain setup

This project is written in Rust. Thankfully, installation and setup of Rust development environment ist pretty straightforward.

### Dev environment:

1. Install Rust toolchain with [rustup](https://rustup.rs/).
2. [VS Code](https://code.visualstudio.com/), [Sublime Text](https://www.sublimetext.com/), [NeoVim](https://neovim.io/) or some other editor of your choice with support for [LSP](https://microsoft.github.io/language-server-protocol/)
3. [rust-analyzer](https://github.com/rust-analyzer/rust-analyzer) as a Language Server to be used with LSP.
4. `clippy` as the linter for code. i.e. for VS Code this setting is in the `rust-analyzer` extension settings, change `cargo check` command to `clippy`
5. Enable auto-formatting of the code, use default `rustfmt` settings.

## Usage

At the moment, `unimarkup-rs` is planned to be published to [crates.io](https://crates.io), but it's not there yet. For now, you can build the unimarkup-rs from source:

1. Clone the repository: `git clone https://github.com/Unimarkup/unimarkup-rs.git`
2. Build the `unimarkup-rs`: `cargo build --release`
3. Execute the binary: `./target/release/unimarkup-rs --formats=html path/to/unimarkup_file.um`

If you want to install `unimarkup-rs` and use it system wide, then use the following command:
`cargo install --path .` inside of the `unimarkup-rs` repository.

In that case, the whole process would look like:

1. Clone the repository: `git clone https://github.com/Unimarkup/unimarkup-rs.git`
2. Install the `unimarkup-rs`: `cargo install --path .`
3. Execute the unimarkup-rs: `unimarkup-rs --formats=html path/to/unimarkup_file.um`

## Documentation

The documentation of `unimarkup-rs` is not yet published, but it can be built from source:

1. Clone the repository: `git clone https://github.com/Unimarkup/unimarkup-rs.git`
2. Change into the directory: `cd ./unimarkup-rs`
3. Build the docs: `cargo doc --no-deps`. This commands builds the docs ONLY for unimarkup-rs. If you want docs for all dependencies, then simply run `cargo doc`

This builds the documentation into the `unimarkup-rs/target/doc/` folder. To open docs, locate and open the `unimarkup-rs/target/doc/unimarkup-rs/index.html` file.
