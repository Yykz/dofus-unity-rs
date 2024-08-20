use std::{collections::{HashMap, HashSet}, fmt::Display};

use crate::parser_items;

use super::{r#enum::Enum, ProtoEntity};

mod field;
use field::*;

#[derive(Debug)]
pub struct Message {
    _namespace: String,
    pub name: String,
    fields: Vec<Field>,
    inner: Vec<ProtoEntity>,
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display_inner(f, 0)
    }
}

pub enum Error {
    ClassDontImplIMessage,
}

impl Message {
    pub fn resolve_types(&mut self, local: &HashSet<String>) {
        self.fields.iter_mut().for_each(|field| field.resolve_type(local));
        self.inner.iter_mut().for_each(|entity| entity.resolve_types(local));
    }

    fn display_inner(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        nest_count: usize,
    ) -> std::fmt::Result {
        let nest: String = "\t".repeat(nest_count);
        writeln!(f, "{nest}message {} {{", self.name)?;
        for field in self.fields.iter() {
            match field {
                Field::Normal(field) => {
                    writeln!(f, "{nest}{}", field)?;
                }
                Field::OneOf(name, fields) => {
                    writeln!(f, "{nest}\toneof {} {{", name)?;
                    for field in fields {
                        writeln!(f, "{nest}\t{}", field)?;
                    }
                    writeln!(f, "{nest}\t}}")?;
                }
            }
        }

        for inner_entity in self.inner.iter() {
            match inner_entity {
                ProtoEntity::Message(m) => m.display_inner(f, nest_count + 1)?,
                ProtoEntity::Enum(e) => e.display_inner(f, nest_count + 1)?,
            }
        }

        writeln!(f, "{nest}}}")
    }

    pub fn try_from_class(class: parser_items::Class, namespace: String) -> Result<Self, Error> {
        if !class.interfaces.iter().any(|i| i.0 == "IMessage") {
            return Err(Error::ClassDontImplIMessage);
        }

        let mut indexs: HashMap<String, u32> = HashMap::new();
        let mut fields = vec![];
        let mut properties: HashMap<String, String> = HashMap::new();
        let mut unmatched_fields = HashMap::new();
        let mut inner_entities = vec![];

        for member in class.body {
            match member {
                parser_items::Member::Field(field) => {
                    if is_parser_field(&field) || is_unknow_field(&field) {
                        continue;
                    }

                    if let Some((index, name)) = index_field(&field) {
                        indexs.insert(name, index);
                        continue;
                    }

                    let name_camel = &field.name[..field.name.len() - 1];
                    let pascal_name = camel_to_pascal(name_camel);
                    let name = pascal_to_snake_case(name_camel);
                    if let Some(index) = indexs.remove(&name) {
                        if properties
                            .get(&format!("Has{pascal_name}"))
                            .is_some_and(|t| *t == "bool")
                        {
                            fields.push(
                                NormalField::new(name, field.field_type, index, Label::Optional)
                                    .into(),
                            );
                            continue;
                        } else if field.field_type.starts_with("RepeatedField") {
                            let r = &field.field_type[14..field.field_type.len() - 1];
                            fields.push(
                                NormalField::new(name, r.to_string(), index, Label::Repeated)
                                    .into(),
                            );

                            continue;
                        }

                        fields.push(
                            NormalField::new(name, field.field_type, index, Label::None).into(),
                        )
                    } else {
                        unmatched_fields.insert(name, field);
                    }
                }
                parser_items::Member::Class(inner_class) => {
                    if inner_class.name != "Types" {
                        panic!(
                            "Unexpected class \"{}\" in {}.{}",
                            inner_class.name, namespace, class.name
                        );
                    }
                    for inner_class_member in inner_class.body {
                        let inner_namespace = format!("{}.{}", namespace, class.name);
                        match inner_class_member {
                            parser_items::Member::Class(inner_inner_class) => {
                                inner_entities.push(ProtoEntity::Message(Self::try_from_class(
                                    inner_inner_class,
                                    inner_namespace,
                                )?))
                            }
                            parser_items::Member::Enum(r#enum) => inner_entities
                                .push(ProtoEntity::Enum(Enum::new(r#enum, inner_namespace))),
                            _ => {}
                        }
                    }
                }
                parser_items::Member::Enum(r#enum) => {
                    if !r#enum.name.ends_with("OneofCase") {
                        panic!(
                            "Unexpected enum \"{}\" in {}.{}",
                            r#enum.name, namespace, class.name
                        );
                    }
                    let mut cases = vec![];
                    for variant in r#enum.variants {
                        if variant.name == "None" {
                            continue;
                        }

                        let name = pascal_to_snake_case(&variant.name);
                        if let Some(index) = indexs.remove(&name) {
                            let r#type = properties
                                .get(&variant.name)
                                .unwrap_or(&variant.name)
                                .clone();
                            cases.push(NormalField::new(name, r#type, index, Label::None))
                        } else {
                            eprintln!("cannot find corresponding field number {}", name);
                        }
                    }
                    let oneof_name_pascal = &r#enum.name[..r#enum.name.len() - 9];
                    let oneof_name_snake = pascal_to_snake_case(oneof_name_pascal);
                    let object_field = unmatched_fields.remove(&oneof_name_snake);
                    let enum_field = unmatched_fields.remove(&format!("{}_case", oneof_name_snake));

                    if let (Some(object_field), Some(enum_field)) = (object_field, enum_field) {
                        if !(is_object_field(&object_field)
                            && enum_field.field_type.ends_with(&format!(
                                "{}.{}OneofCase",
                                class.name, oneof_name_pascal
                            ))
                            && enum_field.visibility == parser_items::Visibility::Private)
                        {
                            continue;
                        }
                        fields.push(Field::OneOf(oneof_name_snake, cases));
                    }
                }
                parser_items::Member::Property(property) => {
                    properties.insert(property.name, property.property_type);
                }
                _ => {}
            }
        }

        Ok(Self {
            name: class.name,
            fields,
            inner: inner_entities,
            _namespace: namespace,
        })
    }
}

fn is_object_field(field: &parser_items::Field) -> bool {
    field.field_type == "object" && field.visibility == parser_items::Visibility::Private
}

fn pascal_to_snake_case<S: AsRef<str>>(s: S) -> String {
    let s = s.as_ref();
    let mut r = String::with_capacity(s.len() + 5);
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i != 0 {
                r.push('_');
            }
            r.push(c.to_ascii_lowercase());
        } else {
            r.push(c);
        }
    }
    r
}

fn camel_to_pascal<S: AsRef<str>>(s: S) -> String {
    let mut c = s.as_ref().chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
