mod class;
mod r#enum;
mod field;
mod method;
mod property;

pub use class::*;
pub use field::*;
pub use method::*;
pub use property::*;
pub use r#enum::*;

use crate::parser::{common::attribute::Attribute, Error, Pair, Rule};

#[derive(Debug, Clone)]
pub struct DataStruct {
    pub namespace: Option<String>,
    pub attributes: Vec<Attribute>,
    pub kind: DataStructInner,
}

impl TryFrom<Pair<'_>> for DataStruct {
    type Error = Error;

    fn try_from(value: Pair<'_>) -> Result<Self, Self::Error> {
        assert!(matches!(value.as_rule(), Rule::fileMember));

        let mut attributes = vec![];

        for inner in value.into_inner() {
            match inner.as_rule() {
                Rule::attribute => match Attribute::try_from(inner) {
                    Ok(attribute) => attributes.push(attribute),
                    Err(e) => panic!("{e}"),
                },
                Rule::class => {
                    return Ok(DataStruct {
                        attributes,
                        kind: DataStructInner::Class(Class::try_from(inner)?),
                        namespace: None,
                    });
                }
                Rule::r#enum => {
                    return Ok(DataStruct {
                        attributes,
                        kind: DataStructInner::Enum(Enum::try_from(inner)?),
                        namespace: None,
                    });
                }
                Rule::r#struct => {
                    return Ok(DataStruct {
                        attributes,
                        kind: DataStructInner::Struct,
                        namespace: None,
                    });
                }
                Rule::interface => {
                    return Ok(DataStruct {
                        attributes,
                        kind: DataStructInner::Interface,
                        namespace: None,
                    });
                }
                r => {
                    return Err(Error::UnexpectedInnerRule(Rule::fileMember, r));
                }
            }
        }

        unreachable!()
    }
}

#[derive(Debug, Clone)]
pub enum DataStructInner {
    Class(Class),
    Enum(Enum),
    Interface,
    Struct,
}
