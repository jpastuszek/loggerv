//! A simple `io::stdout` and `io::stderr` writing `Logger` implementation from the
//! `log` crate, using the `ansi_term` crate for colors and configured at runtime via a verbosity
//! or at compile time with simple function calls. Designed for simple Command Line Interfaces
//! (CLIs).
//!
//! This library includes a Builder pattern API for configuring a logger and three initializing
//! helper functions to create a default logger. Ensure you create and initialize only once
//! a global logger with the Builder pattern API or use one of the three public helper functions
//! early in your program as shown in the examples below.
//!
//! The default configuration colorizes the "tag" portion of the log statement, where the tag is
//! the text to the left of a separator, defaulted as the colon (`:`). The message is the
//! portion to the right of the separator and it is _not_ ever colorized. The tag includes only the
//! module path and the separator by default.
//!
//! ## Example
//!
//! The standard example with [clap](https://crates.io/crates/clap) as the arg parser using the
//! default configuration.
//!
//! ```
//! #[macro_use] extern crate log;
//! extern crate clap;
//! extern crate loggerv;
//!
//! use clap::{Arg, App};
//!
//! fn main() {
//!     let args = App::new("app")
//!         .arg(Arg::with_name("v")
//!             .short("v")
//!             .multiple(true)
//!             .help("Sets the level of verbosity"))
//!         .get_matches();
//!
//!     loggerv::init_with_verbosity(args.occurrences_of("v")).unwrap();
//!
//!     error!("This is always printed");
//!     warn!("This too is always printed to stderr");
//!     info!("This is optionally printed to stdout");  // for ./app -v or higher
//!     debug!("This is optionally printed to stdout"); // for ./app -vv or higher
//!     trace!("This is optionally printed to stdout"); // for ./app -vvv
//! }
//! ```
//!
//! But obviously use whatever argument parsing methods you prefer.
//!
//! ## Example
//!
//! For a compile time switch, all you really need is `log` (for the macros) and `loggerv` for how
//! to print what's being sent to the macros with the default configuration.
//!
//! ```
//! #[macro_use] extern crate log;
//! extern crate loggerv;
//!
//! use log::LogLevel;
//!
//! fn main() {
//!     loggerv::init_with_level(LogLevel::Info).unwrap();
//!     debug!("This is a debug {}", "message"); // Not printed to stdout
//!     error!("This is printed by default");    // Printed to stderr
//! }
//! ```
//!
//! ## Example
//!
//! If you don't really care at all you could just use the plain `init_quiet` function to only show
//! warnings and errors with the default configuration:
//!
//! ```
//! #[macro_use] extern crate log;
//! extern crate loggerv;
//!
//! fn main() {
//!     loggerv::init_quiet().unwrap();
//!     info!("Hidden");
//!     error!("This is printed by default");
//! }
//! ```
//! 
//! ## Example
//!
//! If you want to configure the output, the Builder pattern API can be used.
//!
//! ```
//! #[macro_use] extern crate log;
//! extern crate clap;
//! extern crate loggerv;
//!
//! use clap::{Arg, App};
//!
//! fn main() {
//!     let args = App::new("app")
//!                    .arg(Arg::with_name("v")
//!                             .short("v")
//!                             .multiple(true)
//!                             .help("Sets the level of verbosity"))
//!                    .get_matches();
//!
//!     loggerv::Logger::new()
//!         .verbosity(args.occurrences_of("v"))
//!         .level(true)
//!         .line_numbers(true)
//!         .separator(" = ")
//!         .module_path(false)
//!         .colors(false)
//!         .init()
//!         .unwrap();
//!
//!     error!("This is always printed to stderr");
//!     warn!("This too is always printed to stderr");
//!     info!("This is optionally printed to stdout");  // for ./app -v or higher
//!     debug!("This is optionally printed to stdout"); // for ./app -vv or higher
//!     trace!("This is optionally printed to stdout"); // for ./app -vvv
//! }
//! ```
//!
//! See the [documentation](https://docs.rs/log/0.3.8/log/) for the
//! [log](https://crates.io/crates/log) crate for more information about its API.
//!

extern crate log;

extern crate atty;
extern crate ansi_term;

use log::{Log, LogLevel, LogMetadata, LogRecord, SetLoggerError};
use std::io::{self, Write};
use ansi_term::Colour;

