use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;


fn read_file(file: &str) -> Result<File, std::io::Error> {

    let file_content = File::open(file);
    let mut actual_res = match file_content {
        Ok(content) => content,
        Err(_) => {
            File::create(file).unwrap_or_else(|err| {
                panic!("Error: {}", err)
            })
        }
    };

    Ok(actual_res)
}

pub fn extract_crops(file: &str) -> Result<(), Box<dyn Error>> {
    let mut file_content = read_file(file)?;
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file_content);

    for result in reader.records() {
        let line = result?;

        // Process the line in a crop dto
        println!("{:?}", line);
    }

    Ok(())
}

fn parse_crops() {
    // extract_crops("file.csv").unwrap();
}