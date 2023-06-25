pub use crate::helpers;

pub fn head(args: Vec<String>) {
    let mut option: Option<String> = None;
    let file = &args[0];

    if args.len() > 1 && args[1].starts_with("-") {
        option = Some(args[1].clone());
    }

    match option {
        Some(_) => {
            let num_of_lines_to_read: u16 = args[2].parse().unwrap();
            match helpers::read_lines(file) {
                Ok(mut lines) => {
                    let mut i = 0;
                    while i < num_of_lines_to_read {
                        let test = lines.next().unwrap().unwrap();
                        println!("{}", &test);
                        i += 1;
                    }
                }
                Err(e) => println!("Error: {}", e),
            }
        }
        None => match helpers::read_lines(file) {
            Ok(mut lines) => {
                let mut i = 0;
                while i < 10 {
                    let test = lines.next().unwrap().unwrap();
                    println!("{}", &test);
                    i += 1;
                }
            }
            Err(e) => println!("Error: {}", e),
        },
    }
}
