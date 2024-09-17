use crate::IsNoneOr;
use convert_case::{Case, Casing};

use std::fmt::Debug;

use crate::parser::{
    self,
    common::{literal::Literal, visibility::Visibility},
    data_structs::FieldModifier,
};

#[derive(Debug, Clone)]
pub enum GenField {
    // name, number
    Number(String, u32),
    // name, type
    Type(String, String),
    // name, type
    RepeatedType(String, String),
    // name, type
    MapType(String, String),
    // name, type
    DefaultValue(String, String),
    // name, type
    RepeatedCodec(String, String),
    // name, type
    MapCodec(String, String),
    // name
    OneofCaseObject(String),
    OneofCaseType(String),
    Blanket,
}

#[derive(Debug, Clone, Default)]
pub struct FieldSpec<S: AsRef<str> + Debug> {
    visibility: Option<Visibility>,
    name: Vec<StringSpec<S>>,
    ty: Vec<StringSpec<S>>,
    modifiers: Option<Vec<FieldModifier>>,
    assign_value: Option<ValueSpec>,
}

impl<S: AsRef<str> + Debug> FieldSpec<S> {
    pub fn matches(&self, f: &parser::data_structs::Field) -> bool {
        // TODO use is_none_or from std when stabilized
        self.visibility.is_none_or_(|v| v.eq(&f.visibility))
            && self.name.iter().all(|spec| spec.matches(&f.name))
            && self.ty.iter().all(|spec| spec.matches(&f.ty))
            && self
                .assign_value
                .as_ref()
                .is_none_or_(|spec| spec.matches(&f.assign_value))
            && self
                .modifiers
                .as_ref()
                .is_none_or_(|modifiers| modifiers == &f.modifiers)
    }

    pub fn ty(mut self, spec: StringSpec<S>) -> Self {
        self.ty.push(spec);
        self
    }

    pub fn name(mut self, spec: StringSpec<S>) -> Self {
        self.name.push(spec);
        self
    }

    pub fn modifier(mut self, modifier: FieldModifier) -> Self {
        self.modifiers
            .get_or_insert_with(Default::default)
            .push(modifier);
        self
    }

    pub fn empty_modifiers(mut self) -> Self {
        self.modifiers = Some(Default::default());
        self
    }

    pub fn modifiers<I: IntoIterator<Item = FieldModifier>>(mut self, modifiers: I) -> Self {
        self.modifiers = Some(modifiers.into_iter().collect());
        self
    }

    pub fn assign_value(mut self, assign_value: ValueSpec) -> Self {
        self.assign_value = Some(assign_value);
        self
    }

    pub fn visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = Some(visibility);
        self
    }
}

#[derive(Debug, Clone)]
pub enum StringSpec<S: AsRef<str>> {
    Eq(S),
    StartWith(S),
    EndWith(S),
    IsCase(Case),
}

