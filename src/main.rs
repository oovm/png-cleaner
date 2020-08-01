#![feature(try_trait)]

use std::{fs, fs::File, io::Write};

use glob::glob;
use serde::Deserialize;
use toml;

pub use error::Error;

#[allow(unused_imports)]
use crate::utils::{check_file, estimate_size, PNG};

mod error;
pub mod utils;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub glob: String,
    pub min_size: u64,
    pub min_ratio: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self { glob: String::from("**/*.png"), min_size: 1024 * 1024 * 4, min_ratio: 1.0 }
    }
}

fn main() -> Result<(), Error> {
    let mut cfg = Config::default();
    if let Ok(file) = &fs::read("pngc.toml") {
        if let Ok(o) = toml::from_slice(file) {
            cfg = o
        };
    }
    let mut file = File::create("pngc.csv")?;
    file.write_all("路径,大小(KB),异常等级\n".as_bytes())?;
    for entry in glob(&cfg.glob)? {
        if let Ok(w) = check_file(entry, &cfg) {
            file.write_all(w.as_bytes())?;
        }
    }
    Ok(())
}
