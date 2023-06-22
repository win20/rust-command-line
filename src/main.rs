use std::env;

mod commands;
use commands::cat;
use commands::echo;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match args[0].as_str() {
        "echo" => echo::echo(args[1..].to_vec()),
        "cat" => cat::cat(args[1..].to_vec()),
        _ => println!("Error: Command not recognized"),
    }
}
