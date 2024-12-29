use thiserror::Error;

#[derive(Error, Debug)]
pub enum TypError<'a> {
    #[error("Error: this")]
    NotFound,
    #[error("Error: 0")]
    Invalid,
    #[error("Error: 0")]
    SyntaxError,
    #[error("Failed to load optional file: {0}")]
    NonCriticalError(&'a str),
    #[error("A critical error occurred: {0}")]
    CriticalError(String),
}

/// A macro for logging messages to a file in the Typst Analyzer cache directory.
///
/// This macro includes necessary imports to ensure functionality within its scope.
///
/// # Example
/// ```
/// use typst_analyzer::typ_logger;
///
/// typ_logger!("This is a log message.");
///
/// let value = 42;
/// typ_logger!("Formatted log with value: {}", value);
/// ```
#[macro_export]
macro_rules! typ_logger {
    ($($args:tt)*) => {{
        {
            use std::io::Write;
            use anyhow::{Context, Result};

            fn log_message(msg: &str) -> Result<()> {
                let log_dir = dirs::cache_dir()
                    .ok_or_else(|| anyhow::anyhow!("Cache directory not found"))?
                    .join("typst-analyzer");

                // Create the directory if it doesn't exist.
                if !log_dir.exists() {
                    std::fs::create_dir_all(&log_dir).context("Failed to create log directory")?;
                }

                let log_file = log_dir.join("state.log");
                let mut file = std::fs::OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(&log_file)
                    .context("Failed to open log file")?;

                writeln!(file, "{}", msg).context("Failed to write to log file")?;
                Ok(())
            }

            let msg = format!($($args)*);
            if let Err(e) = log_message(&msg) {
                eprintln!("Failed to log message: {e:?}");
            }
        }
    }};
}
