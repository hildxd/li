use std::{fs::File, io::Read};

use anyhow::Result;
use base64::prelude::*;

use crate::Base64Format;

fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = match input {
        "-" => Box::new(std::io::stdin()),
        _ => Box::new(File::open(input)?),
    };
    Ok(reader)
}

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let encoded = match format {
        Base64Format::Standard => BASE64_STANDARD.encode(buf),
        Base64Format::Url => BASE64_URL_SAFE.encode(buf),
    };
    println!("{}", encoded);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();
    let decoded = match format {
        Base64Format::Standard => BASE64_STANDARD.decode(buf)?,
        Base64Format::Url => BASE64_URL_SAFE.decode(buf)?,
    };
    let decoded = String::from_utf8(decoded)?;
    println!("{}", decoded);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let input = "Cargo.toml";
        assert!(process_encode(input, Base64Format::Standard).is_ok());
    }

    #[test]
    fn test_decode() {
        let input = "fixtures/b64.txt";
        assert!(process_decode(input, Base64Format::Standard).is_ok());
    }
}
