use super::CsvOpts;

pub fn process_csv(opts: CsvOpts) -> String {
    format!("Processing CSV file: {}", opts.input)
}
