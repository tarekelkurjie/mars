pub mod fmt {
    use std::fmt::{Display, Formatter};
    use std::io::prelude::*;
    
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