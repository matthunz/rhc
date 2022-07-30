use super::{parse_char, Error, Literal, ParseStream, Span};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct List {
    pub start: Literal,
    pub end: Option<Literal>,
    pub bracket: Span,
}

impl List {
    pub fn parse(chars: &mut ParseStream) -> Result<Self, Error> {
        let bracket_start = parse_char(chars, '[')?;
        let start = Literal::parse(chars)?;

        for _ in 0..2 {
            parse_char(chars, '.')?;
        }

        let end = Literal::parse(chars).ok();
        let bracket_end = parse_char(chars, ']')?;

        Ok(Self {
            start,
            end,
            bracket: Span {
                start: bracket_start,
                end: bracket_end,
            },
        })
    }

    pub fn to_js(&self, s: &mut String) {
        let start = match &self.start {
            Literal::Int { value, span } => value,
        };

        if let Some(literal) = &self.end {
            let end = match literal {
                Literal::Int { value, span } => value,
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
