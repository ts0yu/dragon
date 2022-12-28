use std::fs;
use colored::*;
use executor::Vm;
use std::time::Instant;
use compiler::token::Token;
use std::collections::HashMap;
use clap::{Parser, Subcommand};
use compiler::assembler::Assembler;

#[derive(Parser)]
#[clap(about, version, author)]
struct Value {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run { path: String },
    Debug { path: String },
}

fn main() {
    let value = Value::parse();

    match &value.command {
        Commands::Run { path } => {
            let contents = fs::read_to_string(path).unwrap();
            let tokens = Token::lex(&contents);

            println!("{} `{}`", "Compiling".green().bold(), path);

            let now = Instant::now();

            let opcodes = Assembler::new(tokens).parse();
            let mut vm = Vm::new(opcodes, HashMap::new());

            println!("{} dev [unoptimized] in {:?}", " Finished".green().bold(), now.elapsed());

            println!("{} `{}`", "  Running".green().bold(), path);

            vm.execute();
        },
        Commands::Debug { path } => {
            let contents = fs::read_to_string(path).unwrap();
            let tokens = Token::lex(&contents);
            let mut vm = Vm::new(
                Assembler::new(tokens).parse(),
                HashMap::new()
            );

            vm.debug();

            println!("{:?}", vm.stack)
        },
    }
}