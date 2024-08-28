use std::{collections::HashMap, fmt::Display, io::Write};

#[derive(Debug, Default)]
pub struct ModuleNode {
    child: HashMap<String, ModuleNode>,
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

#[derive(Debug)]
pub struct EnumBuilder {
    name: String,
    variants: HashMap<String, Vec<Vec<String>>>,
}

impl EnumBuilder {
    fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            variants: Default::default(),
        }
    }

    fn add(&mut self, name: String, namespace: Vec<String>) {
        self.variants.entry(name).or_default().push(namespace);
    }
}

fn longest_common_prefix_len(namespaces: &Vec<Vec<String>>) -> usize {
    let mut parts = namespaces[0].iter();
    let mut parts_ite: Vec<_> = namespaces[1..]
        .iter()
        .map(|strings| strings.iter())
        .collect();
    let mut index = 0;
    while let Some(part) = parts.next() {
        if parts_ite
            .iter_mut()
            .any(|iparts| iparts.next() != Some(part))
        {
            break;
        }
        index += 1;
    }
    index
}

impl Display for EnumBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut decode_map = phf_codegen::Map::new();
        let mut codegen = vec![];

        writeln!(f, "#[allow(non_camel_case_types)]\n#[derive(Debug)]\npub enum {} {{", self.name)?;
        for (name, namespaces) in &self.variants {
            let index = longest_common_prefix_len(namespaces);

            for namespace in namespaces {
                let rust_path = namespace
                    .iter()
                    .map(|part| escape_ident(part))
                    .collect::<Vec<String>>().join("::");
                let rust_name = convert_case(name);
                let rust_struct = format!("{rust_path}::{rust_name}");

                let variant_name = format!("{}{rust_name}", namespace[index..].join("_"));

                //let type_url = format!("type.ankama.com/{}.{name}", namespace.join("."));
                //let decode_path = format!("|bytes| Ok({}::{variant_name}({rust_struct}::decode(bytes)?))", self.name);
                let type_url = format!("{}.{name}", namespace.join("."));
                //decode_map.entry(type_url, &decode_path);
                writeln!(
                    f,
                    "{variant_name}({rust_struct}),"
                )?;
                codegen.push((type_url, variant_name, rust_struct));
            }
        }
        writeln!(f, "}}")?;
        
        writeln!(f, "pub fn unpack_any_match(any: &Any) -> Result<{}, AnyUnpackError> {{", self.name)?;
        writeln!(f, "let (type_url, bytes) = (&any.type_url, &any.value[..]);")?;
        writeln!(f, "match type_url.as_str() {{")?;
        for (type_url, variant_name, rust_struct) in codegen.into_iter() {
            let type_url = format!("type.ankama.com/{type_url}");
            let decode_path = format!("|bytes| Ok({}::{variant_name}({rust_struct}::decode(bytes)?))", self.name);
            writeln!(f, "\"{type_url}\" => Ok({}::{variant_name}({rust_struct}::decode(bytes)?)),", self.name)?;
            decode_map.entry(type_url, &decode_path);
        }
        writeln!(f, " _ => Err(AnyUnpackError::InvalidTypeUrl)")?;
        writeln!(f, "}}\n}}")?;

        writeln!(
            f,
            "static {}_MAP: phf::Map<&'static str, fn(&[u8]) -> Result<{}, DecodeError>> = {};",
            self.name.to_ascii_uppercase(),
            self.name,
            decode_map
                .build()
        )
    }
}

impl ModuleNode {
    fn add<'a, I: Iterator<Item = &'a str>>(
        &mut self,
        mut package: I,
        message_type: Vec<prost_types::DescriptorProto>,
    ) {
        match package.next() {
            Some(e) => {
                let child = self.child.entry(e.to_string()).or_default();
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
        let mut enum_builder = EnumBuilder::new("AnyMessage");
        self.generate_include_inner(
            &mut out,
            &mut enum_builder,
            &mut decode_map,
            &mut module_path,
        );
        writeln!(&mut out, "{}", enum_builder).unwrap();
        /*writeln!(
            &mut out,
            "static DECODE_MAP: phf::Map<&'static str, fn(&[u8]) -> Result<Box<dyn ::prost::Message>, DecodeError>> = {};",
            decode_map
                .build()
        ).unwrap();*/
    }

    pub fn generate_include_inner<'a, W: Write>(
        &'a self,
        out: &mut W,
        enum_builder: &mut EnumBuilder,
        decode_map: &mut phf_codegen::Map<String>,
        mut module_path: &mut Vec<&'a String>,
    ) {
        // DECODE_MAP entries
        if !self.messages.is_empty() {
            /*let rust_namespace = module_path.iter().fold(String::new(), |mut out, s| {
                if !out.is_empty() {
                    out.push_str("::")
                }
                out.push_str(s);
                out
            });
            let mpath = module_path.iter().fold(String::new(), |mut out, s| {
                if !out.is_empty() {
                    out.push('.')
                }
                out.push_str(s);
                out
            });*/
            let path: Vec<String> = module_path.iter().map(|&s| s.clone()).collect();
            for message_name in self.messages.iter() {
                //let message_name_idiom_case = convert_case(message_name);
                //let struct_path = format!("{rust_namespace}::{message_name_idiom_case}");
                //let type_url = format!("type.ankama.com/{mpath}.{message_name}");
                //let func= format!("|bytes| Ok(Box::new({struct_path}::decode(bytes)?))");

                enum_builder.add(message_name.clone(), path.clone());
                //decode_map.entry(type_url, &func);
            }
        }

        for (child_name, child_node) in self.child.iter() {
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

            child_node.generate_include_inner(out, enum_builder, decode_map, &mut module_path);
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
