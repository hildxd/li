use std::fs;

use anyhow::Result;

use crate::{CsvOpts, Format};

pub fn process_csv(opts: CsvOpts) -> Result<()> {
    let mut reader = csv::Reader::from_path(opts.input)?;
    let headers = reader.headers()?.clone();
    let ret = reader
        .records()
        .map(|result| {
            let record = result?;
            let json_value = headers
                .iter()
                .zip(record.iter())
                .collect::<serde_json::Value>();
            Ok(json_value)
        })
        .collect::<Result<Vec<serde_json::Value>>>()?;
    let content = match opts.format {
        Format::Json => serde_json::to_string_pretty(&ret)?,
        Format::Yaml => serde_yaml::to_string(&ret)?,
    };
    fs::write(opts.output, content)?;
    Ok(())
}
