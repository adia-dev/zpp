#![allow(unused_imports)]
#![allow(dead_code)]

use clap::Parser;
use repl::REPL;

mod queue;
mod enums;
mod lexer;
mod token;
mod cli;
mod repl;

fn main() {
    let _args = cli::Cli::parse();

    let mut repl = REPL::new();

    repl.start();
}
