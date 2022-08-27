use csv::{ReaderBuilder, StringRecord};
use std::{fs};

const FILENAME: &str = "history.csv";

fn main() {

    #[derive(Debug)]
    struct RowStory {
        row_type: String,
        tag: String,
        text: String,
        health: i32
    }

    let content = fs::read_to_string(FILENAME).unwrap();
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());

    for result in rdr.records() {
        let result = result.unwrap().clone();

        let row_story = RowStory {
            row_type: get_column_text(&result, 0),
            tag: get_column_text(&result, 1),
            text: get_column_text(&result, 2),
            health: 0
        };
        println!("RowStory: {:?}", row_story);
    }

}


fn get_column_text(result: &StringRecord, column_number: usize) -> String {
    return result.get(column_number).unwrap().trim().to_string();
}
