use std::env;

mod commands;
use commands::cat::cat;
use commands::echo::echo;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match args[0].as_str() {
        "echo" => echo(args[1..].to_vec()),
        "cat" => {
            if args[1].starts_with("-") {
                cat(args[2..].to_vec(), &args[1]);
            } else {
                cat(args[1..].to_vec(), &String::from(""));
            }
        }
        _ => println!("Error: Command not recognized"),
    }
}
