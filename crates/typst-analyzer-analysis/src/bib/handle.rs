#![allow(unused)]

use std::env::current_dir;

fn parse_bib() -> Result<(), anyhow::Error> {
    let path = current_dir()?.join("bibliography.yml");
    Ok(())
}
