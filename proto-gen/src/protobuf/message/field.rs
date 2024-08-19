use std::fmt::Display;

use crate::parser_items;

use super::pascal_to_snake_case;

#[derive(Debug)]
pub enum Field {
    Normal(NormalField),
    OneOf(String, Vec<NormalField>),
}

impl From<NormalField> for Field {
    fn from(value: NormalField) -> Self {
        Self::Normal(value)
    }
}

#[derive(Debug)]
pub struct NormalField {
    label: Label,
    number: u32,
    r#type: Type,
    name: String,
}

impl NormalField {
    pub fn new<T: Into<String>>(name: String, r#type: T, number: u32, label: Label) -> Self {
        Self {
            label,
            number,
            r#type: Type::from(r#type),
            name,
        }
    }
}

impl Display for NormalField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\t{}{} {} = {};",
            self.label, self.r#type, self.name, self.number
        )
    }
}

#[derive(Debug)]
pub enum Label {
    None,
    Optional,
    Repeated,
}

impl Default for Label {
    fn default() -> Self {
        Self::None
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Label::None => Ok(()),
            Label::Optional => write!(f, "optional "),
            Label::Repeated => write!(f, "repeated "),
        }
    }
}

#[derive(Debug)]
pub struct Type(String);

impl<T> From<T> for Type
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        let value: String = value.into();
        // sint32, sint64, fixed32, fixed64, sfixed32, sfixed64 ?
        match value.as_str() {
            "int" => Self(String::from("int32")),
            "long" => Self(String::from("int64")),
            "uint" => Self(String::from("uint32")),
            "ulong" => Self(String::from("uint64")),
            "ByteString" => Self(String::from("string")),
            // double, bool, string, float is same
            _ => Self(value),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn is_parser_field(field: &parser_items::Field) -> bool {
    field.modifiers
        == vec![
            parser_items::Modifier::Static,
            parser_items::Modifier::ReadOnly,
        ]
        && field.name == "_parser"
}

pub fn is_unknow_field(field: &parser_items::Field) -> bool {
    field.name == "_unknownFields" && field.field_type == "UnknownFieldSet"
}

pub fn is_index_field(field: &parser_items::Field) -> bool {
    field.modifiers == [parser_items::Modifier::Const]
        && field.field_type == "int"
        && field.name.ends_with("FieldNumber")
        && field
            .assign_value
            .as_ref()
            .is_some_and(|s| s.parse::<u32>().is_ok())
}

pub fn index_field(field: &parser_items::Field) -> Option<(u32, String)> {
    if !is_index_field(field) {
        return None;
    }

    let index = field
        .assign_value
        .as_ref()
        .and_then(|s| s.parse::<u32>().ok())?;
    let name = pascal_to_snake_case(&field.name[..field.name.len() - 11]);

    Some((index, name))
}
