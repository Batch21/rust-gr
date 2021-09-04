use csv;
use serde::Deserialize;
use serde_json;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct Record {
    date: String,
    rainfall: f64,
    pet: f64,
}

#[derive(Debug)]
pub struct Data {
    pub dates: Vec<String>,
    pub rainfall: Vec<f64>,
    pub pet: Vec<f64>,
}

impl Data {
    fn new() -> Data {
        Data {
            dates: Vec::new(),
            rainfall: Vec::new(),
            pet: Vec::new(),
        }
    }
}

pub fn load_csv_data(path: PathBuf) -> Result<Data, Box<dyn Error>> {
    let mut data = Data::new();

    let mut reader = csv::Reader::from_path(path).unwrap();

    for record in reader.deserialize() {
        let record: Record = record.unwrap();
        data.dates.push(record.date);
        data.rainfall.push(record.rainfall);
        data.pet.push(record.pet);
    }

    Ok(data)
}

#[derive(Debug, Deserialize)]
pub struct Parameters {
    pub production_store_capacity: f64,
    pub exchange_coefficient: f64,
    pub routing_store_capacity: f64,
    pub days: f64,
    pub production_store_content: f64,
    pub routing_store_content: f64,
}

impl Parameters {
    pub fn new(
        production_store_capacity: f64,
        exchange_coefficient: f64,
        routing_store_capacity: f64,
        days: f64,
        production_store_content: f64,
        routing_store_content: f64,
    ) -> Parameters {
        Parameters {
            production_store_capacity: production_store_capacity,
            exchange_coefficient: exchange_coefficient,
            routing_store_capacity: routing_store_capacity,
            days: days,
            production_store_content: production_store_content,
            routing_store_content: routing_store_content,
        }
    }
}

pub fn load_parameter_data(path: PathBuf) -> Result<Parameters, Box<dyn Error>> {
    let file = fs::read_to_string(path).expect("Unable to read file");
    let params: Parameters = serde_json::from_str(&file).unwrap();
    Ok(params)
}

pub fn save_flow(
    dates: Vec<String>,
    flows: Vec<f64>,
    output_file: PathBuf,
) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(output_file)?;

    // TODO add headers

    for (date, flow) in dates.iter().zip(flows) {
        wtr.write_record(&[date, &flow.to_string()])?;
    }

    wtr.flush()?;

    Ok(())
}
