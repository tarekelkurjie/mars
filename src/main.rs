#![allow(non_snake_case)]
#![allow(unreachable_patterns)]

mod globals;
mod lex;
mod parser;
mod interpreter;

use globals::globals::*;
use globals::colorize::*;
use lex::lex::Lexer;
use parser::parser::Parser;
use interpreter::program::Program;

use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        print_usage();
    }

    let mut operations: Vec<Option<Operation>> = Vec::new();

    if args[2].split(".").last().unwrap() != "mars" {
        eprintln!("{}: unsupported file type \"{}\"", red("error"), args[2].split(".").last().unwrap());
        std::process::exit(1);
    } 
    let lex = Lexer::from_file(&args[2]).unwrap();

    for token in lex {
        operations.push(Some(token));
    }

    // println!("{:?}", operations);

    let parse = Parser::new(operations.into_iter().peekable(), args[1].to_string());

    let mut instructions = Vec::new();

    for instr in parse {
        instructions.push(Some(instr));
    }

    if args[1] == "debug" {
        output_to_file(instructions.to_vec());
    } else if args[1] == "sim" {
        let mut program = Program {
            instructions: &mut instructions,
            stack: &mut Vec::new(),
            current_stack: None,
            data_stack: &mut HashMap::new(),
            proc_stack: &mut HashMap::new(),
            stack_stack: &mut HashMap::new(),
            names: &mut HashMap::new(),
            file: args[1].to_string(),
            current_index: 0
        };
    
        program.current_stack = Some(program.stack as *mut Vec<DataTypes>);
    
    
        program.simulate();
    } else if args[1] == "com" {
        eprintln!("Compilation not yet supported");
        std::process::exit(1);
    } else {
        print_usage();
    }
}

fn print_usage() -> ! {
    eprintln!("Usage: cargo run [debug][sim][com(WIP)] <filepath>");
    std::process::exit(1);
}