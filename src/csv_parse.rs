extern crate csv;
extern crate serde;

use std::error::Error;
use std::process;

use csv::Reader; 
use csv::StringRecord;

pub fn read_sensor_data(path : String)  -> Result<Vec<StringRecord>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(path)?;
    let mut csvdata : Vec<StringRecord> = Vec::new();
    for result in rdr.records() {
        let record = result?;
        csvdata.push(record);
    }
    Ok(csvdata)
}
