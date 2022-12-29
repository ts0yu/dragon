#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use std::collections::HashMap;
use colored::*;

/// Type representing a Falbe VM opcode.
#[derive(PartialEq, Debug)]
pub enum Opcode {
    /// Push the value to the top of the stack.
    Push(f64),
    /// Jump to a specified instruction in the program by the program counter.
    Jump(usize),
    /// Set a value in the constant storage context.
    Set(usize),
    /// Get a value from the constant storage context.
    Get(usize),
    /// Pop the top value from the stack
    Pop,
    /// Duplicate the top value from the stack
    Dup,
    /// Add the top two values on the stack.
    Add,
    /// Subtract the second deepest stack value from the top value.
    Sub,
    /// Square root the top of the stack.
    Sqrt,
    /// Multiply the top two values on the stack.
    Mul,
    /// Push the current program counter (pc).
    Pc,
    /// 'Side-effect' opcode; print the top stack value.
    Print,
    /// Indicate to the executor that the program should not continue. Include at the end of an instruction set.
    Halt,
}

/// Type representing a Falbe virtual machine.
#[derive(PartialEq, Debug)]
pub struct Vm {
    /// Current index of the executor.
    pub pc: usize,
    /// List of instructions to execute.
    pub instructions: Vec<Opcode>,
    /// Persistent storage throughout instruction execution.
    pub constants: HashMap<usize, f64>,
    /// Stack context.
    pub stack: Vec<f64>,
}

impl Vm {
    /// Public builder function that instantiates a new VM context.
    pub fn new(instructions: Vec<Opcode>, constants: HashMap<usize, f64>) -> Self {
        Self {
            instructions,
            pc: 0,
            stack: Vec::new(),
            constants,
        }
    }

    /// Execute the current instruction set.
    pub fn execute(&mut self) {
        if !self.instructions.contains(&Opcode::Halt) {
            println!("{}: no `halt` in program.", "error".red().bold());
        }
        while self.instructions[self.pc] != Opcode::Halt {
            match self.instructions[self.pc] {
                Opcode::Push(value) => self.push(value),
                Opcode::Jump(pc) => self.jump(pc),
                Opcode::Set(key) => self.set(key),
                Opcode::Get(key) => self.get(key),
                Opcode::Pop => self.pop(),
                Opcode::Dup => self.dup(),
                Opcode::Add => self.add(),
                Opcode::Sub => self.sub(),
                Opcode::Mul => self.mul(),
                Opcode::Sqrt => self.sqrt(),
                Opcode::Pc => self.push(self.pc as f64),
                Opcode::Print => self.print(),
                Opcode::Halt => continue,
            }
        }
    }

    /// Jump to a specified program counter and increase program counter.
    fn jump(&mut self, pc: usize) {
        self.pc = pc;
    }

    /// Set a key in the storage context to a value and increase program counter.
    fn set(&mut self, key: usize) {
        self.constants.insert(key, self.stack[self.stack.len() - 1]);
        self.pc += 1;
    }

    /// Get a key from the storage context and push a value.
    fn get(&mut self, key: usize) {
        self.stack.push(*self.constants.get(&key).unwrap());
        self.pc += 1;
    }

    /// Push a value to the stack and increase program counter.
    fn push(&mut self, value: f64) {
        self.stack.push(value);
        self.pc += 1;
    }

    /// Print the current value and increase program counter.
    fn print(&mut self) {
        println!("{}", self.stack[self.stack.len() - 1]);
        self.pc += 1;
    }

    /// Pop the top stack value and increase program counter.
    fn pop(&mut self) {
        self.stack.pop();
        self.pc += 1;
    }

    /// Add the top two stack values and increase program counter.
    fn add(&mut self) {
        let a = self.stack[self.stack.len() - 1];
        let b = self.stack[self.stack.len() - 2];

        self.push(a + b);
    }

    /// Duplicate the top stack value.
    fn dup(&mut self) {
        let a = self.stack[self.stack.len() - 1];
    
        self.push(a);
    }

    /// Add the top two stack values and increase program counter.
    fn mul(&mut self) {
        let a = self.stack[self.stack.len() - 1];
        let b = self.stack[self.stack.len() - 2];
    
        self.push(a * b);
    }

    /// Square root the top of the stack.
    fn sqrt(&mut self) {
        let a = self.stack[self.stack.len() - 1];
    
        self.push(f64::sqrt(a));
    }

    /// Subtract the second deepest stack value from the top value and increase program counter.
    fn sub(&mut self) {
        let a = self.stack[self.stack.len() - 1];
        let b = self.stack[self.stack.len() - 2];

        self.push(b - a);
    }
}