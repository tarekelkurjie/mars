#![allow(non_snake_case)]

use std::iter::Peekable;
use std::vec::IntoIter;
use std::collections::HashMap;

use std::{fs, io, env};

#[derive(PartialEq)]
#[derive(Debug)]
enum OpCodes {
    PUSH,
    POP,
    PRINT,
    DUP,
    ADD,
    SUB,
    MULT,
    DIV,
    EQ,
    LT,
    GT,
    IF,
    ELSE,
    WHILE,
    END,
    DO,
    VARDECLARE(String),
    VARDEFINE,
    REFERENCE(String)
}

#[derive(Debug)]
enum Instructions {
    PUSH,
    POP,
    PRINT,
    DUP,
    ADD,
    SUB,
    MULT,
    DIV,
    EQ,
    LT,
    GT,
    VARDECLARE(VariableDefine),
    REFERENCE(String),
    If(IfElse),
    While(While)
}

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
struct While {
    Cond: Vec<Option<Instruction>>,
    Contents: Vec<Option<Instruction>>
}

impl While {
    fn new(cond: Vec<Option<Instruction>>, contents: Vec<Option<Instruction>>) -> Self {
        While {
            Cond: cond,
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

#[derive(Debug)]
struct VariableDefine {
    name: String,
    instructions: Vec<Option<Instruction>>
}

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

    fn get_next_char_while(&mut self, raw_token: String, cond: fn(char) -> bool) -> String {
        let mut res = raw_token;
        loop {
            match self.raw_data.peek() {
                Some(c) if cond(*c) => {
                    res.push(*c);
                    self.raw_data.next();
                }
                _ => break,
            }  
        }
        return res
    }

    fn is_alphanumeric(c: char) -> bool {
        return c.is_alphanumeric() || c == '_';
    }

    fn get_numeric(&mut self, c: char) -> String {
        let mut res: String = c.to_string();
        res = self.get_next_char_while(res, |c| c.is_numeric());

        return res
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

            if !first_char.is_numeric() {
                if first_char == '@' {
                    let token: String = self.raw_data.next().expect("ERROR: No character found").to_string();
                    let name = self.get_next_char_while(token, |c| Self::is_alphanumeric(c));

                    return Some(Operation::new(OpCodes::VARDECLARE(name.to_string()), None));
                } else {
                    let token: String = first_char.to_string();
                    let identifier = self.get_next_char_while(token, |c| Self::is_alphanumeric(c));
                    match identifier.as_str() {
                        "dup" => return Some(Operation::new(OpCodes::DUP, None)),
                        "pop" => return Some(Operation::new(OpCodes::POP, None)),
                        "if" => return Some(Operation::new(OpCodes::IF, None)),
                        "else" => return Some(Operation::new(OpCodes::ELSE, None)),
                        "while" => return Some(Operation::new(OpCodes::WHILE, None)),
                        "end" => return Some(Operation::new(OpCodes::END, None)),
                        "do" => return Some(Operation::new(OpCodes::DO, None)),
                        "+" => return Some(Operation::new(OpCodes::ADD, None)),
                        "-" => return Some(Operation::new(OpCodes::SUB, None)),
                        "print" => return Some(Operation::new(OpCodes::PRINT, None)),
                        "=" => return Some(Operation::new(OpCodes::EQ, None)),
                        "<" => return Some(Operation::new(OpCodes::LT, None)),
                        ">" => return Some(Operation::new(OpCodes::GT, None)),
                        "*" => return Some(Operation::new(OpCodes::MULT, None)),
                        "/" => return Some(Operation::new(OpCodes::DIV, None)),
                        "def" => return Some(Operation::new(OpCodes::VARDEFINE, None)),
                        _ => return Some(Operation::new(OpCodes::REFERENCE(identifier.to_string()), None))
                    }
                }
            }

            else if first_char.is_numeric() {
                return Some(Operation::new(OpCodes::PUSH,
                                           Some(self.get_numeric(first_char)
                                               .parse::<i32>()
                                               .unwrap())))
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
            OpCodes::PRINT => return Some(Instruction::new(Instructions::PRINT, None)),
            OpCodes::POP => return Some(Instruction::new(Instructions::POP, None)),
            OpCodes::DUP => return Some(Instruction::new(Instructions::DUP, None)),
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
                                    return Some(Instruction::new(
                                        Instructions::If(
                                            IfElse::new(
                                                if_block, else_block
                                            )
                                        ), None));
                                } else if j.OpCode == OpCodes::END {
                                    return Some(Instruction::new(
                                        Instructions::If(
                                            IfElse::new(
                                                if_block, else_block
                                            )
                                        ), None));
                                }
                            }
                        },
                        _ => continue
                    }
                } 
                return Some(Instruction::new(
                    Instructions::If(
                        IfElse::new(
                            if_block, else_block
                        )
                    ), None));
            },
            OpCodes::WHILE => {
                let mut cond: Vec<Option<Instruction>> = Vec::new();
                let mut contents: Vec<Option<Instruction>> = Vec::new();

                while let Some(i) = self.operations.next() {
                    if let Some(j) = i {
                        if j.OpCode != OpCodes::DO {
                            cond.push(self.gen_instruction_from_op(j));
                        } else if j.OpCode == OpCodes::DO {
                            while let Some(x) = self.operations.next() {
                                if let Some(y) = x {
                                    if y.OpCode != OpCodes::END {
                                        contents.push(self.gen_instruction_from_op(y));
                                    } else {
                                        return Some(Instruction::new(
                                            Instructions::While(
                                                While::new(
                                                    cond,
                                                    contents
                                                )
                                            ),
                                            None
                                        ))
                                    }
                                }
                            }
                        }
                    }
                }
                return Some(Instruction::new(
                        Instructions::While(
                            While::new(
                                cond,
                                contents
                            )
                        ), None
                    )
                )

            },
            OpCodes::END => {
                eprintln!("ERROR: 'end' statement found without matching block");
                std::process::exit(1);
            },
            OpCodes::ELSE => {
                eprintln!("ERROR: 'else' statement found without match 'if'");
                std::process::exit(1);
            },
            OpCodes::DO => {
                eprintln!("ERROR: 'do' statement found without matching block");
                std::process::exit(1);
            },
            OpCodes::VARDECLARE(name) => {
                let mut instr: Vec<Option<Instruction>> = Vec::new();

                while let Some(i) = self.operations.next() {
                    if let Some(j) = i {
                        if j.OpCode != OpCodes::VARDEFINE {
                            instr.push(self.gen_instruction_from_op(j));
                        } else {
                            return Some(Instruction::new(Instructions::VARDECLARE(VariableDefine {name: name.to_string(), instructions: instr}), None));
                        }
                    }
                }
                return Some(Instruction::new(Instructions::VARDECLARE(VariableDefine {name: name.to_string(), instructions: instr}), None));
            },
            OpCodes::VARDEFINE => {
                eprintln!("ERROR: 'def' statement found without matching variable declaration");
                std::process::exit(1);
            },
            OpCodes::REFERENCE(name) => Some(Instruction::new(Instructions::REFERENCE(name), None))
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



