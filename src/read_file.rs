use std::fs::File;


fn get_file() {
    let file_content = File::open("file.txt");
    let actual_res = match file_content {
        Ok(content) => content,
        Err(_) => {
            File::create("file.txt").unwrap_or_else(|err| {
                panic!("Error: {}", err)
            })
        }
    };

    println!("{:?}", actual_res);
}

fn read_file() {
    let file_content = File::open("file.txt");
    let mut actual_res = match file_content {
        Ok(content) => content,
        Err(_) => {
            File::create("file.txt").unwrap_or_else(|err| {
                panic!("Error: {}", err)
            })
        }
    };

    let mut content = String::new();
    actual_res.read_to_string(&mut content).expect(
        "Error reading file"
    );

    println!("{:?}", content);
}

fn read_with_simpler_expression() -> Result<File, std::io::Error> {
    let file_content = File::open("file.txt")?;

    Ok(file_content)
}

fn complete_read_ops() -> Result<String, std::io::Error> {
    let mut content = String::new();
    let _ = read_with_simpler_expression()?.read_to_string(&mut content);

    Ok(content)
}
