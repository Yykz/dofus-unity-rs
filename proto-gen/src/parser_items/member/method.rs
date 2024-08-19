use pest::iterators::Pair;

use crate::{
    parser_items::{Attribute, Visibility},
    Rule,
};

use super::Parameter;

#[derive(Debug)]
pub struct Method {
    pub _attributes: Vec<Attribute>,
    pub _visibility: Visibility,
    pub _method_type: String,
    pub _name: String,
    pub _parameters: Vec<Parameter>,
}

impl From<Pair<'_, Rule>> for Method {
    fn from(value: Pair<'_, Rule>) -> Self {
        let mut attributes = vec![];
        let mut visibility = None;
        let mut method_type = None;
        let mut name = None;
        let mut parameters = vec![];
        for pair in value.into_inner() {
            match pair.as_rule() {
                Rule::attribute => attributes.push(Attribute::from(pair)),
                Rule::visibility => visibility = Some(Visibility::from(pair)),
                Rule::methodType => method_type = Some(pair.as_str().to_string()),
                Rule::methodName => name = Some(pair.as_str().to_string()),
                Rule::parameters => {
                    for pair in pair.into_inner() {
                        parameters.push(Parameter::from(pair))
                    }
                }
                Rule::methodBody => {}
                _ => unreachable!(),
            }
        }

        let (visibility, method_type, name) = match (visibility, method_type, name) {
            (Some(visibility), Some(method_type), Some(name)) => (visibility, method_type, name),
            _ => unreachable!(),
        };
        Self {
            _attributes: attributes,
            _visibility: visibility,
            _method_type: method_type,
            _name: name,
            _parameters: parameters,
        }
    }
}
