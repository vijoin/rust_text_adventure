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

    impl StoryRow{

        fn new(row: StringRecord) -> StoryRow{
            let health: i32 = StoryRow::get_column_text(&row, 3).parse().unwrap_or(0);
            return StoryRow {
                row_type: StoryRow::get_column_text(&row, 0),
                tag: StoryRow::get_column_text(&row, 1),
                text: StoryRow::get_column_text(&row, 2),
                health: health
            }
        }

        fn get_column_text(row: &StringRecord, column_number: usize) -> String {
            return row.get(column_number).unwrap().trim().to_string();
        }
    }
        
        
    let mut story_data: Vec<StoryRow> = vec![]; 
    let content = fs::read_to_string(FILENAME).unwrap();
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());

    for result in rdr.records() {
        let result = result.unwrap();
        let row_story = StoryRow::new(result);
        story_data.push(row_story);
        // println!("StoryRow: {:?}", row_story);
    }

    println!("Whole history: {:?}", story_data);
    
}
