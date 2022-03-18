pub mod lex {
    use crate::globals::globals::*;
    use std::vec::IntoIter;
    use std::iter::Peekable;

    use std::{fs, io};

    #[derive(Debug)]
    pub struct Lexer {
        raw_data: Peekable<IntoIter<char>>,
    }

    impl Lexer {
        pub fn from_text(text: &str) -> Self {
            Lexer {
                raw_data: text.chars().collect::<Vec<_>>().into_iter().peekable(),
            }
        }

        pub fn from_file(file_path: &str) -> io::Result<Self> {
            Ok(Self::from_text(&fs::read_to_string(file_path)?))
        }

        fn get_next_char_while(&mut self, raw_token: String, cond: fn(char) -> bool) -> String {
            let mut res = raw_token;
            loop {
                match self.raw_data.peek() {
                    Some(c) if cond(*c) => {
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
                    Some(c) if c.is_whitespace() => continue,
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
                    } else if first_char == '@' { // Variable declaration
                        let token: String = self.raw_data.next().expect("ERROR: No character found").to_string();
                        let name = self.get_next_char_while(token, |c| Self::is_alphanumeric(c));

                        return Some(Operation::new(OpCodes::VARDECLARE(name.to_string()), None));
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
                        instr.push(Some(Operation::new(OpCodes::SPAWN(name.clone()), None)));
                        instr.push(Some(Operation::new(OpCodes::DUP, None)));
                        instr.push(Some(Operation::new(OpCodes::SWITCH, None)));
                        for char in res.chars() {
                            instr.push(Some(Operation::new(OpCodes::PUSH, Some(char as u8))))
                        }
                        instr.push(Some(Operation::new(OpCodes::STACK("main".to_string()), None)));
                        instr.push(Some(Operation::new(OpCodes::SWITCH, None)));
                        instr.push(Some(Operation::new(OpCodes::STACK(name.clone()), None)));
                        return Some(Operation::new(OpCodes::STRING(instr), None));
                    } else {
                        let token: String = first_char.to_string();
                        let identifier = self.get_next_char_while(token, |c| Self::is_alphanumeric(c));
                        match identifier.as_str() {
                            "dup" => return Some(Operation::new(OpCodes::DUP, None)),
                            "swap" => return Some(Operation::new(OpCodes::SWAP, None)),
                            "pop" => return Some(Operation::new(OpCodes::POP, None)),
                            "if" => return Some(Operation::new(OpCodes::IF, None)),
                            "else" => return Some(Operation::new(OpCodes::ELSE, None)),
                            "while" => return Some(Operation::new(OpCodes::WHILE, None)),
                            "end" => return Some(Operation::new(OpCodes::END, None)),
                            "do" => return Some(Operation::new(OpCodes::DO, None)),
                            "+" => return Some(Operation::new(OpCodes::ADD, None)),
                            "-" => return Some(Operation::new(OpCodes::SUB, None)),
                            "print" => return Some(Operation::new(OpCodes::PRINT, None)),
                            "print_ascii" => return Some(Operation::new(OpCodes::PRINTASCII, None)),
                            "=" => return Some(Operation::new(OpCodes::EQ, None)),
                            "<" => return Some(Operation::new(OpCodes::LT, None)),
                            ">" => return Some(Operation::new(OpCodes::GT, None)),
                            "*" => return Some(Operation::new(OpCodes::MULT, None)),
                            "/" => return Some(Operation::new(OpCodes::DIV, None)),
                            "def" => return Some(Operation::new(OpCodes::DEFINE, None)),
                            "spawn" => {
                                self.raw_data.next();
                                let token: String = self.raw_data.next().expect("ERROR: No character found").to_string();
                                let name = self.get_next_char_while(token, |c| Self::is_alphanumeric(c));

                                return Some(Operation::new(OpCodes::SPAWN(name.to_string()), None));
                            },
                            "switch" => return Some(Operation::new(OpCodes::SWITCH, None)),
                            "close" => return Some(Operation::new(OpCodes::CLOSE, None)),
                            "stack" => {
                                self.raw_data.next();
                                let token: String = self.raw_data.next().expect("ERROR: No character found").to_string();
                                let name = self.get_next_char_while(token, |c| Self::is_alphanumeric(c));

                                return Some(Operation::new(OpCodes::STACK(name), None));
                            },
                            "this" => return Some(Operation::new(OpCodes::THIS, None)),
                            "stacks" => return Some(Operation::new(OpCodes::STACKS, None)),
                            "stack_size" => return Some(Operation::new(OpCodes::STACKSIZE, None)),
                            "stack_rev" => return Some(Operation::new(OpCodes::STACKREV, None)),
                            "macro" => {
                                self.raw_data.next();
                                let token: String = self.raw_data.next().expect("ERROR: No character found").to_string();
                                let name = self.get_next_char_while(token, |c| Self::is_alphanumeric(c));

                                return Some(Operation::new(OpCodes::MACRO(name), None));
                            }
                            _ => return Some(Operation::new(OpCodes::IDENTIFIER(identifier.trim().to_string()), None))
                        }
                    }
                }

                else if first_char.is_numeric() {
                    return Some(Operation::new(OpCodes::PUSH,
                                               Some(self.get_numeric(first_char)
                                                   .parse::<u8>()
                                                   .unwrap())))
                }
            }
        }
    }
}
