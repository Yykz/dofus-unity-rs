use crate::parser::{Error, Pair, Rule};

#[derive(Debug, Clone)]
pub enum Literal {
    Boolean(bool),
    Char(String),
    String(String),
    //TODO parse string as number
    Number(String),
}

impl Literal {
    pub fn u32(&self) -> Option<u32> {
        if let Self::Number(s) = self {
            s.parse().ok()
        } else {
            None
        }
    }
}

impl TryFrom<Pair<'_>> for Literal {
    type Error = Error;

    fn try_from(value: Pair<'_>) -> Result<Self, Self::Error> {
        Error::rule_matches(Rule::value, value.as_rule())?;
        let inner = value.into_inner().next().unwrap();
        Ok(match inner.as_rule() {
            Rule::boolean => match inner.as_str() {
                "True" => Self::Boolean(true),
                "False" => Self::Boolean(false),
                str => Err(Error::InvalidToken(Rule::boolean, str.to_string()))?,
            },
            Rule::string => Self::String(inner.as_str().to_string()),
            Rule::number => Self::Number(inner.as_str().to_string()),
            Rule::char => Self::Char(inner.as_str().to_string()),
            r => Err(Error::UnexpectedInnerRule(Rule::value, r))?,
        })
    }
}
