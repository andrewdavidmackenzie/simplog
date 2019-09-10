[![Build Status](https://travis-ci.org/andrewdavidmackenzie/simplog.svg?branch=master)](https://travis-ci.org/andrewdavidmackenzie/simplog)

# simplog
A small and easy to use rust crate for logging.

## Add to your project
Add the dependency on `simplog`in your crate's `Cargo.toml`file:

```
[dependencies]
simplog = "~1.2"
```

## Importing
Import the simplog crate in your code, and use the SimpleLogger module.

```
extern crate simplog;
use simplog::simplog::SimpleLogger;
```

## Initializing
Initialize the SimpleLogger using the `init()` function by passing it an `Option<&str>` that has a value of `None` or `Some("log_level_str")`, where `log_level_str` is a `&str` with a valid log level, in any case.

The string will be parsed and if valid set as the log level.

```
SimpleLogger::init(Some("Info"));
```

or if you do not want the Log Level prefix printed at the start of each line, initialize thus:
```
SimpleLogger::init_previx(Some("Info"), false);
```


## Logging
Logging is done using the normal rust `log` framework, with it's macros for easilly logging at different
levels: `error!()`, `info!()`, etc.

To include the rust logging framework in your project, add a dependency to your `Cargo.toml`:

```
[dependencies]
log = "0.3.8"
```

and use the crate in your code with:

```
#[macro_use]   
 extern crate log;
```

## Example

```
#[macro_use]
extern crate log;

extern crate simplog;
use simplog::simplog::SimpleLogger;

fn main() {
    SimpleLogger::init(Some("Info"));
    info!("Hello World!");
}
