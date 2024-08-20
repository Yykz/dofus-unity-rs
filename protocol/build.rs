use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, Write},
    path::PathBuf,
};

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

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let proto_source_path = PathBuf::from("./protos/");
    let Crawler { dirs, files } = Crawler::start(proto_source_path.clone());
    let mut prost_build = prost_build::Config::new();
    prost_build
    //    .out_dir(out_protos.clone())
        .retain_enum_prefix()
        .enable_type_names()
        .type_name_domain(["."], "type.ankama.com")
        .include_file(PathBuf::from(&out_dir).join("_include.rs"))
        .extern_path(".google.protobuf.Any", "::prost_types::Any")
        .compile_protos(&files, &dirs)
        .unwrap();
}
