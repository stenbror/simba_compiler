
// Simba compiler 
// Written by Richard Magnor Stenbro. All rights reserved.

#[derive(Clone, PartialEq, Debug)]
pub enum Symbols
{
    Times(u32, u32),
    Slash(u32, u32),
    Div(u32, u32),
    Mod(u32, u32),
    And(u32, u32),
    Plus(u32, u32),
    Minus(u32, u32),
    Or(u32, u32),
    Eql(u32, u32),
    Neq(u32, u32),
    Lss(u32, u32),
    Leq(u32, u32),
    Gtr(u32, u32),
    Geq(u32, u32),
    In(u32, u32),
    Is(u32, u32),
    Arrow(u32, u32),
    Period(u32, u32),
    Comma(u32, u32),
    Colon(u32, u32),
    Upto(u32, u32),
    Rparen(u32, u32),
    Rbrak(u32, u32),
    Rbrace(u32, u32),
    Of(u32, u32),
    Then(u32, u32),
    Do(u32, u32),
    To(u32, u32),
    By(u32, u32),
    Lparen(u32, u32),
    Lbrak(u32, u32),
    Lbrace(u32, u32),
    Not(u32, u32),
    Becomes(u32, u32),
    Number(u32, u32, Box<std::string::String>),
    Nil(u32, u32),
    String(u32, u32, Box<std::string::String>),
    Ident(u32, u32, Box<std::string::String>),
    Semicolon(u32, u32),
    Bar(u32, u32),
    End(u32, u32),
    Else(u32, u32),
    Elsif(u32, u32),
    Until(u32, u32),
    If(u32, u32),
    Case(u32, u32),
    While(u32, u32),
    Repeat(u32, u32),
    For(u32, u32),
    Loop(u32, u32),
    With(u32, u32),
    Exit(u32, u32),
    Return(u32, u32),
    Array(u32, u32),
    Record(u32, u32),
    Pointer(u32, u32),
    Begin(u32, u32),
    Const(u32, u32),
    Type(u32, u32),
    Var(u32, u32),
    Procedure(u32, u32),
    Import(u32, u32),
    Module(u32, u32),
    False(u32, u32),
    True(u32, u32),
    Continue(u32, u32),
    Await(u32, u32),
    Async(u32, u32),
    Eof
}

pub trait ScannerMethods
{
    fn new(text: &'static str) -> Self;
    fn get_char(&mut self) -> char;
    fn next_char(&mut self) -> ();
    fn start_position(&mut self) -> u32;
    fn get_symbol(&mut self) -> Result<Symbols, Box<std::string::String>>;
}

pub struct Scanner
{
    buffer: Vec<char>,	/* Sourcecode as a vector of chars */
	start_pos: u32,		/* Start of current analyzed symbol */
	index: u32			/* Position into vector */
}

impl ScannerMethods for Scanner
{
    fn new(text: &'static str) -> Self {
        Scanner{
			buffer: text.chars().collect(),
			start_pos: 0,
			index: 0
		}
    }

    fn get_char(&mut self) -> char {
        match self.buffer.get(self.index as usize) {
			Some(x) => {
				return x.clone()
			},
			_ => '\0'
		}
    }

    fn next_char(&mut self) -> () {
        if self.index <= (self.buffer.len() as u32 - 1) {
            self.index = self.index + 1;
        }
    }

    fn start_position(&mut self) -> u32 {
        self.start_pos
    }

