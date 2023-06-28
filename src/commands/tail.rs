use crate::helpers;

pub fn tail() {
    let lines = helpers::read_number_of_lines_in_reverse("test.txt", 2);

    for line in lines.unwrap().iter().rev() {
        println!("{}", line.as_str());
    }
}
