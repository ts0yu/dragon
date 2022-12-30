use clap::{Parser, Subcommand};
use colored::*;
use compiler::assembler::Assembler;
use compiler::token::Token;
use executor::Vm;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

#[derive(Parser)]
#[clap(about, version, author)]
struct Value {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run { path: String },
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

            let constants = HashMap::from([
                (1, std::f64::consts::PI),
                (2, std::f64::consts::TAU),
                (3, std::f64::consts::E),
            ]);

            let mut vm = Vm::new(opcodes, constants);

            println!(
                "{} dev [unoptimized] in {:?}",
                " Finished".green().bold(),
                now.elapsed()
            );

            println!("{} `{}`", "  Running".green().bold(), path);

            vm.execute();
        }
    }
}
