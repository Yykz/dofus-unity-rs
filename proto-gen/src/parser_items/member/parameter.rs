use pest::iterators::Pair;

use crate::Rule;

#[derive(Debug)]
pub struct Parameter {
    _parameter_type: String,
    _ident: String,
}

impl From<Pair<'_, Rule>> for Parameter {
    fn from(value: Pair<'_, Rule>) -> Self {
        let mut ident = None;
        let mut parameter_type = None;
        for pair in value.into_inner() {
            match pair.as_rule() {
                Rule::parameterType => parameter_type = Some(pair.as_str().to_string()),
                Rule::identName => ident = Some(pair.as_str().to_string()),
                _ => unreachable!(),
            }
        }
        let (ident, parameter_type) = match (ident, parameter_type) {
            (Some(ident), Some(parameter_type)) => (ident, parameter_type),
            _ => unreachable!(),
        };
        Self {
            _parameter_type: parameter_type,
            _ident: ident,
        }
    }
}
