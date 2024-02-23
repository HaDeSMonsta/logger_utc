//! # logger_utc: Logging with time stamp for Rust
//!
//! logger_utc provides a handful of functions to log
//! a given message with a time stamp to stdout, a fixed file name,
//! or a dynamic file name, based on the UTC date.

use std::fs::OpenOptions;
use std::io::{BufWriter, Result, Write};

use chrono::Utc;

/// This function logs the given message with the current UTC time stamp to stdout.
///
/// # Arguments
///
/// * `to_log` - The message to be logged.
///
/// # Examples
///
/// ```rust
/// use logger_utc::log;
///
/// fn main() {
///     log("MSG");
/// }
/// ```
///
/// This will print `[%Y-%m-%d] - [%H:%M-%S] - MSG` to the console.
pub fn log(to_log: &str) {
    println!("{}", mk_str(to_log));
}

/// Write a log message including the current UTC time stamp to a file.
///
/// # Arguments
///
/// * `to_log` - The log message to write.
/// * `file_name` - The name of the log file to write to.
///
/// # Panics
///
/// This function will panic if it is unable to open the log file.
///
/// # Example
///
/// ```rust
/// use logger_utc::log_to_file;
///
/// fn main() {
///     log_to_file("MSG", "err.log").unwrap();
/// }
/// ```
///
/// This will log `[%Y-%m-%d] - [%H:%M-%S] - MSG` to the file `err.log`.
///
/// # Errors
///
/// This function will return an error if it is unable to write to the log file.
pub fn log_to_file(to_log: &str, file_name: &str) -> Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(file_name)
        .expect("Unable to open logfile");

    let mut writer = BufWriter::new(file);

    let msg = mk_str(to_log);

    write!(writer, "{msg}")?;
    Ok(())
}

/// Writes the given log string to a dynamic file.
/// The file name will be a combination of the current date in the format of `%Y-%m-%d`
/// and the provided name.
///
/// # Arguments
///
/// - `to_log`: The log string to be written to the file.
/// - `file_path`: Optional file path where the file will be saved at the end.
/// '/' can be included, if not, it will be appended.
/// If not provided, the file will be saved in the current directory.
/// - `file_name`: The name of the file without date prefix.
///
/// # Example
///
/// ```rust
/// use logger_utc::log_to_dyn_file;
///
/// fn main() {
///     log_to_dyn_file("MSG", Some("logs/"), "err.log").unwrap();
/// }
/// ```
///
/// This will log `[%Y-%m-%d] - [%H:%M-%S] - MSG` to the file `logs/%Y-%m-%d-err.log`.
///
/// # Errors
///
/// This function will return an error if it is unable to write to the log file.
pub fn log_to_dyn_file(to_log: &str, file_path: Option<&str>, file_name: &str) -> Result<()> {
    let date = Utc::now()
        .format("%Y-%m-%d")
        .to_string();
    let path = match file_path {
        Some(path) => {
            if path.ends_with("/") { path.to_string() } else { format!("{path}/") }
        }
        None => { String::new() }
    };

    let combined_file_path = &format!("{path}{date}-{file_name}");

    log_to_file(to_log, combined_file_path)?;
    Ok(())
}

/// Returns a string slice with a formatted log message.
///
/// # Arguments
///
/// * `to_log` - The message to include in the log.
///
/// # Examples
fn mk_str(to_log: &str) -> String {
    let now = Utc::now()
        .format("[%Y-%m-%d] - [%H:%M-%S]")
        .to_string();
    let msg = format!("{now} - {to_log}");

    msg
}
