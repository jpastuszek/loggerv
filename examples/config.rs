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
                        .help("Adds module path and line numbers to log statements"))
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
    warn!("This too, and it's printed to stderr");
    info!("This is optional info");
    debug!("This is optional debug");
    trace!("This is optional trace");
}
