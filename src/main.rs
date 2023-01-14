use std::collections::HashMap;
use std::vec::Vec;


struct AnimationCell {
    sprite_sheet_name: String,
    animation_name: String,
    cell_number: u8
}

fn search(directory: &str, sheet_name: &str) -> HashMap<String, Vec<AnimationCell>> {
    HashMap::new()
}

fn main() {
    let hash_map = search("/hello/world", "my_sheet");
    println!("Hello, world!");
}
