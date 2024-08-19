use pest::iterators::Pair;

use crate::{
    parser_items::{Attribute, Modifier, Visibility},
    Rule,
};

#[derive(Debug)]
pub struct Property {
    pub _attributes: Vec<Attribute>,
    pub _visibility: Visibility,
    pub _modifier: Option<Modifier>,
    pub property_type: String,
    pub name: String,
    pub _body: Vec<Accessor>,
}

#[derive(Debug)]
pub struct Accessor;

impl From<Pair<'_, Rule>> for Property {
    fn from(value: Pair<'_, Rule>) -> Self {
        let mut attributes = vec![];
        let mut visibility = None;
        let mut modifier = None;
        let mut property_type = None;
        let mut property_name = None;
        let mut body = vec![];

        for pair in value.into_inner() {
            match pair.as_rule() {
                Rule::attribute => attributes.push(Attribute::from(pair)),
                Rule::visibility => visibility = Some(Visibility::from(pair)),
                Rule::modifier => modifier = Some(Modifier::from(pair)),
                Rule::propertyType => property_type = Some(pair.as_str().to_string()),
                Rule::propertyName => property_name = Some(pair.as_str().to_string()),
                Rule::propertyBody => body.push(Accessor),
                _ => unreachable!(),
            }
        }
        let (visibility, property_type, property_name) =
            match (visibility, property_type, property_name) {
                (Some(visibility), Some(property_type), Some(property_name)) => {
                    (visibility, property_type, property_name)
                }
                _ => unreachable!(),
            };
        Self {
            _attributes: attributes,
            _visibility: visibility,
            _modifier: modifier,
            property_type,
            name: property_name,
            _body: body,
        }
    }
}