impl<S: AsRef<str>> StringSpec<S> {
    pub fn matches(&self, s: &str) -> bool {
        match self {
            StringSpec::Eq(p) => s.eq(p.as_ref()),
            StringSpec::StartWith(p) => s.starts_with(p.as_ref()),
            StringSpec::EndWith(p) => s.ends_with(p.as_ref()),
            StringSpec::IsCase(case) => s.is_case(*case),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ValueSpec {
    None,
    IsNumber,
}

impl ValueSpec {
    pub fn matches(&self, v: &Option<Literal>) -> bool {
        match self {
            ValueSpec::None => v.is_none(),
            ValueSpec::IsNumber => matches!(v, Some(Literal::Number(_))),
        }
    }
}

impl TryFrom<parser::data_structs::Field> for GenField {
    type Error = ();

    fn try_from(field: parser::data_structs::Field) -> Result<Self, Self::Error> {
        use FieldModifier::*;
        use StringSpec::*;
        use ValueSpec::*;
        use Visibility::*;

        let name = &field.name;
        let ty = &field.ty;

        if FieldSpec::default()
            .ty(Eq("int"))
            .visibility(Public)
            .modifier(Const)
            .assign_value(IsNumber)
            .name(EndWith("FieldNumber"))
            .matches(&field)
        {
            let name = StrFormat::default()
                .try_strip_suffix("FieldNumber")
                .try_strip_suffix("_")
                .as_snake(Case::Pascal)
                .apply(name);

            return Ok(GenField::Number(
                name,
                field.assign_value.unwrap().u32().unwrap(),
            ));
        }

        if FieldSpec::default()
            .ty(StartWith("MessageParser"))
            .name(Eq("_parser"))
            .matches(&field)
        {
            return Ok(GenField::Blanket);
        }

        if FieldSpec::default()
            .ty(StartWith("UnknownFieldSet"))
            .name(Eq("_unknownFields"))
            .matches(&field)
        {
            return Ok(GenField::Blanket);
        }

        if FieldSpec::default()
            .ty(Eq("int"))
            .name(Eq("_hasBits0"))
            .matches(&field)
        {
            return Ok(GenField::Blanket);
        }

        if FieldSpec::default()
            .name(EndWith("DefaultValue"))
            .name(IsCase(Case::Pascal))
            .visibility(Private)
            .assign_value(None)
            .modifiers([FieldModifier::Static, FieldModifier::ReadOnly])
            .matches(&field)
        {
            let name = StrFormat::default()
                .try_strip_suffix("DefaultValue")
                .as_snake(Case::Pascal)
                .apply(name);
            return Ok(GenField::DefaultValue(name, field.ty));
        }

        if FieldSpec::default()
            .name(EndWith("_"))
            .visibility(Private)
            .assign_value(None)
            .matches(&field)
        {
            if FieldSpec::<&str>::default()
                .empty_modifiers()
                .ty(EndWith("OneofCase"))
                .matches(&field)
            {
                let fmt_name = StrFormat::default()
                    .try_strip_suffix("Case_")
                    .as_snake(Case::Camel)
                    .apply(name);
                return Ok(GenField::OneofCaseType(fmt_name));
            }

            let fmt_name = StrFormat::default()
                .try_strip_suffix("_")
                .as_snake(Case::Camel)
                .apply(name);
            if FieldSpec::<&str>::default()
                .empty_modifiers()
                .ty(Eq("object"))
                .matches(&field)
            {
                return Ok(GenField::OneofCaseObject(fmt_name));
            }
            if FieldSpec::<&str>::default()
                .empty_modifiers()
                .matches(&field)
            {
                return Ok(GenField::Type(fmt_name, field.ty));
            }
            if FieldSpec::default()
                .modifier(ReadOnly)
                .ty(StartWith("RepeatedField"))
                .matches(&field)
            {
                let ty_fmt = StrFormat::default()
                    .try_strip_prefix("RepeatedField<")
                    .try_strip_suffix(">")
                    .apply(ty);
                return Ok(GenField::RepeatedType(fmt_name, ty_fmt));
            }
            if FieldSpec::default()
                .modifier(ReadOnly)
                .ty(StartWith("MapField"))
                .matches(&field)
            {
                let ty_fmt = StrFormat::default()
                    .try_strip_prefix("MapField<")
                    .try_strip_suffix(">")
                    .apply(ty);
                return Ok(GenField::MapType(fmt_name, ty_fmt));
            }
        }

        if FieldSpec::default()
            .ty(StartWith("FieldCodec"))
            .name(StartWith("_repeated_"))
            .name(EndWith("_codec"))
            .visibility(Private)
            .modifiers([Static, ReadOnly])
            .assign_value(None)
            .matches(&field)
        {
            let name = StrFormat::default()
                .try_strip_prefix("_repeated_")
                .try_strip_suffix("_codec")
                .as_snake(Case::Camel)
                .apply(name);
            return Ok(GenField::RepeatedCodec(name, field.ty));
        }

        if FieldSpec::default()
            .ty(StartWith("MapField.Codec"))
            .name(StartWith("_map_"))
            .name(EndWith("_codec"))
            .visibility(Private)
            .modifiers([Static, ReadOnly])
            .assign_value(None)
            .matches(&field)
        {
            let name = StrFormat::default()
                .try_strip_prefix("_map_")
                .try_strip_suffix("_codec")
                .as_snake(Case::Camel)
                .apply(name);
            return Ok(GenField::MapCodec(name, field.ty));
        }

        Err(())
    }
}

#[derive(Default)]
pub struct StrFormat<S: AsRef<str>>(Vec<StrOperation<S>>);

impl<S: AsRef<str>> StrFormat<S> {
    pub fn apply(&self, s: &str) -> String {
        let mut s = s.to_string();
        for op in self.0.iter() {
            s = op.apply(&s);
        }
        s
    }

    pub fn as_case(&mut self, from: Case, to: Case) -> &mut Self {
        self.0.push(StrOperation::ToCase(from, to));
        self
    }

    pub fn as_snake(&mut self, from: Case) -> &mut Self {
        self.as_case(from, Case::Snake)
    }

    pub fn try_strip_prefix(&mut self, s: S) -> &mut Self {
        self.0.push(StrOperation::TryStripPrefix(s));
        self
    }

    pub fn try_strip_suffix(&mut self, s: S) -> &mut Self {
        self.0.push(StrOperation::TryStripSuffix(s));
        self
    }
}

pub enum StrOperation<S: AsRef<str>> {
    TryStripPrefix(S),
    TryStripSuffix(S),
    ToCase(Case, Case),
}

impl<S: AsRef<str>> StrOperation<S> {
    pub fn apply(&self, s: &str) -> String {
        match self {
            StrOperation::TryStripPrefix(pat) => {
                s.strip_prefix(pat.as_ref()).unwrap_or(s).to_string()
            }
            StrOperation::TryStripSuffix(pat) => {
                s.strip_suffix(pat.as_ref()).unwrap_or(s).to_string()
            }
            StrOperation::ToCase(from, to) => s.from_case(*from).to_case(*to),
        }
    }
}
