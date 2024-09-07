use crate::protobuf::r#enum::ProtoEnum;
use crate::{
    parser::{self, data_structs::Class},
    protobuf::{builders::message::MessageBuilder, message::Unresolved, package::PackageMember},
};

//TODO Delete ProtoMemberBuilder, make TopLevelMap IntoIter also return namespace
#[derive(Debug, Clone)]
pub struct ProtoMemberBuilder {
    member: ProtoMemberBuilderInner,
    namespace: String,
}

impl ProtoMemberBuilder {
    pub fn new<T: Into<ProtoMemberBuilderInner>>(member: T, namespace: String) -> Self {
        Self {
            member: member.into(),
            namespace,
        }
    }

    pub fn inner(&mut self) -> &mut ProtoMemberBuilderInner {
        &mut self.member
    }

    pub fn destruct(self) -> (ProtoMemberBuilderInner, String) {
        let Self { member, namespace } = self;
        (member, namespace)
    }
}

#[derive(Debug, Clone)]
pub enum ProtoMemberBuilderInner {
    Enum(parser::data_structs::Enum),
    Message(MessageBuilder),
}

impl ProtoMemberBuilderInner {
    pub fn build(self) -> PackageMember<Unresolved> {
        match self {
            ProtoMemberBuilderInner::Enum(e) => ProtoEnum::from(e).into(),
            ProtoMemberBuilderInner::Message(m) => m.build().into(),
        }
    }

    pub fn name(&mut self) -> &mut String {
        match self {
            ProtoMemberBuilderInner::Enum(e) => &mut e.name,
            ProtoMemberBuilderInner::Message(m) => &mut m.class.name,
        }
    }

    pub fn type_index(&self) -> u64 {
        match self {
            ProtoMemberBuilderInner::Enum(e) => e.type_index.unwrap(),
            ProtoMemberBuilderInner::Message(m) => m.class.type_index.unwrap(),
        }
    }
}

impl From<Class> for ProtoMemberBuilderInner {
    fn from(value: Class) -> Self {
        Self::Message(MessageBuilder::from(value))
    }
}

impl From<parser::data_structs::Enum> for ProtoMemberBuilderInner {
    fn from(value: parser::data_structs::Enum) -> Self {
        Self::Enum(value)
    }
}
