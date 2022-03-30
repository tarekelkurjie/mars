#![allow(non_snake_case)]
#![allow(unreachable_patterns)]

mod globals;
mod lex;
mod parser;
mod interpreter;
mod fmt;

use globals::globals::*;
use lex::lex::Lexer;
use parser::parser::Parser;
use interpreter::program::Program;

use std::collections::HashMap;
use std::env;

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

    let parse = Parser::new(operations.into_iter().peekable(), args[1].to_string());

    let mut instructions = Vec::new();

    for instr in parse {
        instructions.push(Some(instr));
    }

    let mut program = Program {
        instructions: &instructions,
        stack: &mut Vec::new(),
        current_stack: None,
        data_stack: &mut HashMap::new(),
        proc_stack: &mut HashMap::new(),
        stack_stack: &mut HashMap::new(),
        names: &mut HashMap::new(),
        file: args[1].to_string()
    };

    program.current_stack = Some(program.stack as *mut Vec<DataTypes>);


    program.simulate();
    // output_to_file(instructions);
}
