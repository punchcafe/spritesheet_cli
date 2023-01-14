use std::collections::HashMap;
use std::vec::Vec;
use std::fs;
use std::path::Path;


struct AnimationCell {
    sprite_sheet_name: String,
    animation_name: String,
    cell_number: u8
}

fn search(directory: &str, sheet_name: &str) -> HashMap<String, Vec<AnimationCell>> {
    let entries = recursive_search(directory.to_string());
    print!("{:?}", entries);
    HashMap::new()
}

fn recursive_search(path_str: String) -> Vec<String> {
    let path = Path::new(&path_str);
    if path.is_dir() {
        fs::read_dir(path)
        .expect("Failed to load")
        .flat_map(|res| 
        recursive_search(
            res
            .expect("aaah")
            .path()
            .to_str()
            .expect("failed")
            .to_string()
        ))
        .collect::<Vec<String>>()
    } else {
        vec![path_str]
    }
}

fn main() {
    search("./sample", "hello");
}
