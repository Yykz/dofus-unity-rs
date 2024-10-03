mod field;

use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
    ops::Index,
    slice::SliceIndex,
};

pub use field::*;

use super::{
    package::{ExternTypeMap, PackageMember, ProtoImport},
    DisplayWithTabs,
};

#[derive(Debug, Clone, Default)]
pub struct Unresolved;

#[derive(Debug, Clone, Default)]
pub struct Resolved;

pub type MessageUnresolve = ProtoMessage<Unresolved>;
pub type MessageResolved = ProtoMessage<Resolved>;

#[derive(Debug, Clone)]
pub struct ProtoMessage<State> {
    pub name: String,
    fields: Vec<Field>,
    inner_members: Vec<PackageMember<State>>,
    _marker: PhantomData<State>,
}

#[derive(Debug, Clone)]
pub struct TypePath<'a> {
    inner: Vec<&'a str>,
}

impl<'a> TypePath<'a> {
    pub fn index_range<R>(self, range: R) -> Self
    where
        R: SliceIndex<[&'a str], Output = [&'a str]>,
    {
        Self {
            inner: (self.inner[range]).to_vec(),
        }
    }

    pub fn joined(&self) -> String {
        self.as_slice().join(".")
    }

    pub fn as_slice(&self) -> &[&str] {
        self.inner.as_slice()
    }

    pub fn is_nested(&self) -> bool {
        self.inner.len() >= 2
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn is_onecase(&self) -> bool {
        // TODO empty ?
        self.inner.last().unwrap().ends_with("OneofCase")
    }

    pub fn to_owned(&self) -> Vec<String> {
        self.inner.iter().map(|&s| s.to_owned()).collect()
    }

    pub fn try_from_str(s: &'a str) -> Result<Self, ()> {
        let split = s.split('.');

        let mut inner = vec![];

        let mut wrong = false;
        for (i, part) in split.enumerate() {
            if wrong || part.is_empty() {
                return Err(());
            }
            if i % 2 == 1 {
                if part == "Types" {
                    continue;
                }
                wrong = true;
            }
            inner.push(part)
        }

        let path = Self { inner };
        if wrong && !path.is_onecase() {
            return Err(());
        }

        Ok(path)
    }
}

impl<'a> From<&'a Vec<String>> for TypePath<'a> {
    fn from(value: &'a Vec<String>) -> Self {
        Self {
            inner: value.iter().map(String::as_str).collect(),
        }
    }
}

impl<'a> Index<usize> for TypePath<'a> {
    type Output = str;

    fn index(&self, index: usize) -> &Self::Output {
        self.as_slice().index(index)
    }
}

impl MessageUnresolve {
    pub fn contains(&self, path: &[&str]) -> bool {
        if path.len() == 1 {
            self.name == path[0]
        } else if self.name == path[0] {
            self.inner_members.iter().any(|m| m.contains(&path[1..]))
        } else {
            false
        }
    }

    #[inline]
    pub fn new(
        name: String,
        fields: Vec<Field>,
        inner_members: Vec<PackageMember<Unresolved>>,
    ) -> Self {
        Self {
            name,
            fields,
            inner_members,
            _marker: PhantomData,
        }
    }