pub const DEFAULT_COLORS: bool = true;
pub const DEFAULT_DEBUG_COLOR: Colour = Colour::Fixed(7); // light grey
pub const DEFAULT_ERROR_COLOR: Colour = Colour::Fixed(9); // bright red
pub const DEFAULT_INCLUDE_LEVEL: bool = false;
pub const DEFAULT_INCLUDE_LINE_NUMBERS: bool = false;
pub const DEFAULT_INCLUDE_MODULE_PATH: bool = true;
pub const DEFAULT_INFO_COLOR: Colour = Colour::Fixed(10); // bright green
pub const DEFAULT_LEVEL: LogLevel = LogLevel::Warn;
pub const DEFAULT_OFFSET: u64 = 1;
pub const DEFAULT_SEPARATOR: &str = ": ";
pub const DEFAULT_TRACE_COLOR: Colour = Colour::Fixed(8); // grey
pub const DEFAULT_WARN_COLOR: Colour = Colour::Fixed(11); // bright yellow

#[derive(Debug, Clone, PartialEq)]
pub struct Logger {
    colors: bool,
    include_level: bool,
    include_line_numbers: bool,
    include_module_path: bool,
    level: LogLevel,
    offset: u64,
    separator: String,
    verbosity: Option<u64>,
    error_color: Colour,
    warn_color: Colour,
    info_color: Colour,
    debug_color: Colour,
    trace_color: Colour,
}

impl Logger {
    /// Creates a new instance of the verbosity-based logger.
    ///
    /// The default level is WARN. Color is enabled if the parent application or library is running
    /// from a terminal, i.e. running a tty. The default separator is the ": " string. The default
    /// output format is `module path: message`. The following default colors are used:
    ///
    /// | Level | Color         |
    /// |-------|---------------|
    /// | Error | Bright Red    |
    /// | Warn  | Bright Yellow |
    /// | Info  | Bright Green  |
    /// | Debug | Light Grey    |
    /// | Trace | Grey          |
    pub fn new() -> Logger {
        Logger { 
            colors: DEFAULT_COLORS && atty::is(atty::Stream::Stdout) && atty::is(atty::Stream::Stderr),
            include_level: DEFAULT_INCLUDE_LEVEL,
            include_line_numbers: DEFAULT_INCLUDE_LINE_NUMBERS,
            include_module_path: DEFAULT_INCLUDE_MODULE_PATH,
            level: DEFAULT_LEVEL, 
            offset: DEFAULT_OFFSET,
            separator: String::from(DEFAULT_SEPARATOR),
            verbosity: None,
            error_color: DEFAULT_ERROR_COLOR,
            warn_color: DEFAULT_WARN_COLOR,
            info_color: DEFAULT_INFO_COLOR,
            debug_color: DEFAULT_DEBUG_COLOR,
            trace_color: DEFAULT_TRACE_COLOR,
        }
    }

    /// Sets the output color for the ERROR level. The default is bright red.
    pub fn error_color(mut self, c: Colour) -> Self {
        self.error_color = c;
        self
    }

    /// Sets the output color for the WARN level. The default is bright yellow.
    pub fn warn_color(mut self, c: Colour) -> Self {
        self.warn_color = c;
        self
    }

    /// Sets the output color for the INFO level. The default is bright green.
    pub fn info_color(mut self, c: Colour) -> Self {
        self.info_color = c;
        self
    }

    /// Sets the output color for the DEBUG level. The default is light grey.
    pub fn debug_color(mut self, c: Colour) -> Self {
        self.debug_color = c;
        self
    }

    /// Sets the output color for the TRACE level. The default is grey.
    pub fn trace_color(mut self, c: Colour) -> Self {
        self.trace_color = c;
        self
    }

    /// Sets the separator string.
    ///
    /// The separator is the string between the "tag" and the message that make up a log statement.
    /// The tag will be colorized if enabled, while the message will not. The default is `: `.
    ///
    /// If the level, line numbers, and module path are all _not_ included in the log statement,
    /// then the separator is changed to the empty string to avoid printing a lone string or
    /// character before each message portion of the log statement.
    ///
    /// # Example
    ///
    /// ```rust
    /// #[macro_use] extern crate log;
    /// extern crate loggerv;
    ///
    /// use log::LogLevel;
    ///
    /// fn main() {
    ///     loggerv::Logger::new()
    ///         .separator(" = ")
    ///         .init()
    ///         .unwrap();
    ///
    ///     error!("This is printed with an equal sign between the module path and this message");
    /// }
    /// ```
    pub fn separator(mut self, s: &str) -> Self {
        self.separator = String::from(s);
        self
    }

