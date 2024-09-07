use crate::parser::{Error, Pair, Rule};

#[derive(Debug, Clone)]
pub struct Attribute(pub String);

impl TryFrom<Pair<'_>> for Attribute {
    type Error = Error;

    fn try_from(value: Pair<'_>) -> Result<Self, Self::Error> {
        Error::rule_matches(Rule::attribute, value.as_rule())?;

        Ok(Self(value.as_str().to_string()))
    }
}
