use std::fs;

#[doc = "get a file and return it (wow)"]
pub fn get_file(file_path: &str) -> String {
    let contents =
        fs::read_to_string(&file_path).expect(&format!("Couldn't read file {}", file_path));

    contents
}

#[doc = "gets a file and strips it of newlines (`\\n` and `\\r`) characters"]
pub fn get_line(file_path: &str) -> String {
    get_file(file_path).replace('\n', "").replace('\r', "")
}
