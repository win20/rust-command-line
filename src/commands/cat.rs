use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// [[ Options to add ]]
// -n    show line numbers
// -E    show $ at the end of each line
// -s    suppress repeated blank lines

pub fn cat(files: Vec<String>) {
    for file in files {
        match read_lines(file) {
            Ok(lines) => {
                for line in lines {
                    if let Ok(l) = line {
                        println!("{}", l);
                    }
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// pub fn cat_line_numbers(file) -> String {
//     // format!("{} {}", line_number, line_text)
//
//     println!()
// }
