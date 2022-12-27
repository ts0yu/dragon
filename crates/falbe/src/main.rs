use std::fs;
use colored::*;
use compiler::assembler::Assembler;
use compiler::token::Token;
use executor::Vm;
use std::collections::HashMap;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(about, version, author)]
struct Value {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Compile { path: String },
}

fn main() {
    let value = Value::parse();

    match &value.command {
        Commands::Compile { path } => {
            let contents = fs::read_to_string(path).unwrap();
            let tokens = Token::lex(&contents);

            println!("{} `{}`", "Compiling".green().bold(), path);

            let opcodes = Assembler::new(tokens).parse();
            let mut vm = Vm::new(opcodes, HashMap::new());

            println!("{} `{}`", "  Running".green().bold(), path);

            vm.execute();
        },
    }
}