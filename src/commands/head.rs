use std::process;

pub use crate::helpers;

pub fn head(args: Vec<String>) {
    let default_number_of_lines = 10;
    let mut files: Vec<String> = args[0..].to_vec();
    let mut option: Option<String> = None;

    if args.len() > 0 && args[0].starts_with("-") {
        option = Some(args[0].clone());
        files = args[2..].to_vec();
    }

    match option {
        Some(_) => {
            let num_of_lines_to_read: u16 = match args[1].parse::<u16>() {
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
