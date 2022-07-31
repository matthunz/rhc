use super::{Error, FromTokens, Literal, Span, Tokens};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct List {
    pub start: Literal,
    pub end: Option<Literal>,
    pub bracket: Span,
}

impl List {
    pub fn to_js(&self, s: &mut String) {
        let start = match &self.start {
            Literal::Int { value, span: _ } => value,
        };

        if let Some(literal) = &self.end {
            let end = match literal {
                Literal::Int { value, span: _ } => value,
            };

            let js = format!(
                "{{
                pos: {},
                end: {},
                next() {{
                    if (this.pos < this.end) {{
                        return {{ done: false, value: this.pos++ }};
                    }} else {{
                        return {{ done: true }};
                    }}
                }}
            }}",
                start, end
            );
            s.push_str(&js);
        } else {
            let js = format!(
                "{{
                pos: {},
                next() {{
                    return {{ done: false, value: this.pos++ }};
                }}
            }}",
                start
            );
            s.push_str(&js);
        }
    }
}

impl FromTokens for List {
    fn from_tokens(tokens: &mut Tokens<'_>) -> Result<Self, Error> {
        let bracket_start = tokens.parse_char('[')?;
        let start = Literal::from_tokens(tokens)?;

        for _ in 0..2 {
            tokens.parse_char('.')?;
        }

        let end = Literal::from_tokens(tokens).ok();
        let bracket_end = tokens.parse_char(']')?;

        Ok(Self {
            start,
            end,
            bracket: Span {
                start: bracket_start,
                end: bracket_end,
            },
        })
    }
}
