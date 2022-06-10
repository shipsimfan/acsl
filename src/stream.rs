use std::str::Chars;

pub struct Stream<'a> {
    current_char: Option<char>,
    next_char: Option<char>,
    chars: Chars<'a>,

    column: usize,
    line: usize,
}

impl<'a> Stream<'a> {
    pub fn new(string: &'a str) -> Self {
        let mut chars = string.chars();

        let next_char = chars.next();

        Stream {
            current_char: None,
            next_char,
            chars,
            column: 1,
            line: 1,
        }
    }

    pub fn current_char(&self) -> Option<char> {
        self.current_char
    }

    pub fn next_char(&self) -> Option<char> {
        self.next_char
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn next(&mut self) -> Option<char> {
        // Update column & line
        match self.current_char {
            Some(c) => match c {
                '\n' => {
                    self.line += 1;
                    self.column = 1;
                }
                '\t' => {
                    self.column = if (self.column - 1) % 4 == 0 {
                        self.column + 4
                    } else {
                        self.column + 4 - ((self.column - 1) % 4)
                    }
                }
                _ => self.column += 1,
            },
            None => {}
        }

        // Get next character
        self.current_char = self.next_char;
        self.next_char = self.chars.next();

        self.current_char
    }
}
