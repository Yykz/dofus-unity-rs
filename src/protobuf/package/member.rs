use std::collections::{HashMap, HashSet};

use crate::protobuf::{
    message::{ProtoMessage, Resolved, Unresolved},
    r#enum::ProtoEnum,
    DisplayWithTabs,
};

use super::{import::ProtoImport, ExternTypeMap};

#[derive(Debug, Clone)]
pub enum PackageMember<State> {
    Enum(ProtoEnum),
    Message(ProtoMessage<State>),
}
impl_display_via_display_with_tabs!(PackageMember<Resolved>);

impl PackageMember<Unresolved> {
    pub fn resolve(
        self,
        member_namespace: String,
        top_map: &HashMap<String, Vec<PackageMember<Unresolved>>>,
        imports: &mut HashSet<ProtoImport>,
        extern_ty: &ExternTypeMap,
    ) -> PackageMember<Resolved> {
        match self {
            PackageMember::Enum(e) => e.into(),
            PackageMember::Message(m) => m
                .resolve(member_namespace, top_map, imports, extern_ty)
                .into(),
        }
    }

    pub fn contains(&self, path: &[&str]) -> bool {
        match self {
            PackageMember::Enum(e) => path.len() == 1 && e.name == path[0],
            PackageMember::Message(m) => m.contains(path),
        }
    }
}

impl DisplayWithTabs for PackageMember<Resolved> {
    fn display_with_tabs(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        tabs_count: usize,
    ) -> std::fmt::Result {
        match self {
            PackageMember::Enum(e) => e.display_with_tabs(f, tabs_count),
            PackageMember::Message(m) => m.display_with_tabs(f, tabs_count),
        }
    }
}

impl<State> From<ProtoMessage<State>> for PackageMember<State> {
    #[inline]
    fn from(value: ProtoMessage<State>) -> Self {
        Self::Message(value)
    }
}

impl<State> From<ProtoEnum> for PackageMember<State> {
    #[inline]
    fn from(value: ProtoEnum) -> Self {
        Self::Enum(value)
    }
}
