use std::fs;

pub fn load_file(path: &str) -> String {
    let contents = fs::read_to_string(path).expect("Unable to read file");
    return contents;
}
