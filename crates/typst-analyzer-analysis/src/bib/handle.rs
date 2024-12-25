#![allow(unused)]

use std::env::current_dir;

fn parse_bib() {
    let path = current_dir().unwrap().join("bibliography.yml");
}
