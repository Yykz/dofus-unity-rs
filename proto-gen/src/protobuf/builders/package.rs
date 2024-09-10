use std::{collections::HashMap, iter::FlatMap, vec::IntoIter};

use crate::{parser, protobuf::message::TypePath};

use super::message::MessageBuilder;
mod member;
pub use member::{ProtoMemberBuilder, ProtoMemberBuilderInner};

#[derive(Debug, Clone, Default)]
pub struct TopLevelMembers(HashMap<String, Vec<ProtoMemberBuilder>>);

impl IntoIterator for TopLevelMembers {
    type Item = ProtoMemberBuilder;

    type IntoIter = FlatMap<
        std::collections::hash_map::IntoValues<String, Vec<ProtoMemberBuilder>>,
        IntoIter<ProtoMemberBuilder>,
        fn(Vec<ProtoMemberBuilder>) -> IntoIter<ProtoMemberBuilder>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_values().flat_map(|m| m.into_iter())
    }
}

impl TopLevelMembers {
    pub fn insert_top<T: Into<ProtoMemberBuilderInner>>(&mut self, t: T, namespace: String) {
        self.insert_top_inner(t.into(), namespace)
    }

    fn insert_top_inner(&mut self, mut member: ProtoMemberBuilderInner, namespace: String) {
        self.0
            .entry(member.name().to_string())
            .or_default()
            .push(ProtoMemberBuilder::new(member, namespace))
    }

    pub fn insert_nested(&mut self, path: TypePath, member: ProtoMemberBuilderInner) {
        if let Some(messages) = self.0.get_mut(&path[0]) {
            let message = unambiguous_messages(messages, member.type_index());
            message.insert(path.index_range(1..), member);
        } else {
            println!("Warning: Failed to find the declaring type for the nested class implementing IMessage: {}", path.joined())
        }
    }

    pub fn insert_nested_oneof(&mut self, path: TypePath, member: parser::data_structs::Enum) {
        if let Some(messages) = self.0.get_mut(&path[0]) {
            let message = unambiguous_messages(messages, member.type_index.unwrap());
            message.insert_oneof(path.index_range(1..), member);
        } else {
            println!(
                "Warning: Failed to find the declaring type path for oneof: {}",
                path.joined()
            )
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct NestedMembers(Vec<Vec<(Vec<String>, ProtoMemberBuilderInner)>>);

impl NestedMembers {
    pub fn insert<T: Into<ProtoMemberBuilderInner>>(&mut self, path: Vec<String>, t: T) {
        self.insert_inner(path, t.into())
    }

    fn insert_inner(&mut self, path: Vec<String>, member: ProtoMemberBuilderInner) {
        assert!(!path.is_empty());

        if path.len() > self.0.len() {
            self.0.resize(path.len(), Default::default());
        }

        self.0[path.len() - 1].push((path, member))
    }
}

impl IntoIterator for NestedMembers {
    type Item = (Vec<String>, ProtoMemberBuilderInner);

    type IntoIter = FlatMap<
        IntoIter<Vec<(Vec<String>, ProtoMemberBuilderInner)>>,
        IntoIter<(Vec<String>, ProtoMemberBuilderInner)>,
        fn(
            Vec<(Vec<String>, ProtoMemberBuilderInner)>,
        ) -> IntoIter<(Vec<String>, ProtoMemberBuilderInner)>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().flat_map(|ite| ite.into_iter())
    }
}

fn unambiguous_messages(
    members: &mut [ProtoMemberBuilder],
    inner_type_index: u64,
) -> &mut MessageBuilder {
    let mut closest_message: Option<(u64, &mut MessageBuilder)> = None;
    for member in members.iter_mut() {
        if let ProtoMemberBuilderInner::Message(message) = member.inner() {
            let message_type_index = message.class.type_index.unwrap();
            if message_type_index < inner_type_index {
                continue;
            }
            let dif = message_type_index - inner_type_index;

            match closest_message {
                Some((old_dif, _)) => {
                    if old_dif > dif {
                        closest_message = Some((dif, message))
                    }
                }
                None => closest_message = Some((dif, message)),
            }
        }
    }

    closest_message.map(|(_, message)| message).unwrap()
}
