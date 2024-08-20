use pest::iterators::Pair;

use crate::{
    parser_items::{Attribute, Modifier, Visibility},
    Rule,
};

#[derive(Debug)]
pub struct Field {
    pub _attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub modifiers: Vec<Modifier>,
    pub field_type: String,
    pub name: String,
    pub assign_value: Option<String>,
}

impl From<Pair<'_, Rule>> for Field {
    fn from(value: Pair<'_, Rule>) -> Self {
        let mut attributes = vec![];
        let mut modifiers = vec![];
        let mut visibility = None;
        let mut field_type = None;
        let mut name = None;
        let mut assign_value = None;
        for pair in value.into_inner() {
            match pair.as_rule() {
                Rule::attribute => attributes.push(Attribute::from(pair)),
                Rule::visibility => visibility = Some(Visibility::from(pair)),
                Rule::modifier => modifiers.push(Modifier::from(pair)),
                Rule::fieldType => field_type = Some(pair.as_str().to_string()),
                Rule::identName => name = Some(pair.as_str().to_string()),
                Rule::assignValue => assign_value = Some(pair.as_str()[2..].to_string()),
                _ => unreachable!(),
            }
        }

        let (visibility, field_type, name) = match (visibility, field_type, name) {
            (Some(visibility), Some(field_type), Some(name)) => (visibility, field_type, name),
            _ => unreachable!(),
        };
        Self {
            _attributes: attributes,
            visibility,
            modifiers,
            field_type,
            name,
            assign_value,
        }
    }
}
