use crate::parser::{Error, Pair, Rule};

#[derive(Debug, Clone)]
pub struct Comment(pub String);

impl Comment {
    pub fn try_extract_namespace(&self) -> Option<&str> {
        self.0
            .trim_start_matches('/')
            .trim()
            .strip_prefix("Namespace: ")
    }

    pub fn extract_type_index(&self) -> Option<&str> {
        self.0
            .trim_start_matches('/')
            .trim()
            .strip_prefix("TypeDefIndex: ")
    }
}

impl TryFrom<Pair<'_>> for Comment {
    type Error = Error;

    fn try_from(value: Pair<'_>) -> Result<Self, Self::Error> {
        Error::rule_matches(Rule::COMMENT, value.as_rule())?;

        Ok(Self(value.as_str().to_string()))
    }
}
