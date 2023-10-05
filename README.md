# unimarkup-rs [![Unimarkup Status](https://github.com/Unimarkup/unimarkup-rs/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/Unimarkup/unimarkup-rs/actions/workflows/rust.yml)

Compiler implementation for the Unimarkup markup language.

## Usage

- **Command line tool**

  The `unimarkup` compiler is available as command line tool.
  Because the compiler is written in Rust, it is possible to install the compiler using the [Rust toolchain](https://rustup.rs/).
  Once you have installed the toolchain, `unimarkup` can be insalled using

  ```
  cargo install unimarkup
  ```

  To convert `*.um` files to supported output formats, you may run `unimarkup --help` to see all options.
  Below are some examples for most common use cases.

  **Convert to HTML:**

  ```
  unimarkup --formats=html my_file.um
  ```

  **Note:** If no format is set, the file is converted to all supported formats.

  **Define natural language of the content:**

  ```
  unimarkup --lang=en-US my_file.um
  ```

  **Define output filename:**

  ```
  unimarkup --output-file=my_output_file my_file.um
  ```

  **Note:** If this setting is not set, the output filename is taken from the input filename.

- **Library**

  The reference implementation is built with the intent to allow others to build applications on top of it.
  Therefore, all libraries are bundled inside the [unimarkup-core](/core/README.md) crate.
  For more information, checkout the [core documentation](https://docs.rs/unimarkup-core/latest/unimarkup_core/).

## Repository structure

This repository is split into several crates:

- [unimarkup](/cli/README.md) ... This crate is located under [/cli](/cli/README.md), and is a CLI wrapper over [core](/core/README.md)
- [unimarkup-commons](/commons/README.md) ... This crate contains common functionalities needed in other Unimarkup crates
- [unimarkup-core](/core/README.md) ... This crate wraps all Unimarkup library crates, offering a single dependency point for crates building on top of `unimarkup-rs`
- [unimarkup-inline](/inline/README.md) ... This crate contains the parser for inline elements
- [unimarkup-render](/render/README.md) ... This crate contains traits and implementations to render Unimarkup content to supported output formats
- [unimarkup-parser](/parser/README.md) ... This crate contains the parser for Unimarkup elements except inlines

# License

Apache 2.0 Licensed
