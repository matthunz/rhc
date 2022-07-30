use super::{Error, ParseStream, Span};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Literal {
    Int { value: String, span: Span },
}

impl Literal {
    pub fn parse(chars: &mut ParseStream) -> Result<Self, Error> {
        let (start, first) = if let Some((pos, c)) = chars.peek().copied() {
            if c.is_ascii_digit() {
                chars.next();
                (pos, c)
            } else {
                return Err(Error::new(Span::new(pos, pos)));
            }
        } else {
            return Err(Error::empty());
        };

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

        Ok(Literal::Int { value: s, span })
    }

    pub fn to_js(&self, s: &mut String) {
        match self {
            Self::Int { value, span: _ } => s.push_str(value),
        }
    }
}
