pub use crate::helpers;

pub fn head(args: Vec<String>) {
    let mut option: Option<String> = None;
    let file = &args[0];
    let default_number_of_lines = 10;

    if args.len() > 1 && args[1].starts_with("-") {
        option = Some(args[1].clone());
    }

    match option {
        Some(_) => {
            let num_of_lines_to_read: u16 = args[2].parse().unwrap();
            match helpers::read_number_of_lines(file, num_of_lines_to_read) {
                Ok(lines) => {
                    for line in lines {
                        println!("{}", &line);
                    }
                }
                Err(e) => println!("{}", e),
            }
        }
        None => match helpers::read_number_of_lines(file, default_number_of_lines) {
            Ok(lines) => {
                for line in lines {
                    println!("{}", &line);
                }
            }
            Err(e) => println!("Error: {}", e),
        },
    }
}
