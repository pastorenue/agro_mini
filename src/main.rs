mod crop_parser;
mod dto;

use crop_parser::extract_crops;


fn main() {
    let _ = extract_crops("crop.csv");
}
