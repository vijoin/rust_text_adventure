use csv::{ReaderBuilder, StringRecord};
use std::{fs};

const FILENAME: &str = "history.csv";

fn main() {
    let content = fs::read_to_string(FILENAME).unwrap();
    // println!("{}", content)

    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());

    for result in rdr.records() {
        let result = result.unwrap();
        println!("Texto: {}", get_column_text(result, 2));
    }

}


fn get_column_text(result: StringRecord, column_number: usize) -> String {
    return result.get(column_number).unwrap().trim().to_string();
}
