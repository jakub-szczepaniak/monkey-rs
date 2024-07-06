use std::{
    io::{self, BufRead, Write},
    path::PathBuf,
};

use clap::Parser;
use lexer::Lexer;
use monkey_rs::*;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    filename: Option<PathBuf>,
}

fn main() {
    println!("Hello!");
    let cli = Cli::parse();
    if let Some(filename) = cli.filename {
        todo!("Not there yet");
    } else {
        println!("Starting Monkey REPL");
        repl()
    }
}

fn repl() {
    let stdin = io::stdin();
    print!("Monkey>>");
    let _ = io::stdout().flush();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            let mut lexer = Lexer::new(&line);
            loop {
                let token = lexer.next_token();
                if token.is_eof() || token.is_illegal() {
                    break;
                }
                println!("{}", token);
            }
        }
        print!(">>");
        let _ = io::stdout().flush();
    }
}