    /// Enables or disables colorizing the output. 
    ///
    /// If the logger is _not_ used in a terminal, then the output is _not_ colorized regardless of
    /// this value.
    ///
    /// # Example
    ///
    /// ```rust
    /// #[macro_use] extern crate log;
    /// extern crate loggerv;
    ///
    /// use log::LogLevel;
    ///
    /// fn main() {
    ///     loggerv::Logger::new()
    ///         .colors(false)
    ///         .init()
    ///         .unwrap();
    ///
    ///     error!("This is printed without any colorization");
    /// }
    /// ```
    pub fn colors(mut self, c: bool) -> Self {
        self.colors = c && atty::is(atty::Stream::Stdout) && atty::is(atty::Stream::Stderr);
        self
    }

    /// Disables colorizing the output.
    ///
    /// The default is to colorize the output unless `stdout` and `stderr` are redirected or piped,
    /// i.e. not a tty.
    ///
    /// # Example
    ///
    /// ```rust
    /// #[macro_use] extern crate log;
    /// extern crate loggerv;
    ///
    /// use log::LogLevel;
    ///
    /// fn main() {
    ///     loggerv::Logger::new()
    ///         .no_colors()
    ///         .init()
    ///         .unwrap();
    ///
    ///     error!("This is printed without any colorization");
    /// }
    /// ```
    pub fn no_colors(mut self) -> Self {
        self. colors = false;
        self
    }

    /// Enables or disables including line numbers in the "tag" portion of the log statement. 
    ///
    /// The tag is the text to the left of the separator.
    ///
    /// # Example
    ///
    /// ```rust
    /// #[macro_use] extern crate log;
    /// extern crate loggerv;
    ///
    /// use log::LogLevel;
    ///
    /// fn main() {
    ///     loggerv::Logger::new()
    ///         .line_numbers(true)
    ///         .init()
    ///         .unwrap();
    ///
    ///     error!("This is printed with the module path and the line number surrounded by
    ///     parentheses");
    /// }
    /// ```
    pub fn line_numbers(mut self, i: bool) -> Self {
        self.include_line_numbers = i;
        self
    }

    /// Enables or disables including the level in the log statement's tag portion. The tag of the
    /// log statement is the text to the left of the separator.
    ///
    /// If the level and the module path are both inculded, then the module path is surrounded by
    /// square brackets.
    ///
    /// # Example
    ///
    /// ```rust
    /// #[macro_use] extern crate log;
    /// extern crate loggerv;
    ///
    /// use log::LogLevel;
    ///
    /// fn main() {
    ///     loggerv::Logger::new()
    ///         .level(true)
    ///         .init()
    ///         .unwrap();
    ///
    ///     error!("This is printed with the 'ERROR' and the module path is surrounded in square
    ///     brackets");
    /// }
    /// ```
    pub fn level(mut self, i: bool) -> Self {
        self.include_level = i;
        self
    }

    /// Explicitly sets the log level instead of through a verbosity.
    ///
    /// # Example
    ///
    /// ```rust
    /// #[macro_use] extern crate log;
    /// extern crate loggerv;
    ///
    /// use log::LogLevel;
    ///
    /// fn main() {
    ///     loggerv::Logger::new()
    ///         .max_level(LogLevel::Info)
    ///         .init()
    ///         .unwrap();
    ///
    ///     error!("This is printed to stderr");
    ///     warn!("This is printed to stderr");
    ///     info!("This is printed to stdout");
    ///     debug!("This is not printed to stdout");
    ///     trace!("This is not printed to stdout");
    /// }
    /// ```
    pub fn max_level(mut self, l: LogLevel) -> Self {
        self.level = l;
        // It is important to set the Verbosity to None here because later with the `init` method,
        // a `None` value indicates the verbosity has _not_ been set or overriden by using this
        // method (`max_level`). If the verbosity is some value, then it will be used and the use
        // of this method will be dismissed.
        self.verbosity = None;
        self
    }

    /// Enables or disables including the module path in the "tag" portion of the log statement.
    ///
    /// The tag is the text to the left of the separator. The default is to include the module
    /// path. Ifthe level is also included, the module path is surrounded by square brackets.
    ///
    /// # Example
    ///
    /// ```rust
    /// #[macro_use] extern crate log;
    /// extern crate loggerv;
    ///
    /// fn main() {
    ///     loggerv::Logger::new()
    ///         .module_path(false)
    ///         .init()
    ///         .unwrap();
    ///
    ///     error!("This is printed without leading module path and separator");
    /// }
    /// ```
    pub fn module_path(mut self, i: bool) -> Self {
        self.include_module_path = i;
        self
    }

