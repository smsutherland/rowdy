use super::token::Location;
use std::str::Chars;

pub struct Cursor<'a> {
    chars: Chars<'a>,
    pub current_loc: Location,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars(),
            current_loc: Location { line: 0, col: 0 },
        }
    }

    pub fn next(&mut self) -> Option<(char, Location)> {
        let loc = self.current_loc;
        let next = self.chars.next();
        if next.is_some() {
            self.current_loc.col += 1;
            if let Some('\n') = next {
                self.current_loc = Location {
                    line: self.current_loc.line,
                    col: 1,
                }
            }
        }
        next.map(|c| (c, loc))
    }

    pub fn peek(&mut self, num_ahead: usize) -> Option<(char, Location)> {
        let loc = self.current_loc;
        self.chars.clone().nth(num_ahead).map(|c| (c, loc))
    }

    pub fn consume(&mut self, num_ahead: usize) -> Option<(char, Location)> {
        let loc = self.current_loc;
        self.chars.nth(num_ahead - 1).map(|c| (c, loc))
    }
}
