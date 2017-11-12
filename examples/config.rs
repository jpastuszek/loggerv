//! An example using the Builder pattern API to configure the logger based on command line
//! arguments.
//!
//! The [clap](https://crates.io/crates/clap) argument parser is used in this example, but loggerv
//! works with any argument parser.

#[macro_use] extern crate log;
extern crate loggerv;
extern crate clap;

use clap::{Arg, App};

fn main() {
    let args = App::new("app")
       .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
       .arg(Arg::with_name("debug")
            .short("d")
            .long("debug")
            .help("Adds the module path and line numbers to log statements"))
       .arg(Arg::with_name("no-color")
            .short("n")
            .long("no-color")
            .help("Disables colorized output"))
       .get_matches();

    loggerv::Logger::new()
        .verbosity(args.occurrences_of("v"))
        .line_numbers(args.is_present("debug"))
        .module_path(args.is_present("debug"))
        .colors(!args.is_present("no-color"))
        .init()
        .unwrap();

    error!("This is always printed to stderr");
    warn!("This too is always printed to stderr");
    info!("This is optional info and printed to stdout");
    debug!("This is optional debug and printed to stdout");
    trace!("This is optional trace and printed to stdout");
}

