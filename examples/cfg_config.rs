//! An example using the Builder pattern API to configure the logger based on conditional
//! compilation.
//!
//! The default output is `module::path: message`, and the "tag", which is the text to the left of
//! the colon, is colorized. This example shows how to change the output based on a conditional
//! compilation, such as a Debug versus Release build. In a Debug build, the default output is
//! used with a maximum level of TRACE. In a Release build, the module path is replaced with the
//! level and the maximum level is limited to INFO.
//!
//! The [clap](https://crates.io/crates/clap) argument parser is used in this example, but loggerv
//! works with any argument parser.

extern crate ansi_term;
#[macro_use] extern crate log;
extern crate loggerv;

fn main() {
    // Add the following line near the beginning of the main function for an application to enable
    // colorized output on Windows 10.
    //
    // Based on documentation for the ansi_term crate, Windows 10 supports ANSI escape characters,
    // but it must be enabled first using the `ansi_term::enable_ansi_support()` function. It is
    // conditionally compiled and only exists for Windows builds. To avoid build errors on
    // non-windows platforms, a cfg guard should be put in place.
    #[cfg(windows)] ansi_term::enable_ansi_support().unwrap();

    // This is the correct way to determine the build profile using the `cfg` macro according to
    // this [discussion](https://users.rust-lang.org/t/conditional-compilation-for-debug-release/1098)
    // and the [documentation](http://doc.crates.io/manifest.html#the-profile-sections) for the
    // Cargo manifest.
    if cfg!(debug_assertions) {
        loggerv::Logger::new()
            .max_level(log::Level::Trace)
    } else {
        loggerv::Logger::new()
            .no_module_path()
            .level(true)
            .max_level(log::Level::Info)
    }.init().unwrap();

    error!("This is printed to stderr with a Debug or Release build");
    warn!("This is printed to stderr with a Debug or Release build");
    info!("This is printed to stdout with a Debug or Release build");
    debug!("This is only printed to stdout with a Debug build");
    trace!("This is not printed to stdout with a Debug build");
}

