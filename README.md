# logger_utc

This package provides a few essential logging features for your
Rust application. You can log your messages with a timestamp
to stdout or a file.

## Features

- Logs the given message with the current UTC time stamp to stdout.
- Writes a log message including the current UTC time stamp to a file.
- Writes any provided log string to a dynamic file.
The filename will be a combination of the current date 
and the provided filename.

## Usage

Add this to your `Cargo.toml`:
```toml
[dependencies]
logger_utc = "<version>"
```

## License

This project is licensed under the Apache-2.0 License.