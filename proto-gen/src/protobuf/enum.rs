use std::fmt::Display;

use crate::parser_items;

#[derive(Debug)]
pub struct Enum {
    name: String,
    variants: Vec<String>,
    _namespace: String,
}

fn original_name(attribute: parser_items::Attribute) -> Option<String> {
    match attribute.0.starts_with("[OriginalName") {
        true => Some((&attribute.0[15..attribute.0.len() - 3]).to_string()),
        false => None,
    }
}

fn variant_name(variant: parser_items::Variant) -> String {
    match variant
        .attributes
        .into_iter()
        .map(original_name)
        .find(|a| a.is_some())
        .flatten()
    {
        Some(original_name) => original_name,
        None => variant.name.to_ascii_uppercase(),
    }
}

impl Enum {
    pub fn new(value: parser_items::Enum, namespace: String) -> Self {
        Self {
            name: value.name,
            variants: value.variants.into_iter().map(variant_name).collect(),
            _namespace: namespace,
        }
    }

    pub fn display_inner(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        nest_count: usize,
    ) -> std::fmt::Result {
        let nest: String = "\t".repeat(nest_count);
        writeln!(f, "{nest}enum {} {{", self.name)?;
        for (i, variant) in self.variants.iter().enumerate() {
            writeln!(f, "{nest}\t{} = {};", variant, i)?;
        }
        writeln!(f, "{nest}}}")
    }
}

impl Display for Enum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display_inner(f, 0)
    }
}
