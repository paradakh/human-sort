use std::{iter::{Peekable}, str::Chars};

pub struct IterPair<'a> {
    pub fst: Peekable<Chars<'a>>,
    pub lst: Peekable<Chars<'a>>,
}

impl<'a> IterPair<'a> {
    pub fn from(i1: Chars<'a>, i2: Chars<'a>) -> Self {
        Self {
            fst: i1.peekable(),
            lst: i2.peekable(),
        }
    }

    pub fn next(&mut self) -> [Option<char>; 2] {
        [self.fst.next(), self.lst.next()]
    }

    pub fn peek(&mut self) -> [Option<&char>; 2] {
        [self.fst.peek(), self.lst.peek()]
    }
}
