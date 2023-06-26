use std::process;

pub use crate::helpers;

pub fn head(args: Vec<String>) {
    let default_number_of_lines = 10;
    let files: Vec<String>;
    let mut option: Option<String> = None;

    let mut option_index = args.len();
    for i in 0..args.len() {
        if args[i].starts_with("-") {
            option_index = i;
            option = Some(args[option_index].clone());
        }
    }
    files = args[0..option_index].to_vec();

    match option {
        Some(_) => {
            let num_of_lines_to_read: u16 = match args[option_index + 1].parse::<u16>() {
                Ok(int) => int - 1,
                Err(e) => {
                    println!("Error: {}", e);
                    process::exit(1);
                }
            };

            for file in files {
                println!();
                println!("==> {} <==", file);
                match helpers::read_number_of_lines(&file, num_of_lines_to_read) {
                    Ok(lines) => {
                        for line in lines {
                            println!("{}", &line);
                        }
                    }
                    Err(e) => println!("{}", e),
                }
            }
        }
        None => {
            for file in files {
                println!();
                println!("==> {} <==", file);
                match helpers::read_number_of_lines(&file, default_number_of_lines) {
                    Ok(lines) => {
                        for line in lines {
                            println!("{}", &line);
                        }
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
        }
    }
}