    pub fn resolve(
        self,
        current_namespace: String,
        top_map: &HashMap<String, Vec<PackageMember<Unresolved>>>,
        imports: &mut HashSet<ProtoImport>,
        extern_ty: &ExternTypeMap,
    ) -> MessageResolved {
        let ProtoMessage {
            name,
            fields,
            inner_members,
            _marker,
        } = self;
        let fields = fields
            .into_iter()
            .map(|mut f| {
                f.custom_types().into_iter().for_each(|ty| {
                    let ty_path = TypePath::try_from_str(ty).unwrap();

                    let mut possibles_path: Vec<PathKind> = vec![];
                    for (namespace, members) in top_map.iter() {
                        for member in members {
                            if member.contains(ty_path.as_slice()) {
                                possibles_path.push(PathKind::Local(namespace.clone()));
                            }
                        }
                    }

                    if let Some((path, import)) = extern_ty.get(&ty_path[0]).cloned() {
                        possibles_path.push(PathKind::Extern(path, import));
                    }

                    let path = match &possibles_path.as_slice() {
                        &[] => panic!("failed to resolve {ty}"),
                        &[p] => p,
                        paths => {
                            let namespace = find_closest(paths, &current_namespace);
                            let ty_name = ty_path.joined();
                            println!("Warning: Ambiguous type reference for '{ty_name}' in namespace \"{current_namespace}\".");
                            println!("  The type '{ty_name}' could refer to multiple namespaces:");
                            for p in paths.iter() {
                                println!("    - {p:?}");
                            }
                            println!("  Assuming type from namespace: {namespace:?}");
                            namespace
                        }
                    };

                    match path {
                        PathKind::Extern(path, import) => {
                            imports.insert(ProtoImport::External(import.to_string()));
                            *ty = path.to_string();
                        }
                        PathKind::Local(namespace) => {
                            if *namespace != current_namespace {
                                imports.insert(ProtoImport::Local(namespace.to_string()));
                            }
                            *ty = format!(".{namespace}.{}", ty_path.joined());
                        }
                    }
                });
                f
            })
            .collect();

        let inner_members = inner_members
            .into_iter()
            .map(|m| m.resolve(current_namespace.clone(), top_map, imports, extern_ty))
            .collect();
        ProtoMessage {
            name,
            fields,
            inner_members,
            _marker: PhantomData,
        }
    }
}

fn calc_namespace_similarity(n: &str, n2: &str) -> usize {
    n.split('.')
        .zip(n2.split('.'))
        .take_while(|(p, p2)| p == p2)
        .count()
}

impl DisplayWithTabs for MessageResolved {
    fn display_with_tabs(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        tabs_count: usize,
    ) -> std::fmt::Result {
        let tabs = "\t".repeat(tabs_count);
        writeln!(f, "{tabs}message {} {{", self.name)?;
        for field in self.fields.iter() {
            field.display_with_tabs(f, tabs_count + 1)?;
            writeln!(f)?;
        }
        for inner_member in self.inner_members.iter() {
            writeln!(f)?;
            inner_member.display_with_tabs(f, tabs_count + 1)?;
        }
        writeln!(f, "{tabs}}}")
    }
}

#[derive(Debug, Clone)]
pub enum PathKind {
    Extern(String, String),
    Local(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PathSim {
    Sim(usize),
    Ext,
    Eq,
}

impl PathSim {
    fn value(&self) -> u8 {
        match self {
            PathSim::Sim(_) => 0,
            PathSim::Ext => 1,
            PathSim::Eq => 2,
        }
    }

    fn sim(path: &PathKind, current_namespace: &str) -> Self {
        match path {
            PathKind::Extern(_, _) => PathSim::Ext,
            PathKind::Local(n) => {
                if n == current_namespace {
                    return PathSim::Eq;
                }
                PathSim::Sim(calc_namespace_similarity(n, current_namespace))
            }
        }
    }
}

impl PartialOrd for PathSim {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PathSim {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (PathSim::Sim(this), PathSim::Sim(other)) => this.cmp(other),
            _ => self.value().cmp(&other.value()),
        }
    }
}

// TODO refractor namespace prioritization
pub fn find_closest<'a>(paths: &'a [PathKind], current_namespace: &str) -> &'a PathKind {
    let first = paths.first().unwrap();
    let mut best_sim = PathSim::sim(first, current_namespace);
    let mut best: Vec<&'a PathKind> = vec![first];

    for path in &paths[1..] {
        let sim = PathSim::sim(path, current_namespace);
        match sim.cmp(&best_sim) {
            std::cmp::Ordering::Less => {}
            std::cmp::Ordering::Equal => best.push(path),
            std::cmp::Ordering::Greater => {
                best_sim = sim;
                best = vec![path];
            }
        }
    }
    best.sort_by_key(|p| match p {
        PathKind::Extern(n, _) => n,
        PathKind::Local(n) => n,
    });
    best[0]
}
