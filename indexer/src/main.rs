extern crate regex;
extern crate rustc_serialize;

use rustc_serialize::json::{self, Json};
use std::io::prelude::*;

fn get_title_from_html(page: &str) -> &str {
    // We assue a particular text layout for now.
    // TODO use lazy_static
    let re = regex::Regex::new(r"<title>(.*)</title>").unwrap();
    return re.captures(page).unwrap().get(1).unwrap().as_str();
}

#[test]
fn test_get_title_from_html() {
    assert_eq!(get_title_from_html("<title>hi</title>"), "hi");
}

// Right now just return the title but in the future perhaps more.
fn get_data_from_recipe(path: &str) -> String {
    let mut file = std::fs::File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    get_title_from_html(&contents).to_string()
}

fn main() {
    let input_dir = std::env::args().nth(1).unwrap();
    let output_file_path = std::env::args().nth(2).unwrap();

    let mut entries = json::Array::new();

    for dir_entry_result in std::fs::read_dir(&input_dir).unwrap() {
        let path_bufer: std::path::PathBuf = dir_entry_result.unwrap().path();
        let path_str: String = path_bufer.to_str().unwrap().to_string();
        println!("Reading: {}", path_str);
        let mut entry = json::Object::new();
        let title = get_data_from_recipe(&path_str);
        entry.insert("name".to_string(), Json::String(title));
        entry.insert(
            "path".to_string(),
            Json::String(
                "recipes/".to_string() + path_bufer.file_name().unwrap().to_str().unwrap(),
            ),
        );
        entries.push(Json::Object(entry));
    }
    let index: Json = Json::Array(entries);
    let output_contents = json::encode(&index).unwrap();

    println!("Writing output file: {}", &output_file_path);
    let mut output_file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&output_file_path)
        .expect("Unable to open output file");
    output_file
        .write(&output_contents.into_bytes())
        .expect("Unable to write contents to output file");
}
