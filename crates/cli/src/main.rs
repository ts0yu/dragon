use clap::{Parser, Subcommand};
use colored::*;

use compiler::{
	assembler::{Assembler, Macro},
	token::{Token, TokenType}
};

use std::{
	fs, 
	time::Instant,
	collections::HashMap
};

use executor::Vm;

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
            let mut main_macro: Macro = Macro {
                name: "",
                body: Vec::new(),
            };

            let mut occurences: usize = 0;

            tokens.clone().into_iter().for_each(|token| {
                if token.ttype == TokenType::Macro {
                    occurences += 1
                }
            });

            let opcodes = Assembler::new(tokens);
            let mut macros = HashMap::new();
			
            // let mut constants = HashMap::new();

            // match opcodes.parse_macro() {
            // 	Ok(mac) => {
            // 		macros.insert(mac.clone().name, mac);
            // 	}
            // 	Err(_) => {
            // 		match opcodes.parse_variable() {
            // 			Ok(var) => constants.insert(var.clone().name, var),
            // 			Err(_) => continue,
            // 		}
            // 	}
            // }

			// let constants = opcodes.parse_variable();

			// println!("{constants:?}");

            for _ in 0..occurences {
                let mac = opcodes.parse_macro().unwrap();

                if macros.get(&mac.name).is_some() {
                    println!(
                        "{}: macro with name `{}` already exists",
                        "error".red().bold(),
                        &mac.name
                    );
                    std::process::exit(1);
                }

                if macros.get(&mac.name).is_none() {
                    macros.insert(mac.clone().name, mac);
                }
            }

            match macros.get(&"main") {
                Some(r#main) => main_macro = r#main.clone(),
                None => panic!("no main macro found"),
            }

            for (i, n) in main_macro.clone().body.iter().enumerate() {
                if n.ttype == TokenType::Identifier {
                    let replacer = macros.get(&n.slice);
                    let mut index: usize = i;

                    for g in &replacer.unwrap().body {
                        main_macro.body.insert(index, *g);
                        index += 1;
                    }
                }
            }

            println!("{} `{}`", "Compiling".green().bold(), path);

            let now = Instant::now();
            let opcodes = Assembler::new(main_macro.body).assemble();

            for i in opcodes {
                println!("{i}");
            }

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
