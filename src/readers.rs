use csv::Reader;
use serde::Deserialize;
use std::error::Error;
use std::fmt;

#[derive(Debug, Deserialize)]
struct CsvRecord {
    location: String,
    size: String,
    feature: String,
}

#[derive(Debug, Clone)]
pub struct CustomRecord {
    location: String,
    size: u64,
    feature: String,
}

impl fmt::Display for CustomRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "CSV_Record {{\n\tGeographic location: {},\n\tLength: {},\n\tGenomic feature: {} \n}}",
            self.location, self.size, self.feature
        )
    }
}

#[allow(clippy::single_char_pattern)]
pub fn parse_record_size(size: &str) -> Result<u64, std::num::ParseIntError> {
    let no_underscores = size.replace("_", "");

    no_underscores.parse::<u64>()
}

pub fn read_csv(filename: &str) -> Result<Vec<CustomRecord>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(filename)?;
    let mut result = Vec::new();

    for record in rdr.deserialize() {
        let instance: CsvRecord = record?;

        let parsed_size = parse_record_size(&instance.size)
            .map_err(|_| "Failed to parse size for custom record")?;

        let custom_record = CustomRecord {
            location: instance.location,
            size: parsed_size,
            feature: instance.feature,
        };

        result.push(custom_record.clone());

        println!("\n{}", custom_record);
    }

    Ok(result)
}

#[cfg(test)]
mod test_read_csv {
    use super::*;

    #[test]
    fn test_read_csv() {
        let filename = "data/example.csv";
        let records = read_csv(filename);

        assert!(records.is_ok());

        let records = records.unwrap();

        assert!(records.len() == 3);

        for record in records {
            assert!(!record.location.is_empty());
            assert!(record.size > 0);
            assert!(!record.feature.is_empty());
        }
    }
}
