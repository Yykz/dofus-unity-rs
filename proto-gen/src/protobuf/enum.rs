use std::fmt::Display;

use crate::parser_items;

#[derive(Debug)]
pub struct Enum {
    name: String,
    variants: Vec<String>,
    namespace: String,
}

impl Enum {
    pub fn new(value: parser_items::Enum, namespace: String) -> Self {
        Self {
            name: value.name,
            variants: value
                .variants
                .into_iter()
                .map(|v| v.name.to_ascii_uppercase())
                .collect(),
            namespace,
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
