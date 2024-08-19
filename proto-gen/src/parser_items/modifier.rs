use pest::iterators::Pair;

use crate::Rule;

#[derive(Debug, PartialEq)]
pub enum Modifier {
    Static,
    ReadOnly,
    Const,
    Sealed,
}

impl From<Pair<'_, Rule>> for Modifier {
    fn from(value: Pair<'_, Rule>) -> Self {
        match value.as_str() {
            "static" => Self::Static,
            "readonly" => Self::ReadOnly,
            "const" => Self::Const,
            "sealed" => Self::Sealed,
            _ => unreachable!(),
        }
    }
}
