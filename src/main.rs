#![allow(unused_imports)]
#![allow(dead_code)]

use clap::Parser;
use repl::REPL;

mod ast;
mod cli;
mod enums;
mod lexer;
mod parser;
mod queue;
mod repl;
mod token;
mod traits;

fn main() {
    let _args = cli::Cli::parse();

    let mut repl = REPL::new();

    repl.start();
}
