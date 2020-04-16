#![feature(try_trait)]
extern crate clap;

use std::fs;
use std::fs::File;
use std::os::macos::fs::MetadataExt;

use clap::{App, Arg};
use serde::Deserialize;
use toml;

pub use error::Error;

use crate::utils::{estimate_size, PNG, write_to_file};

mod error;
pub mod utils;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    glob: Option<String>,
    pub min_size: Option<u64>,
    pub min_ratio: Option<f32>,
}

impl Default for Config {
    fn default() -> Self {
        Self { glob: Some(String::from("**/*.png")), min_size: Some(0), min_ratio: Some(1.0) }
    }
}

fn main() -> Result<(), Error> {
    #[rustfmt::skip]
        let matches = App::new("PNG Cleaner")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(false)
            .index(1))
        .arg(Arg::with_name("Text")
            .short("t")
            .long("text")
            .help("Generates m file")
            .multiple(true)
            .takes_value(false))
        .arg(Arg::with_name("Binary")
            .short("b")
            .long("binary")
            .help("Generates wxf file")
            .multiple(true)
            .takes_value(false))
        .arg(Arg::with_name("Compress")
            .short("c")
            .long("compress")
            .help("Generates mx file")
            .multiple(true)
            .takes_value(false))
        .arg(Arg::with_name("File Format")
            .short("f")
            .long("format")
            .value_name("Format")
            .help("Sets the input file format")
            .takes_value(true))
        .get_matches();
    let cfg: Config = toml::from_slice(&fs::read("pngc.toml")?)?;
    println!("{:?}", cfg);

    let mut dir = "tests".to_string();
    dir.push_str("/*.png");
    for entry in glob(&dir)? {
        let path = &entry?.to_path_buf();
        let (info, _) = png::Decoder::new(File::open(path)?).read_info()?;
        let size = fs::metadata(path)?.st_size();
        let ratio = size as f32 / estimate_size(&info);
        if size > cfg.min_size || ratio > cfg.min_ratio {
            println!("{:?}", PNG { path: Box::from(path.to_str()?), size, ratio });
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

#[cfg(test)]
mod test {
    use std::{fs, fs::File, os::macos::fs::MetadataExt};

    use glob::glob;

    use crate::{
        Error,
        utils::{estimate_size, PNG},
    };

    #[test]
    fn test() -> Result<(), Error> {}
}
