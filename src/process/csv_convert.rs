use std::fs;

use crate::options::Format;
use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub fn process_csv(input: &str, output: &str, format: Format) -> Result<()> {
    let mut reader = Reader::from_path(input).unwrap();

    let headers = reader.headers().unwrap().clone();

    let mut result = Vec::with_capacity(128);
    reader.records().for_each(|record| {
        let record = record.unwrap();
        let value = headers.iter().zip(record.iter()).collect::<Value>();
        result.push(value);
    });

    let result = match format {
        Format::Json => serde_json::to_string_pretty(&result)?,
        Format::Toml => toml::to_string_pretty(&result)?,
        Format::Yaml => serde_yaml::to_string(&result)?,
    };

    fs::write(output, result)?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Player {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}