    /// Disables the module path in the "tag" portion of the log statement.
    ///
    /// The tag is the text to the left of the separator. The default is to include the module
    /// path.
    ///
    /// # Example
    ///
    /// ```rust
    /// #[macro_use] extern crate log;
    /// extern crate loggerv;
    ///
    /// fn main() {
    ///     loggerv::Logger::new()
    ///         .no_module_path()
    ///         .init()
    ///         .unwrap();
    ///
    ///     error!("This is printed without leading module path and separator");
    /// }
    /// ```
    pub fn no_module_path(mut self) -> Self {
        self.include_module_path = false;
        self
    }

    /// Sets the base level.
    ///
    /// The base level is the level used with zero (0) verbosity. The default is WARN. So, ERROR
    /// and WARN statements will be written and INFO statements will be written with a verbosity of
    /// 1 or greater. If the base level was changed to ERROR, then only ERROR statements will be
    /// written and WARN statements will be written with a verbosity of 1 or greater. Use this
    /// adjust the correlation of verbosity, i.e. number of `-v` occurrences, to level.
    ///
    /// # Example
    ///
    /// ```rust
    /// #[macro_use] extern crate log;
    /// extern crate loggerv;
    ///
    /// use log::LogLevel;
    ///
    /// fn main() {
    ///     loggerv::Logger::new()
    ///         .base_level(LogLevel::Error)
    ///         .verbosity(0)
    ///         .init()
    ///         .unwrap();
    ///
    ///     error!("This is printed");
    ///     warn!("This is not printed");
    ///     info!("This is not printed");
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```rust
    /// #[macro_use] extern crate log;
    /// extern crate loggerv;
    /// 
    /// use log::LogLevel;
    ///
    /// fn main() {
    ///     loggerv::Logger::new()
    ///         .base_level(LogLevel::Info)
    ///         .verbosity(0)
    ///         .init()
    ///         .unwrap();
    ///
    ///     error!("This is printed");
    ///     warn!("This is also printed");
    ///     info!("This is now printed, too");
    /// }
    /// ```
    pub fn base_level(mut self, b: LogLevel) -> Self {
        self.offset = match b {
            LogLevel::Error => 0,
            LogLevel::Warn => 1,
            LogLevel::Info => 2,
            LogLevel::Debug => 3,
            LogLevel::Trace => 4,
                
        };
        self
    }

    /// Sets the level based on verbosity and the offset.
    ///
    /// A verbosity of zero (0) is the default, which means ERROR and WARN log statements are
    /// printed to `stderr`. No other log statements are printed on any of the standard streams
    /// (`stdout` or `stderr`). As the verbosity is increased, the log level is increased and more
    /// log statements will be printed to `stdout`. 
    ///
    /// # Example
    ///
    /// ```rust
    /// #[macro_use] extern crate log;
    /// extern crate loggerv;
    ///
    /// use log::LogLevel;
    ///
    /// fn main() {
    ///     loggerv::Logger::new()
    ///         .verbosity(1)
    ///         .init()
    ///         .unwrap();
    ///
    ///     error!("This is printed to stderr");
    ///     warn!("This is printed to stderr");
    ///     info!("This is printed to stdout");
    ///     debug!("This is not printed to stdout");
    ///     trace!("This is not printed to stdout");
    /// }
    /// ```
    pub fn verbosity(mut self, v: u64) -> Self {
        self.verbosity = Some(v);
        self
    }

