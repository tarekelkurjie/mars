pub mod globals {
    use crate::globals::fmt::*;
    use crate::globals::colorize::*;

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
        STAR,
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
        DROP,
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
        RETURN,
        IMPORT(Vec<Option<Operation>>, String), // Begin import
        EXIT 
    }

    #[derive(Debug, Clone, PartialEq)]
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
        DROP(String),
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

    #[derive(Debug, Clone, PartialEq)]
    pub struct ProcedureDefine {
        pub name: String,
        pub args: Vec<String>,
        pub instructions: Vec<Instruction>,
        pub returns: bool
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

    #[derive(Debug, Clone, PartialEq)]
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

    #[derive(Debug, Clone, PartialEq)]
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

    #[derive(Debug, Clone, PartialEq)]
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

    #[derive(Debug, Clone, PartialEq)]
    pub struct VariableDefine {
        pub name: String,
        pub instructions: Vec<Option<Instruction>>
    }

    #[derive(Debug, Clone, PartialEq)]
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
        println!("Outputting to file {}", blue("output.txt"));
        file.write_all(pretty_print_instructions(instructions).as_bytes()).unwrap();
    }
}

pub mod colorize {
    pub fn blue(input: &str) -> String {
        format!("\x1b[94m{}\x1b[0m", input)
    }

    pub fn red(input: &str) -> String {
        format!("\x1b[91m{}\x1b[0m", input)
    }
    
    pub fn orange(input: &str) -> String {
        format!("\x1b[93m{}\x1b[0m", input)
    }
}

pub mod fmt {
    use std::fmt::{Display, Formatter};
    
    use crate::globals::globals::*;

    impl Display for Instructions {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            match self {
                Instructions::PUSH(x) => write!(f, "PUSH {}\n", x),
                Instructions::POP => write!(f, "POP\n"),
                Instructions::PRINT => write!(f, "PRINT\n"),
                Instructions::PRINTASCII => write!(f, "PRINTASCII\n"),
                Instructions::DUP => write!(f, "DUP\n"),
                Instructions::SWAP => write!(f, "SWAP\n"),
                Instructions::ADD => write!(f, "ADD\n"),
                Instructions::SUB => write!(f, "SUB\n"),
                Instructions::MULT => write!(f, "MULT\n"),
                Instructions::DIV => write!(f, "DIV\n"),
                Instructions::EQ => write!(f, "EQ\n"),
                Instructions::LT => write!(f, "LT\n"),
                Instructions::GT => write!(f, "GT\n"),
                Instructions::VARDECLARE(x) => write!(f, "{}\n", x),
                Instructions::DROP(x) => write!(f, "DROP {}\n", x),
                Instructions::IDENTIFIER(x) => write!(f, "IDENTIFIER {:?}\n", x),
                Instructions::If(x) => write!(f, "IF {:?}\n", x),
                Instructions::While(x) => write!(f, "{}", x),
                Instructions::SPAWN(x) => write!(f, "SPAWN {:?}\n", x),
                Instructions::SWITCH => write!(f, "SWITCH\n"),
                Instructions::CLOSE => write!(f, "CLOSE\n"),
                Instructions::STACK(x) => write!(f, "STACK {:?}\n", x),
                Instructions::THIS => write!(f, "THIS\n"),
                Instructions::STACKS => write!(f, "STACKS\n"),
                Instructions::STACKSIZE => write!(f, "STACKSIZE\n"),
                Instructions::STACKREV => write!(f, "STACKREV\n"),
                Instructions::STRING(x) => write!(f, "STRING {}\n", pretty_print_instructions(x.to_vec())),
                Instructions::PROCEDURE(x) => {write!(f, "{}\n", x)},
                Instructions::IMPORT(x) => write!(f, "IMPORT {:?}\n{}ENDIMPORT\n\n", x.to_vec().into_iter().next().unwrap().unwrap().file_name, pretty_print_instructions(x.to_vec())),
                Instructions::EXIT => write!(f, "EXIT\n")
            }
        }
    }

    impl Display for Instruction {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            self.Instruction.fmt(f)
        }
    }

    impl Display for ProcedureDefine {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "PROCEDURE {}", self.name)?;
            for arg in self.args.iter() {
                write!(f, "\nARG {}\n", arg)?;
            }
            for instruction in self.instructions.iter() {
                write!(f, "{}", instruction)?;
            }
            write!(f, "ENDPROCEDURE\n")
        }
    }

    impl Display for VariableDefine {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "VARIABLE {}\n", self.name)?;
            for instruction in self.instructions.iter() {
                write!(f, "{}", instruction.clone().unwrap())?;
            }
            write!(f, "ENDVARIABLE\n")
        }
    }

    impl Display for While {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            for instruction in self.Cond.iter() {
                write!(f, "{}", instruction.clone().unwrap())?;
            }
            for instruction in self.Contents.iter() {
                write!(f, "{}", instruction.clone().unwrap())?;
            }
            write!(f, "ENDWHILE\n")
        }
    }

    pub fn pretty_print_instructions(instructions: Vec<Option<Instruction>>) -> String {
        let mut res = String::new();
        for instr in instructions {
            res.push_str(format!("{}", instr.unwrap()).as_str());
        }
        res
    }
}