use pest::iterators::Pair;

use crate::Rule;

#[derive(Debug)]
pub struct Attribute(pub String);

impl From<Pair<'_, Rule>> for Attribute {
    fn from(value: Pair<'_, Rule>) -> Self {
        Self(value.as_str().to_string())
    }
}