    /// Get next valid symbol from sourcefile removing whitespace and comments or return error
    fn get_symbol(&mut self) -> Result<Symbols, Box<std::string::String>> {
        
        /* Remove whitespace and lineshift */
		loop {
			let ch = self.get_char();
			match ch {
				' '  | '\t' | '\r' | '\n'  => {
					self.next_char();
					continue
				},
				_ => {
					break
				}
			}
		}

        self.start_pos = self.index; /* Save start position of current symbol */


        // Analyze all valid symbols and return the result to parser
        return match self.get_char() {
            '\0' => Ok(Symbols::Eof),
            '(' => {
                self.next_char();
                match self.get_char() {
                    '*' => {
                        self.next_char();
                        let mut level = 1;
                        while level > 0 && self.get_char() != '\0' {

                            match self.get_char() {
                                '*' => {
                                    self.next_char();

                                    match self.get_char() {
                                        '\0' => break,
                                        ')' => {
                                            self.next_char();
                                            level = level - 1

                                        },
                                        _ => self.next_char()
                                    }
                                },
                                '(' => {
                                    self.next_char();

                                    match self.get_char() {
                                        '\0' => break,
                                        '*' => {
                                            self.next_char();
                                            level = level + 1

                                        },
                                        _ => self.next_char()
                                    }

                                },
                                _ => self.next_char()
                            }
                        }

                        if level != 0 {
							return Err(Box::new(format!("Unterminated comment at position: '{}'", self.index)))
						}
                        return self.get_symbol()
                    },
                    _ => ()
                }
                Ok(Symbols::Lparen(self.start_pos, self.index))
            },
            ')' => {
                self.next_char();
                Ok(Symbols::Rparen(self.start_pos, self.index))
            },
            '[' => {
                self.next_char();
                Ok(Symbols::Lbrak(self.start_pos, self.index))
            },
            ']' => {
                self.next_char();
                Ok(Symbols::Rbrak(self.start_pos, self.index))
            },
            '{' => {
                self.next_char();
                Ok(Symbols::Lbrace(self.start_pos, self.index))
            },
            '}' => {
                self.next_char();
                Ok(Symbols::Rbrace(self.start_pos, self.index))
            },
            '|' => {
                self.next_char();
                Ok(Symbols::Bar(self.start_pos, self.index))
            },
            '#' => {
                self.next_char();
                Ok(Symbols::Neq(self.start_pos, self.index))
            },
            '&' => {
                self.next_char();
                Ok(Symbols::And(self.start_pos, self.index))
            },
            ',' => {
                self.next_char();
                Ok(Symbols::Comma(self.start_pos, self.index))
            },
            '*' => {
                self.next_char();
                Ok(Symbols::Times(self.start_pos, self.index))
            },
            '-' => {
                self.next_char();
                Ok(Symbols::Minus(self.start_pos, self.index))
            },
            '/' => {
                self.next_char();
                Ok(Symbols::Slash(self.start_pos, self.index))
            },
            ';' => {
                self.next_char();
                Ok(Symbols::Semicolon(self.start_pos, self.index))
            },
            '=' => {
                self.next_char();
                Ok(Symbols::Eql(self.start_pos, self.index))
            },
            '^' => {
                self.next_char();
                Ok(Symbols::Arrow(self.start_pos, self.index))
            },
            '~' => {
                self.next_char();
                Ok(Symbols::Not(self.start_pos, self.index))
            },
            ':' => {
                self.next_char();
                match self.get_char() {
                    '=' => {
                        self.next_char();
                        Ok(Symbols::Becomes(self.start_pos, self.index))
                    },
                    _ => Ok(Symbols::Colon(self.start_pos, self.index))
                }
            },
            '+' => {
                self.next_char();
                Ok(Symbols::Plus(self.start_pos, self.index))
            },
            '<' => {
                self.next_char();
                match self.get_char() {
                    '=' => {
                        self.next_char();
                        Ok(Symbols::Leq(self.start_pos, self.index))
                    },
                    _ => Ok(Symbols::Lss(self.start_pos, self.index))
                }
            },
            '>' => {
                self.next_char();
                match self.get_char() {
                    '=' => {
                        self.next_char();
                        Ok(Symbols::Geq(self.start_pos, self.index))
                    },
                    _ => Ok(Symbols::Gtr(self.start_pos, self.index))
                }
            },
            '.' => {
                self.next_char();
                match self.get_char() {
                    '.' => {
                        self.next_char();
                        Ok(Symbols::Upto(self.start_pos, self.index))
                    },
                    _ => Ok(Symbols::Period(self.start_pos, self.index))
                }
            },
            'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'i' | 'm' | 'n' | 'o' | 'p' | 'r' | 't' | 'u' | 'v' | 'w' => {
                let mut buffer = std::string::String::new();
				loop {
					let _cur = self.get_char();
					if _cur.is_alphanumeric() || _cur == '_' {
						buffer.push(_cur);
                        self.next_char();
						continue
					}
					break
				}

                match buffer.as_str() {
                    "array" => Ok(Symbols::Array(self.start_pos, self.index)),
                    "async" => Ok(Symbols::Async(self.start_pos, self.index)),
                    "await" => Ok(Symbols::Await(self.start_pos, self.index)),
                    "begin" => Ok(Symbols::Begin(self.start_pos, self.index)),
                    "by" => Ok(Symbols::By(self.start_pos, self.index)),
                    "case" => Ok(Symbols::Case(self.start_pos, self.index)),
                    "const" => Ok(Symbols::Const(self.start_pos, self.index)),
                    "continue" => Ok(Symbols::Continue(self.start_pos, self.index)),
                    "div" => Ok(Symbols::Div(self.start_pos, self.index)),
                    "do" => Ok(Symbols::Do(self.start_pos, self.index)),
                    "else" => Ok(Symbols::Else(self.start_pos, self.index)),
                    "elsif" => Ok(Symbols::Elsif(self.start_pos, self.index)),
                    "end" => Ok(Symbols::End(self.start_pos, self.index)),
                    "exit" => Ok(Symbols::Exit(self.start_pos, self.index)),
                    "false" => Ok(Symbols::False(self.start_pos, self.index)),
                    "for" => Ok(Symbols::For(self.start_pos, self.index)),
                    "if" => Ok(Symbols::If(self.start_pos, self.index)),
                    "import" => Ok(Symbols::Import(self.start_pos, self.index)),
                    "in" => Ok(Symbols::In(self.start_pos, self.index)),
                    "is" => Ok(Symbols::Is(self.start_pos, self.index)),
                    "loop" => Ok(Symbols::Loop(self.start_pos, self.index)),
                    "mod" => Ok(Symbols::Mod(self.start_pos, self.index)),
                    "module" => Ok(Symbols::Module(self.start_pos, self.index)),
                    "nil" => Ok(Symbols::Nil(self.start_pos, self.index)),
                    "of" => Ok(Symbols::Of(self.start_pos, self.index)),
                    "or" => Ok(Symbols::Or(self.start_pos, self.index)),
                    "pointer" => Ok(Symbols::Pointer(self.start_pos, self.index)),
                    "procedure" => Ok(Symbols::Procedure(self.start_pos, self.index)),
                    "record" => Ok(Symbols::Record(self.start_pos, self.index)),
                    "repeat" => Ok(Symbols::Repeat(self.start_pos, self.index)),
                    "return" => Ok(Symbols::Return(self.start_pos, self.index)),
                    "then" => Ok(Symbols::Then(self.start_pos, self.index)),
                    "to" => Ok(Symbols::To(self.start_pos, self.index)),
                    "true" => Ok(Symbols::True(self.start_pos, self.index)),
                    "type" => Ok(Symbols::Type(self.start_pos, self.index)),
                    "until" => Ok(Symbols::Until(self.start_pos, self.index)),
                    "var" => Ok(Symbols::Var(self.start_pos, self.index)),
                    "while" => Ok(Symbols::While(self.start_pos, self.index)),
                    "with" => Ok(Symbols::With(self.start_pos, self.index)),
                    _ => Ok(Symbols::Ident(self.start_pos, self.index, Box::new(buffer)))
                }

            },

            _ => Result::Err(Box::new(format!("")))
        }
    }
    
    
}

