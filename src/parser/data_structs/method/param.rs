use derive_builder::Builder;

use crate::parser::{common::literal::Literal, Error, Pair, Rule};

#[allow(dead_code)]
#[derive(Debug, Clone, Builder)]
pub struct Param {
    #[builder(setter(strip_option), default)]
    modifier: Option<ParamModifier>,
    #[builder(setter(into))]
    ty: String,
    #[builder(setter(strip_option, into), default)]
    name: Option<String>,
    #[builder(setter(strip_option), default)]
    assign_value: Option<Literal>,
}

impl TryFrom<Pair<'_>> for Param {
    type Error = Error;

    fn try_from(value: Pair<'_>) -> Result<Self, Self::Error> {
        Error::rule_matches(Rule::methodParam, value.as_rule())?;

        let mut param = ParamBuilder::default();

        for inner in value.into_inner() {
            match inner.as_rule() {
                Rule::COMMENT | Rule::paramAttr => {}
                Rule::methodParamModifier => match ParamModifier::try_from(inner) {
                    Ok(modifier) => {
                        param.modifier(modifier);
                    }
                    Err(e) => panic!("{e}"),
                },
                Rule::Type => {
                    param.ty(inner.as_str());
                }
                Rule::Ident => {
                    param.name(inner.as_str());
                }
                Rule::fieldAssignValue => {
                    for inner in inner.into_inner() {
                        match inner.as_rule() {
                            Rule::COMMENT => {}
                            Rule::value => match Literal::try_from(inner) {
                                Ok(literal) => {
                                    param.assign_value(literal);
                                }
                                Err(e) => panic!("{e}"),
                            },
                            r => Err(Error::UnexpectedInnerRule(Rule::fieldAssignValue, r))?,
                        }
                    }
                }
                r => Err(Error::UnexpectedInnerRule(Rule::methodParam, r))?,
            }
        }

        param.build().map_err(Error::from_builder_error)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParamModifier {
    Out,
    In,
    Ref,
}

impl TryFrom<Pair<'_>> for ParamModifier {
    type Error = Error;

    fn try_from(value: Pair<'_>) -> Result<Self, Self::Error> {
        Error::rule_matches(Rule::methodParamModifier, value.as_rule())?;

        Ok(match value.as_str() {
            "out" => Self::Out,
            "in" => Self::In,
            "ref" => Self::Ref,
            str => Err(Error::InvalidToken(
                Rule::methodParamModifier,
                str.to_string(),
            ))?,
        })
    }
}
