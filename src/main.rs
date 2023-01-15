use std::collections::HashMap;
use std::vec::Vec;
use std::fs;
use std::path::Path;
use regex::Regex;

#[derive(Debug)]
struct AnimationCell {
    animation_name: String,
    cell_number: u8,
    file_path: String
}

fn search(directory: &str, sheet_name: &str) -> HashMap<String, Vec<AnimationCell>> {
    let pattern = regex_expression(sheet_name);
    let entries = recursive_search(directory.to_string(), &pattern);
    print!("{:?}", entries);
    HashMap::new()
}

fn recursive_search(path_str: String, valid_file_pattern: &Regex) -> Vec<AnimationCell> {
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
        .collect::<Vec<AnimationCell>>()
    } else {
        if valid_file_pattern.is_match(&path_str) {
            let cap = valid_file_pattern.captures_iter(&path_str)
                .next()
                .expect("Couldn't find a capture group");
            let animation_name = &cap[1];
            let cell_number :u8 = cap[2].parse::<u8>().unwrap();
            vec![AnimationCell{
                animation_name: animation_name.to_string(), 
                file_path: path_str, 
                cell_number
            }]
        } else {
            vec![]
        }
    }
}

fn regex_expression(sheet_name: &str) -> Regex {
    let pattern = format!("{sheet_name}\\.([^\\.]+)\\.(\\d+)\\.(png)");
    println!("pattern: {pattern}");
    Regex::new(&pattern[..]).unwrap()
}

fn main() {
    search("./sample", "skater_base");
}
