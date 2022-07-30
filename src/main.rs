use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

pub type ParseStream<'a> = Peekable<Enumerate<Chars<'a>>>;

pub fn parse_stream(s: &str) -> ParseStream<'_> {
    s.chars().enumerate().peekable()
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Span {
    start: usize,
    end: usize,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Literal {
    Int { value: String, span: Span },
}

impl Literal {
    pub fn parse(chars: &mut ParseStream) -> Self {
        let (start, first) = chars
            .peek()
            .filter(|(_, c)| c.is_ascii_digit())
            .copied()
            .unwrap();
        chars.next();
        let mut s = String::from(first);

        while let Some((_, c)) = chars.peek() {
            if c.is_ascii_digit() {
                s.push(*c);
                chars.next();
            } else {
                break;
            }
        }
        let span = Span {
            start,
            end: start + s.len() - 1,
        };

        Literal::Int { value: s, span }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct List {
    start: Literal,
    end: Literal,
    bracket: Span,
}

impl List {
    pub fn parse(chars: &mut ParseStream) -> Self {
        let bracket_start = if let Some((pos, '[')) = chars.peek().copied() {
            chars.next();
            pos
        } else {
            todo!()
        };

        let start = Literal::parse(chars);

        for _ in 0..2 {
            if chars.next_if(|(_, c)| *c == '.').is_none() {
                todo!()
            }
        }

        let end = Literal::parse(chars);

        let bracket_end = if let Some((pos, ']')) = chars.peek().copied() {
            chars.next();
            pos
        } else {
            todo!()
        };

        Self {
            start,
            end,
            bracket: Span {
                start: bracket_start,
                end: bracket_end,
            },
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::{parse_stream, List, Literal, Span};

    #[test]
    fn it_parses_int_literal() {
        let mut chars = parse_stream("42069");
        let lit = Literal::parse(&mut chars);
        assert_eq!(
            lit,
            Literal::Int {
                value: "42069".to_owned(),
                span: Span { start: 0, end: 4 }
            }
        )
    }

    #[test]
    fn it_parses_list_range() {
        let mut chars = parse_stream("[1..5]");
        let list = List::parse(&mut chars);
        assert_eq!(
            list,
            List {
                start: Literal::Int {
                    value: "1".to_owned(),
                    span: Span { start: 1, end: 1 }
                },
                end: Literal::Int {
                    value: "5".to_owned(),
                    span: Span { start: 4, end: 4 }
                },
                bracket: Span { start: 0, end: 5 }
            }
        )
    }
}
