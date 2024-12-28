#![deny(
    clippy::unwrap_used,
    clippy::panicking_unwrap,
    clippy::or_then_unwrap,
    clippy::get_unwrap,
    unreachable_pub
)]

mod actions;
pub mod bibliography;
pub mod completion;
mod diagnostics;
pub mod dict;
mod hints;
pub mod node;
pub mod definition;

pub use completion::resources::*;
pub use diagnostics::handle::*;
pub use hints::handle::*;

/// A macro for logging messages to a file in the Typst Analyzer cache directory.
///
/// This macro includes necessary imports to ensure functionality within its scope.
///
/// # Example
/// ```
/// use typst_analyzer_analysis::typ_logger;
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
