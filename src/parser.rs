pub mod parser {
    use crate::globals::globals::*;
    use std::vec::IntoIter;

    pub struct Parser {
        operations: IntoIter<Option<Operation>>
    }

    impl Parser {
        pub fn new(data: IntoIter<Option<Operation>>) -> Self {
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
}