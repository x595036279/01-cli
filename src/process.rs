use std::fs;

use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::opts::OutputFormat;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    pub name: String,
    pub position: String,
    #[serde(rename = "DOB")]
    pub dob: String,
    pub nationality: String,
    #[serde(rename = "Kit Number")]
    pub kit: u8,
}
pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    // 1
    // let records = reader
    //     .deserialize()
    //     .map(|record| record.unwrap())
    //     .collect::<Vec<Player>>();
    // println!("{:#?}", records);

    // 2
    // let mut ret = Vec::with_capacity(128);
    // for result in reader.deserialize() {
    //     let record: Player = result?;
    //     ret.push(record);
    // }
    // let json = serde_json::to_string_pretty(&ret)?;
    // fs::write(output, json)?;

    //3
    let headers = reader.headers()?.clone();
    let mut ret = Vec::with_capacity(128);
    for result in reader.records() {
        let record = result?;
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        // println!("{:?}", json_value);
        ret.push(json_value);
    }
    let content = match format {
        OutputFormat::Json => serde_json::to_string(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };
    fs::write(output, content)?;

    Ok(())
}
