use std::io::{self, BufRead};

use rev_buf_reader::RevBufReader;

pub fn tail() {
    // let default_number_of_lines = 10;
    // let files: Vec<String>;
    // let mut option: Option<String> = None;

    let data = "a\nb\nc";
    let inner = io::Cursor::new(&data);
    let reader = RevBufReader::new(inner);
    let mut lines = reader.lines();

    for line in lines {
        println!("{}", line.unwrap());
    }
}
