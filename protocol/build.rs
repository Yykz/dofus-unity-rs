use std::{collections::HashMap, env, fs::File, io::{self, Write}, path::PathBuf};

#[derive(Debug, Default)]
pub struct Crawler {
    dirs: Vec<PathBuf>,
    files: Vec<PathBuf>,
}

impl Crawler {
    pub fn start(p: PathBuf) -> Self {
        let mut crawler = Self::default();
        crawler.in_dir(p);
        crawler
    }

    fn in_dir(&mut self, p: PathBuf) {
        for entry in std::fs::read_dir(&p).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            match path.is_file() {
                true => self.files.push(path),
                false => self.in_dir(path),
            }
        }
        self.dirs.push(p);
    }
}

#[derive(Debug, Default)]
struct ModuleNode {
    include: bool,
    children: HashMap<String, ModuleNode>,
}

fn fix_name(s: &str) -> &str {
    match s {
        "move" => "r#move",
        _ => { s }
    }
}

impl ModuleNode {
    fn insert<I: Iterator<Item = String>>(&mut self, mut path: I) {
        if let Some(first) = path.next() {
            let child = self.children.entry(first.to_string()).or_default();
            child.insert(path)
        } else {
            self.include = true;
        }
    }

    fn generate_code(&self, out: &mut File, p: Option<String>) -> io::Result<()> {
        for (child_name, child) in self.children.iter() {
            let mut child_name = child_name.as_str();
            child_name = fix_name(&child_name);
            writeln!(out, "pub mod {} {{", child_name)?;
            let child_path = match p {
                Some(ref s) => format!("{}.{}", s, child_name),
                None => child_name.to_string(),
            };
            if child.include {
                writeln!(out, "include!(concat!(env!(\"OUT_DIR\"), \"/protos/{}.rs\"));", child_path)?;
            }
            child.generate_code(out, Some(child_path))?;
            writeln!(out, "}}")?;
        }

        Ok(())
    }
}

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_protos = PathBuf::from(&out_dir).join("protos/");
    std::fs::create_dir_all(&out_protos).unwrap();

    let proto_source_path = PathBuf::from("./protos/");
    let Crawler { dirs, files } = Crawler::start(proto_source_path.clone());
    let mut prost_build = prost_build::Config::new();
    prost_build
        .out_dir(out_protos)
        .extern_path(".google.protobuf.Any", "::prost_types::Any")
        .compile_protos(&files, &dirs)
        .unwrap();

    let path_modules = PathBuf::from(&out_dir).join("modules.rs");
    let mut nodes = ModuleNode::default();
    for file in files.iter() {
        let s: Vec<String> = file
            .strip_prefix(&proto_source_path)
            .unwrap()
            .with_extension("")
            .to_str()
            .unwrap()
            .replace("/", ".")
            .split('.')
            .map(|s| s.to_string()).collect();
        nodes.insert(s.into_iter());
    }
    let mut modules = std::fs::File::create(path_modules).unwrap();
    eprintln!("{nodes:#?}");
    nodes.generate_code(&mut modules, None).unwrap();
}
