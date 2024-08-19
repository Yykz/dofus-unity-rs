use pest::iterators::Pair;

use crate::Rule;

#[derive(Debug, PartialEq)]
pub enum Visibility {
    Private,
    Public,
    Internal,
}

impl From<Pair<'_, Rule>> for Visibility {
    fn from(value: Pair<'_, Rule>) -> Self {
        match value.as_str() {
            "public" => Self::Public,
            "private" => Self::Private,
            "internal" => Self::Internal,
            _ => unreachable!(),
        }
    }
}
