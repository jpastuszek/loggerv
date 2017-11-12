# loggerv

[![build status](https://secure.travis-ci.org/clux/loggerv.svg)](http://travis-ci.org/clux/loggerv)
[![coverage status](http://img.shields.io/coveralls/clux/loggerv.svg)](https://coveralls.io/r/clux/loggerv)
[![crates status](https://img.shields.io/crates/v/loggerv.svg)](https://crates.io/crates/loggerv)

A simple `stdout` and `stderr` writing `Logger` implementation of the `log` crate, using `ansi_term` for colors and configured via a log level. Designed for simple Command Line Interfaces (CLIs).

## Usage

First, add this to your `Cargo.toml`:

```toml
[dependencies]
log = "0.3"
loggerv = "0.4"
```

Next, add this to the `main.rs` or the file containing the `main` function for your CLI program:

```rust
extern crate loggerv;
```

## Getting Started

Clone this repository, then run the following command:

```
$ cargo run --example clap -- -vvv
```

This will run an example that uses the [clap](https://crates.io/crates/clap) argument parser to change the log level at run-time based on the number of `-v` arguments that are passed to the application. 

Next, run the following command:

```
$ cargo run --example config -- -vvv -d -n
```

This will run an example that allows for run-time configuration of output based on command line arguments. For both examples, the `-h,--help` flag can be used to display information about the various flags and their effects on logging and output.

## [Documentation](http://clux.github.io/loggerv)

## License

MIT-Licensed. See LICENSE file for details.

