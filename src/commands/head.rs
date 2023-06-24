pub use crate::helpers;

pub fn head() {
    match helpers::read_lines("test.txt") {
        Ok(mut lines) => {
            let mut i = 0;
            while i < 10 {
                let test = lines.next().unwrap().unwrap();
                println!("{}", &test);
                i += 1;
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
