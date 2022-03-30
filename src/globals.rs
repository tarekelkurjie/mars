pub mod globals {
    use crate::fmt::fmt::*;
    use std::fmt::{Display, Formatter};
    use std::fs::File;
    use std::io::prelude::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum OpCodes {
        PUSH(u8), // Begin stack manipulation
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
        SWITCH,
        CLOSE,
        STACK(String),
        THIS,
        STACKS,
        STACKSIZE, 
        STACKREV,
        STRING(Vec<Option<Operation>>), // String literal
        PROCEDURE, // Begin procedure
        IN,
        IMPORT(Vec<Option<Operation>>, String), // Begin import
        EXIT 
    }

    #[derive(Debug, Clone)]
    pub enum Instructions {
        PUSH(u8),
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
        SWITCH,
        CLOSE,
        STACK(String),
        THIS,
        STACKS,
        STACKSIZE,
        STACKREV,
        STRING(Vec<Option<Instruction>>),
        PROCEDURE(ProcedureDefine),
        IMPORT(Vec<Option<Instruction>>),
        EXIT
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum DataTypes {
        INT(u8),
        STACKPOINTER(*mut Vec<DataTypes>),
    }

    #[derive(Debug, Clone)]
    pub struct ProcedureDefine {
        pub name: String,
        pub args: Vec<String>,
        pub instructions: Vec<Instruction>,
    }

    #[derive(Debug, Clone, PartialEq)]
     pub struct Operation {
        pub OpCode: OpCodes,
        pub line_num: u8
    }

     impl Operation {
        pub fn new(opcode: OpCodes, line_num: u8) -> Self {
            Operation {
                OpCode: opcode,
                line_num: line_num
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Instruction {
        pub Instruction: Instructions,
        pub line_num: u8,
        pub file_name: String
    }

    impl Instruction {
        pub fn new(instr: Instructions, line_num: u8, file_name: String) -> Self {
            Instruction {
                Instruction: instr,
                line_num: line_num,
                file_name
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct While {
        pub Cond: Vec<Option<Instruction>>,
        pub Contents: Vec<Option<Instruction>>
    }

    impl While {
        pub fn new(cond: Vec<Option<Instruction>>, contents: Vec<Option<Instruction>>) -> Self {
            While {
                Cond: cond,
                Contents: contents
            }
        }
    }

    #[derive(Debug , Clone)]
    pub struct IfElse {
        pub If: Option<Vec<Option<Instruction>>>,
        pub Else: Option<Vec<Option<Instruction>>>
    }

    impl IfElse {
        pub fn new(IfBlock: Vec<Option<Instruction>>, ElseBlock: Vec<Option<Instruction>>) -> Self {
            IfElse {
                If: Some(IfBlock),
                Else: Some(ElseBlock)
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct VariableDefine {
        pub name: String,
        pub instructions: Vec<Option<Instruction>>
    }

    #[derive(Debug, Clone)]
    pub struct Macro {
        pub name: String,
        pub instructions: Vec<Option<Instruction>>
    }

    pub const RESERVED_KEYWORDS: [&str; 21] = [
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
        "stack",
        "this",
        "stacks",
        "stack_rev",
        "stack_size",
        "close",
        "macro"
    ];

    pub fn report_err(message: &str, file: &str, line_num: u8) -> ! {
        eprintln!("{}:{} {}: {}", file, line_num, red("error"), message);
        std::process::exit(1);
    }

    pub fn report_warn(message: &str, file: &str, line_num: u8) {
        println!("{}:{} {}: {}", file, line_num, orange("warning"), message);
    }

    pub fn output_to_file(instructions: Vec<Option<Instruction>>){
        let mut file = File::create("output.txt").unwrap();
        file.write_all(pretty_print_instructions(instructions).as_bytes()).unwrap();
    }


    fn red(input: &str) -> String {
        format!("\x1b[91m{}\x1b[0m", input)
    }
    
    fn orange(input: &str) -> String {
        format!("\x1b[93m{}\x1b[0m", input)
    }
    
}