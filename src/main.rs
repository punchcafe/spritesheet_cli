use std::collections::HashMap;
use std::vec::Vec;
use std::fs;
use std::cmp;
use std::path::Path;
use regex::Regex;
use image::{RgbImage, GenericImage};
use image::imageops::FilterType;
use argparse::{ArgumentParser, Store};

#[derive(Debug)]
struct AnimationCell {
    animation_name: String,
    cell_number: u8,
    file_path: String
}

type SheetDescriptor = HashMap<String, Vec<AnimationCell>>;

#[derive(Debug)]
struct RunConfig {
    to_width: u32,
    cwd: String,
    sheet_name: String,
    out: String
}

struct SheetDetails {
    tile_width: u32,
    tile_height: u32,
    total_rows: u32,
    max_columns: u32
}

fn sheet_details(cells: &SheetDescriptor) -> SheetDetails {
    let mut max_size: u32 = 0;
    let mut sample_size = (0,0);

    for (_key, value) in cells {
        max_size = cmp::max(value.len() as u32, max_size);
        sample_size = image::image_dimensions(value.get(0)
            .expect("aah")
            .file_path
            .to_owned())
        .expect("aaaah!");
    }

    let number_of_rows: u32 = cells.keys().len().try_into().expect("aaah!");
    SheetDetails{
        tile_width: sample_size.0,
        tile_height: sample_size.1,
        max_columns: max_size,
        total_rows: number_of_rows
    }
}

// Implement for Sheet Details
fn apply_scale(details: SheetDetails, to_width: u32) -> SheetDetails {
    let divide_by = details.tile_width / to_width;
    SheetDetails{
        tile_width: to_width,
        tile_height: details.tile_height / divide_by,
        ..details
    }
}

fn new_rgb_canvas(details: &SheetDetails) -> RgbImage {
    RgbImage::new(details.tile_width * details.max_columns, details.tile_height * details.total_rows)
}

fn render_cell(buffer: &mut RgbImage, cell: &AnimationCell, row_number: u32, sheet_details: &SheetDetails) -> () {
    let image = image::open(cell.file_path.to_string())
    .expect("aaah!")
    .to_rgb8();

    let image = image::imageops::resize(&image, sheet_details.tile_width, sheet_details.tile_height, FilterType::CatmullRom);
    let cell_number: u32 = cell.cell_number.into();
    let x = (cell_number - 1) * sheet_details.tile_width;
    let y = (row_number - 1) * sheet_details.tile_height;
    buffer.copy_from(&image, x, y).expect("aaah!");
}

fn render_result(cells: HashMap<String, Vec<AnimationCell>>, config: &RunConfig) -> () {

    let sheet_details = sheet_details(&cells);
    let sheet_details = apply_scale(sheet_details, config.to_width);

    let mut row_number = 1;

    let mut canvas = new_rgb_canvas(&sheet_details);

    for (_key, value) in cells {
        for cell in value {
            render_cell(&mut canvas, &cell, row_number, &sheet_details);
        }
        row_number = row_number + 1;
    }    
    canvas.save(config.out.as_str()).expect("aaah!");
}

fn search(directory: &str, sheet_name: &str) -> HashMap<String, Vec<AnimationCell>> {
    let pattern = regex_expression(sheet_name);
    let entries = recursive_search(directory.to_string(), &pattern);
    let result_map = collect_as_map(entries);
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

fn run_config() -> RunConfig {
    let mut width: String = "512".to_string();
    let mut sheet_name = String::new();
    let cwd = std::env::current_dir().expect("aaaah!").to_owned().to_str().expect("aaah").to_owned();
    let mut out = cwd.clone();
    out.push_str("/");
    out.push_str(&sheet_name);
    out.push_str(".png");

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Creates a sprite sheet for all sprites following naming conventions.");
        ap.refer(&mut width)
            .add_option(&["-w", "--width"], Store,
            "Scales all tiles to given width, preserving aspect ratio.");

        ap.refer(&mut out)
            .add_option(&["-o", "--out"], Store,
            "Output file name.");
        ap.refer(&mut sheet_name)
            .add_argument("sheet_name", Store, "The name of the sprite sheet.")
            .required();
        ap.parse_args_or_exit();
    }
    RunConfig{
        to_width: width.parse::<u32>().expect("Invalid width value"),
        cwd,
        sheet_name,
        out
    }
}

fn main() {
    let config = run_config();
    render_result(search(&config.cwd, &config.sheet_name), &config);
}
