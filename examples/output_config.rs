//! An example using the Builder pattern API to configure the logger at run-time to change
//! the output stream for the INFO, DEBUG, and TRACE levels from `stdout` to `stderr`.
//!
//! The default output stream for INFO, DEBUG, and TRACE levels is `stdout`. This example
//! demonstrates changing from the defaults at run-time, but it can also be done at compile-time. 
//!
//! The [clap](https://crates.io/crates/clap) argument parser is used in this example, but loggerv
//! works with any argument parser.

extern crate ansi_term;
#[macro_use] extern crate log;
extern crate loggerv;
extern crate clap;

use clap::{Arg, App};
use log::LogLevel;
use loggerv::Output;

fn main() {
    // Add the following line near the beginning of the main function for an application to enable
    // colorized output on Windows 10. 
    //
    // Based on documentation for the ansi_term crate, Windows 10 supports ANSI escape characters,
    // but it must be enabled first using the `ansi_term::enable_ansi_support()` function. It is
    // conditionally compiled and only exists for Windows builds. To avoid build errors on
    // non-windows platforms, a cfg guard should be put in place.
    #[cfg(windows)] ansi_term::enable_ansi_support().unwrap();

    let args = App::new("app")
       .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
       .arg(Arg::with_name("debug")
            .short("d")
            .long("debug")
            .help("Changes the output stream for INFO, DEBUG, and TRACE from stdout to stderr."))
       .get_matches();

    if args.is_present("debug") {
        loggerv::Logger::new()
            .output(&LogLevel::Info, Output::Stderr)
            .output(&LogLevel::Debug, Output::Stderr)
            .output(&LogLevel::Trace, Output::Stderr)
    } else {
        loggerv::Logger::new()
    }.verbosity(args.occurrences_of("v"))
    .init()
    .unwrap();

    error!("This is always printed to stderr");
    warn!("This too is always printed to stderr");
    info!("This is optionally printed to stdout or stderr based on the verbosity and the debug flag");
    debug!("This is optionally printed to stdout or stderr based on the verbosity and the debug flag");
    trace!("This is optionally printed to stdout or stderr based on the verbosity and the debug flag");
}

