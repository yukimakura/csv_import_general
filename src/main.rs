extern crate csv_import_general;
use csv_import_general::csv_parse;


extern crate csv;
extern crate serde;

use std::error::Error;
use std::process;

use serde::{Deserialize};

use csv::Reader; 

enum SCANDATA {
    date,
    time,
    ir,
    lidar
}

fn main() {
    // let mut datas: Vec<csv_parse::SCANDATA> = Vec::new();
    let mut datas = csv_parse::read_sensor_data("sensor_data_600.csv".to_string()).unwrap();
    
    for item in &datas{
        println!("{:?}",item.get(SCANDATA::ir as usize).unwrap().parse::<i32>().unwrap());
    }
    
}
