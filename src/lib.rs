#![deny(missing_docs)]

//! `simplog` is as its name suggests a very simpler logging implementation for rust
//! It provides three main features
//!    - Settable log level (or verbosity) (default is Log::Level::Error)
//!    - Optional prefix each log line with the Level it corresponds to (after timestamp if present)
//!    - Optional timestamp prefixed to each line

use chrono::{DateTime, Utc};
use std::io::{stderr, stdout, Write};
use std::str::FromStr;

use log::{Level, Log, Metadata, Record};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use atty::Stream;


/// Use the `SimpleLogger` struct to initialize a logger. From then on, the rust `log` framework
/// should be used to output log statements as usual.
///
/// # Example
/// ```
/// use log::{info, error};
/// use simplog::SimpleLogger;
///
/// SimpleLogger::init(None); // Log level defaults to `Error`
/// info!("Hello World!");
/// // Produces nothing
/// error!("Goodbye World!");
/// // Produces "Goodbye World"
/// ```
#[derive(Clone)]
pub struct SimpleLogger {
    log_level: Level,
    prefix: bool,
    timestamps: Option<String>,
}

const DEFAULT_LOG_LEVEL: Level = Level::Error;

impl SimpleLogger {
    /// Initialize the logger, with an optionally provided log level (`verbosity`) in a `&str`
    /// If `None` is provided -> The log level will be set to `Error`
    /// If 'Some(`verbosity') is a &str with a valid log level, the string will be parsed and if
    /// valid set as the log level.
    ///
    /// # Example
    /// ```
    /// use log::info;
    /// use simplog::SimpleLogger;
    ///
    /// SimpleLogger::init(Some("info"));
    /// info!("Hello World!");
    /// // Produces "Hello World"
    /// ```
    pub fn init(verbosity: Option<&str>) -> Box<Self> {
        Self::init_prefix(verbosity, true)
    }

    /// Initialize the logger, with an optionally provided log level (`verbosity`) in a &str
    /// The default log level is Error if `None` is provided.
    /// `prefix` determines whether each log line output is prefixed with the level that produced it
    ///
    /// # Example
    /// ```
    /// use log::info;
    /// use simplog::SimpleLogger;
    ///
    /// SimpleLogger::init_prefix(Some("info"), true);
    /// info!("Hello World!");
    /// // Produces "INFO   - Hello World"
    /// ```
    pub fn init_prefix(verbosity: Option<&str>, prefix: bool) -> Box<Self> {
        let log_level = parse_log_level(verbosity);
        let simplogger = SimpleLogger {
            log_level,
            prefix,
            timestamps: None,
        };
        let logger = Box::new(simplogger);
        let _ = log::set_boxed_logger(logger.clone());
        log::set_max_level(log_level.to_level_filter());
        logger
    }

    /// Set the format for Timestamps to be printed at the start of each log line
    /// Set to `None` to disable timestamp printing
    ///
    /// The Format string fields can be found
    /// here [https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html]
    /// e.g. "%T"
    ///
    /// # Example
    /// ```
    /// use log::info;
    /// use simplog::SimpleLogger;
    ///
    /// let mut logger = SimpleLogger::init(None);
    /// logger.set_timestamp_format(Some("%T"));
    /// info!("Hello World!");
    /// ```
    pub fn set_timestamp_format(&mut self, format: Option<&str>) {
        self.timestamps = format.map(|s| s.to_string());
    }
}

/*
    Parse an optional String argument ("debug", "info", "trace", "error") into a log level that
    can be used to set verbosity of output. If none is supplied or there is an error parsing the
    String, then the DEFAULT_LOG_LEVEL of "Error" is used.
*/
fn parse_log_level(arg: Option<&str>) -> Level {
    match arg {
        None => DEFAULT_LOG_LEVEL,
        Some(arg) => match Level::from_str(arg) {
            Ok(ll) => ll,
            Err(_) => DEFAULT_LOG_LEVEL
        }
    }
}

/*
    Implement the simpler logger.
    - depending on the way Logger was created a prefix with the level of the output is printed or not
    - "Error" level output is printed to STDERR, all other levels are printed to STDOUT
*/
impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.log_level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let mut stdout = StandardStream::stdout(ColorChoice::Always);

            let message = if self.prefix {
                format!("{}\t- {}", record.level(), record.args())
            } else {
                format!("{}", record.args())
            };

            if atty::is(Stream::Stdout) {
                match record.level() {
                    Level::Error => stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap(),
                    Level::Warn => stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow))).unwrap(),
                    Level::Info=> stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta))).unwrap(),
                    Level::Debug=> stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue))).unwrap(),
                    Level::Trace=> stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap()
                }
            }

            match &self.timestamps {
                Some(format) => {
                    let now: DateTime<Utc> = Utc::now();
                    writeln!(&mut stdout, "{} {}", now.format(format), message).unwrap()
                },
                None => writeln!(&mut stdout, "{}", message).unwrap(),
            }
        }
    }

    fn flush(&self) {
        stdout().flush().unwrap();
        stderr().flush().unwrap();
    }
}

#[cfg(test)]
mod test {
    use log::Level;

    use super::SimpleLogger;

    #[test]
    fn no_log_level_arg() {
        assert_eq!(super::parse_log_level(None), super::DEFAULT_LOG_LEVEL);
    }

    #[test]
    fn invalid_log_level_arg() {
        assert_eq!(super::parse_log_level(Some("garbage")), super::DEFAULT_LOG_LEVEL);
    }

    #[test]
    fn info_log_level_arg() {
        assert_eq!(super::parse_log_level(Some("INFO")), Level::Info);
    }

    #[test]
    fn error_log_level_arg() {
        assert_eq!(super::parse_log_level(Some("ERROR")), Level::Error);
    }

    #[test]
    fn parse_debug_log_level_arg() {
        assert_eq!(super::parse_log_level(Some("DEBUG")), Level::Debug);
        assert_eq!(super::parse_log_level(Some("debug")), Level::Debug);
    }

    #[test]
    fn init_legacy_no_levels() {
        SimpleLogger::init(None);
    }

    #[test]
    fn init_legacy_debug_level() {
        SimpleLogger::init(Some("DEBUG"));
    }

    #[test]
    fn init_no_level_no_prefix() {
        SimpleLogger::init_prefix(None, false);
    }
}
