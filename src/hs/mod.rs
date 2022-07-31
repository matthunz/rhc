mod expression;
pub use expression::{BinaryOp, Expression};

mod function;
pub use function::{Function, Pattern};

mod literal;
pub use literal::Literal;

mod list;
pub use list::List;

mod stmt;
pub use stmt::Statement;

use std::{iter::Peekable, str::Chars};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LineColumn {
    pub line: usize,
    pub column: usize,
}

impl LineColumn {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

impl Default for LineColumn {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Span {
    start: LineColumn,
    end: LineColumn,
}

impl Span {
    pub fn new(start: LineColumn, end: LineColumn) -> Self {
        Self { start, end }
    }
}

impl From<LineColumn> for Span {
    fn from(line_column: LineColumn) -> Self {
        Self::new(line_column.clone(), line_column)
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Error {
    span: Span,
}

impl Error {
    pub fn new(span: Span) -> Self {
        Self { span }
    }
}

pub struct Module {
    pub funcs: Vec<Function>,
}

pub struct Token {
    pub c: char,
    pub line_column: LineColumn,
}

impl Token {
    pub fn span(&self) -> Span {
        Span {
            start: self.line_column.clone(),
            end: self.line_column.clone(),
        }
    }
}

pub struct Tokens<'a> {
    chars: Peekable<Chars<'a>>,
    line_column: LineColumn,
}

impl<'a> Tokens<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            chars: s.chars().peekable(),
            line_column: LineColumn::default(),
        }
    }

    pub fn peek_char(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    pub fn peek(&mut self) -> Option<Token> {
        if let Some(next) = self.chars.peek() {
            let _line_column = self.line_column.clone();

            let _line_column = if *next == '\n' {
                LineColumn {
                    line: self.line_column.line + 1,
                    column: 0,
                }
            } else {
                LineColumn {
                    line: self.line_column.line,
                    column: self.line_column.column + 1,
                }
            };

            Some(Token {
                c: *next,
                line_column: self.line_column.clone(),
            })
        } else {
            None
        }
    }

    fn parse_char(&mut self, c: char) -> Result<LineColumn, Error> {
        if let Some(token) = self.next() {
            if token.c == c {
                Ok(token.line_column)
            } else {
                Err(Error::new(Span::from(token.line_column)))
            }
        } else {
            Err(Error::default())
        }
    }
}

impl Iterator for Tokens<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.chars.next() {
            let _line_column = self.line_column.clone();

            if next == '\n' {
                self.line_column.line += 1;
                self.line_column.column = 0;
            } else {
                self.line_column.column += 1;
            }

            Some(Token {
                c: next,
                line_column: self.line_column.clone(),
            })
        } else {
            None
        }
    }
}

pub trait FromTokens: Sized {
    fn from_tokens(tokens: &mut Tokens<'_>) -> Result<Self, Error>;
}

impl FromTokens for Module {
    fn from_tokens(tokens: &mut Tokens<'_>) -> Result<Self, Error> {
        let mut funcs = Vec::new();
        loop {
            let func = Function::from_tokens(tokens)?;
            funcs.push(func);

            if let Some(token) = tokens.next() {
                if token.c != '\n' {
                    return Err(Error::new(Span::from(token.line_column)));
                }
            } else {
                break;
            }
        }
        Ok(Self { funcs })
    }
}
