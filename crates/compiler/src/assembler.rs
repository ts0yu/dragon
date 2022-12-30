use crate::token::Token;
use crate::token::TokenType;
use executor::Opcode;

/// Type representing an Opcode parser.
#[derive(Debug)]
pub struct Assembler<'a> {
    /// Tokens to be parsed.
    pub tokens: Vec<Token<'a>>,
}

impl<'a> Assembler<'a> {
    /// Public constructor function that instantiates a `Parser`.
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self { tokens }
    }

    /// Parse okens to Opcodes.
    pub fn parse(&self) -> Vec<Opcode> {
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
                TokenType::SetSymb => opcodes.push(Opcode::Set(
                    self.tokens[index + 1].slice.parse::<usize>().unwrap(),
                )),
                TokenType::GetSymb => opcodes.push(Opcode::Get(
                    self.tokens[index + 1].slice.parse::<usize>().unwrap(),
                )),
                TokenType::Dup => opcodes.push(Opcode::Dup),
                TokenType::Add => opcodes.push(Opcode::Add),
                TokenType::Sub => opcodes.push(Opcode::Sub),
                TokenType::Mul => opcodes.push(Opcode::Mul),
                TokenType::Sqrt => opcodes.push(Opcode::Sqrt),
                TokenType::Pc => opcodes.push(Opcode::Pc),
                TokenType::Print => opcodes.push(Opcode::Print),
                TokenType::Halt => opcodes.push(Opcode::Halt),
                TokenType::Literal => continue,
                TokenType::Error => continue,
                TokenType::Comment => continue,
            }
        }
        opcodes
    }
}
