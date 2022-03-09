#![allow(non_snake_case)]

use std::iter::Peekable;
use std::vec::IntoIter;

use std::{fs, io, env};

#[derive(Copy, Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
enum OpCodes {
    PUSH,
    POP,
    ADD,
    SUB,
    MULT,
    DIV,
    EQ,
    LT,
    GT,
    IF,
    ELSE,
    END
}

#[derive(Debug)]
enum Instructions {
    PUSH,
    POP,
    ADD,
    SUB,
    MULT,
    DIV,
    EQ,
    LT,
    GT,
    If(IfElse)
}

#[derive(Copy, Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
struct Operation {
    OpCode: OpCodes,
    Contents: Option<i32>
}

impl Operation {
    fn new(opcode: OpCodes, contents: Option<i32>) -> Self {
        Operation {
            OpCode: opcode,
            Contents: contents
        }
    }
}

#[derive(Debug)]
struct Instruction {
    Instruction: Instructions,
    Contents: Option<i32>
}

impl Instruction {
    fn new(instr: Instructions, contents: Option<i32>) -> Self {
        Instruction {
            Instruction: instr,
            Contents: contents
        }
    }
}

#[derive(Debug)]
struct IfElse {
    If: Option<Vec<Option<Instruction>>>,
    Else: Option<Vec<Option<Instruction>>>
}

impl IfElse {
    fn new(IfBlock: Vec<Option<Instruction>>, ElseBlock: Vec<Option<Instruction>>) -> Self {
        IfElse {
            If: Some(IfBlock),
            Else: Some(ElseBlock)
        }
    }
}

const KEYWORDS: [&str; 11] = [
    "if",
    "else",
    "end",
    "+",
    "-",
    ".",
    "=",
    "<",
    ">",
    "*",
    "/"
];

#[derive(Debug)]
struct Lexer {
    raw_data: Peekable<IntoIter<char>>,
}


impl Lexer {
    fn from_text(text: &str) -> Self {
        Lexer {
            raw_data: text.chars().collect::<Vec<_>>().into_iter().peekable(),
        }
    }

    fn from_file(file_path: &str) -> io::Result<Self> {
        Ok(Self::from_text(&fs::read_to_string(file_path)?))
    }

    fn get_next_char_while(&mut self, raw_token: &mut String, cond: fn(char) -> bool) {
        loop {
            match self.raw_data.peek() {
                Some(c) if cond(*c) => {
                    raw_token.push(*c);
                    self.raw_data.next();
                }
                _ => break,
            }
        }
    }

    fn get_keyword(&mut self, current_char: char) -> Option<String> {
        let mut token: String = current_char.to_string();
        self.get_next_char_while(&mut token, Self::is_alphanumeric);
        
        if KEYWORDS.contains(&token.as_str()) {
            return Some(token);
        } else {
            return None;
        }
    }

    fn is_alphanumeric(c: char) -> bool {
        return c.is_alphanumeric();
    }

    fn is_numeric(c: char) -> bool {
        return c.is_numeric();
    }
}

impl Iterator for Lexer {
    type Item = Operation;

    fn next(&mut self) ->  Option<Self::Item> {
        let mut first_char: char;

        loop {
            match self.raw_data.next() {
                Some(c) if c.is_whitespace() => continue,
                Some(c) => {
                    first_char = c;
                }
                None => return None,
            }

            if let Some(token) = self.get_keyword(first_char) {
                match token.as_str() {
                    "if" => return Some(Operation::new(OpCodes::IF, None)),
                    "else" => return Some(Operation::new(OpCodes::ELSE, None)),
                    "end" => return Some(Operation::new(OpCodes::END, None)),
                    "+" => return Some(Operation::new(OpCodes::ADD, None)),
                    "-" => return Some(Operation::new(OpCodes::SUB, None)),
                    "." => return Some(Operation::new(OpCodes::POP, None)),
                    "=" => return Some(Operation::new(OpCodes::EQ, None)),
                    "<" => return Some(Operation::new(OpCodes::LT, None)),
                    ">" => return Some(Operation::new(OpCodes::GT, None)),
                    "*" => return Some(Operation::new(OpCodes::MULT, None)),
                    "/" => return Some(Operation::new(OpCodes::DIV, None)),
                    _ => panic!("impossible edge case")
                }
            }

            else if first_char.is_numeric() {
                let mut num: String = String::from(first_char);

                self.get_next_char_while(&mut num, Self::is_numeric);
                return Some(Operation::new(OpCodes::PUSH, Some(num.parse::<i32>().unwrap())))
            }
        }
    }
}


struct Parser {
    operations: IntoIter<Option<Operation>>
}

impl Parser {
    fn new(data: IntoIter<Option<Operation>>) -> Self {
        Parser {
            operations: data
        }
    }

