use std::env;
use std::io::{self, Write};

mod commands;
use commands::cat::cat;
use commands::echo::echo;
use commands::ls::ls;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    let stderr = io::stderr();
    let mut handle = stderr.lock();

    match args[0].as_str() {
        "echo" => Ok(echo(args[1..].to_vec())),
        "cat" => Ok(cat(args[1..].to_vec())),
        "ls" => Ok(ls()),
        _ => handle.write_all(b"Error: Command not recognized"),
    }
}
