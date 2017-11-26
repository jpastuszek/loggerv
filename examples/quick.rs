//! An example using the Builder pattern API to configure the logger at run-time based on command
//! line arguments.
//!
//! The default output is `module::path: message`, and the "tag", which is the text to the left of
//! the colon, is colorized. This example allows the user to dynamically change the output based
//! on command line arguments.
//!
//! The [clap](https://crates.io/crates/clap) argument parser is used in this example, but loggerv
//! works with any argument parser.

extern crate ansi_term;
#[macro_use] extern crate log;
extern crate loggerv;
extern crate clap;

use clap::{Arg, App};

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
       .get_matches();
    
    loggerv::init_with_verbosity(args.occurrences_of("v")).unwrap();

    error!("This is always printed to stderr");
    warn!("This too is always printed to stderr");
    info!("This is optional info printed to stdout");  // for ./app -v or higher
    debug!("This is optional debug printed to stdout"); // for ./app -vv or higher
    trace!("This is optional trace printed to stdout"); // for ./app -vvv
}
