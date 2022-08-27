use csv::{ReaderBuilder, StringRecord};
use std::{fs};

const FILENAME: &str = "history.csv";

fn main() {

    #[derive(Debug)]
    struct StoryRow {
        row_type: String,
        tag: String,
        text: String,
        health: i32
    }

    let mut story_data: Vec<StoryRow> = vec![]; 

    let content = fs::read_to_string(FILENAME).unwrap();
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());

    for result in rdr.records() {
        let result = result.unwrap().clone();
        let health: i32 = get_column_text(&result, 3).parse().unwrap_or(0);

        let row_story = StoryRow {
            row_type: get_column_text(&result, 0),
            tag: get_column_text(&result, 1),
            text: get_column_text(&result, 2),
            health: health
        };

        story_data.push(row_story);
        // println!("StoryRow: {:?}", row_story);
    }

    println!("Whole history: {:?}", story_data);
    

}


fn get_column_text(result: &StringRecord, column_number: usize) -> String {
    return result.get(column_number).unwrap().trim().to_string();
}
