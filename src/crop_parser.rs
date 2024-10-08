#![allow(dead_code)]

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use crate::dto::{Crop, GrowthStage};


pub fn read_file(file: &str) -> Result<File, Box<dyn Error>> {
    let file_content = File::open(file);
    let actual_res = match file_content {
        Ok(content) => content,
        Err(_) => {
            File::create(file).unwrap_or_else(|err| {
                panic!("Error: {}", err)
            })
        }
    };
    Ok(actual_res)
}

pub fn extract_content(file: &str) -> Vec<HashMap<String, String>> {
    let actual_res = read_file(file).unwrap_or_else(|err| panic!("Error: {}", err));
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(actual_res);

    let mut records = Vec::new();
    let headers = reader.headers().unwrap().clone();
    for result in reader.records() {
        let line = result.unwrap();
        let mut map = HashMap::new();
        for (header, value) in headers.iter().zip(line.iter()) {
            map.insert(header.to_string(), value.to_string());
        }
        records.push(map);
    }

    records
}

pub fn extract(file: &str) -> Result<Vec<Crop>, Box<dyn Error>> {
    let file_content = read_file(file)?;
    let mut crops: Vec<Crop> = Vec::new();
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file_content);

    for result in reader.deserialize() {
        let mut line: Crop = result?;
        line.current_stage = Some(GrowthStage::Seed);
        
        // Process the line in a crop dto
        crops.push(line);
    }

    Ok(crops)
}

pub fn group_crops() {
    let mut group: HashMap<String, i32> = HashMap::new();
    let crops = extract("test_data/crops.csv").unwrap_or_else(|err| panic!("Error: {}", err));
    println!("{:?}", crops);

    for crop in crops {
        let count = group.entry(crop.verbose_name).or_insert(0);
        *count += 1;
    }

    println!("{:?}", group);
}

pub fn split_a_crop() {
    let mut crops = extract("test_data/crops.csv").unwrap_or_else(|err| panic!("Error: {}", err));
    println!("Length of crops before split: {}", crops.len());
    
    let mut first_crop = crops.pop().unwrap();
    let splits = Crop::split(&mut first_crop, 3);

    println!("Length of splits: {}", splits.len());
    println!("Splits: {:?}", splits);
    println!("----------------------------------------");
    println!("Length of crops after split: {}", crops.len());
    let x_crop = first_crop.simulate_growth();
    println!("First crop: {:?}", x_crop);
}
