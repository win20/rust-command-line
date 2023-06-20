use std::fs;

// [[ Options to add ]]
// -n    show line numbers
// -E    show $ at the end of each line
// -s    suppress repeated blank lines

pub fn cat(files: Vec<String>) {
    for file in files {
        match fs::read_to_string(file) {
            Ok(contents) => println!("{}", contents),
            Err(e) => println!("Error: {}", e),
        }
    }
}
