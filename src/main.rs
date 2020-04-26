#![feature(try_trait)]

use std::{fs, fs::File, io::Write};

#[allow(unused_imports)]
use clap::{App, Arg};
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
    App::new("PNG Cleaner")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();
    let cfg: Config = match toml::from_slice(&fs::read("pngc.toml")?) {
        Ok(o) => o,
        Err(_) => Config::default(),
    };
    let mut file = File::create("pngc.csv")?;
    file.write_all("路径,大小(MB),异常等级\n".as_bytes())?;
    for entry in glob(&cfg.glob)? {
        if let Ok(w) = check_file(entry, &cfg) {
            file.write_all(w.as_bytes())?;
        }
    }
    Ok(())
    /*
    let input = matches.value_of("INPUT").unwrap();

    let value = parse_file(input, matches.value_of("File Format"))?;
    match matches.occurrences_of("Text") {
        0 => (),
        _ => write_to_file(&format!("{}.m", input), value.to_string().as_bytes())?,
    }
    match matches.occurrences_of("Binary") {
        0 => (),
        _ => write_to_file(&format!("{}.wxf", input), &value.to_bytes())?,
    }
    match matches.occurrences_of("Compress") {
        0 => (),
        _ => write_to_file(&format!("{}.mx", input), &value.to_compressed())?,
    };
    */
}
