use std::env;

mod commands;
use commands::echo::echo;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match args[0].as_str() {
        "echo" => echo(args[1..].to_vec()),
        _ => println!("Error: Command not recognized"),
    }
}
