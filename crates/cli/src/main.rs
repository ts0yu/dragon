use clap::{Parser, Subcommand};
use colored::*;
use compiler::assembler::Assembler;
use compiler::assembler::Macro;
use compiler::token::Token;
use compiler::token::TokenType;
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

            let mut main_macro: Macro = Macro {
                name: "",
                body: Vec::new(),
            };

            // let mut occurences: usize = 0;

            // tokens.clone().into_iter().for_each(|token| {
            //     if token.ttype == TokenType::Macro {
            //         occurences += 1
            //     }
            // });

            let opcodes = Assembler::new(tokens);

            let mut macros = HashMap::new();
            let mut constants = HashMap::new();

            loop {
                let _m;
                let _c;

                match opcodes.parse_macro() {
                    Ok(mac) => _m = macros.insert(mac.clone().name, mac).unwrap(),
                    Err(_) => {
                        match opcodes.parse_constant() {
                            Ok(con) => _c = constants.insert(con.clone().name, con).unwrap(),
                            Err(_) => break,
                        };
                    }
                }
            }

            // for _ in 0..occurences {
            //     let mmacro = opcodes.parse_macro();
            //     macros.insert(mmacro.clone().unwrap().name, mmacro.unwrap());
            // }

            match macros.get(&"main") {
                Some(r#main) => main_macro = r#main.clone(),
                None => panic!("no main macro found"),
            }

            for (i, n) in main_macro.clone().body.iter().enumerate() {
                let mut slice: f64;
                if n.ttype == TokenType::Invocation {
                    let invocation_name = &n.slice[0..n.slice.len() - 2];
                    let replacer = macros.get(&invocation_name);
                    let mut index: usize = i;

                    for g in &replacer.unwrap().body {
                        main_macro.body.insert(index, *g);
                        index += 1;
                    }
                }
            }

            main_macro.body.iter_mut().for_each(|tok| if tok.ttype == TokenType::Identifier {
                *tok = Token {
                    ttype: TokenType::Literal,
                    slice: &constants.get(&tok.slice).unwrap().value.to_string()
                };
            });

            println!("{} `{}`", "Compiling".green().bold(), path);

            let now = Instant::now();
            let opcodes = Assembler::new(main_macro.body).assemble();

            println!("{opcodes:#?}");

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
