use std::fs::OpenOptions;
use std::io::{BufWriter, Result, Write};

use chrono::Utc;

/// This function logs the given message with the current UTC time string.
///
/// # Arguments
///
/// * `to_log` - The message to be logged.
pub fn log(to_log: &str) {
    println!("{}", mk_str(to_log));
}

/// Write a log message to a file.
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
/// # Errors
///
/// This function will return an error if it is unable to write to the log file.
pub fn log_file(to_log: &str, file_name: &str) -> Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(file_name)
        .expect("Unable to open logfile");

    let mut writer = BufWriter::new(file);

    let msg = mk_str(to_log);

    write!(writer, "{msg}")?;
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
        .format("[%Y-%m-%d] - [%h:%M-%S]")
        .to_string();
    let msg = format!("{now} - {to_log}");

    msg
}
