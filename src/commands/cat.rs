use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// [[ Options to add ]]
// -n    show line numbers
// -E    show $ at the end of each line
// -s    suppress repeated blank lines

pub fn cat(files: Vec<String>, option: &String) {
    let lines = concat_files(files);

    match option.as_str() {
        "-n" => {
            for (idx, line) in lines.iter().enumerate() {
                println!("{} {}", idx, line);
            }
        }
        "" => {
            for line in lines {
                println!("{}", line);
            }
        }
        _ => println!("Error: option not recognized"),
    }
}

fn concat_files(files: Vec<String>) -> Vec<String> {
    let mut concat_lines: Vec<String> = Vec::from([]);
    for file in files {
        match read_lines(file) {
            Ok(lines) => {
                for line in lines {
                    if let Ok(l) = line {
                        concat_lines.push(l);
                    }
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }
    concat_lines
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
