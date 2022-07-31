use super::{expression::Expression, FromTokens, Literal, Tokens};
use crate::Statement;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Pattern {
    Literal(Literal),
    Ident(String),
}

impl FromTokens for Pattern {
    fn from_tokens(tokens: &mut super::Tokens<'_>) -> Result<Self, super::Error> {
        if let Ok(lit) = Literal::from_tokens(tokens) {
            // remove whitespace?
            tokens.next();
            Ok(Self::Literal(lit))
        } else {
            let ident: String = tokens
                .map(|token| token.c)
                .take_while(|c| *c != ' ')
                .collect();
            if ident.is_empty() {
                todo!()
            }
            Ok(Self::Ident(ident))
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Function {
    pub ident: String,
    pub patterns: Vec<Pattern>,
    pub stmt: Statement,
}

impl FromTokens for Function {
    fn from_tokens(tokens: &mut Tokens<'_>) -> Result<Self, super::Error> {
        let ident: String = tokens
            .take_while(|token| token.c != ' ')
            .map(|token| token.c)
            .collect();
        if ident.is_empty() {
            todo!()
        }

        let mut patterns = Vec::new();
        loop {
            let pattern = Pattern::from_tokens(tokens)?;
            patterns.push(pattern);
            if tokens.peek().map(|token| token.c) == Some('=') {
                tokens.next();
                break;
            }
        }

        if tokens.next().unwrap().c != ' ' {
            todo!()
        }

        let expr = Expression::from_tokens(tokens)?;
        let stmt = Statement::Expression(expr);
        Ok(Self {
            ident,
            patterns,
            stmt,
        })
    }
}
