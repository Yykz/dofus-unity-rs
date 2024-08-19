use pest::iterators::Pair;

use crate::Rule;

mod class;
mod constructor;
mod r#enum;
mod field;
mod method;
mod parameter;
mod property;

pub use class::Class;
pub use constructor::Constructor;
pub use field::Field;
pub use method::Method;
pub use parameter::Parameter;
pub use property::Property;
pub use r#enum::{Enum, Variant};

#[allow(dead_code)]
#[derive(Debug)]
pub enum Member {
    Property(Property),
    Constructor(Constructor),
    Method(Method),
    Field(Field),
    Class(Class),
    Enum(Enum),
}

impl From<Pair<'_, Rule>> for Member {
    fn from(value: Pair<'_, Rule>) -> Self {
        for pair in value.into_inner() {
            match pair.as_rule() {
                Rule::property => return Self::Property(Property::from(pair)),
                Rule::constructor => return Self::Constructor(Constructor::from(pair)),
                Rule::method => return Self::Method(Method::from(pair)),
                Rule::field => return Self::Field(Field::from(pair)),
                Rule::class => return Self::Class(Class::from(pair)),
                Rule::r#enum => return Self::Enum(Enum::from(pair)),
                _ => unreachable!(),
            }
        }
        unreachable!()
    }
}
