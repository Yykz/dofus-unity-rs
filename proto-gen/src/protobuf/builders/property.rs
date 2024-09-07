use convert_case::Case;

use crate::{
    parser::{
        common::visibility::Visibility,
        data_structs::{Property, PropertyKind, PropertyModifier},
    },
    protobuf::builders::field::StrFormat,
};

use super::field::StringSpec;

#[derive(Debug, Clone, PartialEq)]
pub enum GenProperty {
    Blanket,
    Field(String, String),
    HasField(String),
    OneOfCase(String),
    RepeatedField(String),
    MapField(String),
}

impl TryFrom<&Property> for GenProperty {
    type Error = ();

    fn try_from(value: &Property) -> Result<Self, Self::Error> {
        use PropertyKind::*;
        use StringSpec::*;
        use Visibility::*;

        if PropertySpec::default()
            .name(Eq("Parser"))
            .ty(StartWith("MessageParser"))
            .matches(value)
        {
            return Ok(GenProperty::Blanket);
        }

        if PropertySpec::default()
            .name(Eq("Descriptor"))
            .ty(Eq("MessageDescriptor"))
            .matches(value)
        {
            return Ok(GenProperty::Blanket);
        }

        if PropertySpec::default()
            .name(Eq("pb::Google.Protobuf.IMessage.Descriptor"))
            .ty(Eq("MessageDescriptor"))
            .matches(value)
        {
            return Ok(GenProperty::Blanket);
        }

        if PropertySpec::<&str>::default()
            .visibility(Public)
            .kind(Get)
            .name(StartWith("Has"))
            .ty(Eq("bool"))
            .name(IsCase(Case::Pascal))
            .matches(value)
        {
            let fmt_name = StrFormat::<&str>::default()
                .try_strip_prefix("Has")
                .as_snake(Case::Pascal)
                .apply(&value.name);
            return Ok(GenProperty::HasField(fmt_name));
        }

        if PropertySpec::<&str>::default()
            .visibility(Public)
            .kind(Get)
            .name(EndWith("Case"))
            .ty(EndWith("OneofCase"))
            .matches(value)
        {
            let fmt_name = StrFormat::<&str>::default()
                .try_strip_suffix("Case")
                .as_snake(Case::Pascal)
                .apply(&value.name);
            return Ok(GenProperty::OneOfCase(fmt_name));
        }

        if PropertySpec::<&str>::default()
            .visibility(Public)
            .kind(Get)
            .ty(StartWith("RepeatedField<"))
            .ty(EndWith(">"))
            .matches(value)
        {
            let fmt_name = StrFormat::default()
                .try_strip_suffix("_")
                .as_snake(Case::Pascal)
                .apply(&value.name);
            return Ok(GenProperty::RepeatedField(fmt_name));
        }

        if PropertySpec::<&str>::default()
            .visibility(Public)
            .kind(Get)
            .ty(StartWith("MapField<"))
            .ty(EndWith(">"))
            .matches(value)
        {
            let fmt_name = StrFormat::default()
                .try_strip_suffix("_")
                .as_snake(Case::Pascal)
                .apply(&value.name);
            return Ok(GenProperty::MapField(fmt_name));
        }

        if PropertySpec::<&str>::default()
            .visibility(Public)
            .kinds([Get, Set])
            .matches(value)
        {
            let fmt_name = StrFormat::<&str>::default()
                .try_strip_suffix("_")
                .as_snake(Case::Pascal)
                .apply(&value.name);
            return Ok(GenProperty::Field(fmt_name, value.ty.clone()));
        }

        Err(())
    }
}

#[derive(Default)]
pub struct PropertySpec<S: AsRef<str>> {
    visibility: Option<Visibility>,
    name: Vec<StringSpec<S>>,
    ty: Vec<StringSpec<S>>,
    modifiers: Option<Vec<PropertyModifier>>,
    kinds: Option<Vec<PropertyKind>>,
}

#[allow(dead_code)]
impl<S: AsRef<str>> PropertySpec<S> {
    pub fn matches(&self, p: &Property) -> bool {
        self.visibility.is_none_or(|v| v.eq(&p.visibility))
            && self.name.iter().all(|spec| spec.matches(&p.name))
            && self.ty.iter().all(|spec| spec.matches(&p.ty))
            && self
                .modifiers
                .as_ref()
                .is_none_or(|modifiers| modifiers == &p.modifiers)
            && self.kinds.as_ref().is_none_or(|kinds| kinds == &p.kinds)
    }

    pub fn ty(mut self, spec: StringSpec<S>) -> Self {
        self.ty.push(spec);
        self
    }

    pub fn name(mut self, spec: StringSpec<S>) -> Self {
        self.name.push(spec);
        self
    }

    pub fn modifier(mut self, modifier: PropertyModifier) -> Self {
        self.modifiers
            .get_or_insert_with(Default::default)
            .push(modifier);
        self
    }

    pub fn empty_modifiers(mut self) -> Self {
        self.modifiers = Some(Default::default());
        self
    }

    pub fn modifiers<I: IntoIterator<Item = PropertyModifier>>(mut self, modifiers: I) -> Self {
        self.modifiers = Some(modifiers.into_iter().collect());
        self
    }

    pub fn kind(mut self, kind: PropertyKind) -> Self {
        self.kinds.get_or_insert_with(Default::default).push(kind);
        self
    }

    pub fn empty_kinds(mut self) -> Self {
        self.kinds = Some(Default::default());
        self
    }

    pub fn kinds<I: IntoIterator<Item = PropertyKind>>(mut self, kinds: I) -> Self {
        self.kinds = Some(kinds.into_iter().collect());
        self
    }

    pub fn visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = Some(visibility);
        self
    }
}
