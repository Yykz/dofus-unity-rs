use crate::parser::{Error, Pair, Rule};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MethodModifier {
    Override,
    Static,
    Abstract,
    Virtual,
    Sealed,
    Extern,
}

impl TryFrom<Pair<'_>> for MethodModifier {
    type Error = Error;

    fn try_from(value: Pair<'_>) -> Result<Self, Self::Error> {
        Error::rule_matches(Rule::methodModifier, value.as_rule())?;

        Ok(match value.as_str() {
            "override" => Self::Override,
            "static" => Self::Static,
            "abstract" => Self::Abstract,
            "virtual" => Self::Virtual,
            "sealed" => Self::Sealed,
            "extern" => Self::Extern,
            str => Err(Error::InvalidToken(
                Rule::methodParamModifier,
                str.to_string(),
            ))?,
        })
    }
}
