use derive_builder::Builder;

use crate::parser::{
    common::{attribute::Attribute, comment::Comment, visibility::Visibility},
    Error, Pair, Rule,
};

use super::Field;

#[derive(Debug, Clone, Builder)]
pub struct Enum {
    pub visibility: Visibility,
    #[builder(setter(into))]
    pub name: String,
    #[builder(setter(strip_option, into), default)]
    pub type_index: Option<u64>,
    #[builder(setter(each(name = "member")), default)]
    pub members: Vec<EnumMember>,
}

impl TryFrom<Pair<'_>> for Enum {
    type Error = Error;

    fn try_from(value: Pair<'_>) -> Result<Self, Self::Error> {
        Error::rule_matches(Rule::r#enum, value.as_rule())?;

        let mut r#enum = EnumBuilder::default();

        for inner in value.into_inner() {
            match inner.as_rule() {
                Rule::modifierVisibility => match Visibility::try_from(inner) {
                    Ok(visibility) => {
                        r#enum.visibility(visibility);
                    }
                    Err(e) => panic!("{e}"),
                },
                Rule::COMMENT => {
                    let comment = Comment::try_from(inner).unwrap();

                    if let Some(type_index) = comment.extract_type_index() {
                        let type_index: u64 = type_index.parse().map_err(|_e| {
                            Error::InvalidToken(Rule::COMMENT, type_index.to_string())
                        })?;
                        r#enum.type_index(type_index);
                    }
                }
                Rule::Ident => {
                    r#enum.name(inner.as_str());
                }
                Rule::enumBody => {
                    let mut attributes = vec![];
                    for inner in inner.into_inner() {
                        match inner.as_rule() {
                            Rule::COMMENT => {}
                            Rule::attribute => match Attribute::try_from(inner) {
                                Ok(a) => attributes.push(a),
                                Err(e) => panic!("{e}"),
                            },
                            Rule::field => match Field::try_from(inner) {
                                Ok(field) => {
                                    r#enum.member(EnumMember {
                                        field,
                                        attributes: attributes.clone(),
                                    });
                                    attributes.clear();
                                }
                                Err(e) => panic!("{e}"),
                            },
                            r => Err(Error::UnexpectedInnerRule(Rule::enumBody, r))?,
                        }
                    }
                }
                r => Err(Error::UnexpectedInnerRule(Rule::r#enum, r))?,
            }
        }

        r#enum.build().map_err(Error::from_builder_error)
    }
}

#[derive(Debug, Clone)]
pub struct EnumMember {
    pub field: Field,
    pub attributes: Vec<Attribute>,
}
