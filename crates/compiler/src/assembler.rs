use crate::token::Token;
use crate::token::TokenType;
use executor::Opcode;
// use std::path::PathBuf;
use std::collections::HashMap;
use std::cell::Cell;

/// Type representing an Opcode parser.
#[derive(Debug)]
pub struct Assembler<'a> {
    /// Tokens to be parsed.
    pub tokens: Vec<Token<'a>>,
    /// Cursor
    pub cursor: Cell<usize>,
}

#[derive(Debug, Clone)]
pub struct Macro<'a> {
    pub name: &'a str,
    pub body: Vec<Token<'a>>,
}

// #[derive(Debug, Clone)]
// pub struct Variable<'a> {
//     pub name: &'a str,
//     pub value: f64,
// }

// #[derive(Debug)]
// pub struct Import {
// 	pub path: PathBuf
// }



impl<'a> Assembler<'a> {
    /// Public constructor function that instantiates a `Parser`.
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self {
            tokens,
            cursor: Cell::new(0),
        }
    }

    /// Expand all macros.
    pub fn parse_macro(&self) -> Result<Macro<'a>, ()> {
        let mut body: Vec<Token<'a>> = Vec::new();
        let mut name: &str = "";

        self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Macro)?;
        self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Identifier)?;
        name = self.tokens[self.cursor.get() - 1].slice;
        self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::OpenBrace)?;

        while self.tokens[self.cursor.get()].ttype != TokenType::CloseBrace {
            body.push(self.tokens[self.cursor.get()]);
            let mut curr = self.cursor.get();
            curr += 1;
            self.cursor.set(curr);
        }

        self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::CloseBrace)?;

        Ok(Macro { name, body })
    }

	// pub fn parse_import(&self) -> Result<Import, ()>{
	// 	self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Use)?;
	// 	self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Quotation)?;
	// 	self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Identifier)?;
 //        let path = self.tokens[self.cursor.get() - 1].slice;
	// 	self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Quotation)?;

	// 	let path_buf = PathBuf::from(path);

	// 	Ok(Import { path: path_buf })
	// }

    // pub fn parse_variables(&self) -> Result<HashMap<&'a str, Variable<'a>>, ()> {
    // 	let mut name: &str = "";
    // 	let mut value: f64 = 0.0;

    // 	self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Const)?;
    // 	self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Identifier)?;

    // 	name = self.tokens[self.cursor.get() - 1].slice;

    // 	self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Assign)?;
    // 	self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Literal)?;

    // 	value = self.tokens[self.cursor.get() - 1].slice.parse::<f64>().unwrap();

    // 	Ok(Variable { name, value })
    // }

    pub fn match_token(&self, actual: TokenType, expected: TokenType) -> Result<(), ()> {
        if actual == expected {
            let mut curr = self.cursor.get();
            curr += 1;
            self.cursor.set(curr);
            Ok(())
        } else {
            Err(())
        }
    }

    /// Parse tokens to Opcodes.
    pub fn assemble(&self) -> Vec<Opcode> {
        let mut opcodes = Vec::new();
        for (index, token) in self.tokens.iter().enumerate() {
            match token.ttype {
                TokenType::Push => opcodes.push(Opcode::Push(
                    self.tokens[index + 1].slice.parse::<f64>().unwrap(),
                )),
                TokenType::Jump => opcodes.push(Opcode::Jump(
                    self.tokens[index + 1].slice.parse::<usize>().unwrap(),
                )),
                TokenType::Set => opcodes.push(Opcode::Set(
                    self.tokens[index + 1].slice.parse::<usize>().unwrap(),
                )),
                TokenType::Get => opcodes.push(Opcode::Get(
                    self.tokens[index + 1].slice.parse::<usize>().unwrap(),
                )),
                TokenType::Pop => opcodes.push(Opcode::Pop),
                TokenType::AddSymb => opcodes.push(Opcode::Add),
                TokenType::MulSymb => opcodes.push(Opcode::Mul),
                TokenType::SubSymb => opcodes.push(Opcode::Sub),
                TokenType::GetSymb => opcodes.push(Opcode::Get(
                    self.tokens[index + 1].slice.parse::<usize>().unwrap(),
                )),
                TokenType::Dup => opcodes.push(Opcode::Dup),
                TokenType::Add => opcodes.push(Opcode::Add),
                TokenType::Sub => opcodes.push(Opcode::Sub),
                TokenType::Mul => opcodes.push(Opcode::Mul),
                TokenType::Sqrt => opcodes.push(Opcode::Sqrt),
                TokenType::Pc => opcodes.push(Opcode::Pc),
                TokenType::Pi => opcodes.push(Opcode::Get(1)),
                TokenType::Tau => opcodes.push(Opcode::Get(2)),
                TokenType::E => opcodes.push(Opcode::Get(3)),
                TokenType::Print => opcodes.push(Opcode::Print),
                TokenType::Halt => opcodes.push(Opcode::Halt),
                TokenType::Literal => continue,
                TokenType::Error => continue,
                TokenType::Comment => continue,
                TokenType::Identifier => continue,
                _ => panic!("test"),
            }
        }
        opcodes
    }
}
