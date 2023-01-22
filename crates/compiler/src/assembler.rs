use crate::token::Token;
use std::collections::HashMap;
use crate::token::TokenType;
use std::cell::Cell;
use executor::Opcode;

/// Type representing an Opcode parser.
#[derive(Debug)]
pub struct Assembler<'a> {
    /// Tokens to be parsed.
    pub tokens: Vec<Token<'a>>,
	/// Cursor
	pub cursor: Cell<usize>,
}

/// Representation of a macro.
/// A macro is essentially a block of opcodes that are pasted with the necessary arguments whenever invoked.
#[derive(Debug, Clone)]
pub struct Macro<'a> {
    /// Name of the macro.
	pub name: &'a str,
    /// Body of the macro.
	pub body: Vec<Token<'a>>,
}

/// Constants are inlined whenever they are referenced.
#[derive(Debug, Clone)]
pub struct Constant<'a> {
    /// Name of the constant.
	pub name: &'a str,
    /// Value.
	pub value: f64,
}

impl<'a> Assembler<'a> {
    /// Public constructor function that instantiates a `Parser`.
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self { tokens, cursor: Cell::new(0) }
    }

	/// Expand all macros.
	pub fn parse_macro(&self) -> Result<Macro<'a>, ()> {
		let mut body: Vec<Token<'a>> = Vec::new();
		let mut name: &str = "";

		self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Macro)?;
		self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Invocation)?;
        let raw_invocation = self.tokens[self.cursor.get() - 1].slice;
		name = &raw_invocation[0..raw_invocation.len() - 2];
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

    /// Expand all macros.
	pub fn parse_constant(&self) -> Result<Constant<'a>, ()> {
		let mut name: &str = "";
        let mut value: f64 = 0.0;

		self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Constant)?;
		self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Identifier)?;
		name = self.tokens[self.cursor.get() - 1].slice;
		self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Assign)?;

        self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Literal)?;
		value = self.tokens[self.cursor.get() - 1].slice.parse::<f64>().unwrap();

		Ok(Constant { name, value })
	}

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
    pub fn assemble(&self, lookup_table: HashMap<&str, Constant>) -> Vec<Opcode> {
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
                // TokenType::SetSymb => opcodes.push(Opcode::Set(
                //     self.tokens[index + 1].slice.parse::<usize>().unwrap(),
                // )),
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
				TokenType::Identifier => {
                    opcodes.push(Opcode::Push(lookup_table.get(&token.clone().slice).unwrap().value.to_string().parse::<f64>().unwrap()))
                },
				_ => panic!("parsing error")
            }
        }
        opcodes
    }
}
