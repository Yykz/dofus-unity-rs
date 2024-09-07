use derive_builder::Builder;
use param::Param;

mod modifier;
mod param;

use crate::parser::{common::visibility::Visibility, Error, Pair, Rule};
use modifier::MethodModifier;

#[allow(dead_code)]
#[derive(Debug, Clone, Builder)]
pub struct Method {
    visibility: Visibility,
    #[builder(setter(into))]
    name: String,
    #[builder(setter(each(name = "modifier")), default)]
    modifiers: Vec<MethodModifier>,
    #[builder(setter(into))]
    ty: String,
    #[builder(setter(each(name = "param")), default)]
    params: Vec<Param>,
}

impl TryFrom<Pair<'_>> for Method {
    type Error = Error;

    fn try_from(value: Pair<'_>) -> Result<Self, Self::Error> {
        Error::rule_matches(Rule::method, value.as_rule())?;

        let mut method = MethodBuilder::default();

        for inner in value.into_inner() {
            match inner.as_rule() {
                Rule::COMMENT | Rule::methodBody => {}
                Rule::Ident => {
                    method.name(inner.as_str());
                }
                Rule::Type => {
                    method.ty(inner.as_str());
                }
                Rule::modifierVisibility => match Visibility::try_from(inner) {
                    Ok(visibility) => {
                        method.visibility(visibility);
                    }
                    Err(e) => panic!("{e}"),
                },
                Rule::methodModifier => match MethodModifier::try_from(inner) {
                    Ok(modifier) => {
                        method.modifier(modifier);
                    }
                    Err(e) => panic!("{e}"),
                },
                Rule::methodParams => {
                    for inner in inner.into_inner() {
                        if matches!(inner.as_rule(), Rule::methodParam) {
                            match Param::try_from(inner) {
                                Ok(param) => {
                                    method.param(param);
                                }
                                Err(e) => panic!("{e}"),
                            }
                        }
                    }
                }
                r => Err(Error::UnexpectedInnerRule(Rule::method, r))?,
            }
        }

        method.build().map_err(Error::from_builder_error)
    }
}
