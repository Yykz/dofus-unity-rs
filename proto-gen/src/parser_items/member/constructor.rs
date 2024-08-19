use pest::iterators::Pair;

use crate::parser_items::{Attribute, Rule, Visibility};

use super::Parameter;

#[derive(Debug)]
pub struct Constructor {
    _attributes: Vec<Attribute>,
    _visibility: Visibility,
    _name: String,
    _parameters: Vec<Parameter>,
}

impl From<Pair<'_, Rule>> for Constructor {
    fn from(value: Pair<'_, Rule>) -> Self {
        let mut attributes = vec![];
        let mut visibility = None;
        let mut name = None;
        let mut parameters = vec![];
        for pair in value.into_inner() {
            match pair.as_rule() {
                Rule::attribute => attributes.push(Attribute::from(pair)),
                Rule::visibility => visibility = Some(Visibility::from(pair)),
                Rule::identName => name = Some(pair.as_str().to_string()),
                Rule::parameters => {
                    for pair in pair.into_inner() {
                        parameters.push(Parameter::from(pair))
                    }
                }
                Rule::constructorBody => {}
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
            _name: name,
            _parameters: parameters,
        }
    }
}
