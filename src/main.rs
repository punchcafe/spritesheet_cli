use std::collections::HashMap;
use std::vec::Vec;
use std::fs;
use std::path::Path;
use regex::Regex;


struct AnimationCell {
    sprite_sheet_name: String,
    animation_name: String,
    cell_number: u8
}

fn search(directory: &str, sheet_name: &str) -> HashMap<String, Vec<AnimationCell>> {
    let pattern = regex_expression(sheet_name);
    let entries = recursive_search(directory.to_string(), &pattern);
    print!("{:?}", entries);
    HashMap::new()
}

fn recursive_search(path_str: String, valid_file_pattern: &Regex) -> Vec<String> {
    let path = Path::new(&path_str);
    if path.is_dir() {
        fs::read_dir(&path)
        .expect("Failed to load")
        .flat_map(|res| 
        recursive_search(
            res
            .expect("unexpected error")
            .path()
            .to_str()
            .expect("unexpected error")
            .to_string(),
            valid_file_pattern
        ))
        .collect::<Vec<String>>()
    } else {
        if valid_file_pattern.is_match(&path_str) {
            vec![path_str]
        } else {
            vec![]
        }
    }
}

fn regex_expression(sheet_name: &str) -> Regex {
    let pattern = format!("{sheet_name}\\.([^\\.])+\\.(\\d)+\\.(png)");
    println!("pattern: {pattern}");
    Regex::new(&pattern[..]).unwrap()
}

fn main() {
    search("./sample", "skater_base");
}
