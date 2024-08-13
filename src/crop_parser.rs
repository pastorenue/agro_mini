use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use crate::dto::Crop;
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
        let line: Crop = result?;

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