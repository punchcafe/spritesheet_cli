use std::collections::HashMap;
use std::vec::Vec;
use std::fs;
use std::cmp;
use std::path::Path;
use regex::Regex;
use image::{RgbImage, GenericImage};

#[derive(Debug)]
struct AnimationCell {
    animation_name: String,
    cell_number: u8,
    file_path: String
}

fn render_cell(buffer: &mut RgbImage, cell: &AnimationCell) -> () {
    let image = image::open(cell.file_path.to_string())
    .expect("aaah!")
    .to_rgb8();
    let cell_number: u32 = cell.cell_number.into();
    let x = (cell_number - 1) * image.width() ;
    buffer.copy_from(&image, x, 1).expect("aaah!");
}

fn render_result(cells: HashMap<String, Vec<AnimationCell>>) -> () {
    let mut max_size: u32 = 0;
    let mut sample_size = (0,0);

    for (_key, value) in &cells {
        max_size = cmp::max(value.len() as u32, max_size);
        sample_size = image::image_dimensions(value.get(0)
            .expect("aah")
            .file_path
            .to_owned())
        .expect("aaaah!");
    }

    let mut canvas = RgbImage::new(sample_size.0 * max_size, sample_size.1 + 10);
    for (_key, value) in cells {
        for cell in value {
            println!("Rendering individual");
            render_cell(&mut canvas, &cell);
            canvas.save("./sample_output.png").expect("aaah!");
        }
    }    
}

fn search(directory: &str, sheet_name: &str) -> HashMap<String, Vec<AnimationCell>> {
    let pattern = regex_expression(sheet_name);
    let entries = recursive_search(directory.to_string(), &pattern);
    let width = entries.iter()
        .map(|entry| entry.cell_number)
        .max()
        .expect("No entries found");
    let result_map = collect_as_map(entries);
    println!("{:?}", result_map);
    result_map
}



fn collect_as_map(animation_cells: Vec<AnimationCell>) -> HashMap<String, Vec<AnimationCell>> {
    let mut result_map : HashMap<String, Vec<AnimationCell>>= HashMap::new();
    for cell in animation_cells {
        let cells =  match result_map.get_mut(&cell.animation_name) {
            Some(cells) => cells,
            None => {
                result_map.insert(cell.animation_name.clone(), Vec::new());
                result_map.get_mut(&cell.animation_name).expect("unexpected error")
            }
        };
        cells.push(cell);
        cells.sort_by_key(|cell| cell.cell_number);
    }
    result_map
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
    render_result(search("./sample", "skater_base"));
}
