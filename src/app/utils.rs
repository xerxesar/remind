use std::fs;


pub fn open_md_file(file_path: &str) -> String {
    return fs::read_to_string(file_path)
        .expect("File not found");
}