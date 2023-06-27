use std::env;
use std::io::{self, Write};

mod commands;
pub mod helpers;
use commands::{cat::cat, echo::echo, head::head, ls::ls, tail::tail};

fn main() -> io::Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let args: Vec<String> = env::args().skip(1).collect();

    let stderr = io::stderr();
    let mut handle = stderr.lock();

    match args[0].as_str() {
        "echo" => Ok(echo(args[1..].to_vec())),
        "cat" => Ok(cat(args[1..].to_vec())),
        "ls" => Ok(ls(args[1..].to_vec())),
        "head" => Ok(head(args[1..].to_vec())),
        "tail" => Ok(tail()),
        _ => handle.write_all(b"Error: Command not recognized"),
    }
}
