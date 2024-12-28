//! # bibliograpy
//!
//! This module contains funtions related to bibliograpy and serialization and deserialization of .yml file.

#![allow(unused, dead_code, clippy::enum_variant_names)]
#![deny(clippy::unwrap_used)]

use serde::{Deserialize, Serialize};
use serde_yml::Value;
use tinyerror::TinyError;

#[derive(TinyError)]
enum BibError {
    #[error("Failed to connect to the database")]
    ConnectionError,

    #[error("Invalid configuration provided")]
    ConfigError,

    #[error("IO error occurred")]
    IoError,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

/// will look up the Text/Document and gets the #bibliograpy("anyword") and look if `anyword` is in
/// specific dir.
///
/// will create one with code action
/// if the specified profile is missing from  bibliograpy add a fake one to satisfy diagnostics
use std::env::current_dir;
use std::path::{Path, PathBuf};

use crate::typ_logger;

pub fn parse_bib() -> Result<(), anyhow::Error> {
    let path = Path::new("/home/abhi/git/typst-analyzer/examples/bibliography.yml");
    let content: String = std::fs::read_to_string(path)?;
    typ_logger!("{:#?}", content);
    let deserialized_point = serde_yml::Deserializer::from_str(&content);
    let value = Value::deserialize(deserialized_point)?;
    typ_logger!("{:#?}", value);
    Ok(())
}
