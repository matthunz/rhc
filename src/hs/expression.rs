use super::{FromTokens, Literal, Tokens};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BinaryOp {
    Add,
    Sub,
}

impl BinaryOp {
    pub fn to_js(&self, s: &mut String) {
        match self {
            Self::Add => s.push('+'),
            Self::Sub => s.push('-'),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expression {
    BinaryOp {
        left: Box<Self>,
        op: BinaryOp,
        right: Box<Self>,
    },
    Path(Vec<String>),
    Literal(Literal),
    Call {
        ident: String,
        args: Vec<Self>,
    },
}

impl FromTokens for Expression {
    fn from_tokens(tokens: &mut Tokens) -> Result<Self, super::Error> {
        let left = if let Ok(lit) = Literal::from_tokens(tokens) {
            Self::Literal(lit)
        } else {
            let part = tokens
                .map(|token| token.c)
                .take_while(|c| *c != ' ')
                .collect();

            if tokens.peek_char() == Some('(') {
                tokens.next();

                let mut args = Vec::new();
                loop {
                    let arg = Self::from_tokens(tokens)?;
                    args.push(arg);
                    if tokens.peek_char() == Some(')') {
                        tokens.next();
                        // Remove space
                        tokens.next();
                        break;
                    }
                }

                Self::Call { ident: part, args }
            } else {
                Self::Path(vec![part])
            }
        };

        match tokens.peek_char() {
            Some('+') => {
                tokens.next();
                // Remove space
                tokens.next();

                let right = Self::from_tokens(tokens)?;
                Ok(Self::BinaryOp {
                    left: Box::new(left),
                    op: BinaryOp::Add,
                    right: Box::new(right),
                })
            }
            Some('-') => {
                tokens.next();
                // Remove space
                tokens.next();

                let right = Self::from_tokens(tokens)?;
                Ok(Self::BinaryOp {
                    left: Box::new(left),
                    op: BinaryOp::Sub,
                    right: Box::new(right),
                })
            }
            _ => Ok(left),
        }
    }
}

impl Expression {
    pub fn to_js(&self, s: &mut String) {
        match self {
            Self::BinaryOp { left, op, right } => {
                left.to_js(s);
                op.to_js(s);
                right.to_js(s);
            }
            Self::Literal(lit) => lit.to_js(s),
            Self::Path(path) => {
                for part in path {
                    s.push_str(part)
                }
            }
            Self::Call { ident, args } => {
                s.push_str(ident);
                s.push('(');
                for (pos, arg) in args.iter().enumerate() {
                    arg.to_js(s);
                    if pos < args.len() - 1 {
                        s.push(',');
                    }
                }

                s.push(')');
            }
        }
    }
}
