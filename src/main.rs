use csv::{ReaderBuilder, StringRecord};
use std::{fs, collections::HashMap, io::Read};

const FILENAME: &str = "history.csv";
const FIRST_TAG: &str = "LUZ";

#[derive(Debug)]
struct StoryRow {
    row_type: String,
    tag: String,
    text: String,
    health: i32,
    options: Vec<StoryRow>,
}

impl StoryRow{

    fn new(row: StringRecord) -> StoryRow{
        let health: i32 = StoryRow::get_column_text(&row, 3).parse().unwrap_or(0);
        return StoryRow {
            row_type: StoryRow::get_column_text(&row, 0),
            tag: StoryRow::get_column_text(&row, 1),
            text: StoryRow::get_column_text(&row, 2),
            health: health,
            options: vec![] 
        }
    }

    fn get_column_text(row: &StringRecord, column_number: usize) -> String {
        return row.get(column_number).unwrap().trim().to_string();
    }
}

fn init_story_data() -> HashMap<String, StoryRow> {
    let mut story_data: HashMap<String, StoryRow> = HashMap::new();
    
    let mut last_situation_tag = "".to_string();
     
    let content = fs::read_to_string(FILENAME).unwrap();
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());

    for result in rdr.records() {
        let result = result.unwrap();
        let row_story = StoryRow::new(result);

        if row_story.row_type == "SITUACION" {
            let row_story_tag = row_story.tag.clone();
            story_data.insert(row_story_tag.clone(), row_story);
            last_situation_tag = row_story_tag;
        } else if row_story.row_type == "OPCION" {
            if let Some(data) = story_data.get_mut(&last_situation_tag) {
                (*data).options.push(row_story);
            }

        }
    }
    return story_data
}

fn main() {
    let story_data = init_story_data();
    let mut current_tag = FIRST_TAG;

    //Game Loop
    let mut health = 100;
    loop {
        println!("Your health is at {}%", health);

        

        if let Some(data) = story_data.get(current_tag){
            println!("{}", data.text);

            for (idx, option) in data.options.iter().enumerate() {
                println!("[{}] {}", idx, option.text);
            }

            let mut selection = String::new();
            std::io::stdin().read_line(&mut selection).unwrap();
            let selection = selection.trim().parse().unwrap_or(99);

            if let Some(chosen_option) = &data.options.get(selection) {
                current_tag = &chosen_option.tag;
            }else{
                println!("Invalid Command!")
            }

            health += data.health;
            println!("");

        }else{
            break;
        }

        if health <= 0 {
            println!("You lost!");
            break;
        }

    }
        
    
}
