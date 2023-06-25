use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_number_of_lines(file: &String, num_of_lines: u16) -> io::Result<Vec<String>> {
    let mut lines = read_lines(file)?;
    let mut lines_to_return: Vec<String> = Vec::new();

    for _i in 0..=num_of_lines {
        lines_to_return.push(lines.next().unwrap().unwrap());
    }

    Ok(lines_to_return)
}
