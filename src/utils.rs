use std::fs;

pub fn read_file(path: &str) -> String {
    let data = fs::read_to_string(path).expect("Unable to find file");
    data
}
