use std::process;

use crate::helpers;

pub fn tail(args: Vec<String>) {
    let default_number_of_lines = 5;
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
        Some(opt) => {
            match opt.as_str() {
                "-n" => {
                    let num_of_lines_to_read: u16 = match args[option_index + 1].parse::<u16>() {
                        Ok(int) => int,
                        Err(e) => {
                            println!("Error: {}", e);
                            process::exit(1);
                        }
                    };

                    for file in files {
                        println!();
                        println!("==> {} <==", file);
                        match helpers::read_number_of_lines_in_reverse(&file, num_of_lines_to_read) {
                            Ok(lines) => {
                                for line in lines.iter().rev() {
                                    println!("{}", &line);
                                }
                            }
                            Err(e) => println!("{}", e),
                        }
                    }

                }
                _ => println!("Error: Command not found"),

            }
        }
        None => {
            for file in files {
                println!();
                println!("==> {} <==", file);
                match helpers::read_number_of_lines_in_reverse(&file, default_number_of_lines) {
                    Ok(lines) => {
                        for line in lines.iter().rev() {
                            println!("{}", &line);
                        }
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
        }
    }
}
