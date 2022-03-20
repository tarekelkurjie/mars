pub mod lex {
    use crate::globals::globals::*;
    use std::vec::IntoIter;
    use std::iter::Peekable;
    use std::path::PathBuf;

    use std::{fs, io};

    #[derive(Debug)]
    pub struct Lexer {
        raw_data: Peekable<IntoIter<char>>,
        file: String,
        line_num: u8
    }

    impl Lexer {
        pub fn from_text(text: &str, file: String) -> Self {
            Lexer {
                raw_data: text.chars().collect::<Vec<_>>().into_iter().peekable(),
                line_num: 1,
                file
            }
        }

        pub fn from_file(file_path: &str) -> io::Result<Self> {
            Ok(Self::from_text(&fs::read_to_string(file_path)?, file_path.to_string()))
        }

        fn get_next_char_while(&mut self, raw_token: String, cond: fn(char) -> bool) -> String {
            let mut res = raw_token;
            loop {
                match self.raw_data.peek() {
                    Some(c) if cond(*c) && *c != '\n' => {
                        res.push(*c);
                        self.raw_data.next();
                    }
                    _ => break,
                }
            }
            return res
        }

        fn is_alphanumeric(c: char) -> bool {
            return c.is_alphanumeric() || c == '_';
        }

        fn is_file_path(c: char) -> bool {return !c.is_whitespace(); }

        fn get_numeric(&mut self, c: char) -> String {
            let mut res: String = c.to_string();
            res = self.get_next_char_while(res, |c| c.is_numeric());

            return res
        }
    }

    impl Iterator for Lexer {
        type Item = Operation;

        fn next(&mut self) ->  Option<Self::Item> {
            let mut first_char: char;

            loop {
                match self.raw_data.next() {
                    Some(c) if c.is_whitespace() && c != '\n' => continue,
                    Some(c) => {
                        first_char = c;
                    }
                    None => return None,
                }

                if !first_char.is_numeric() {
                    if first_char == '/' {
                        if let Some(char) = self.raw_data.next() {
                            if char == '/' {
                                while let Some(c) = self.raw_data.next() {
                                    if c != '\n' { continue; } else { break; }
                                }
                            }
                        }
                    } else if first_char == '\n' {
                        self.line_num += 1;
                    } else if first_char == '@' { // Variable declaration
                        let token: String = self.raw_data.next().expect("ERROR: No character found").to_string();
                        let name = self.get_next_char_while(token, |c| Self::is_alphanumeric(c));

                        return Some(Operation::new(OpCodes::VARDECLARE(name.to_string()), self.line_num));
                    } else if first_char == '"' { // String literal
                        let mut res: String = self.raw_data.next().expect("ERROR: Unexpected character \"").to_string();
                        while self.raw_data.peek() != Some(&'"') {
                            let char = self.raw_data.next();
                            if let Some(c) = char {
                                if c == '\\' {
                                    if self.raw_data.next().unwrap() == 'n' {
                                        res.push('\n');
                                    }
                                } else {res.push(c);}
                            }
                        }
                        self.raw_data.next();

                        let mut name: String = String::new();
                        let mut words = res.split_whitespace();
                        for _i in 0..3 {
                            if let Some(unwrapped) = words.next() {
                                for char in unwrapped.to_string().chars() {
                                    if char.is_alphabetic() {
                                        name.push(char);
                                    }
                                }
                                name.push('_');
                            }
                        }

                        let mut instr: Vec<Option<Operation>> = Vec::new();
                        instr.push(Some(Operation::new(OpCodes::SPAWN(name.clone()), self.line_num)));
                        instr.push(Some(Operation::new(OpCodes::DUP, self.line_num)));
                        instr.push(Some(Operation::new(OpCodes::SWITCH, self.line_num)));
                        for char in res.chars() {
                            instr.push(Some(Operation::new(OpCodes::PUSH(char as u8), self.line_num)))
                        }
                        instr.push(Some(Operation::new(OpCodes::STACK("main".to_string()), self.line_num)));
                        instr.push(Some(Operation::new(OpCodes::SWITCH, self.line_num)));
                        instr.push(Some(Operation::new(OpCodes::STACK(name.clone()), self.line_num)));
                        return Some(Operation::new(OpCodes::STRING(instr), self.line_num));
                    } else {
                        let token: String = first_char.to_string();
                        let identifier = self.get_next_char_while(token, |c| Self::is_alphanumeric(c));
                        match identifier.as_str() {
                            "dup" => return Some(Operation::new(OpCodes::DUP, self.line_num)),
                            "swap" => return Some(Operation::new(OpCodes::SWAP, self.line_num)),
                            "pop" => return Some(Operation::new(OpCodes::POP, self.line_num)),
                            "if" => return Some(Operation::new(OpCodes::IF, self.line_num)),
                            "else" => return Some(Operation::new(OpCodes::ELSE, self.line_num)),
                            "while" => return Some(Operation::new(OpCodes::WHILE, self.line_num)),
                            "end" => return Some(Operation::new(OpCodes::END, self.line_num)),
                            "do" => return Some(Operation::new(OpCodes::DO, self.line_num)),
                            "+" => return Some(Operation::new(OpCodes::ADD, self.line_num)),
                            "-" => return Some(Operation::new(OpCodes::SUB, self.line_num)),
                            "print" => return Some(Operation::new(OpCodes::PRINT, self.line_num)),
                            "print_ascii" => return Some(Operation::new(OpCodes::PRINTASCII, self.line_num)),
                            "=" => return Some(Operation::new(OpCodes::EQ, self.line_num)),
                            "<" => return Some(Operation::new(OpCodes::LT, self.line_num)),
                            ">" => return Some(Operation::new(OpCodes::GT, self.line_num)),
                            "*" => return Some(Operation::new(OpCodes::MULT, self.line_num)),
                            "/" => return Some(Operation::new(OpCodes::DIV, self.line_num)),
                            "def" => return Some(Operation::new(OpCodes::DEFINE, self.line_num)),
                            "spawn" => {
                                self.raw_data.next();
                                let token: String = self.raw_data.next().expect("ERROR: No character found").to_string();
                                let name = self.get_next_char_while(token, |c| Self::is_alphanumeric(c));

                                return Some(Operation::new(OpCodes::SPAWN(name.to_string()), self.line_num));
                            },
                            "switch" => return Some(Operation::new(OpCodes::SWITCH, self.line_num)),
                            "close" => return Some(Operation::new(OpCodes::CLOSE, self.line_num)),
                            "stack" => {
                                self.raw_data.next();
                                let token: String = self.raw_data.next().expect("ERROR: No character found").to_string();
                                let name = self.get_next_char_while(token, |c| Self::is_alphanumeric(c));

                                return Some(Operation::new(OpCodes::STACK(name), self.line_num));
                            },
                            "this" => return Some(Operation::new(OpCodes::THIS, self.line_num)),
                            "stacks" => return Some(Operation::new(OpCodes::STACKS, self.line_num)),
                            "stack_size" => return Some(Operation::new(OpCodes::STACKSIZE, self.line_num)),
                            "stack_rev" => return Some(Operation::new(OpCodes::STACKREV, self.line_num)),
                            "macro" => {
                                self.raw_data.next();
                                let token: String = self.raw_data.next().expect("ERROR: No character found").to_string();
                                let name = self.get_next_char_while(token, |c| Self::is_alphanumeric(c));

                                return Some(Operation::new(OpCodes::MACRO(name), self.line_num));
                            },
                            "using" => {
                                self.raw_data.next();
                                let token: String = self.raw_data.next().expect("ERROR: No character found").to_string();
                                let value = self.get_next_char_while(token, |c| Self::is_file_path(c));

                                let mut res = Vec::new();

                                let mut absolute_path = std::env::current_dir().ok()?;
                                absolute_path.push(self.file.to_string());
                                let mut path = PathBuf::from(absolute_path.parent().unwrap());
                                path.push(value.to_string());

                                let nested_lex = Self::from_file(path.to_str().unwrap()).unwrap();
                                for item in nested_lex {
                                    res.push(Some(item));
                                }

                                return Some(Operation::new(OpCodes::IMPORT(res, value), self.line_num));
                            },
                            "exit" => return Some(Operation::new(OpCodes::EXIT, self.line_num)),
                            _ => return Some(Operation::new(OpCodes::IDENTIFIER(identifier.trim().to_string()), self.line_num))
                        }
                    }
                }

                else if first_char.is_numeric() {
                    return Some(Operation::new(OpCodes::PUSH(self.get_numeric(first_char).parse::<u8>().unwrap()), self.line_num));
                }
            }
        }
    }
}
