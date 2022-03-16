#![allow(non_snake_case)]

use std::iter::Peekable;
use std::vec::IntoIter;
use std::collections::HashMap;

use std::{fs, io, env};

#[derive(PartialEq)]
#[derive(Debug)]
enum OpCodes {
    PUSH, // Begin stack manipulation
    POP,
    PRINT,
    PRINTASCII,
    DUP,
    SWAP,
    ADD, // Begin arithmetic
    SUB,
    MULT,
    DIV,
    EQ, // Begin control flow
    LT,
    GT,
    IF,
    ELSE,
    WHILE,
    END,
    DO,
    VARDECLARE(String), // Begin variable declaration
    DEFINE,
    IDENTIFIER(String), // Identifier
    SPAWN(String), // Begin spawnable stacks
    SWITCH(String),
    CLOSE(String),
    STACKS,
    STACKSIZE,
    STACKREV,
    STRING(Vec<Option<Operation>>), // String literal
    MACRO(String)
}

#[derive(Debug, Clone)]
enum Instructions {
    PUSH,
    POP,
    PRINT,
    PRINTASCII,
    DUP,
    SWAP,
    ADD,
    SUB,
    MULT,
    DIV,
    EQ,
    LT,
    GT,
    VARDECLARE(VariableDefine),
    IDENTIFIER(String),
    If(IfElse),
    While(While),
    SPAWN(String),
    SWITCH(String),
    CLOSE(String),
    STACKS,
    STACKSIZE,
    STACKREV,
    STRING(Vec<Option<Instruction>>),
    MACRO(Macro)
}

#[derive(Debug, PartialEq)]
struct Operation {
    OpCode: OpCodes,
    Contents: Option<u8>
}

