use std::collections::HashSet;
use std::fmt::Display;

use super::message::{Error, Message};
use super::r#enum::Enum;
use crate::parser_items;

#[derive(Debug)]
pub enum ProtoEntity {
    Message(Message),
    Enum(Enum),
}

impl ProtoEntity {
    pub fn name(&self) -> String {
        match self {
            ProtoEntity::Message(message) => message.name.clone(),
            ProtoEntity::Enum(e) => e.name.clone(),
        }
    }

    pub fn resolve_types(&mut self, local: &HashSet<String>) {
        match self {
            ProtoEntity::Message(mesage) => { mesage.resolve_types(local) },
            ProtoEntity::Enum(_enum) => {},
        }
    }
}

impl TryFrom<parser_items::Namespace> for ProtoEntity {
    type Error = Error;

    fn try_from(namespace: parser_items::Namespace) -> Result<Self, Self::Error> {
        match namespace.member {
            parser_items::Member::Class(class) => Ok(Self::Message(Message::try_from_class(
                class,
                namespace.name,
            )?)),
            parser_items::Member::Enum(r#enum) => Ok(Self::Enum(Enum::new(r#enum, namespace.name))),
            _ => unreachable!(),
        }
    }
}

impl Display for ProtoEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtoEntity::Message(message) => message.fmt(f),
            ProtoEntity::Enum(e) => e.fmt(f),
        }
    }
}
