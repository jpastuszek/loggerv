# loggerv

[![build status](https://secure.travis-ci.org/clux/loggerv.svg)](http://travis-ci.org/clux/loggerv)
[![coverage status](http://img.shields.io/coveralls/clux/loggerv.svg)](https://coveralls.io/r/clux/loggerv)
[![crates status](https://img.shields.io/crates/v/loggerv.svg)](https://crates.io/crates/loggerv)

A simple `stdout` and `stderr` writing `Logger` implementation of the `log` crate, using `ansi_term` for colors and configured via a log level. Designed for simple CLIs.

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

## [documentation](http://clux.github.io/loggerv)

## License

MIT-Licensed. See LICENSE file for details.