fn evaluate_instruction<'a>(instruction: &'a Instruction, stack: &mut Vec<i32>, data_stack: &mut HashMap<String, i32>) {
    match &instruction.Instruction {
        Instructions::PUSH => stack.push(instruction.Contents.expect("no data given to push to the stack")),
        Instructions::PRINT => {
            println!("{:?}", stack.pop().expect("Cannot pop value from empty stack"))
        },
        Instructions::POP => {
            stack.pop();
        }
        Instructions::DUP => {
            let val = stack.pop().expect("ERROR: No data on stack to duplicate");
            stack.push(val);
            stack.push(val);
        }
        Instructions::ADD => {
            let first_val = stack.pop().expect("Insufficient data on the stack");
            let second_val = stack.pop().expect("Insufficient data on the stack");
            stack.push(first_val + second_val);
        },
        Instructions::SUB => {
            let first_val = stack.pop().expect("Insufficient data on the stack");
            let second_val = stack.pop().expect("Insufficient data on the stack");
            stack.push(second_val - first_val);
        },
        Instructions::MULT => {
            let first_val = stack.pop().expect("Insufficient data on the stack");
            let second_val = stack.pop().expect("Insufficient data on the stack");
            stack.push(first_val * second_val);
        },
        Instructions::DIV => {
            let first_val = stack.pop().expect("Insufficient data on the stack");
            let second_val = stack.pop().expect("Insufficient data on the stack");
            stack.push(second_val / first_val);
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
            if second_val < first_val {
                stack.push(1);
            } else {
                stack.push(0);
            }
        },
        Instructions::GT => {
            let first_val = stack.pop().expect("Insufficient data on the stack");
            let second_val = stack.pop().expect("Insufficient data on the stack");
            if second_val > first_val {
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
                            evaluate_instruction(&j, stack, data_stack)
                        }
                    }
                },
                0 => {
                    if let Some(instr) = nested_struct.Else.as_ref() {
                        if instr.len() > 0 {
                            for i in instr {
                                if let Some(j) = i {
                                    evaluate_instruction(&j, stack, data_stack)
                                }
                            }
                        }
                    } else {
                        return;
                    }
                },
                _ => panic!("Binary boolean not found")
            }
        },
        Instructions::While(nested_struct) => {
            for instr in &nested_struct.Cond {
                if let Some(i) = instr {
                    evaluate_instruction(&i, stack, data_stack)
                }
            }
            while stack.pop().expect("No value found on stack") == 1 {
                for instr in &nested_struct.Contents {
                    if let Some(i) = instr {
                        evaluate_instruction(&i, stack, data_stack)
                    }
                }
                for instr in &nested_struct.Cond {
                    if let Some(i) = instr {
                        evaluate_instruction(&i, stack, data_stack)
                    }
                }
            }
        },
        Instructions::VARDECLARE(nested_struct) => {
            for instr in &nested_struct.instructions {
                evaluate_instruction(&instr.as_ref().unwrap(), stack, data_stack);
            }
            data_stack.insert(
                nested_struct.name.to_string(),
                stack.pop().unwrap()
            );
        },
        Instructions::REFERENCE(name) => {
            if let Some(data) = data_stack.get(name) {
                stack.push(*data);
            } else {
                eprintln!("Unexpected token {}", name);
                std::process::exit(1);
            }
        }
    }
}

fn simulate<'a>(stack: &'a mut Vec<i32>, data_stack: &'a mut HashMap<String, i32>, instructions: &'a Vec<Option<Instruction>>) {
    for instruction in instructions {
        match &instruction {
            Some(i) => {
                evaluate_instruction(&i, stack, data_stack);
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
    let mut data_stack: HashMap<String, i32> = HashMap::new();

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

    simulate(&mut stack, &mut data_stack, &instructions);
    // println!("{:?}", instructions);
}
