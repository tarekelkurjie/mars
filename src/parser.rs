pub mod parser {
    use crate::globals::globals::*;
    use std::vec::IntoIter;

    pub struct Parser {
        operations: IntoIter<Option<Operation>>,
        file: String
    }

    impl Parser {
        pub fn new(data: IntoIter<Option<Operation>>, file: String) -> Self {
            Parser {
                operations: data,
                file
            }
        }

        fn gen_instruction_from_op(&mut self, op: Operation) -> Option<Instruction> {
            match op.OpCode {
                OpCodes::PUSH(v) => return Some(Instruction::new(Instructions::PUSH(v), op.line_num)),
                OpCodes::PRINT => return Some(Instruction::new(Instructions::PRINT, op.line_num)),
                OpCodes::PRINTASCII => return Some(Instruction::new(Instructions::PRINTASCII, op.line_num)),
                OpCodes::POP => return Some(Instruction::new(Instructions::POP, op.line_num)),
                OpCodes::DUP => return Some(Instruction::new(Instructions::DUP, op.line_num)),
                OpCodes::SWAP => return Some(Instruction::new(Instructions::SWAP, op.line_num)),
                OpCodes::ADD => return Some(Instruction::new(Instructions::ADD, op.line_num)),
                OpCodes::SUB => return Some(Instruction::new(Instructions::SUB, op.line_num)),
                OpCodes::EQ => return Some(Instruction::new(Instructions::EQ, op.line_num)),
                OpCodes::LT => return Some(Instruction::new(Instructions::LT, op.line_num)),
                OpCodes::GT => return Some(Instruction::new(Instructions::GT, op.line_num)),
                OpCodes::MULT => return Some(Instruction::new(Instructions::MULT, op.line_num)),
                OpCodes::DIV => return Some(Instruction::new(Instructions::DIV, op.line_num)),
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
                                            ), op.line_num));
                                    } else if j.OpCode == OpCodes::END {
                                        return Some(Instruction::new(
                                            Instructions::If(
                                                IfElse::new(
                                                    if_block, else_block
                                                )
                                            ), op.line_num));
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
                        ), op.line_num));
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
                                                op.line_num
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
                        ), op.line_num
                    )
                    )

                },
                OpCodes::END => report_err("ERROR: 'end' statement found without matching block", self.file.as_str(), op.line_num),
                OpCodes::ELSE => report_err("ERROR: 'else' statement found without match 'if'", self.file.as_str(), op.line_num),
                OpCodes::DO => report_err("ERROR: 'do' statement found without matching block", self.file.as_str(), op.line_num),
                OpCodes::VARDECLARE(name) => {
                    let mut instr: Vec<Option<Instruction>> = Vec::new();

                    while let Some(i) = self.operations.next() {
                        if let Some(j) = i {
                            if j.OpCode != OpCodes::DEFINE {
                                instr.push(self.gen_instruction_from_op(j));
                            } else {
                                if RESERVED_KEYWORDS.contains(&name.as_str()) {
                                    report_err(format!("ERROR: Cannot assign variable with name of assigned keyword ({})", name).as_str(), self.file.as_str(), op.line_num);
                                }
                                return Some(Instruction::new(Instructions::VARDECLARE(VariableDefine {name: name.to_string(), instructions: instr}), op.line_num));
                            }
                        }
                    }
                    return Some(Instruction::new(Instructions::VARDECLARE(VariableDefine {name: name.to_string(), instructions: instr}), op.line_num));
                },
                OpCodes::DEFINE => report_err("ERROR: 'def' statement found without matching variable declaration", self.file.as_str(), op.line_num),
                OpCodes::IDENTIFIER(name) => Some(Instruction::new(Instructions::IDENTIFIER(name), op.line_num)),
                OpCodes::SPAWN(name) => Some(Instruction::new(Instructions::SPAWN(name), op.line_num)),
                OpCodes::SWITCH => Some(Instruction::new(Instructions::SWITCH, op.line_num)),
                OpCodes::CLOSE => Some(Instruction::new(Instructions::CLOSE, op.line_num)),
                OpCodes::STACK(name) => Some(Instruction::new(Instructions::STACK(name), op.line_num)),
                OpCodes::THIS => Some(Instruction::new(Instructions::THIS, op.line_num)),
                OpCodes::STACKS => Some(Instruction::new(Instructions::STACKS, op.line_num)),
                OpCodes::STACKSIZE => Some(Instruction::new(Instructions::STACKSIZE, op.line_num)),
                OpCodes::STRING(contents) => {
                    let mut instrs = Vec::new();
                    for i in contents {
                        if let Some(instr)= i {
                            instrs.push(self.gen_instruction_from_op(instr));
                        }
                    }
                    Some(Instruction::new(Instructions::STRING(instrs), op.line_num))
                },
                OpCodes::STACKREV => Some(Instruction::new(Instructions::STACKREV, op.line_num)),
                OpCodes::MACRO(name) => {
                    let mut instrs: Vec<Option<Instruction>> = Vec::new();

                    while let Some(i) = self.operations.next() {
                        match i {
                            Some(j) => {
                                if j.OpCode != OpCodes::END {
                                    instrs.push(self.gen_instruction_from_op(j))
                                } else {return Some(Instruction::new(Instructions::MACRO( Macro { name: name, instructions: instrs}), op.line_num))}
                            },
                            None => continue
                        }
                    }
                    Some(Instruction::new(Instructions::MACRO( Macro { name: name, instructions: instrs}), op.line_num))
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