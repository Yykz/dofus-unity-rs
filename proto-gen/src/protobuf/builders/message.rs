use std::collections::{HashMap, HashSet};

use convert_case::{Case, Casing};

use crate::{
    parser::{self, data_structs::Class},
    protobuf::message::{Field, MapField, NormalField, ProtoMessage, Unresolved},
};

use super::{field::GenField, package::ProtoMemberBuilderInner, property::GenProperty};

#[derive(Debug, Clone)]
pub struct MessageBuilder {
    pub class: Class,
    pub inner_members: Vec<ProtoMemberBuilderInner>,
    pub oneof: Vec<parser::data_structs::Enum>,
}

impl MessageBuilder {
    pub fn build(self) -> ProtoMessage<Unresolved> {
        let inner_members = self.inner_members.into_iter().map(|m| m.build()).collect();

        let Class {
            visibility: _,
            modifier: _,
            name,
            interfaces: _,
            type_index: _,
            methods: _,
            fields,
            properties,
        } = self.class;

        let mut ty_prop = HashMap::new();

        for property in properties {
            if let GenProperty::Field(n, t) = GenProperty::try_from(&property).unwrap() {
                ty_prop.insert(n, t);
            }
        }

        let mut numbers_map = HashMap::new();
        let mut ty = vec![];
        let mut oneof_ty = HashSet::new();
        let mut oneof_obj = HashSet::new();
        let mut re_ty = vec![];
        let mut map_ty = vec![];

        // TODO check default, codecs at the end
        let mut default = HashSet::new();
        let mut re_codec = HashMap::new();
        let mut map_codec = HashMap::new();

        for f in fields {
            match GenField::try_from(f).unwrap() {
                GenField::Number(n, x) => {
                    numbers_map.insert(n, x);
                }
                GenField::Type(n, t) => ty.push((n, t)),
                GenField::RepeatedType(n, t) => re_ty.push((n, t)),
                GenField::MapType(n, t) => map_ty.push((n, t)),
                GenField::DefaultValue(n, _t) => {
                    default.insert(n);
                }
                GenField::RepeatedCodec(n, t) => {
                    re_codec.insert(n, t);
                }
                GenField::MapCodec(n, t) => {
                    map_codec.insert(n, t);
                }
                GenField::Blanket => {}
                GenField::OneofCaseObject(n) => {
                    oneof_obj.insert(n);
                }
                GenField::OneofCaseType(n) => {
                    oneof_ty.insert(n);
                }
            }
        }

        let mut fields = vec![];
        // optional/normal fields
        for (n, t) in ty.into_iter() {
            let x = numbers_map.remove(&n).unwrap();
            ty_prop.remove(&n).unwrap();
            if default.remove(&n) {
                let f = NormalField::new(t, n, x);
                fields.push(Field::Optional(f));
            } else {
                let f = NormalField::new(t, n, x);
                fields.push(Field::Normal(f));
            }
        }

        // repeated fields
        for (n, t) in re_ty.into_iter() {
            let x = numbers_map.remove(&n).unwrap();
            let f = NormalField::new(t, n, x);
            fields.push(Field::Repeated(f));
        }

        // map fields
        for (n, t) in map_ty.into_iter() {
            let x = numbers_map.remove(&n).unwrap();
            let (key_ty, val_ty) = t.split_once(", ").unwrap();
            let f = MapField::new(key_ty, val_ty, n, x);
            fields.push(Field::Map(f));
        }

        for e in self.oneof.into_iter() {
            let name = e.name.strip_suffix("OneofCase");
            if name.is_none() {
                continue;
            }
            let name = name.unwrap().from_case(Case::Pascal).to_case(Case::Snake);
            oneof_ty.remove(&name);
            oneof_obj.remove(&name);

            let cases: Vec<_> = e
                .members
                .clone()
                .into_iter()
                .filter(|m| !["value__", "None"].contains(&m.field.name.as_str()))
                .map(|m| {
                    (
                        m.field.name,
                        m.field.assign_value.and_then(|l| l.u32()).unwrap(),
                    )
                })
                .map(|(n, x)| {
                    let mut name = n.from_case(Case::Pascal).to_case(Case::Snake);
                    if numbers_map.remove(&name).is_none() {
                        name = name.strip_suffix('_').unwrap().to_string();
                        numbers_map.remove(&name).unwrap();
                    }

                    let ty = ty_prop.remove(&name).unwrap();

                    NormalField::new(ty, name, x)
                })
                .collect();
            fields.push(Field::OneOf(name, cases))
        }

        assert!(numbers_map.is_empty());
        assert!(ty_prop.is_empty());
        assert!(oneof_obj.is_empty());
        assert!(oneof_ty.is_empty());

        ProtoMessage::new(name, fields, inner_members)
    }

    pub fn insert_oneof(&mut self, path: &[String], mut member: parser::data_structs::Enum) {
        assert!(!path.is_empty());

        if path.len() == 1 {
            member.name = path[0].clone();
            self.oneof.push(member);
            return;
        }

        for inner in self.inner_members.iter_mut() {
            if let ProtoMemberBuilderInner::Message(message) = inner {
                if message.class.name == path[0] {
                    message.insert_oneof(&path[1..], member);
                    return;
                }
            }
        }
        unreachable!()
    }

    pub fn insert(&mut self, path: &[String], mut member: ProtoMemberBuilderInner) {
        assert!(!path.is_empty());

        if path.len() == 1 {
            *member.name() = path[0].clone();
            self.inner_members.push(member);
            return;
        }

        for inner in self.inner_members.iter_mut() {
            if let ProtoMemberBuilderInner::Message(message) = inner {
                if message.class.name == path[0] {
                    message.insert(&path[1..], member);
                    return;
                }
            }
        }
        unreachable!()
    }
}

impl From<Class> for MessageBuilder {
    fn from(value: Class) -> Self {
        Self {
            class: value,
            inner_members: Default::default(),
            oneof: Default::default(),
        }
    }
}
