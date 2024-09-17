use std::{collections::HashMap, io::Write};

#[derive(Debug, Default)]
pub struct ModuleNode {
    childs: HashMap<String, ModuleNode>,
    messages: Vec<String>,
}

fn escape_ident(s: &str) -> String {
    match s {
        "move" => "r#move",
        s => s,
    }
    .to_string()
}

fn convert_case(s: &str) -> String {
    let mut r = String::new();
    let mut chars = s.chars().peekable();

    let mut last_was_caps = false;
    while let Some(c) = chars.next() {
        let is_caps = c.is_uppercase();
        let next_is_caps = chars.peek().is_some_and(|c| c.is_uppercase());

        if is_caps && last_was_caps && next_is_caps {
            r.push(c.to_ascii_lowercase())
        } else {
            r.push(c)
        }

        last_was_caps = is_caps;
    }

    r
}

impl ModuleNode {
    fn add<'a, I: Iterator<Item = &'a str>>(
        &mut self,
        mut package: I,
        message_type: Vec<prost_types::DescriptorProto>,
    ) {
        match package.next() {
            Some(e) => {
                let child = self.childs.entry(e.to_string()).or_default();
                child.add(package, message_type)
            }
            None => self
                .messages
                .extend(message_type.into_iter().filter_map(|t| t.name)),
        }
    }

    pub fn generate_include<W: Write>(self, mut out: W) {
        let mut module_path = vec![];
        let mut decode_map = phf_codegen::Map::new();
        writeln!(out, "#[doc(hidden)]").unwrap();
        self.generate_include_inner(&mut out, &mut decode_map, &mut module_path);
        writeln!(
            &mut out,
            "static DECODE_MAP: phf::Map<&'static str, fn(&[u8]) -> Result<Box<dyn ::prost::Message>, DecodeError>> = {};",
            decode_map
                .build()
        ).unwrap();
    }

    pub fn generate_include_inner<'a, W: Write>(
        &'a self,
        out: &mut W,
        decode_map: &mut phf_codegen::Map<String>,
        module_path: &mut Vec<&'a String>,
    ) {
        // DECODE_MAP entries
        if !self.messages.is_empty() {
            let rust_namespace = module_path.iter().fold(String::new(), |mut out, s| {
                if !out.is_empty() {
                    out.push_str("::")
                }
                out.push_str(&escape_ident(s));
                out
            });
            let mpath = module_path.iter().fold(String::new(), |mut out, s| {
                if !out.is_empty() {
                    out.push('.')
                }
                out.push_str(s);
                out
            });
            for message_name in self.messages.iter() {
                let message_name_idiom_case = convert_case(message_name);
                let struct_path = format!("{rust_namespace}::{message_name_idiom_case}");
                let type_url = format!("type.ankama.com/{mpath}.{message_name}");
                let func = format!("|bytes| Ok(Box::new({struct_path}::decode(bytes)?))");

                decode_map.entry(type_url, &func);
            }
        }

        for (child_name, child_node) in self.childs.iter() {
            if child_name == "google" {
                continue;
            }
            writeln!(out, "pub mod {} {{", escape_ident(child_name)).unwrap();
            module_path.push(child_name);
            if !child_node.messages.is_empty() {
                let mpath = module_path
                    .iter()
                    .map(|s| escape_ident(s))
                    .collect::<Vec<String>>()
                    .join(".");

                writeln!(
                    out,
                    "include!(concat!(env!(\"OUT_DIR\"), \"/protos/{mpath}.rs\"));"
                )
                .unwrap();
            }

            child_node.generate_include_inner(out, decode_map, module_path);
            module_path.pop();
            writeln!(out, "}}").unwrap();
        }
    }
}

impl From<prost_types::FileDescriptorSet> for ModuleNode {
    fn from(value: prost_types::FileDescriptorSet) -> Self {
        let mut root = Self::default();
        for file in value.file {
            let (package, message_type) = (file.package.unwrap(), file.message_type);
            root.add(package.split('.'), message_type);
        }
        root
    }
}
