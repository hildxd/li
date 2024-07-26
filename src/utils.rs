use anyhow::Result;
use std::path::Path;

pub fn verify_file(filename: &str) -> Result<String, &'static str> {
    // if input is "-" or file exists, return Ok
    match filename == "-" || Path::new(filename).exists() {
        true => Ok(filename.into()),
        false => Err("File does not exist"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("nonexistent"), Err("File does not exist"));
    }
}
