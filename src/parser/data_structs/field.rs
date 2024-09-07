use derive_builder::Builder;

use crate::parser::{
    common::{literal::Literal, visibility::Visibility},
    Error, Pair, Rule,
};

#[derive(Debug, Clone, Builder)]
pub struct Field {
    #[builder(setter(into))]
    pub ty: String,
    #[builder(setter(into))]
    pub name: String,
    pub visibility: Visibility,
    #[builder(setter(each(name = "modifier")), default)]
    pub modifiers: Vec<FieldModifier>,
    #[builder(setter(strip_option), default)]
    pub assign_value: Option<Literal>,
}

impl TryFrom<Pair<'_>> for Field {
    type Error = Error;

    fn try_from(value: Pair<'_>) -> Result<Self, Self::Error> {
        Error::rule_matches(Rule::field, value.as_rule())?;

        let mut field = FieldBuilder::default();

        for inner in value.into_inner() {
            match inner.as_rule() {
                Rule::COMMENT => {}
                Rule::fieldModifier => match FieldModifier::try_from(inner) {
                    Ok(modifier) => {
                        field.modifier(modifier);
                    }
                    Err(e) => panic!("{e}"),
                },
                Rule::modifierVisibility => match Visibility::try_from(inner) {
                    Ok(visibility) => {
                        field.visibility(visibility);
                    }
                    Err(e) => panic!("{e}"),
                },
                Rule::Type => {
                    field.ty(inner.as_str());
                }
                Rule::Ident => {
                    field.name(inner.as_str());
                }
                Rule::fieldAssignValue => {
                    for inner in inner.into_inner() {
                        match inner.as_rule() {
                            Rule::COMMENT => {}
                            Rule::value => match Literal::try_from(inner) {
                                Ok(literal) => {
                                    field.assign_value(literal);
                                }
                                Err(e) => panic!("{e}"),
                            },
                            r => Err(Error::UnexpectedInnerRule(Rule::fieldAssignValue, r))?,
                        }
                    }
                }
                r => Err(Error::UnexpectedInnerRule(Rule::field, r))?,
            }
        }

        field.build().map_err(Error::from_builder_error)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FieldModifier {
    ReadOnly,
    Const,
    Static,
}

impl TryFrom<Pair<'_>> for FieldModifier {
    type Error = Error;

    fn try_from(value: Pair<'_>) -> Result<Self, Self::Error> {
        Error::rule_matches(Rule::fieldModifier, value.as_rule())?;

        Ok(match value.as_str() {
            "readonly" => Self::ReadOnly,
            "const" => Self::Const,
            "static" => Self::Static,
            str => Err(Error::InvalidToken(Rule::fieldModifier, str.to_string()))?,
        })
    }
}
