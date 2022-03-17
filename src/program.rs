pub mod program {
    use crate::globals::globals::*;
    use std::collections::HashMap;

    pub struct Program<'a> {
        pub instructions: &'a Vec<Option<Instruction>>,
        pub stack: Vec<DataTypes>,
        pub current_stack: String,
        pub data_stack: &'a mut HashMap<String, u8>,
        pub macro_stack: &'a mut HashMap<String, Vec<Option<Instruction>>>,
        pub stack_stack: &'a mut HashMap<String, Vec<DataTypes>>,
    }

    impl<'a> Program<'a> {
        fn evaluate_instruction(&mut self, instruction: &Instruction) {
            match &instruction.Instruction {
                Instructions::PUSH => {
                    self.stack.push(DataTypes::INT(instruction.Contents.expect("ERROR: No data found to push to stack")));
                },
                Instructions::PRINT => {
                    println!("{:?}", match self.stack.pop().expect("Cannot pop value from empty stack") {
                        DataTypes::INT(u) => u,
                        _ => {report_err("Cannot print non-numeric values"); std::process::exit(1);}
                    });
                },
                Instructions::PRINTASCII => {
                    print!("{}", match self.stack.pop().expect("Cannot pop value from empty stack") {
                        DataTypes::INT(u) => u as char,
                        _ => {
                            report_err("Cannot print non-numeric values as ASCII");
                            std::process::exit(1);
                        }
                    });
                }
                Instructions::POP => {
                    self.stack.pop();
                },
                Instructions::DUP => {
                    match self.stack.pop().expect("ERROR: No data on stack to duplicate") {
                        DataTypes::INT(u) => {
                            self.stack.push(DataTypes::INT(u));
                            self.stack.push(DataTypes::INT(u));
                        },
                        DataTypes::STACKPOINTER(p) => {
                            self.stack.push(DataTypes::STACKPOINTER(p));
                            self.stack.push(DataTypes::STACKPOINTER(p));
                        }
                    }
                },
                Instructions::SWAP => {
                    let first_val = self.stack.pop().expect("Insufficient data on the stack");
                    let second_val = self.stack.pop().expect("Insufficient data on the stack");
                    self.stack.push(first_val);
                    self.stack.push(second_val);
                },
                Instructions::ADD => {
                    let first_val = match self.stack.pop().expect("Insufficient data on the stack") {
                      DataTypes::INT(u) => u,
                      _ => {report_err("Cannot perform arithmetic operations on non-numeric values"); std::process::exit(1);}
                    };
                    let second_val = match self.stack.pop().expect("Insufficient data on the stack") {
                        DataTypes::INT(u) => u,
                        _ => {report_err("Cannot perform arithmetic operations on non-numeric values"); std::process::exit(1);}
                    };
                    self.stack.push(DataTypes::INT(second_val + first_val));
                },
                Instructions::SUB => {
                    let first_val = match self.stack.pop().expect("Insufficient data on the stack") {
                        DataTypes::INT(u) => u,
                        _ => {report_err("Cannot perform arithmetic operations on non-numeric values"); std::process::exit(1);}
                    };
                    let second_val = match self.stack.pop().expect("Insufficient data on the stack") {
                        DataTypes::INT(u) => u,
                        _ => {report_err("Cannot perform arithmetic operations on non-numeric values"); std::process::exit(1);}
                    };
                    self.stack.push(DataTypes::INT(second_val - first_val));
                },
                Instructions::MULT => {
                    let first_val = match self.stack.pop().expect("Insufficient data on the stack") {
                        DataTypes::INT(u) => u,
                        _ => {report_err("Cannot perform arithmetic operations on non-numeric values"); std::process::exit(1);}
                    };
                    let second_val = match self.stack.pop().expect("Insufficient data on the stack") {
                        DataTypes::INT(u) => u,
                        _ => {report_err("Cannot perform arithmetic operations on non-numeric values"); std::process::exit(1);}
                    };
                    self.stack.push(DataTypes::INT(second_val * first_val));
                },
                Instructions::DIV => {
                    let first_val = match self.stack.pop().expect("Insufficient data on the stack") {
                        DataTypes::INT(u) => u,
                        _ => {report_err("Cannot perform arithmetic operations on non-numeric values"); std::process::exit(1);}
                    };
                    let second_val = match self.stack.pop().expect("Insufficient data on the stack") {
                        DataTypes::INT(u) => u,
                        _ => {report_err("Cannot perform arithmetic operations on non-numeric values"); std::process::exit(1);}
                    };
                    self.stack.push(DataTypes::INT(second_val / first_val));
                },
                Instructions::EQ => {
                    let first_val = match self.stack.pop().expect("Insufficient data on the stack") {
                        DataTypes::INT(u) => u,
                        _ => {report_err("Cannot perform arithmetic operations on non-numeric values"); std::process::exit(1);}
                    };
                    let second_val = match self.stack.pop().expect("Insufficient data on the stack") {
                        DataTypes::INT(u) => u,
                        _ => {report_err("Cannot perform arithmetic operations on non-numeric values"); std::process::exit(1);}
                    };
                    if second_val == first_val {
                        self.stack.push(DataTypes::INT(1));
                    } else {
                        self.stack.push(DataTypes::INT(0));
                    }
                },
                Instructions::LT => {
                    let first_val = match self.stack.pop().expect("Insufficient data on the stack") {
                        DataTypes::INT(u) => u,
                        _ => {report_err("Cannot perform arithmetic operations on non-numeric values"); std::process::exit(1);}
                    };
                    let second_val = match self.stack.pop().expect("Insufficient data on the stack") {
                        DataTypes::INT(u) => u,
                        _ => {report_err("Cannot perform arithmetic operations on non-numeric values"); std::process::exit(1);}
                    };
                    if second_val < first_val {
                        self.stack.push(DataTypes::INT(1));
                    } else {
                        self.stack.push(DataTypes::INT(0));
                    }
                },
                Instructions::GT => {
                    let first_val = match self.stack.pop().expect("Insufficient data on the stack") {
                        DataTypes::INT(u) => u,
                        _ => {report_err("Cannot perform arithmetic operations on non-numeric values"); std::process::exit(1);}
                    };
                    let second_val = match self.stack.pop().expect("Insufficient data on the stack") {
                        DataTypes::INT(u) => u,
                        _ => {report_err("Cannot perform arithmetic operations on non-numeric values"); std::process::exit(1);}
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
                        _ => panic!("Binary boolean not found")
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
                    for instr in &nested_struct.instructions {
                        self.evaluate_instruction(&instr.as_ref().unwrap());
                    }
                    self.data_stack.insert(
                        nested_struct.name.to_string(),
                        match self.stack.pop().unwrap() {
                            DataTypes::INT(u) => u,
                            _ => { report_err("Cannot assign variables with non-numeric types"); std::process::exit(1); }
                        }
                    );
                },
                Instructions::IDENTIFIER(data_name) => {
                    if let Some(data) = self.data_stack.get(data_name) {
                        self.stack.push(DataTypes::INT(*data));
                    } else {
                        let mut value: &Vec<Option<Instruction>> = &Vec::new();
                        if let Some(val) = self.macro_stack.get(data_name) {
                            value = val;
                        }

                        for instr in value.to_vec() {
                            if let Some(i) = instr {
                                self.evaluate_instruction(&i);
                            }
                        }
                    }
                },
                Instructions::SPAWN(name) => {
                    if RESERVED_KEYWORDS.contains(&name.as_str()) {
                        eprintln!("ERROR: Cannot assign variable with name of assigned keyword ({})", name);
                        std::process::exit(1);
                    }
                    self.stack_stack.insert(
                        name.to_string(),
                        Vec::new()
                    );
                    self.stack.push(DataTypes::STACKPOINTER(self.stack_stack.get("name").unwrap() as *const Vec<DataTypes>))
                },
                Instructions::SWITCH(name) => {
                    let tmp_stack: Vec <DataTypes>;
                    self.stack = match self.stack_stack.get {
                        Some(vec) => {
                            tmp_stack = vec.to_vec();
                            self.stack_stack.insert(
                                self.current_stack.to_string(),
                                self.stack.clone()
                            );
                            self.current_stack = name.to_string();
                            tmp_stack
                        },
                        None => {
                            eprintln!("ERROR: Stack with name {} not found", name);
                            std::process::exit(1);
                        }
                    }
                },
                Instructions::CLOSE(name) => {
                    if name == "main" {
                        eprintln!("ERROR: Cannot remove main stack");
                        std::process::exit(1);
                    } else if name.to_string() == self.current_stack {
                        eprintln!("ERROR: Cannot remove stack you are currently working in");
                        std::process::exit(1);
                    }
                    self.stack_stack.remove(name);
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
                    let prev_stack = self.current_stack.clone();
                    for instruction in nested_instructions {
                        if let Some(instr) = instruction {
                            self.evaluate_instruction(instr);
                        }
                    }
                    self.evaluate_instruction(&Instruction::new(Instructions::SWITCH(prev_stack), None));
                },
                Instructions::MACRO(nested_instructions) => {
                    if RESERVED_KEYWORDS.contains(&nested_instructions.name.as_str()) {
                        eprintln!("ERROR: Cannot assign variable with name of assigned keyword ({})", nested_instructions.name);
                        std::process::exit(1);
                    }
                    self.macro_stack.insert(
                        nested_instructions.to_owned().name,
                        nested_instructions.to_owned().instructions
                    );
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