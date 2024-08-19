use pest::iterators::Pair;

use crate::Rule;

#[derive(Debug)]
pub struct Import(pub String);

impl From<Pair<'_, Rule>> for Import {
    fn from(value: Pair<'_, Rule>) -> Self {
        let str = value.as_str();
        Self((&str[6..str.len() - 1]).to_string())
    }
}
