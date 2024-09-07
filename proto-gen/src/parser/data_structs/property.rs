use derive_builder::Builder;

use crate::parser::{common::visibility::Visibility, Error, Pair, Rule};

#[derive(Debug, Clone, Builder)]
pub struct Property {
    pub visibility: Visibility,
    #[builder(setter(into))]
    pub name: String,
    #[builder(setter(each(name = "modifier")), default)]
    pub modifiers: Vec<PropertyModifier>,
    #[builder(setter(each(name = "kind")), default)]
    pub kinds: Vec<PropertyKind>,
    #[builder(setter(into))]
    pub ty: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyKind {
    Get,
    Set,
}

impl TryFrom<Pair<'_>> for PropertyKind {
    type Error = Error;

    fn try_from(value: Pair<'_>) -> Result<Self, Self::Error> {
        Error::rule_matches(Rule::propertyKind, value.as_rule())?;

        match value.as_str() {
            "get" => Ok(Self::Get),
            "set" => Ok(Self::Set),
            str => Err(Error::InvalidToken(Rule::propertyKind, str.to_string())),
        }
    }
}

impl TryFrom<Pair<'_>> for Property {
    type Error = Error;

    fn try_from(value: Pair<'_>) -> Result<Self, Self::Error> {
        Error::rule_matches(Rule::property, value.as_rule())?;

        let mut property = PropertyBuilder::default();

        for inner in value.into_inner() {
            match inner.as_rule() {
                Rule::COMMENT => {}
                Rule::propertyBody => {
                    for inner in inner.into_inner() {
                        match inner.as_rule() {
                            Rule::COMMENT => {}
                            Rule::propertyKind => {
                                let kind = PropertyKind::try_from(inner).unwrap();
                                property.kind(kind);
                            }
                            r => Err(Error::UnexpectedInnerRule(Rule::propertyBody, r))?,
                        }
                    }
                }
                Rule::Ident => {
                    property.name(inner.as_str());
                }
                Rule::Type => {
                    property.ty(inner.as_str());
                }
                Rule::modifierVisibility => match Visibility::try_from(inner) {
                    Ok(visibility) => {
                        property.visibility(visibility);
                    }
                    Err(e) => panic!("{e}"),
                },
                Rule::propertyModifier => match PropertyModifier::try_from(inner) {
                    Ok(modifier) => {
                        property.modifier(modifier);
                    }
                    Err(e) => panic!("{e}"),
                },
                r => Err(Error::UnexpectedInnerRule(Rule::property, r))?,
            }
        }

        property.build().map_err(Error::from_builder_error)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PropertyModifier {
    Virtual,
    Static,
    Sealed,
    Override,
    Abstract,
}

impl TryFrom<Pair<'_>> for PropertyModifier {
    type Error = Error;

    fn try_from(value: Pair<'_>) -> Result<Self, Self::Error> {
        Error::rule_matches(Rule::propertyModifier, value.as_rule())?;

        Ok(match value.as_str() {
            "virtual" => Self::Virtual,
            "static" => Self::Static,
            "sealed" => Self::Sealed,
            "override" => Self::Override,
            "abstract" => Self::Abstract,
            str => Err(Error::InvalidToken(Rule::propertyModifier, str.to_string()))?,
        })
    }
}