    /// Initializes the logger. 
    ///
    /// This also consumes the logger. It cannot be further modified after initialization. 
    ///
    /// # Example
    ///
    /// ```rust
    /// #[macro_use] extern crate log;
    /// extern crate loggerv;
    ///
    /// use log::LogLevel;
    ///
    /// fn main() {
    ///     loggerv::Logger::new()
    ///         .init()
    ///         .unwrap();
    ///
    ///     error!("This is printed to stderr");
    ///     warn!("This is printed to stderr");
    ///     info!("This is not printed to stdout");
    ///     debug!("This is not printed to stdout");
    ///     trace!("This is not printed to stdout");
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// If the tag will be empty because the level, line numbers, and module path were all
    /// disabled, then the separator is changed to the empty string to avoid writing a long
    /// character in front of each message for each log statement.
    ///
    ///
    /// ```rust
    /// #[macro_use] extern crate log;
    /// extern crate loggerv;
    ///
    /// use log::LogLevel;
    ///
    /// fn main() {
    ///     loggerv::Logger::new()
    ///         .module_path(false)
    ///         .level(false) 
    ///         .line_numbers(false)
    ///         .init()
    ///         .unwrap();
    ///
    ///     error!("This is printed to stderr without the separator");
    ///     warn!("This is printed to stderr without the separator");
    ///     info!("This is not printed to stdout");
    ///     debug!("This is not printed to stdout");
    ///     trace!("This is not printed to stdout");
    /// }
    /// ```
    pub fn init(mut self) -> Result<(), SetLoggerError> {
        // If there is no level, line number, or module path in the tag, then the tag will always
        // be empty. The separator should also be empty so only the message component is printed
        // for the log statement; otherwise, there is a weird floating colon in front of every log
        // statement.
        //
        // It is better to do it here than in the `log` function because it only has to be
        // determined once at initialization as opposed to every call to the `log` function. So
        // a potentially slight performance improvement.
        if !self.include_level && !self.include_line_numbers && !self.include_module_path {
            self.separator = String::new();
        }
        // The level is set based on verbosity only if the `verbosity` method has been used and
        // _not_ overwridden a later call to the `max_level` method. If neither the `verbosity` or
        // `max_level` method is used, then the `DEFAULT_LEVEL` is used because it is set with the
        // `new` function. It makes more sense to calculate the level based on verbosity _after_
        // all configuration methods have been called as opposed to during the call to the
        // `verbosity` method. This change enables the offset feature so that the `offset` method
        // can be used at any time during the "building" procedure before the call to `init`.
        // Otherwise, calling the `offset` _after_ the `verbosity` method would have no effect and
        // be difficult to communicate this limitation to users.
        if let Some(v) = self.verbosity {
            self.level = match v + self.offset {
                0 => LogLevel::Error,  
                1 => LogLevel::Warn,  
                2 => LogLevel::Info,  
                3 => LogLevel::Debug, 
                _ => LogLevel::Trace, 
            };
        }
        log::set_logger(|max_level| {
            max_level.set(self.level.to_log_level_filter());
            Box::new(self)
        })
    }

    /// Gets the color to use for the log statement's tag based on level.
    fn color(&self, l: &LogLevel) -> Colour {
        match *l {
            LogLevel::Error => self.error_color,
            LogLevel::Warn => self.warn_color,
            LogLevel::Info => self.info_color,
            LogLevel::Debug => self.debug_color,
            LogLevel::Trace => self.trace_color,
        }
    }

    /// Creates the tag portion of the log statement based on the configuration. 
    ///
    /// The tag portion is the of the log statement is the text to the left of the separator, while
    /// the text to the right of the separator is the message.
    fn create_tag(&self, record: &LogRecord) -> String {
        let level = record.level();
        let level_text = if self.include_level {
            level.to_string()
        } else {
            String::new()
        };
        let module_path_text = if self.include_module_path {
            if self.include_level {
                format!(" [{}]", record.location().module_path())
            } else {
                record.location().module_path().to_string()
            }
        } else {
            String::new()
        };
        let line_text = if self.include_line_numbers {
            format!(" (line {})", record.location().line())
        } else {
            String::new()
        };
        let mut tag = format!("{}{}{}", level_text, module_path_text, line_text);
        if self.colors {
            tag = self.color(&level).paint(tag).to_string();
        }
        tag
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            if record.level() <= LogLevel::Warn {
                writeln!(
                    &mut io::stderr(), 
                    "{}{}{}", 
                    self.create_tag(&record), 
                    self.separator, 
                    record.args()
                ).expect("Writing to stderr");
            } else {
                println!(
                    "{}{}{}", 
                    self.create_tag(&record), 
                    self.separator, 
                    record.args()
                );
            }
        }
    }
}

impl Default for Logger {
    fn default() -> Logger {
        Logger::new()
    }
}

/// Initialize loggerv with a maximal log level.
///
/// See the main loggerv documentation page for an example.
pub fn init_with_level(level: LogLevel) -> Result<(), SetLoggerError> {
    Logger::new().max_level(level).init()
}

/// Initialize loggerv with a verbosity level.
///
/// Intended to be used with an arg parser counting the amount of -v flags.
/// See the main loggerv documentation page for an example.
pub fn init_with_verbosity(verbosity: u64) -> Result<(), SetLoggerError> {
    Logger::new().verbosity(verbosity).init()
}

