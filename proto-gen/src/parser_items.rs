use crate::Rule;
use pest::iterators::Pair;

#[derive(Debug)]
pub struct Comment(String);

impl From<Pair<'_, Rule>> for Comment {
    fn from(value: Pair<'_, Rule>) -> Self {
        Self(value.as_str().to_string())
    }
}

#[derive(Debug)]
pub struct Interface(pub String);

impl From<Pair<'_, Rule>> for Interface {
    fn from(value: Pair<'_, Rule>) -> Self {
        Self(value.as_str().to_string())
    }
}

mod namespace;
pub use namespace::*;

mod visibility;
pub use visibility::*;

mod modifier;
pub use modifier::*;

mod attribute;
pub use attribute::*;

mod member;
pub use member::*;

mod file;
pub use file::*;
