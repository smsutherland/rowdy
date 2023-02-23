use rowdy_location::Location;
use std::str::Chars;

#[derive(Debug, Clone)]
pub struct Cursor<'a> {
    chars: Chars<'a>,
    pub current_loc: Location,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars(),
            current_loc: Location {
                line: 1,
                col: 1,
                char_num: 0,
            },
        }
    }

    pub fn next(&mut self) -> Option<(char, Location)> {
        let loc = self.current_loc;
        let next = self.chars.next();
        if next.is_some() {
            self.current_loc.col += 1;
            self.current_loc.char_num += 1;
            if let Some('\n') = next {
                self.current_loc = Location {
                    col: 1,
                    line: self.current_loc.line + 1,
                    ..self.current_loc
                }
            }
        }
        next.map(|c| (c, loc))
    }

    /// Peeks at the nth next item.
    /// peek(0) returns the next item.
    pub fn peek(&self, n: usize) -> Option<(char, Location)> {
        let mut temp_cursor = self.clone();
        for _ in 0..n {
            temp_cursor.next();
        }
        temp_cursor.next()
    }

    /// Peeks at the nth next item.
    /// peek(0) returns the next item.
    /// Less expensive than peek because this doesn't have to return the location.
    pub fn peek_char(&self, n: usize) -> Option<char> {
        self.chars.clone().nth(n)
    }

    /// Consumes n items and returns the last one.
    /// consume(0) returns the next item.
    pub fn consume(&mut self, n: usize) -> Option<(char, Location)> {
        for _ in 0..n {
            self.next();
        }
        self.next()
    }

    /// Consumes chars while `f` is true.
    /// Returns the loation of the last char matching `f`.
    pub fn eat_while(&mut self, f: impl Fn(&char) -> bool) -> Option<Location> {
        let mut last_loc = None;
        while let Some(c) = self.peek_char(0) {
            if !f(&c) {
                break;
            }
            // SAFETY: unwrap never fails because we already checked if the next char is a Some(_)
            last_loc = unsafe { Some(self.consume(0).unwrap_unchecked().1) };
        }
        last_loc
    }
}