/// Initializes loggerv with only warnings and errors.
///
/// See the main loggerv documentation page for an example.
pub fn init_quiet() -> Result<(), SetLoggerError> {
    init_with_level(LogLevel::Warn)
}

#[cfg(test)]
mod tests {
    use log::LogLevel;
    use ansi_term::Colour;
    use super::*;

    #[test]
    fn defaults_are_correct() {
        let logger = Logger::new();
        assert_eq!(logger.include_level, DEFAULT_INCLUDE_LEVEL);
        assert_eq!(logger.include_line_numbers, DEFAULT_INCLUDE_LINE_NUMBERS);
        assert_eq!(logger.include_module_path, DEFAULT_INCLUDE_MODULE_PATH);
        assert_eq!(logger.colors, DEFAULT_COLORS);
        assert_eq!(logger.level, DEFAULT_LEVEL);
        assert_eq!(logger.separator, String::from(DEFAULT_SEPARATOR));
        assert_eq!(logger.error_color, DEFAULT_ERROR_COLOR);
        assert_eq!(logger.warn_color, DEFAULT_WARN_COLOR);
        assert_eq!(logger.info_color, DEFAULT_INFO_COLOR);
        assert_eq!(logger.debug_color, DEFAULT_DEBUG_COLOR);
        assert_eq!(logger.trace_color, DEFAULT_TRACE_COLOR);
    }

    #[test]
    fn error_color_works() {
        let logger = Logger::new().error_color(Colour::Fixed(8));
        assert_eq!(logger.error_color, Colour::Fixed(8));
    }

    #[test]
    fn warn_color_works() {
        let logger = Logger::new().warn_color(Colour::Fixed(8));
        assert_eq!(logger.warn_color, Colour::Fixed(8));
    }

    #[test]
    fn info_color_works() {
        let logger = Logger::new().info_color(Colour::Fixed(8));
        assert_eq!(logger.info_color, Colour::Fixed(8));
    }

    #[test]
    fn debug_color_works() {
        let logger = Logger::new().debug_color(Colour::Fixed(8));
        assert_eq!(logger.debug_color, Colour::Fixed(8));
    }

    #[test]
    fn trace_color_works() {
        let logger = Logger::new().trace_color(Colour::Fixed(11));
        assert_eq!(logger.trace_color, Colour::Fixed(11));
    }

    #[test]
    fn separator_works() {
        const EXPECTED: &str = " = ";
        let logger = Logger::new().separator(EXPECTED);
        assert_eq!(logger.separator, EXPECTED);
    }

    #[test]
    fn colors_works() {
        let logger = Logger::new().colors(false);
        assert!(!logger.colors);
    }

    #[test]
    fn no_colors_works() {
        let logger = Logger::new().no_colors();
        assert!(!logger.colors);
    }

    #[test]
    fn line_numbers_works() {
        let logger = Logger::new().line_numbers(true);
        assert!(logger.include_line_numbers);
    }

    #[test]
    fn level_works() {
        let logger = Logger::new().level(true);
        assert!(logger.include_level);
    }

    #[test]
    fn max_level_works() {
        let logger = Logger::new().max_level(LogLevel::Trace);
        assert_eq!(logger.level, LogLevel::Trace);
        assert!(logger.verbosity.is_none());
    }

    #[test]
    fn base_level_works() {
        let logger = Logger::new().base_level(LogLevel::Info);
        assert_eq!(logger.offset, 2);
    }

    #[test]
    fn module_path_works() {
        let logger = Logger::new().module_path(false);
        assert!(!logger.include_module_path);
    }

    #[test]
    fn no_module_path_works() {
        let logger = Logger::new().no_module_path();
        assert!(!logger.include_module_path);
    }

    #[test]
    fn verbosity_works() {
        let logger = Logger::new().verbosity(3);
        assert_eq!(logger.verbosity, Some(3));
    }

    #[test]
    fn init_works() {
        let result = Logger::new().init();
        assert!(result.is_ok());
    }

    #[test]
    fn color_works() {
        let logger = Logger::new();
        assert_eq!(logger.color(&LogLevel::Error), DEFAULT_ERROR_COLOR);
        assert_eq!(logger.color(&LogLevel::Warn), DEFAULT_WARN_COLOR);
        assert_eq!(logger.color(&LogLevel::Info), DEFAULT_INFO_COLOR);
        assert_eq!(logger.color(&LogLevel::Debug), DEFAULT_DEBUG_COLOR);
        assert_eq!(logger.color(&LogLevel::Trace), DEFAULT_TRACE_COLOR);
    }
}

