use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    fs::create_dir_all,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use crate::parser::{self, data_structs::DataStructInner};

mod import;
mod member;
pub use import::{ExternTypeMap, ProtoImport};
pub use member::PackageMember;
use regex::Regex;

use super::{
    builders::package::{NestedMembers, TopLevelMembers},
    message::{Resolved, TypePath, Unresolved},
};

#[derive(Debug, Clone)]
pub struct ProtoPackage {
    name: String,
    imports: Vec<ProtoImport>,
    members: Vec<PackageMember<Resolved>>,
}

impl ProtoPackage {
    pub fn new(
        name: String,
        mut imports: Vec<ProtoImport>,
        mut members: Vec<PackageMember<Resolved>>,
    ) -> Self {
        imports.sort_unstable_by_key(|i| i.to_string());
        members.sort_unstable_by_key(|m| m.name().to_string());
        Self {
            name,
            imports,
            members,
        }
    }
}

impl Display for ProtoPackage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "syntax = \"proto3\";\n")?;

        for import in &self.imports {
            writeln!(f, "{import}")?;
        }

        let _ = writeln!(f, "package {};\n", self.name);

        for member in &self.members {
            writeln!(f, "{member}")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ProtoPackages(Vec<ProtoPackage>);

impl ProtoPackages {
    pub fn write_to_dir<P: AsRef<Path>>(&self, p: P) {
        let dir = PathBuf::from(p.as_ref());
        create_dir_all(&dir).unwrap();
        assert!(dir.is_dir());

        for package in &self.0 {
            let mut path = dir.clone();
            path.push(format!("{}.proto", package.name));
            let file = std::fs::File::create(path).unwrap();
            let mut bufwriter = BufWriter::new(file);

            let _ = write!(bufwriter, "{package}");
        }
    }

    pub fn from_parsed(parsed: parser::file::ParsedFile, filter_namespace: Regex) -> Self {
        let mut extern_ty_map = ExternTypeMap::default();
        extern_ty_map.insert(".google.protobuf.Any", "google/protobuf/any.proto");

        let mut top_level_map = TopLevelMembers::default();
        let mut nested_members = NestedMembers::default();
        let mut oneof = vec![];

        for member in parsed.members.into_iter() {
            match member.kind {
                DataStructInner::Class(class) => {
                    if !class.interfaces.iter().any(|i| i == "IMessage") {
                        continue;
                    }
                    if let Some(namespace) = member.namespace {
                        if !filter_namespace.is_match(&namespace) {
                            continue;
                        }
                        top_level_map.insert_top(class, namespace)
                    } else if let Ok(namespace) = TypePath::try_from_str(&class.name) {
                        if namespace.is_nested() {
                            nested_members.insert(namespace.to_owned(), class);
                        }
                    }
                }
                DataStructInner::Enum(enumm) => {
                    if let Some(namespace) = member.namespace {
                        if !filter_namespace.is_match(&namespace) {
                            continue;
                        }
                        top_level_map.insert_top(enumm, namespace)
                    } else if let Ok(path) = TypePath::try_from_str(&enumm.name) {
                        if path.is_onecase() {
                            oneof.push((path.to_owned(), enumm))
                        } else if path.is_nested() {
                            nested_members.insert(path.to_owned(), enumm);
                        }
                    }
                }
                DataStructInner::Struct | DataStructInner::Interface => {}
            }
        }

        for (path, member) in nested_members.into_iter() {
            top_level_map.insert_nested(TypePath::from(&path), member)
        }

        for (path, member) in oneof.into_iter() {
            top_level_map.insert_nested_oneof(TypePath::from(&path), member)
        }

        let mut map: HashMap<String, Vec<PackageMember<Unresolved>>> = HashMap::new();
        for member in top_level_map.into_iter() {
            let (member, namespace) = member.destruct();
            let package = namespace.to_lowercase();

            map.entry(package).or_default().push(member.build());
        }

        let map_clone = map.clone();

        let mut packages = vec![];
        for (namespace, members) in map.into_iter() {
            let mut imports = HashSet::default();
            let members = members
                .into_iter()
                .map(|m| m.resolve(namespace.clone(), &map_clone, &mut imports, &extern_ty_map))
                .collect();

            let imports = imports.into_iter().collect();
            packages.push(ProtoPackage::new(namespace, imports, members))
        }

        Self(packages)
    }
}
