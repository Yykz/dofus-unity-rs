use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum ProtoImport {
    External(String),
    Local(String),
}

impl Display for ProtoImport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtoImport::External(s) => writeln!(f, "import \"{s}\";"),
            ProtoImport::Local(s) => writeln!(f, "import \"{s}.proto\";"),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ExternTypeMap {
    map: HashMap<String, (String, String)>,
}

impl ExternTypeMap {
    pub fn insert<S: Into<String>, S2: Into<String>>(
        &mut self,
        fully_qualified_path: S,
        import_path: S2,
    ) {
        let (fully_qualified_path, import_path) = (fully_qualified_path.into(), import_path.into());
        let k = fully_qualified_path.split('.').last().unwrap().to_string();
        self.map.insert(k, (fully_qualified_path, import_path));
    }

    pub fn get(&self, ty: &str) -> Option<&(String, String)> {
        self.map.get(ty)
    }
}
