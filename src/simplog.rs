use log;
use log::{Log, Record, Level, Metadata};
use std::io::{stdout, stderr, Write};
use std::str::FromStr;

pub struct SimpleLogger {
    log_level: Level
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
        let level = parse_log_level(arg);
        log::set_boxed_logger(Box::new(SimpleLogger{ log_level: level })).unwrap();
        log::set_max_level(level.to_level_filter());
    }
}

fn parse_log_level(arg: Option<&str>) -> Level {
    match arg {
        None => DEFAULT_LOG_LEVEL,
        Some(arg) => match Level::from_str(arg) {
            Ok(ll) => ll,
            Err(_) => DEFAULT_LOG_LEVEL
        }
    }
}

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.log_level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            if record.level() == Level::Error {
                eprintln!("{}\t- {}", record.level(), record.args());
            } else {
                println!("{}\t- {}", record.level(), record.args());
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
    fn debug_log_level_arg() {
        assert_eq!(super::parse_log_level(Some("DEBUG")), Level::Debug);
    }
}