#[cfg(test)]
mod tests {
	use crate::scanner::{Scanner, ScannerMethods, Symbols};

    #[test]
    fn reserved_keyword_array() {
        let mut scan = Box::new(Scanner::new("array"));
        let symbol = scan.get_symbol();

        match symbol {
            Ok(x) => {
                match x {
					Symbols::Array(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 5);
					},
					_ => assert!(false)
				}
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_async() {
        let mut scan = Box::new(Scanner::new("async"));
        let symbol = scan.get_symbol();

        match symbol {
            Ok(x) => {
                match x {
					Symbols::Async(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 5);
					},
					_ => assert!(false)
				}
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_await() {
        let mut scan = Box::new(Scanner::new("await"));
        let symbol = scan.get_symbol();

        match symbol {
            Ok(x) => {
                match x {
					Symbols::Await(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 5);
					},
					_ => assert!(false)
				}
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_begin() {
        let mut scan = Box::new(Scanner::new("begin"));
        let symbol = scan.get_symbol();

        match symbol {
            Ok(x) => {
                match x {
					Symbols::Begin(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 5);
					},
					_ => assert!(false)
				}
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_by() {
        let mut scan = Box::new(Scanner::new("by"));
        let symbol = scan.get_symbol();

        match symbol {
            Ok(x) => {
                match x {
					Symbols::By(s, e) => {
						assert_eq!(s, 0);
						assert_eq!(e, 2);
					},
					_ => assert!(false)
				}
            },
            _ => assert!(false)
        }
    }

}