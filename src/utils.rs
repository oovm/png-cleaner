use crate::error::Error;
use png::{BitDepth, ColorType, OutputInfo};
use std::{fs::File, io::Write};

/*
pub fn parse_file(path: &str, format: Option<&str>) -> Result<WolframValue, Error> {
    let s = fs::read_to_string(path)?;
    let suffix = match format {
        Some(s) => s,
        None => path.split('/').last()?.split('.').last()?,
    };
    let format = match suffix.to_lowercase().as_str() {
        "yml" | "yaml" => SupportedFormat::YAML,
        "json" => SupportedFormat::JSON,
        _ => SupportedFormat::TOML,
    };
    println!("Parsing the file {} as {:?}", path, format);
    match format {
        SupportedFormat::JSON => parse_json(&s).map_err(|_| Error::ParseFailed),
        SupportedFormat::TOML => parse_toml(&s).map_err(|_| Error::ParseFailed),
        SupportedFormat::YAML => parse_yaml(&s).map_err(|_| Error::ParseFailed),
        SupportedFormat::Pickle => unimplemented!(),
    }
}
*/

pub fn round_size(info: &OutputInfo) -> u32 {
    let w = info.width;
    let h = info.height;
    let b = match info.bit_depth {
        BitDepth::One => 1,
        BitDepth::Two => 2,
        BitDepth::Four => 4,
        BitDepth::Eight => 8,
        BitDepth::Sixteen => 16,
    };
    let d = match info.color_type {
        ColorType::Grayscale => 1,
        ColorType::RGB => 3,
        ColorType::Indexed => 1,
        ColorType::GrayscaleAlpha => 1,
        ColorType::RGBA => 4,
    };
    return w * h * b * d;
}

pub fn write_to_file(path: &str, bytes: &[u8]) -> Result<(), Error> {
    println!("Generating {}", path);
    let mut file = File::create(path)?;
    file.write_all(bytes)?;
    Ok(())
}
