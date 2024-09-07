use derive_builder::Builder;

use crate::parser::{
    common::{comment::Comment, visibility::Visibility},
    Error, Pair, Rule,
};

use super::{Field, Method, Property};

#[derive(Debug, Clone, Builder)]
pub struct Class {
    pub visibility: Visibility,
    #[builder(setter(strip_option), default)]
    pub modifier: Option<ClassModifier>,
    #[builder(setter(into))]
    pub name: String,
    #[builder(setter(each(name = "interface", into)), default)]
    pub interfaces: Vec<String>,
    #[builder(setter(strip_option, into), default)]
    pub type_index: Option<u64>,
    #[builder(setter(each(name = "method")), default)]
    pub methods: Vec<Method>,
    #[builder(setter(each(name = "field")), default)]
    pub fields: Vec<Field>,
    #[builder(setter(each(name = "property")), default)]
    pub properties: Vec<Property>,
}

impl Class {
    fn process_body(class: &mut ClassBuilder, pair: Pair) -> Result<(), Error> {
        Error::rule_matches(Rule::classBody, pair.as_rule())?;

        for class_member in pair.into_inner() {
            if !matches!(class_member.as_rule(), Rule::classMember) {
                continue;
            }
            for inner in class_member.into_inner() {
                match inner.as_rule() {
                    Rule::COMMENT => {}
                    Rule::attribute => {}
                    Rule::method => match Method::try_from(inner) {
                        Ok(method) => {
                            class.method(method);
                        }
                        Err(e) => panic!("{e}"),
                    },
                    Rule::field => match Field::try_from(inner) {
                        Ok(field) => {
                            class.field(field);
                        }
                        Err(e) => panic!("{e}"),
                    },
                    Rule::property => match Property::try_from(inner) {
                        Ok(property) => {
                            class.property(property);
                        }
                        Err(e) => panic!("{e}"),
                    },
                    r => Err(Error::UnexpectedInnerRule(Rule::classBody, r))?,
                }
            }
        }

        Ok(())
    }
}

impl TryFrom<Pair<'_>> for Class {
    type Error = Error;

    fn try_from(value: Pair) -> Result<Self, Self::Error> {
        Error::rule_matches(Rule::class, value.as_rule())?;

        let mut class = ClassBuilder::default();

        for inner in value.into_inner() {
            match inner.as_rule() {
                Rule::COMMENT => {
                    let comment = Comment::try_from(inner).unwrap();

                    if let Some(type_index) = comment.extract_type_index() {
                        let type_index: u64 = type_index.parse().map_err(|_e| {
                            Error::InvalidToken(Rule::COMMENT, type_index.to_string())
                        })?;
                        class.type_index(type_index);
                    }
                }
                Rule::modifierVisibility => match Visibility::try_from(inner) {
                    Ok(visibility) => {
                        class.visibility(visibility);
                    }
                    Err(e) => panic!("{e}"),
                },
                Rule::Ident => {
                    class.name(inner.as_str());
                }
                Rule::classBody => {
                    if let Err(e) = Self::process_body(&mut class, inner) {
                        panic!("{e}")
                    }
                }
                Rule::classModifier => match ClassModifier::try_from(inner) {
                    Ok(modifier) => {
                        class.modifier(modifier);
                    }
                    Err(e) => panic!("{e}"),
                },
                Rule::classInterfaces => {
                    for inner in inner.into_inner() {
                        if matches!(inner.as_rule(), Rule::Ident) {
                            class.interface(inner.as_str());
                        }
                    }
                }
                r => Err(Error::UnexpectedInnerRule(Rule::classBody, r))?,
            }
        }

        class.build().map_err(Error::from_builder_error)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClassModifier {
    Sealed,
    Static,
    Abstract,
}

impl TryFrom<Pair<'_>> for ClassModifier {
    type Error = Error;

    fn try_from(value: Pair<'_>) -> Result<Self, Self::Error> {
        Error::rule_matches(Rule::classModifier, value.as_rule())?;
        Ok(match value.as_str() {
            "sealed" => Self::Sealed,
            "static" => Self::Static,
            "abstract" => Self::Abstract,
            str => Err(Error::InvalidToken(Rule::classModifier, str.to_string()))?,
        })
    }
}
