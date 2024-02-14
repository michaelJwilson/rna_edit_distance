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

/**
Reads a CSV file and returns a vector of `CustomRecord` instances.

# Arguments

* `filename` - A string slice that holds the name of the CSV file.

# Returns

* `Ok(Vec<CustomRecord>)` if the CSV file is successfully read and all records are successfully parsed.
* `Err(Box<dyn Error>)` if an error occurs while reading the CSV file or parsing a record.

# Errors

This function will return an error if the CSV file cannot be read (for example, if the file does not exist or the program does not have read permissions), or if a record cannot be parsed into a `CustomRecord` instance (for example, if a record does not have the expected number of fields or the `size` field cannot be parsed into a `u64`).

# Example

```
let filename = "data/example.csv";
let records = read_csv(filename);
assert!(records.is_ok());
```
*/
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
