//! # bibliograpy
//!
//! This module contains funtions related to bibliograpy and serialization and deserialization of .yml file.

// #![allow(unused, dead_code, clippy::enum_variant_names)]
#![allow(clippy::unwrap_used)]

use std::env::current_dir;
use std::path::PathBuf;
use std::rc::Rc;

use anyhow::anyhow;
use hayagriva::io::{from_yaml_str, to_yaml_str};
use hayagriva::types::EntryType;
use hayagriva::{Entry, Library};
use walkdir::{DirEntry, WalkDir};

// use crate::typ_logger;

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

pub fn bibliography_file_path() -> anyhow::Result<PathBuf, anyhow::Error> {
    let path = current_dir()?;
    let walker = WalkDir::new(&path).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let val = Rc::new(entry?);
        let bib = val
            .clone()
            .path()
            .file_name()
            .ok_or(anyhow!("err"))?
            .to_string_lossy()
            .contains("bibliography.yml");
        if bib {
            return Ok(val.clone().path().to_owned());
        };
    }
    Err(anyhow!("err"))
}

pub fn parse_bib() -> anyhow::Result<Library, anyhow::Error> {
    let file = bibliography_file_path()?; 
    let content = std::fs::read_to_string(file)?;
    // Parse a bibliography
    Ok(from_yaml_str(content.as_str())?)
}

pub fn get_bib_keys() -> anyhow::Result<Vec<String>, anyhow::Error> {
    let bib = parse_bib()?;
    let mut vec = Vec::new();

    for item in bib.iter() {
        // typ_logger!("bibliography: {:#?}", item);
        vec.push(item.key().to_owned());
    }
    Ok(vec)
}

pub fn new_bib_key(key: &str) -> anyhow::Result<bool, anyhow::Error> {
    if let Ok(mut bib) = parse_bib() {
        bib.push(&Entry::new(key, EntryType::Reference));
        let s = to_yaml_str(&bib)?;
        std::fs::write(bibliography_file_path()?, s)?;
        return Ok(true);
    }
    Err(anyhow!("failed in bibliography funtion"))
}
