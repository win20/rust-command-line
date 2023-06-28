use crate::helpers;

pub fn tail(args: Vec<String>) {
    let default_number_of_lines = 10;
    let files: Vec<String> = args;

    // let lines = helpers::read_number_of_lines_in_reverse("test.txt", 2);

    for file in files {
        println!();
        match helpers::read_number_of_lines_in_reverse(&file, 1) {
            Ok(lines) => {
                for line in lines {
                    println!("{}", &line);
                }
            }
            Err(e) => println!("{}", e),
        }
    }

    // for line in lines.unwrap().iter().rev() {
    //     println!("{}", line.as_str());
    // }
}
