pub mod globals {
    use colored::*;

    #[derive(PartialEq)]
    #[derive(Debug)]
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
        MACRO(String),
        IMPORT(Vec<Option<Operation>>, String),
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
        MACRO(Macro),
        IMPORT(Vec<Option<Instruction>>),
        EXIT
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum DataTypes {
        INT(u8),
        STACKPOINTER(*mut Vec<DataTypes>),
    }

    #[derive(Debug, PartialEq)]
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
        pub line_num: u8
    }

    impl Instruction {
        pub fn new(instr: Instructions, line_num: u8) -> Self {
            Instruction {
                Instruction: instr,
                line_num: line_num
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
        eprintln!("{}:{} {}: {}", file, line_num, format!("ERROR").red(), message);
        std::process::exit(1);
    }
}