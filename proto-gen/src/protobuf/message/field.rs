use super::pascal_to_snake_case;
use crate::parser_items;
use std::{collections::HashSet, fmt::Display};

#[derive(Debug)]
pub enum Field {
    Normal(NormalField),
    OneOf(String, Vec<NormalField>),
}

impl Field {
    pub fn resolve_type(&mut self, local: &HashSet<String>) {
        match self {
            Field::Normal(field) => field.resolve_type(local),
            Field::OneOf(_s, oneof) => oneof.iter_mut().for_each(|field| field.resolve_type(local)),
        }
    }
}

impl NormalField {
    pub fn resolve_type(&mut self, local: &HashSet<String>) {
        self.r#type.resolve(local)
    }
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
enum Type {
    Map(String, String),
    Normal(String)
}

fn resolve_type(s: &mut String, local: &HashSet<String>) {
    if s == "Any" {
        *s = format!(".google.protobuf.{}", s)
    } else if !Type::is_primitive(s) && !local.contains(s.split(".").next().unwrap()) {
        *s = format!(".game.common.{}", s)
    }
}

impl Type {
    pub fn resolve(&mut self, local: &HashSet<String>) {
        match self {
            Type::Map(s, s1) => {
                resolve_type(s, local);

                resolve_type(s1, local)
            },
            Type::Normal(s) => {
                resolve_type(s, local)
            },
        }
    }

    fn is_primitive(s: &str) -> bool {
        ["int32", "int64", "uint32", "uint64", "string", "double", "bool", "string", "float"].contains(&s)
    }
}

impl<T> From<T> for Type
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        let value: String = value.into();

        if value.starts_with("MapField") {
            let value = &value[9..value.len() - 1];
            if let Some((key, value)) = value.split_once(", ") {
                let key = Self::from(key);
                let value = Self::from(value);
                return Self::Map(key.to_string(), value.to_string());
            }
        }
        // sint32, sint64, fixed32, fixed64, sfixed32, sfixed64 ?
        Self::Normal(match value.as_str() {
            "int" => String::from("int32"),
            "long" => String::from("int64"),
            "uint" => String::from("uint32"),
            "ulong" => String::from("uint64"),
            "ByteString" => String::from("string"),
            "double" | "bool" | "string" | "float" => value,
            _ => value.replace(".Types.", "."),
        })
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Map(k, v) => write!(f, "map<{}, {}>", k, v),
            Type::Normal(s) => write!(f, "{}", s),
        }
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
