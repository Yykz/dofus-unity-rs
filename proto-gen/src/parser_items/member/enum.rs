use pest::iterators::Pair;

use crate::parser_items::{Attribute, Rule, Visibility};

#[derive(Debug)]
pub struct Enum {
    pub _attributes: Vec<Attribute>,
    pub _visibility: Visibility,
    pub name: String,
    pub variants: Vec<Variant>,
}

#[derive(Debug)]
pub struct Variant {
    pub attributes: Vec<Attribute>,
    pub name: String,
    pub _value: Option<String>,
}

impl From<Pair<'_, Rule>> for Variant {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let mut attributes = vec![];
        let mut name = None;
        let mut value = None;

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::attribute => attributes.push(Attribute::from(pair)),
                Rule::identName => name = Some(pair.as_str().to_string()),
                Rule::assignEnum => value = Some(pair.as_str().to_string()),
                _ => unreachable!(),
            }
        }

        Self {
            attributes,
            name: name.unwrap(),
            _value: value,
        }
    }
}

impl From<Pair<'_, Rule>> for Enum {
    fn from(value: Pair<'_, Rule>) -> Self {
        let mut attributes = vec![];
        let mut visibility = None;
        let mut name = None;
        let mut variants = vec![];
        for pair in value.into_inner() {
            match pair.as_rule() {
                Rule::attribute => attributes.push(Attribute::from(pair)),
                Rule::visibility => visibility = Some(Visibility::from(pair)),
                Rule::identName => name = Some(pair.as_str().to_string()),
                Rule::enumBody => {
                    for pair in pair.into_inner() {
                        variants.push(Variant::from(pair))
                    }
                }
                _ => unreachable!(),
            }
        }
        let (visibility, name) = match (visibility, name) {
            (Some(visibility), Some(name)) => (visibility, name),
            _ => unreachable!(),
        };
        Self {
            _attributes: attributes,
            _visibility: visibility,
            name,
            variants,
        }
    }
}
