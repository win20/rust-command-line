use std::env;

mod commands;
use commands::cat;
use commands::echo;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match args[0].as_str() {
        "echo" => echo::echo(args[1..].to_vec()),
        "cat" => {
            cat::cat(args[1..].to_vec());
            // if args[1].starts_with("-") {
            //     cat.cat(args[2..].to_vec(), cat::Option);
            // } else {
            //     cat.cat(args[1..].to_vec(), &String::from(""));
            // }
        }
        _ => println!("Error: Command not recognized"),
    }
}
