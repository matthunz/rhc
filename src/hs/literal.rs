use super::{Error, FromTokens, Span};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Literal {
    Int { value: String, span: Span },
}

impl Literal {
    pub fn to_js(&self, s: &mut String) {
        match self {
            Self::Int { value, span: _ } => s.push_str(value),
        }
    }
}

impl FromTokens for Literal {
    fn from_tokens(tokens: &mut super::Tokens<'_>) -> Result<Self, Error> {
        let first_token = if let Some(token) = tokens.peek() {
            if token.c.is_ascii_digit() {
                tokens.next().unwrap()
            } else {
                return Err(Error::new(token.span()));
            }
        } else {
            return Err(Error::new(Span::default()));
        };

        let mut s = String::from(first_token.c);

        let end = first_token.line_column.clone();
        while let Some(token) = tokens.peek() {
            if token.c.is_ascii_digit() {
                s.push(token.c);
                tokens.next();
            } else {
                break;
            }
        }
        let span = Span::new(first_token.line_column, end);

        Ok(Literal::Int { value: s, span })
    }
}
