use std::{ffi::OsString, fmt};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Location {
    pub line: usize,
    pub col: usize,
    pub char_num: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source {
    File(OsString),
    Anonymous,
}

#[derive(Clone)]
pub struct SourceLocation {
    pub file: Source,
    pub loc: Location,
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    // pub file: Source<'a>,
    pub start: Location,
    pub end: Location,
}

impl Location {
    fn add_source(self, source: Source) -> SourceLocation {
        SourceLocation {
            file: source,
            loc: self,
        }
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.char_num.partial_cmp(&other.char_num)
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.char_num.cmp(&other.char_num)
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}

impl fmt::Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.file {
            Source::File(file) => write!(f, "{:?}:{}:{}", file, self.loc.line, self.loc.col),
            Source::Anonymous => write!(f, "anon:{}:{}", self.loc.line, self.loc.col),
        }
    }
}

impl fmt::Debug for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Span {
    pub fn from_loc(loc: Location) -> Self {
        Self {
            // file: Source::Anonymous,
            start: loc,
            end: loc,
        }
    }

    pub fn from_source_loc(loc: SourceLocation) -> Self {
        Self {
            // file: loc.file,
            start: loc.loc,
            end: loc.loc,
        }
    }

    pub fn from_start_end(start: Location, end: Location) -> Self {
        Self {
            // file: Source::Anonymous,
            start,
            end,
        }
    }

    pub fn from_source_start_end(start: SourceLocation, end: SourceLocation) -> Self {
        if start.file != end.file {
            panic!("Cannot create a span across multiple files.");
        }
        Self {
            // file: start.file,
            start: start.loc,
            end: end.loc,
        }
    }

    pub fn slice<'b>(&self, s: &'b str) -> &'b str {
        &s[self.start.char_num..=self.end.char_num]
    }

    pub fn combine(&self, other: Self) -> Self {
        let start = self.start.min(other.start);
        let end = self.end.max(other.end);
        Self { start, end }
    }
}
