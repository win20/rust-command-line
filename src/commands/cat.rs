use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// [[ Options to add ]]
// -n    show line numbers
// -E    show $ at the end of each line
// -s    suppress repeated blank lines

#[derive(Debug)]
pub enum Option {
    ShowLineNumber,
    ShowEndOfLine,
    SuppressRepeatedBlankLines,
    NoOption,
}

impl Option {
    fn as_option(option_str: &str) -> Result<Option, &str> {
        match option_str {
            "-n" => Ok(Option::ShowLineNumber),
            "-E" => Ok(Option::ShowEndOfLine),
            "-s" => Ok(Option::SuppressRepeatedBlankLines),
            "" => Ok(Option::NoOption),
            _ => Err("Error: unknown option provided"),
        }
    }
}

pub fn cat(args: Vec<String>) {
    if args.len() == 0 {
        panic!("Please provide at least one argument");
    }

    if args[0].starts_with("-") {
        let lines = concat_files(args[1..].to_vec());
        let option = Option::as_option(args[0].as_str()).unwrap();

        match option {
            Option::ShowLineNumber => {
                for (idx, line) in lines.iter().enumerate() {
                    println!("{} {}", idx, line);
                }
            }
            Option::ShowEndOfLine => {
                for line in lines {
                    println!("{}$", line);
                }
            }
            Option::SuppressRepeatedBlankLines => {
                let mut is_previous_line_blank = false;
                let mut new_output = Vec::from([]);

                for line in lines {
                    // if is_previous_line_blank && line == "" {
                    //     println!("blank");
                    // } else {
                    //     is_previous_line_blank = false;
                    //     new_output.push(line);
                    // }

                    if !(is_previous_line_blank && line == "") {
                        is_previous_line_blank = false;
                        new_output.push(line.clone());
                    }

                    if line == "" {
                        is_previous_line_blank = true;
                    }
                }

                for line in new_output {
                    println!("{}", line);
                }
            }
            _ => {}
        }
    } else {
        let lines = concat_files(args[0..].to_vec());

        for line in lines {
            println!("{}", line);
        }
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
