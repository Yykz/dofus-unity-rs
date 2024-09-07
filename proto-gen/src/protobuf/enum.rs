use crate::parser::{
    self,
    common::attribute::Attribute,
    data_structs::{EnumMember, Field},
};

use super::DisplayWithTabs;

#[derive(Debug, Clone)]
pub struct ProtoEnum {
    pub name: String,
    variants: Vec<(String, u32)>,
}

impl DisplayWithTabs for ProtoEnum {
    fn display_with_tabs(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        tabs_count: usize,
    ) -> std::fmt::Result {
        let tabs = "\t".repeat(tabs_count);
        writeln!(f, "{tabs}enum {} {{", self.name)?;
        for (name, number) in self.variants.iter() {
            writeln!(f, "{tabs}\t{name} = {number};")?;
        }
        writeln!(f, "{tabs}}}")
    }
}

impl From<parser::data_structs::Enum> for ProtoEnum {
    fn from(value: parser::data_structs::Enum) -> Self {
        let mut variants = vec![];
        for member in value.members {
            let EnumMember { field, attributes } = member;
            let Field {
                ty: _,
                name,
                visibility: _,
                modifiers: _,
                assign_value,
            } = field;

            if name == "value__" {
                continue;
            }

            //[OriginalName(\"UNKNOWN\")]
            let original_name = original_name(&attributes);
            let number = assign_value.unwrap().u32().unwrap();

            let name = original_name.map(|s| s.to_string()).unwrap_or(name);
            variants.push((name, number))
        }

        Self {
            name: value.name,
            variants,
        }
    }
}

//TODO move this to Attribute method
fn original_name(attrs: &[Attribute]) -> Option<&str> {
    for attr in attrs.iter() {
        if attr.0.starts_with("[OriginalName") {
            return Some(&attr.0[15..attr.0.len() - 3]);
        }
    }
    None
}
