use csv::Reader;
use serde::Deserialize;
use std::error::Error;
use std::fmt;

#[derive(Debug, Deserialize, Clone)]
pub struct Record {
    location: String,
    size: String,
    feature: String,
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "CSV_Record {{\n\tGeographic location: {},\n\tLength: {},\n\tGenomic feature: {} \n}}",
            self.location, self.size, self.feature
        )
    }
}

pub fn read_csv(filename: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(filename)?;
    let mut result = Vec::new();

    for record in rdr.deserialize() {
        let instance: Record = record?;

        result.push(instance.clone());

        println!("\n{}", instance);
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
            assert!(!record.size.is_empty());
            assert!(!record.feature.is_empty());
        }
    }
}