impl Operation {
    fn new(opcode: OpCodes, contents: Option<u8>) -> Self {
        Operation {
            OpCode: opcode,
            Contents: contents
        }
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    Instruction: Instructions,
    Contents: Option<u8>
}

impl Instruction {
    fn new(instr: Instructions, contents: Option<u8>) -> Self {
        Instruction {
            Instruction: instr,
            Contents: contents
        }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug , Clone)]
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

#[derive(Debug, Clone)]
struct VariableDefine {
    name: String,
    instructions: Vec<Option<Instruction>>
}

#[derive(Debug, Clone)]
struct Macro {
    name: String,
    instructions: Vec<Option<Instruction>>
}

#[derive(Debug)]
struct Lexer {
    raw_data: Peekable<IntoIter<char>>,
}

const RESERVED_KEYWORDS: [&str; 19] = [
    "print",
    "print_ascii",
    "pop",
    "push",
    "swap",
    "dup",
    "do",
    "end",
    "def",
    "if",
    "else",
    "while",
    "spawn",
    "switch",
    "stacks",
    "stack_rev",
    "stack_size",
    "close",
    "macro"
];


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
                if first_char == '/' {
                    if let Some(char) = self.raw_data.next() {
                        if char == '/' {
                            while let Some(c) = self.raw_data.next() {
                                if c != '\n' { continue; } else { break; }
                            }
                        }
                    }
                } else if first_char == '@' { // Variable declaration
                    let token: String = self.raw_data.next().expect("ERROR: No character found").to_string();
                    let name = self.get_next_char_while(token, |c| Self::is_alphanumeric(c));

                    return Some(Operation::new(OpCodes::VARDECLARE(name.to_string()), None));
                } else if first_char == '"' { // String literal
                    let mut res: String = self.raw_data.next().expect("ERROR: Unexpected character \"").to_string();
                    while self.raw_data.peek() != Some(&'"') {
                        let char = self.raw_data.next();
                        if let Some(c) = char {
                            if c == '\\' {
                                if self.raw_data.next().unwrap() == 'n' {
                                    res.push('\n');
                                }
                            } else {res.push(c);}
                        }
                    }
                    self.raw_data.next();

                    let mut name: String = String::new();
                    let mut words = res.split_whitespace();
                    for _i in 0..3 {
                        if let Some(unwrapped) = words.next() {
                            for char in unwrapped.to_string().chars() {
                                if char.is_alphabetic() {
                                    name.push(char);
                                }
                            }
                            name.push('_');
                        }
                    }

                    let mut instr: Vec<Option<Operation>> = Vec::new();
                    instr.push(Some(Operation::new(OpCodes::SPAWN(name.clone()), None)));
                    instr.push(Some(Operation::new(OpCodes::SWITCH(name.clone()), None)));
                    for char in res.chars() {
                        instr.push(Some(Operation::new(OpCodes::PUSH, Some(char as u8))))
                    }
                    return Some(Operation::new(OpCodes::STRING(instr), None));
                } else {
                    let token: String = first_char.to_string();
                    let identifier = self.get_next_char_while(token, |c| Self::is_alphanumeric(c));
                    match identifier.as_str() {
                        "dup" => return Some(Operation::new(OpCodes::DUP, None)),
                        "swap" => return Some(Operation::new(OpCodes::SWAP, None)),
                        "pop" => return Some(Operation::new(OpCodes::POP, None)),
                        "if" => return Some(Operation::new(OpCodes::IF, None)),
                        "else" => return Some(Operation::new(OpCodes::ELSE, None)),
                        "while" => return Some(Operation::new(OpCodes::WHILE, None)),
                        "end" => return Some(Operation::new(OpCodes::END, None)),
                        "do" => return Some(Operation::new(OpCodes::DO, None)),
                        "+" => return Some(Operation::new(OpCodes::ADD, None)),
                        "-" => return Some(Operation::new(OpCodes::SUB, None)),
                        "print" => return Some(Operation::new(OpCodes::PRINT, None)),
                        "print_ascii" => return Some(Operation::new(OpCodes::PRINTASCII, None)),
                        "=" => return Some(Operation::new(OpCodes::EQ, None)),
                        "<" => return Some(Operation::new(OpCodes::LT, None)),
                        ">" => return Some(Operation::new(OpCodes::GT, None)),
                        "*" => return Some(Operation::new(OpCodes::MULT, None)),
                        "/" => return Some(Operation::new(OpCodes::DIV, None)),
                        "def" => return Some(Operation::new(OpCodes::DEFINE, None)),
                        "spawn" => {
                            self.raw_data.next();
                            let token: String = self.raw_data.next().expect("ERROR: No character found").to_string();
                            let name = self.get_next_char_while(token, |c| Self::is_alphanumeric(c));

                            return Some(Operation::new(OpCodes::SPAWN(name.to_string()), None));
                        },
                        "switch" => {
                            self.raw_data.next();
                            let token: String = self.raw_data.next().expect("ERROR: No character found").to_string();
                            let name = self.get_next_char_while(token, |c| Self::is_alphanumeric(c));

                            return Some(Operation::new(OpCodes::SWITCH(name.to_string()), None));
                        },
                        "close" => {
                            self.raw_data.next();
                            let token: String = self.raw_data.next().expect("ERROR: No character found").to_string();
                            let name = self.get_next_char_while(token, |c| Self::is_alphanumeric(c));

                            return Some(Operation::new(OpCodes::CLOSE(name.to_string()), None));
                        },
                        "stacks" => return Some(Operation::new(OpCodes::STACKS, None)),
                        "stack_size" => return Some(Operation::new(OpCodes::STACKSIZE, None)),
                        "stack_rev" => return Some(Operation::new(OpCodes::STACKREV, None)),
                        "macro" => {
                            self.raw_data.next();
                            let token: String = self.raw_data.next().expect("ERROR: No character found").to_string();
                            let name = self.get_next_char_while(token, |c| Self::is_alphanumeric(c));

                            return Some(Operation::new(OpCodes::MACRO(name), None));
                        }
                        _ => return Some(Operation::new(OpCodes::IDENTIFIER(identifier.trim().to_string()), None))
                    }
                }
            }

            else if first_char.is_numeric() {
                return Some(Operation::new(OpCodes::PUSH,
                                           Some(self.get_numeric(first_char)
                                               .parse::<u8>()
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
            OpCodes::PRINTASCII => return Some(Instruction::new(Instructions::PRINTASCII, None)),
            OpCodes::POP => return Some(Instruction::new(Instructions::POP, None)),
            OpCodes::DUP => return Some(Instruction::new(Instructions::DUP, None)),
            OpCodes::SWAP => return Some(Instruction::new(Instructions::SWAP, None)),
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
                        if j.OpCode != OpCodes::DEFINE {
                            instr.push(self.gen_instruction_from_op(j));
                        } else {
                            if RESERVED_KEYWORDS.contains(&name.as_str()) {
                                eprintln!("ERROR: Cannot assign variable with name of assigned keyword ({})", name);
                                std::process::exit(1);
                            }
                            return Some(Instruction::new(Instructions::VARDECLARE(VariableDefine {name: name.to_string(), instructions: instr}), None));
                        }
                    }
                }
                return Some(Instruction::new(Instructions::VARDECLARE(VariableDefine {name: name.to_string(), instructions: instr}), None));
            },
            OpCodes::DEFINE => {
                eprintln!("ERROR: 'def' statement found without matching variable declaration");
                std::process::exit(1);
            },
            OpCodes::IDENTIFIER(name) => Some(Instruction::new(Instructions::IDENTIFIER(name), None)),
            OpCodes::SPAWN(name) => Some(Instruction::new(Instructions::SPAWN(name), None)),
            OpCodes::SWITCH(name) => Some(Instruction::new(Instructions::SWITCH(name), None)),
            OpCodes::CLOSE(name) => Some(Instruction::new(Instructions::CLOSE(name), None)),
            OpCodes::STACKS => Some(Instruction::new(Instructions::STACKS, None)),
            OpCodes::STACKSIZE => Some(Instruction::new(Instructions::STACKSIZE, None)),
            OpCodes::STRING(contents) => {
                let mut instrs = Vec::new();
                for i in contents {
                    if let Some(instr)= i {
                        instrs.push(self.gen_instruction_from_op(instr));
                    }
                }
                Some(Instruction::new(Instructions::STRING(instrs), None))
            },
            OpCodes::STACKREV => Some(Instruction::new(Instructions::STACKREV, None)),
            OpCodes::MACRO(name) => {
                let mut instrs: Vec<Option<Instruction>> = Vec::new();

                while let Some(i) = self.operations.next() {
                    match i {
                        Some(j) => {
                            if j.OpCode != OpCodes::END {
                                instrs.push(self.gen_instruction_from_op(j))
                            } else {return Some(Instruction::new(Instructions::MACRO( Macro { name: name, instructions: instrs}), None))}
                        },
                        None => continue
                    }
                }
                Some(Instruction::new(Instructions::MACRO( Macro { name: name, instructions: instrs}), None))
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

struct Program<'a> {
    instructions: &'a Vec<Option<Instruction>>,
    stack: Vec<u8>,
    current_stack: String,
    data_stack: &'a mut HashMap<String, u8>,
    macro_stack: &'a mut HashMap<String, Vec<Option<Instruction>>>,
    stack_stack: &'a mut HashMap<String, Vec<u8>>,
}

impl<'a> Program<'a> {
    fn evaluate_instruction(&mut self, instruction: &Instruction) {
        match &instruction.Instruction {
            Instructions::PUSH => {
                self.stack.push(instruction.Contents.expect("ERROR: No data found to push to stack"));
            },
            Instructions::PRINT => {
                println!("{:?}", self.stack.pop().expect("Cannot pop value from empty stack"));
            },
            Instructions::PRINTASCII => {
                print!("{}", self.stack.pop().expect("Cannot pop value from empty stack") as char);
            }
            Instructions::POP => {
                self.stack.pop();
            },
            Instructions::DUP => {
                let val = self.stack.pop().expect("ERROR: No data on stack to duplicate");
                self.stack.push(val);
                self.stack.push(val);
            },
            Instructions::SWAP => {
                let first_val = self.stack.pop().expect("Insufficient data on the stack");
                let second_val = self.stack.pop().expect("Insufficient data on the stack");
                self.stack.push(first_val);
                self.stack.push(second_val);
            },
            Instructions::ADD => {
                let first_val = self.stack.pop().expect("Insufficient data on the stack");
                let second_val = self.stack.pop().expect("Insufficient data on the stack");
                self.stack.push(first_val + second_val);
            },
            Instructions::SUB => {
                let first_val = self.stack.pop().expect("Insufficient data on the stack");
                let second_val = self.stack.pop().expect("Insufficient data on the stack");
                self.stack.push(second_val - first_val);
            },
            Instructions::MULT => {
                let first_val = self.stack.pop().expect("Insufficient data on the stack");
                let second_val = self.stack.pop().expect("Insufficient data on the stack");
                self.stack.push(first_val * second_val);
            },
            Instructions::DIV => {
                let first_val = self.stack.pop().expect("Insufficient data on the stack");
                let second_val = self.stack.pop().expect("Insufficient data on the stack");
                self.stack.push(second_val / first_val);
            },
            Instructions::EQ => {
                let first_val = self.stack.pop().expect("Insufficient data on the stack");
                let second_val = self.stack.pop().expect("Insufficient data on the stack");
                if first_val == second_val {
                    self.stack.push(1);
                } else {
                    self.stack.push(0);
                }
            },
            Instructions::LT => {
                let first_val =  self.stack.pop().expect("Insufficient data on the stack");
                let second_val = self.stack.pop().expect("Insufficient data on the stack");
                if second_val < first_val {
                    self.stack.push(1);
                } else {
                    self.stack.push(0);
                }
            },
            Instructions::GT => {
                let first_val = self.stack.pop().expect("Insufficient data on the stack");
                let second_val = self.stack.pop().expect("Insufficient data on the stack");
                if second_val > first_val {
                    self.stack.push(1);
                } else {
                    self.stack.push(0);
                }
            }
            Instructions::If(nested_struct) => {
                match self.stack.pop().expect("No binary condition found") {
                    1 => {
                        for i in nested_struct.If.as_ref().unwrap() {
                            if let Some(j) = i {
                                self.evaluate_instruction(&j);
                            }
                        }
                    },
                    0 => {
                        if let Some(instr) = nested_struct.Else.as_ref() {
                            if instr.len() > 0 {
                                for i in instr {
                                    if let Some(j) = i {
                                        self.evaluate_instruction(&j);
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
                        self.evaluate_instruction(&i);
                    }
                }
                while self.stack.pop().expect("No value found on stack") == 1 {
                    for instr in &nested_struct.Contents {
                        if let Some(i) = instr {
                            self.evaluate_instruction(&i);
                        }
                    }
                    for instr in &nested_struct.Cond {
                        if let Some(i) = instr {
                            self.evaluate_instruction(&i);
                        }
                    }
                }
            },
            Instructions::VARDECLARE(nested_struct) => {
                
                for instr in &nested_struct.instructions {
                    self.evaluate_instruction(&instr.as_ref().unwrap());
                }
                self.data_stack.insert(
                    nested_struct.name.to_string(),
                    self.stack.pop().unwrap()
                );
            },
            Instructions::IDENTIFIER(data_name) => {
                if let Some(data) = self.data_stack.get(data_name) {
                    self.stack.push(*data);
                } else {
                    let mut value: &Vec<Option<Instruction>> = &Vec::new();
                    if let Some(val) = self.macro_stack.get(data_name) {
                        value = val;
                    }

                    for instr in value.to_vec() {
                        if let Some(i) = instr {
                            self.evaluate_instruction(&i);
                        }
                    }
                }
            },
            Instructions::SPAWN(name) => {
                if RESERVED_KEYWORDS.contains(&name.as_str()) {
                    eprintln!("ERROR: Cannot assign variable with name of assigned keyword ({})", name);
                    std::process::exit(1);
                }
                self.stack_stack.insert(
                    name.to_string(),
                    Vec::new()
                );
            },
            Instructions::SWITCH(name) => {
                let tmp_stack: Vec <u8>;
                self.stack = match self.stack_stack.get(name) {
                    Some(vec) => {
                        tmp_stack = vec.to_vec();
                        self.stack_stack.insert(
                            self.current_stack.to_string(),
                            self.stack.clone()
                        );
                        self.current_stack = name.to_string();
                        tmp_stack
                    },
                    None => {
                        eprintln!("ERROR: Stack with name {} not found", name);
                        std::process::exit(1);
                    }
                }
            },
            Instructions::CLOSE(name) => {
                if name == "main" {
                    eprintln!("ERROR: Cannot remove main stack");
                    std::process::exit(1);
                } else if name.to_string() == self.current_stack {
                    eprintln!("ERROR: Cannot remove stack you are currently working in");
                    std::process::exit(1);
                }
                self.stack_stack.remove(name);
            },
            Instructions::STACKS => {
                println!("Stacks: ");
                for k in self.stack_stack.keys() {println!("  {}", k)};
            },
            Instructions::STACKSIZE => {
                self.stack.push(self.stack.len() as u8);
            },
            Instructions::STACKREV => {
                self.stack.reverse();
            },
            Instructions::STRING(nested_instructions) => {
                let prev_stack = self.current_stack.clone();
                for instruction in nested_instructions {
                    if let Some(instr) = instruction {
                        self.evaluate_instruction(instr);
                    }
                }
                self.evaluate_instruction(&Instruction::new(Instructions::SWITCH(prev_stack), None));
            },
            Instructions::MACRO(nested_instructions) => {
                if RESERVED_KEYWORDS.contains(&nested_instructions.name.as_str()) {
                    eprintln!("ERROR: Cannot assign variable with name of assigned keyword ({})", nested_instructions.name);
                    std::process::exit(1);
                }
                self.macro_stack.insert(
                    nested_instructions.to_owned().name,
                    nested_instructions.to_owned().instructions
                );
            }
        }
    }

    fn simulate(&mut self) {
        self.stack_stack.insert(
            "main".to_string(),
            Vec::new()
        );

        for instruction in self.instructions {
            match &instruction {
                Some(i) => {
                    self.evaluate_instruction(&i);
                },
                None => continue
            }
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

    let mut program = Program {
        instructions: &instructions,
        stack: Vec::new(),
        current_stack: "main".to_string(),
        data_stack: &mut HashMap::new(),
        macro_stack: &mut HashMap::new(),
        stack_stack: &mut HashMap::new()
    };

    program.simulate();
    // println!("{:?}", instructions);
}
