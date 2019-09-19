use std::io::{stderr, stdout, Write};
use std::str::FromStr;

use log;
use log::{Level, Log, Metadata, Record};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub struct SimpleLogger {
    log_level: Level,
    prefix: bool,
}

const DEFAULT_LOG_LEVEL: Level = Level::Error;

/// Initialize the SimpleLogger using the 'init()' function by passing it an Option<&str>
/// that has 'None' or 'Some("log_level_str")', where 'log_level_str' is a &str with a valid
/// log level, in any case. The string will be parsed and if valid set as the log level.
///
/// # Example
/// ```
/// #[macro_use]
/// extern crate log;
///
/// extern crate simplog;
/// use simplog::simplog::SimpleLogger;
///
/// fn main() {
///     SimpleLogger::init(None);
///     info!("Hello World!");
/// }
///
/// ```
impl SimpleLogger {
    pub fn init(arg: Option<&str>) {
        Self::init_prefix(arg, true);
    }

    pub fn init_prefix(arg: Option<&str>, prefix: bool) {
        let log_level = parse_log_level(arg);
        let logger = Box::new(SimpleLogger {
            log_level,
            prefix,
        });
        let _ = log::set_boxed_logger(logger);
        log::set_max_level(log_level.to_level_filter());
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

            match record.level() {
                Level::Error => stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap(),
                Level::Warn => stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow))).unwrap(),
                Level::Info=> stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta))).unwrap(),
                Level::Debug=> stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue))).unwrap(),
                Level::Trace=> stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap()
            }

            writeln!(&mut stdout, "{}", message).unwrap();
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

    use simplog::SimpleLogger;

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