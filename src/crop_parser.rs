use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use crate::dto::{Crop, GrowthEvent, GrowthStage};
use chrono::prelude::*;


fn read_file(file: &str) -> Result<File, std::io::Error> {
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

fn extract_crops(file: &str) -> Result<Vec<Crop>, Box<dyn Error>> {
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
    let crops = extract_crops("test_data/crops.csv").unwrap_or_else(|err| panic!("Error: {}", err));
    println!("{:?}", crops);

    for crop in crops {
        let count = group.entry(crop.verbose_name).or_insert(0);
        *count += 1;
    }

    println!("{:?}", group);
}

pub fn split_a_crop() {
    let mut crops = extract_crops("test_data/crops.csv").unwrap_or_else(|err| panic!("Error: {}", err));
    println!("Length of crops before split: {}", crops.len());
    
    let mut first_crop = crops.pop().unwrap();
    let splits = Crop::split(&mut first_crop, 3);

    println!("Length of splits: {}", splits.len());
    println!("Splits: {:?}", splits);
    println!("----------------------------------------");
    println!("Length of crops after split: {}", crops.len());
    let x_crop = first_crop.simulate_growth();
    println!("First crop: {:?}", x_crop);
    println!("New split_size of first_crop: {:?}", x_crop.split_size);
    println!("{:?}", x_crop.current_stage.unwrap_or_default());
}
