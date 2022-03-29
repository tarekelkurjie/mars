pub mod parser {
    use crate::globals::globals::*;
    use std::vec::IntoIter;
    use std::iter::Peekable;

    pub struct Parser {
        operations: Peekable<IntoIter<Option<Operation>>>,
        file: String
    }

    impl Parser {
        pub fn new(data: Peekable<IntoIter<Option<Operation>>>, file: String) -> Self {
            Parser {
                operations: data,
                file
            }
        }

        fn gen_instruction_from_op(&mut self, op: Operation) -> Option<Instruction> {
            match op.OpCode {
                OpCodes::PUSH(v) => return Some(Instruction::new(Instructions::PUSH(v), op.line_num, self.file.clone())),
                OpCodes::PRINT => return Some(Instruction::new(Instructions::PRINT, op.line_num, self.file.clone())),
                OpCodes::PRINTASCII => return Some(Instruction::new(Instructions::PRINTASCII, op.line_num, self.file.clone())),
                OpCodes::POP => return Some(Instruction::new(Instructions::POP, op.line_num, self.file.clone())),
                OpCodes::DUP => return Some(Instruction::new(Instructions::DUP, op.line_num, self.file.clone())),
                OpCodes::SWAP => return Some(Instruction::new(Instructions::SWAP, op.line_num, self.file.clone())),
                OpCodes::ADD => return Some(Instruction::new(Instructions::ADD, op.line_num, self.file.clone())),
                OpCodes::SUB => return Some(Instruction::new(Instructions::SUB, op.line_num, self.file.clone())),
                OpCodes::EQ => return Some(Instruction::new(Instructions::EQ, op.line_num, self.file.clone())),
                OpCodes::LT => return Some(Instruction::new(Instructions::LT, op.line_num, self.file.clone())),
                OpCodes::GT => return Some(Instruction::new(Instructions::GT, op.line_num, self.file.clone())),
                OpCodes::MULT => return Some(Instruction::new(Instructions::MULT, op.line_num, self.file.clone())),
                OpCodes::DIV => return Some(Instruction::new(Instructions::DIV, op.line_num, self.file.clone())),
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
                                            ), op.line_num, self.file.clone()));
                                    } else if j.OpCode == OpCodes::END {
                                        return Some(Instruction::new(
                                            Instructions::If(
                                                IfElse::new(
                                                    if_block, else_block
                                                )
                                            ), op.line_num, self.file.clone()));
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
                        ), op.line_num, self.file.clone()));
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
                                                op.line_num, self.file.clone()
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
                        ), op.line_num, self.file.clone()
                    )
                    )

                },
                OpCodes::END => report_err("'end' statement found without matching block", self.file.as_str(), op.line_num),
                OpCodes::ELSE => report_err("'else' statement found without match 'if'", self.file.as_str(), op.line_num),
                OpCodes::DO => report_err("'do' statement found without matching block", self.file.as_str(), op.line_num),
                OpCodes::VARDECLARE(name) => {
                    let mut instr: Vec<Option<Instruction>> = Vec::new();

                    while let Some(i) = self.operations.next() {
                        if let Some(j) = i {
                            if j.OpCode != OpCodes::DEFINE {
                                instr.push(self.gen_instruction_from_op(j));
                            } else {
                                if RESERVED_KEYWORDS.contains(&name.as_str()) {
                                    report_err(format!("Cannot assign variable with name of assigned keyword ({})", name).as_str(), self.file.as_str(), op.line_num);
                                }
                                return Some(Instruction::new(Instructions::VARDECLARE(VariableDefine {name: name.to_string(), instructions: instr}), op.line_num, self.file.clone()));
                            }
                        }
                    }
                    return Some(Instruction::new(Instructions::VARDECLARE(VariableDefine {name: name.to_string(), instructions: instr}), op.line_num, self.file.clone()));
                },
                OpCodes::DEFINE => report_err("'def' statement found without matching variable declaration", self.file.as_str(), op.line_num),
                OpCodes::IDENTIFIER(name) => Some(Instruction::new(Instructions::IDENTIFIER(name), op.line_num, self.file.clone())),
                OpCodes::SPAWN(name) => Some(Instruction::new(Instructions::SPAWN(name), op.line_num, self.file.clone())),
                OpCodes::SWITCH => Some(Instruction::new(Instructions::SWITCH, op.line_num, self.file.clone())),
                OpCodes::CLOSE => Some(Instruction::new(Instructions::CLOSE, op.line_num, self.file.clone())),
                OpCodes::STACK(name) => Some(Instruction::new(Instructions::STACK(name), op.line_num, self.file.clone())),
                OpCodes::THIS => Some(Instruction::new(Instructions::THIS, op.line_num, self.file.clone())),
                OpCodes::STACKS => Some(Instruction::new(Instructions::STACKS, op.line_num, self.file.clone())),
                OpCodes::STACKSIZE => Some(Instruction::new(Instructions::STACKSIZE, op.line_num, self.file.clone())),
                OpCodes::STRING(contents) => {
                    let mut instrs = Vec::new();
                    for i in contents {
                        if let Some(instr)= i {
                            instrs.push(self.gen_instruction_from_op(instr));
                        }
                    }
                    Some(Instruction::new(Instructions::STRING(instrs), op.line_num, self.file.clone()))
                },
                OpCodes::STACKREV => Some(Instruction::new(Instructions::STACKREV, op.line_num, self.file.clone())),
                OpCodes::PROCEDURE => {
                    let operation = self.operations.next().unwrap_or_else(|| report_err("'procedure' statement found without matching block", self.file.as_str(), op.line_num));

                    if let OpCodes::IDENTIFIER(name) = operation.unwrap().OpCode {
                        let slice: Vec<Option<Operation>> = self.operations.clone().collect();

                        let mut args = Vec::new();
                        let mut instructions = Vec::new();

                        if self.operations.next().unwrap_or_else(|| report_err("'procedure' statement found with excessive name parameters", self.file.as_str(), op.line_num)).unwrap().OpCode == OpCodes::IN {
                            loop {
                                let operation = self.operations.peek().unwrap_or_else(|| report_err("'procedure' statement found with excessive name parameters", self.file.as_str(), op.line_num));
                                if let OpCodes::IDENTIFIER(name) = &operation.as_ref().unwrap().OpCode {
                                    args.push(name.to_string());
                                    self.operations.next();
                                } else {
                                    break;
                                }
                            }
                            if self.operations.next().unwrap_or_else(|| report_err("'procedure' statement found without body", self.file.as_str(), op.line_num)).unwrap().OpCode == OpCodes::DO {
                                while let Some(i) = self.operations.next() {
                                    if let Some(j) = i {
                                        if j.OpCode != OpCodes::END {
                                            instructions.push(self.gen_instruction_from_op(j).unwrap());
                                        } else {
                                            return Some(Instruction::new(Instructions::PROCEDURE(ProcedureDefine {
                                                name: name.to_string(),
                                                args: args,
                                                instructions: instructions
                                            }), op.line_num, self.file.clone()));
                                        }
                                    }
                                }
                            }
                        }
                        else if self.operations.next().unwrap_or_else(|| report_err("'procedure' statement found without body", self.file.as_str(), op.line_num)).unwrap().OpCode == OpCodes::DO {
                            while let Some(i) = self.operations.next() {
                                if let Some(j) = i {
                                    if j.OpCode != OpCodes::END {
                                        instructions.push(self.gen_instruction_from_op(j).unwrap());
                                    } else {
                                        return Some(Instruction::new(Instructions::PROCEDURE(ProcedureDefine {
                                            name: name.to_string(),
                                            args: args,
                                            instructions: instructions
                                        }), op.line_num, self.file.clone()));
                                    }
                                }
                            }
                        }
                        else {
                            report_err("'procedure' statement found with unfinished definition", self.file.as_str(), op.line_num);
                        }
                    }
                    report_err("'procedure' statement found with unfinished definition", self.file.as_str(), op.line_num);
                },
                OpCodes::IN => report_err("'in' statement found without matching 'procedure'", self.file.as_str(), op.line_num),
                OpCodes::IMPORT(ops, file_path) => {
                    let parse = Parser {
                        operations: ops.into_iter().peekable(),
                        file: file_path
                    };

                    let mut instrs = Vec::new();
                    for i in parse {
                        instrs.push(Some(i));
                    }

                    Some(Instruction::new(Instructions::IMPORT(instrs), op.line_num, self.file.clone()))
                },
                OpCodes::EXIT => Some(Instruction::new(Instructions::EXIT, op.line_num, self.file.clone()))
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