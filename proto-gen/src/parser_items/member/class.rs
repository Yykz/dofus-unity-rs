use pest::iterators::Pair;

use crate::{
    parser_items::{Attribute, Interface, Modifier, Visibility},
    Rule,
};

use super::Member;

#[derive(Debug)]
pub struct Class {
    pub _attributes: Vec<Attribute>,
    pub _visibility: Visibility,
    pub _modifiers: Vec<Modifier>,
    pub name: String,
    pub interfaces: Vec<Interface>,
    pub body: Vec<Member>,
}

impl From<Pair<'_, Rule>> for Class {
    fn from(value: Pair<'_, Rule>) -> Self {
        let mut attributes = vec![];
        let mut visibility = None;
        let mut modifiers = vec![];
        let mut name = None;
        let mut interfaces = vec![];
        let mut body = vec![];
        for pair in value.into_inner() {
            match pair.as_rule() {
                Rule::attribute => attributes.push(Attribute::from(pair)),
                Rule::visibility => visibility = Some(Visibility::from(pair)),
                Rule::modifier => modifiers.push(Modifier::from(pair)),
                Rule::identName => name = Some(pair.as_str().to_string()),
                Rule::interfaces => {
                    for pair in pair.into_inner() {
                        interfaces.push(Interface::from(pair))
                    }
                }
                Rule::classBody => {
                    for pair in pair.into_inner() {
                        body.push(Member::from(pair))
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
            _modifiers: modifiers,
            name,
            interfaces,
            body,
        }
    }
}
