pub mod program {
    use crate::globals::globals::*;
    use std::collections::HashMap;

    #[derive(Debug, Clone, PartialEq)]
    pub enum StorageTypes {
        Stack,
        Variable,
        Procedure
    }

    pub struct Program<'a> {
        pub instructions: &'a Vec<Option<Instruction>>,
        pub stack: &'a mut Vec<DataTypes>,
        pub current_stack: Option<*mut Vec<DataTypes>>,
        pub data_stack: &'a mut HashMap<String, DataTypes>,
        pub proc_stack: &'a mut HashMap<String, ProcedureDefine>,
        pub stack_stack: &'a mut HashMap<String, Vec<DataTypes>>,
        pub names: &'a mut HashMap<String, StorageTypes>,
        pub file: String,
    }

    impl<'a> Program<'a> {
        fn evaluate_instruction(&mut self, instruction: &Instruction) {
            match &instruction.Instruction {
                Instructions::PUSH(val) => {
                    self.stack.push(DataTypes::INT(val.clone()));
                },
                Instructions::PRINT => {
                    if let Some(v) = self.stack.pop() {
                        match v {
                            DataTypes::INT(u) => println!("{:?}", u),
                            DataTypes::STACKPOINTER(p) => println!("{:?}", p),
                            _ => report_err("Cannot print non-numeric types", instruction.file_name.as_str(), instruction.line_num.clone())
                        }
                    }
                },
                Instructions::PRINTASCII => {
                    print!("{}", match self.stack.pop().unwrap_or_else(|| report_err("Cannot pop value from empty stack", instruction.file_name.as_str(), instruction.line_num.clone())) {
                        DataTypes::INT(u) => u as char,
                        _ => report_err("Cannot print non-numeric values as ASCII", instruction.file_name.as_str(), instruction.line_num.clone()),
                    });
                }
                Instructions::POP => {
                    self.stack.pop();
                },
                Instructions::DUP => {
                    match self.stack.pop().unwrap_or_else(|| report_err("No data on stack to duplicate", instruction.file_name.as_str(), instruction.line_num.clone())) {
                        DataTypes::INT(u) => {
                            self.stack.push(DataTypes::INT(u));
                            self.stack.push(DataTypes::INT(u));
                        },
                        DataTypes::STACKPOINTER(p) => {
                            self.stack.push(DataTypes::STACKPOINTER(p));
                            self.stack.push(DataTypes::STACKPOINTER(p));
                        },
                        _ => report_err("Cannot duplicate extraneous types", instruction.file_name.as_str(), instruction.line_num.clone())
                    }
                },
                Instructions::SWAP => {
                    let first_val = self.stack.pop().unwrap_or_else(|| report_err("Insufficient data on the stack", instruction.file_name.as_str(), instruction.line_num.clone()));
                    let second_val = self.stack.pop().unwrap_or_else(|| report_err("Insufficient data on the stack", instruction.file_name.as_str(), instruction.line_num.clone()));
                    self.stack.push(first_val);
                    self.stack.push(second_val);
                },
                Instructions::ADD => {
                    let first_val = match self.stack.pop().unwrap_or_else(|| report_err("Insufficient data on the stack", instruction.file_name.as_str(), instruction.line_num.clone())) {
                      DataTypes::INT(u) => u,
                      _ => {report_err("Cannot perform arithmetic operations on non-numeric values", instruction.file_name.as_str(), instruction.line_num.clone()); }
                    };
                    let second_val = match self.stack.pop().unwrap_or_else(|| report_err("Insufficient data on the stack", instruction.file_name.as_str(), instruction.line_num.clone())) {
                        DataTypes::INT(u) => u,
                        _ => {report_err("Cannot perform arithmetic operations on non-numeric values", instruction.file_name.as_str(), instruction.line_num.clone()); }
                    };
                    self.stack.push(DataTypes::INT(second_val + first_val));
                },
                Instructions::SUB => {
                    let first_val = match self.stack.pop().unwrap_or_else(|| report_err("Insufficient data on the stack", instruction.file_name.as_str(), instruction.line_num.clone())) {
                        DataTypes::INT(u) => u,
                        _ => {report_err("Cannot perform arithmetic operations on non-numeric values", instruction.file_name.as_str(), instruction.line_num.clone()); }
                    };
                    let second_val = match self.stack.pop().unwrap_or_else(|| report_err("Insufficient data on the stack", instruction.file_name.as_str(), instruction.line_num.clone())) {
                        DataTypes::INT(u) => u,
                        _ => {report_err("Cannot perform arithmetic operations on non-numeric values", instruction.file_name.as_str(), instruction.line_num.clone()); }
                    };
                    self.stack.push(DataTypes::INT(second_val - first_val));
                },
                Instructions::MULT => {
                    let first_val = match self.stack.pop().unwrap_or_else(|| report_err("Insufficient data on the stack", instruction.file_name.as_str(), instruction.line_num.clone())) {
                        DataTypes::INT(u) => u,
                        _ => {report_err("Cannot perform arithmetic operations on non-numeric values", instruction.file_name.as_str(), instruction.line_num.clone()); }
                    };
                    let second_val = match self.stack.pop().unwrap_or_else(|| report_err("Insufficient data on the stack", instruction.file_name.as_str(), instruction.line_num.clone())) {
                        DataTypes::INT(u) => u,
                        _ => {report_err("Cannot perform arithmetic operations on non-numeric values", instruction.file_name.as_str(), instruction.line_num.clone()); }
                    };
                    self.stack.push(DataTypes::INT(second_val * first_val));
                },
                Instructions::DIV => {
                    let first_val = match self.stack.pop().unwrap_or_else(|| report_err("Insufficient data on the stack", instruction.file_name.as_str(), instruction.line_num.clone())) {
                        DataTypes::INT(u) => u,
                        _ => {report_err("Cannot perform arithmetic operations on non-numeric values", instruction.file_name.as_str(), instruction.line_num.clone()); }
                    };
                    let second_val = match self.stack.pop().unwrap_or_else(|| report_err("Insufficient data on the stack", instruction.file_name.as_str(), instruction.line_num.clone())) {
                        DataTypes::INT(u) => u,
                        _ => {report_err("Cannot perform arithmetic operations on non-numeric values", instruction.file_name.as_str(), instruction.line_num.clone()); }
                    };
                    self.stack.push(DataTypes::INT(second_val / first_val));
                },
                Instructions::EQ => {
                    let first_val = match self.stack.pop().unwrap_or_else(|| report_err("Insufficient data on the stack", instruction.file_name.as_str(), instruction.line_num.clone())) {
                        DataTypes::INT(u) => u,
                        _ => {report_err("Cannot perform arithmetic operations on non-numeric values", instruction.file_name.as_str(), instruction.line_num.clone()); }
                    };
                    let second_val = match self.stack.pop().unwrap_or_else(|| report_err("Insufficient data on the stack", instruction.file_name.as_str(), instruction.line_num.clone())) {
                        DataTypes::INT(u) => u,
                        _ => {report_err("Cannot perform arithmetic operations on non-numeric values", instruction.file_name.as_str(), instruction.line_num.clone()); }
                    };
                    if second_val == first_val {
                        self.stack.push(DataTypes::INT(1));
                    } else {
                        self.stack.push(DataTypes::INT(0));
                    }
                },
                Instructions::LT => {
                    let first_val = match self.stack.pop().unwrap_or_else(|| report_err("Insufficient data on the stack", instruction.file_name.as_str(), instruction.line_num.clone())) {
                        DataTypes::INT(u) => u,
                        _ => {report_err("Cannot perform comparative operations on non-numeric values", instruction.file_name.as_str(), instruction.line_num.clone()); }
                    };
                    let second_val = match self.stack.pop().unwrap_or_else(|| report_err("Insufficient data on the stack", instruction.file_name.as_str(), instruction.line_num.clone())) {
                        DataTypes::INT(u) => u,
                        _ => {report_err("Cannot perform comparative operations on non-numeric values", instruction.file_name.as_str(), instruction.line_num.clone()); }
                    };
                    if second_val < first_val {
                        self.stack.push(DataTypes::INT(1));
                    } else {
                        self.stack.push(DataTypes::INT(0));
                    }
                },
                Instructions::GT => {
                    let first_val = match self.stack.pop().unwrap_or_else(|| report_err("Insufficient data on the stack", instruction.file_name.as_str(), instruction.line_num.clone())) {
                        DataTypes::INT(u) => u,
                        _ => {report_err("Cannot perform comparative operations on non-numeric values", instruction.file_name.as_str(), instruction.line_num.clone()); }
                    };
                    let second_val = match self.stack.pop().unwrap_or_else(|| report_err("Insufficient data on the stack", instruction.file_name.as_str(), instruction.line_num.clone())) {
                        DataTypes::INT(u) => u,
                        _ => {report_err("Cannot perform comparative operations on non-numeric values", instruction.file_name.as_str(), instruction.line_num.clone()); }
                    };
                    if second_val > first_val {
                        self.stack.push(DataTypes::INT(1));
                    } else {
                        self.stack.push(DataTypes::INT(0));
                    }
                }
                Instructions::If(nested_struct) => {
                    match self.stack.pop().expect("No binary condition found") {
                        DataTypes::INT(1) => {
                            for i in nested_struct.If.as_ref().unwrap() {
                                if let Some(j) = i {
                                    self.evaluate_instruction(&j);
                                }
                            }
                        },
                        DataTypes::INT(0) => {
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
                        _ => report_err("Binary boolean not found", instruction.file_name.as_str(), instruction.line_num.clone())
                    }
                },
                Instructions::While(nested_struct) => {
                    for instr in &nested_struct.Cond {
                        if let Some(i) = instr {
                            self.evaluate_instruction(&i);
                        }
                    }
                    while self.stack.pop().expect("No value found on stack") == DataTypes::INT(1) {
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
                    if let None = self.names.get(&nested_struct.name.to_string()) {
                        self.names.insert(nested_struct.name.to_string(), StorageTypes::Variable);
                        for instr in &nested_struct.instructions {
                            self.evaluate_instruction(&instr.as_ref().unwrap());
                        }
                        self.data_stack.insert(
                            nested_struct.name.to_string(),
                            self.stack.pop().unwrap_or_else(|| report_err(format!("No data on stack to assign to variable {}", &nested_struct.name).as_str(), instruction.file_name.as_str(), instruction.line_num.clone()))
                        );
                    } else {
                        if self.names.get(&nested_struct.name).unwrap() != &StorageTypes::Variable {
                            report_err(format!("{} with name '{}' already exists", match &self.names.get(&nested_struct.name) {
                                Some(StorageTypes::Procedure) => "Procedure",
                                Some(StorageTypes::Variable) => "Variable",
                                Some(StorageTypes::Stack) => "Stack",
                                None => "Unknown",
                            }, nested_struct.name).as_str(), instruction.file_name.as_str(), instruction.line_num.clone())
                        }
                    }
                },
                Instructions::IDENTIFIER(data_name) => {
                    if let Some(data) = self.data_stack.get(data_name) {
                        self.stack.push(data.clone());
                    } else if let Some(data) = self.proc_stack.clone().get(data_name) {
                        for i in data.args.iter() {
                            self.data_stack.insert(i.to_string(), self.stack.pop().unwrap_or_else(|| report_err("No value on stack to assign to parameter", instruction.file_name.as_str(), instruction.line_num.clone())));
                        }

                        for instr in &data.instructions.to_vec() {   
                            self.evaluate_instruction(&instr);
                        }

                        for i in data.args.iter() {
                            self.data_stack.remove(&i.to_string());
                        }
                    }
                },
                Instructions::SPAWN(name) => {
                    if RESERVED_KEYWORDS.contains(&name.as_str()) { report_err(format!("ERROR: Cannot assign variable with name of assigned keyword ({})", name).as_str(), instruction.file_name.as_str(), instruction.line_num.clone()); }
                    if let None = self.names.get(name) {
                        self.names.insert(name.to_string(), StorageTypes::Stack);
                        self.stack_stack.insert(
                            name.to_string(),
                            Vec::new()
                        );
                        self.stack.push(DataTypes::STACKPOINTER(self.stack_stack.get_mut(name).unwrap() as *mut Vec<DataTypes>))
                    } else {
                        report_err(format!("{} with name '{}' already exists", match &self.names.get(name) {
                            Some(StorageTypes::Procedure) => "Procedure",
                            Some(StorageTypes::Variable) => "Variable",
                            Some(StorageTypes::Stack) => "Stack",
                            None => "Unknown",
                        }, name).as_str(), instruction.file_name.as_str(), instruction.line_num.clone());
                    }
                },
                Instructions::SWITCH => {
                    if let Some(value) = self.stack.pop() {
                        match value {
                            DataTypes::STACKPOINTER(p) => {
                                unsafe {
                                    self.stack = &mut *p as &mut Vec<DataTypes>;
                                    self.current_stack = Some(p);
                                }
                            },
                            _ => report_err("Cannot switch to pointer with non-stack type", instruction.file_name.as_str(), instruction.line_num.clone()),
                        }
                    }
                },
                Instructions::CLOSE => {
                    let name: Option<String> = None;
                    let top = self.stack.pop();
                    for k in self.stack_stack.clone().keys() {
                        let r = self.stack_stack.get_mut(k).unwrap();
                        let p1 = r as *mut Vec<DataTypes>;
                        let mut p2 = None;
                        if let Some(v) = top.clone() {
                            p2 = match v {
                                DataTypes::STACKPOINTER(p) => Some(p),
                                _ => report_err("Cannot swtich to non-pointer type", instruction.file_name.as_str(), instruction.line_num.clone()),
                            };
                        }

                        if let Some(p) = p2 {
                            if p1 == p {
                                self.stack_stack.remove(k);
                            }
                        }
                    }

                    if let Some(hash_name) = name {
                        self.stack_stack.remove(hash_name.as_str());
                    }
                },
                Instructions::STACK(name) => {
                  self.stack.push(DataTypes::STACKPOINTER(self.stack_stack.get_mut(name.as_str()).unwrap_or_else(|| report_err(format!("Cannot locate function with name {}", name).as_str(), instruction.file_name.as_str(), instruction.line_num.clone())) as *mut Vec<DataTypes>));
                },
                Instructions::THIS => {
                  self.stack.push(DataTypes::STACKPOINTER(self.current_stack.unwrap()));
                },
                Instructions::STACKS => {
                    println!("Stacks: ");
                    for k in self.stack_stack.keys() {println!("  {}", k)};
                },
                Instructions::STACKSIZE => {
                    self.stack.push(DataTypes::INT(self.stack.len() as u8));
                },
                Instructions::STACKREV => {
                    self.stack.reverse();
                },
                Instructions::STRING(nested_instructions) => {
                    for instruction in nested_instructions {
                        if let Some(instr) = instruction {
                            self.evaluate_instruction(instr);
                        }
                    }
                },
                Instructions::PROCEDURE(nested_struct) => {
                    if let None = self.names.get(&nested_struct.name.to_string()) {
                        self.names.insert(nested_struct.name.to_string(), StorageTypes::Procedure);
                        self.proc_stack.insert(
                            nested_struct.name.to_string(),
                            nested_struct.clone()
                        );
                    } else {
                        report_err(format!("{} with name '{}' already exists", match &self.names.get(&nested_struct.name.to_string()) {
                            Some(StorageTypes::Procedure) => "Procedure",
                            Some(StorageTypes::Variable) => "Variable",
                            Some(StorageTypes::Stack) => "Stack",
                            None => "Unknown",
                        }, nested_struct.name).as_str(), instruction.file_name.as_str(), instruction.line_num.clone());
                    }
                }
                Instructions::IMPORT(nested_instructions) => {
                    for instr in nested_instructions {
                        self.evaluate_instruction(&instr.as_ref().unwrap());
                    }
                },
                Instructions::EXIT => {
                    let code = self.stack.pop().unwrap_or_else(|| report_err("No exit code to exit with", instruction.file_name.as_str(), instruction.line_num));
                    if let DataTypes::INT(exit_code) = code {
                        std::process::exit(exit_code as i32);
                    } else {report_err("Cannot exit with status as pointer", instruction.file_name.as_str(), instruction.line_num);}
                }
            }
        }

        pub fn simulate(&mut self) {
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
}