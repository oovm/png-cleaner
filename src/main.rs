#![feature(try_trait)]
extern crate clap;

mod error;
pub mod utils;

use crate::utils::{round_size, write_to_file};
use clap::{App, Arg};
use error::Error;

fn main() -> Result<(), Error> {
    #[rustfmt::skip]
        let matches = App::new("Wolfram Format Converter")
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
mod test{
    use std::fs::File;
    use crate::utils::round_size;

    #[test]
    fn test() {
        // The decoder is a build for reader and can be used to set various decoding options
        // via `Transformations`. The default output transformation is `Transformations::EXPAND
        // | Transformations::STRIP_ALPHA`.
        let decoder = png::Decoder::new(File::open("tests/A9EF29DF-306B-43D4-80CD-ED38F1028AA8.png").unwrap());
        let (info, mut reader) = decoder.read_info().unwrap();
        // Allocate the output buffer.
        println!("{:?}", round_size(&info));

        let mut buf = vec![0; info.buffer_size()];
        // Read the next frame. An APNG might contain multiple frames.
        reader.next_frame(&mut buf).unwrap();
        // Inspect more details of the last read frame.
        let in_animation = reader.info().frame_control.is_some();
    }


}

