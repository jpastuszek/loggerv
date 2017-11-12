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
                   .get_matches();
    
    loggerv::Logger::new()
        .verbosity(args.occurrences_of("v"))
        .line_numbers(true)
        .module_path(false)
        .init()
        .unwrap();

    error!("this is always printed");
    warn!("this too, and it's printed to stderr");
    info!("this is optional info");  // for ./app -v or higher
    debug!("this is optional debug"); // for ./app -vv or higher
    trace!("this is optional trace"); // for ./app -vvv
}
