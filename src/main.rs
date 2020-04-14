#![feature(try_trait)]
extern crate clap;

use clap::{App, Arg};

pub use error::Error;

use crate::utils::{PNG, write_to_file};

mod error;
pub mod utils;

fn main() -> Result<(), Error> {
    #[rustfmt::skip]
        let matches = App::new("PNG Cleaner")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(true)
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
    Ok(())
}


#[cfg(test)]
mod test {
    use glob::glob;

    use crate::Error;

    #[test]
    fn test() -> Result<(), Error> {
        let mut dir = "tests".to_string();
        dir.push_str("/*.png");
        for entry in glob(&dir)? {
            println!("{}", entry?.display());
        }
        Ok(())
    }
}

