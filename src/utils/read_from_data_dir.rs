use std::fs;

pub fn read_data(file_name: String) -> String {
    let path = format!(
        r"C:\Users\Danie\Desktop\Apps\author-background\src\data\{}",
        file_name
    );
    let content = fs::read_to_string(path).expect("Failed to read file");
    content
}
