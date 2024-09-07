use crate::parser::{Error, Pair, Rule};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Visibility {
    ProtectedInternal,
    Public,
    Private,
    Internal,
    Protected,
}

impl TryFrom<Pair<'_>> for Visibility {
    type Error = Error;

    fn try_from(value: Pair<'_>) -> Result<Self, Self::Error> {
        Error::rule_matches(Rule::modifierVisibility, value.as_rule())?;

        Ok(match value.as_str() {
            "internal" => Self::Internal,
            "private" => Self::Private,
            "protected" => Self::Protected,
            "public" => Self::Public,
            "protected internal" => Self::ProtectedInternal,
            str => Err(Error::InvalidToken(
                Rule::modifierVisibility,
                str.to_string(),
            ))?,
        })
    }
}