    fn gen_instruction_from_op(&mut self, op: Operation) -> Option<Instruction> {
        match op.OpCode { 
            OpCodes::PUSH => return Some(Instruction::new(Instructions::PUSH, Some(op.Contents.expect("this literally should not be possible")))),
            OpCodes::POP => return Some(Instruction::new(Instructions::POP, None)),
            OpCodes::ADD => return Some(Instruction::new(Instructions::ADD, None)),
            OpCodes::SUB => return Some(Instruction::new(Instructions::SUB, None)),
            OpCodes::EQ => return Some(Instruction::new(Instructions::EQ, None)),
            OpCodes::LT => return Some(Instruction::new(Instructions::LT, None)),
            OpCodes::GT => return Some(Instruction::new(Instructions::GT, None)),
            OpCodes::MULT => return Some(Instruction::new(Instructions::MULT, None)),
            OpCodes::DIV => return Some(Instruction::new(Instructions::DIV, None)),
            OpCodes::IF => {
                let mut if_block: Vec<Option<Instruction>> = Vec::new();
                let mut else_block: Vec<Option<Instruction>> = Vec::new();
                while let Some(i) = self.operations.next() {
                    match i {
                        Some(j) => {
                            if j.OpCode != OpCodes::END && j.OpCode != OpCodes::ELSE {
                                if_block.push(self.gen_instruction_from_op(j))
                            } else {
                                if j.OpCode == OpCodes::ELSE {
                                    while let Some(x) = self.operations.next() {
                                        match x {
                                            Some(y) => {
                                                if y.OpCode != OpCodes::END {
                                                    else_block.push(self.gen_instruction_from_op(y));
                                                } else {break;}
                                            },
                                            None => continue
                                        }
                                    }
                                    return Some(Instruction::new(Instructions::If(IfElse::new(if_block, else_block)), None));
                                } else if j.OpCode == OpCodes::END {
                                    return Some(Instruction::new(Instructions::If(IfElse::new(if_block, else_block)), None));
                                }
                            }
                        },
                        _ => continue
                    }
                } 
                return Some(Instruction::new(Instructions::If(IfElse::new(if_block, else_block)), None));
            },
            OpCodes::END => {
                eprintln!("ERROR: 'end' statement found without matching block");
                std::process::exit(1);
            },
            OpCodes::ELSE => {
                eprintln!("ERROR: 'else' statement found without match 'if'");
                std::process::exit(1);
            }
        }
    }
}

impl Iterator for Parser {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.operations.next() {
                Some(i) => {
                    if let Some(j) = i {
                        return self.gen_instruction_from_op(j);
                    }
                    
                },
                None => return None 
                
            }
        }
    }
}



fn evaluate_instruction(instruction: &Instruction, stack: &mut Vec<i32>) {
    match &instruction.Instruction {
        Instructions::PUSH => stack.push(instruction.Contents.expect("no data given to push to the stack")),
        Instructions::POP => {
            println!("{:?}", stack.pop().expect("Cannot pop value from empty stack"))
        },
        Instructions::ADD => {
            let first_val = stack.pop().expect("Insufficient data on the stack");
            let second_val = stack.pop().expect("Insufficient data on the stack");
            stack.push(first_val + second_val);
        },
        Instructions::SUB => {
            let first_val = stack.pop().expect("Insufficient data on the stack");
            let second_val = stack.pop().expect("Insufficient data on the stack");
            stack.push(first_val - second_val);
        },
        Instructions::MULT => {
            let first_val = stack.pop().expect("Insufficient data on the stack");
            let second_val = stack.pop().expect("Insufficient data on the stack");
            stack.push(first_val * second_val);
        },
        Instructions::DIV => {
            let first_val = stack.pop().expect("Insufficient data on the stack");
            let second_val = stack.pop().expect("Insufficient data on the stack");
            stack.push(first_val / second_val);
        },
        Instructions::EQ => {
            let first_val = stack.pop().expect("Insufficient data on the stack");
            let second_val = stack.pop().expect("Insufficient data on the stack");
            if first_val == second_val {
                stack.push(1);
            } else {
                stack.push(0);
            }
        },
        Instructions::LT => {
            let first_val = stack.pop().expect("Insufficient data on the stack");
            let second_val = stack.pop().expect("Insufficient data on the stack");
            if first_val < second_val {
                stack.push(1);
            } else {
                stack.push(0);
            }
        },
        Instructions::GT => {
            let first_val = stack.pop().expect("Insufficient data on the stack");
            let second_val = stack.pop().expect("Insufficient data on the stack");
            if first_val > second_val {
                stack.push(1);
            } else {
                stack.push(0);
            }
        }
        Instructions::If(nested_struct) => {
            match stack.pop().expect("No binary condition found") {
                1 => {
                    for i in nested_struct.If.as_ref().unwrap() {
                        if let Some(j) = i {
                            evaluate_instruction(&j, stack)
                        }
                    }
                },
                0 => {
                    if let Some(instr) = nested_struct.Else.as_ref() {
                        if instr.len() > 0 {
                            for i in instr {
                                if let Some(j) = i {
                                    evaluate_instruction(&j, stack)
                                }
                            }
                        }
                    } else {
                        return;
                    }
                },
                _ => panic!("Binary boolean not found")
            }
        }
    }
}

fn simulate(stack: &mut Vec<i32>, instructions: Vec<Option<Instruction>>) {
    for instruction in instructions {
        match instruction {
            Some(i) => {
                evaluate_instruction(&i, stack);
            },
            None => continue
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: cargo run <filepath>");
        std::process::exit(1);
    }

    let mut operations: Vec<Option<Operation>> = Vec::new();
    let mut stack: Vec<i32> = Vec::new();

    let lex = Lexer::from_file(&args[1]).unwrap();

    for token in lex {
        operations.push(Some(token));
    }

    // println!("{:?}", operations);

    let parse = Parser::new(operations.into_iter());

    let mut instructions = Vec::new();

    for instr in parse {
        instructions.push(Some(instr));
    }

    simulate(&mut stack, instructions)
    // println!("{:?}", instructions);
}
