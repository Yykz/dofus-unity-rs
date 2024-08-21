use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, Read, Write},
    path::PathBuf,
};

use prost::Message;

#[derive(Debug, Default)]
pub struct Crawler {
    dirs: Vec<PathBuf>,
    files: Vec<PathBuf>,
}

impl Crawler {
    pub fn start<P: Into<PathBuf>>(p: P) -> Self {
        let mut crawler = Self::default();
        crawler.in_dir(p.into());
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

fn escape_indent(s: &str) -> String {
    match s {
        "move" => "r#move",
        s => s
    }.to_string()
}

fn convert_case(s: String) -> String {
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

fn leading_domain_for_namespace(s: &str) -> &'static str {
    match s {
        s if s.starts_with("com.ankama") => {
            "type.ankama.com"
        }
        s if s.starts_with("google.protobuf") => {
            "type.googleapis.com"
        }
        _ => unreachable!()
    }
}

fn rust_path_for(namespace: &str, message_name: String) -> String {
    //.extern_path(".google.protobuf.Any", "::prost_types::Any")
    let message_name_idiom_case = convert_case(message_name);
    match namespace {
        s if s.starts_with("com.ankama") => {
            let rust_namespace: Vec<String> = namespace.split('.').map(escape_indent).collect();
            let rust_namespace = rust_namespace.join("::");
            format!("{rust_namespace}::{message_name_idiom_case}")
        }
        s if s.starts_with("google.protobuf") => {
            format!("::prost_types::{message_name_idiom_case}")
        }
        _ => unreachable!()
    }
}

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let file_descriptor_set_path = PathBuf::from(&out_dir).join("file_descriptor_set.bin");

    let Crawler { dirs, files } = Crawler::start("./protos/");
    let mut prost_build = prost_build::Config::new();
    prost_build
        .file_descriptor_set_path(file_descriptor_set_path.clone())
        .retain_enum_prefix()
        .enable_type_names()
        .type_name_domain(["."], "type.ankama.com")
        .include_file(PathBuf::from(&out_dir).join("_include.rs"))
        .extern_path(".google.protobuf.Any", "::prost_types::Any")
        .compile_protos(&files, &dirs)
        .unwrap();
    

    let mut file = File::open(file_descriptor_set_path).unwrap();
    let mut file_descriptor_set_bytes = Vec::new();
    file.read_to_end(&mut file_descriptor_set_bytes).unwrap();
    let file_descriptor_set =
        prost_types::FileDescriptorSet::decode(&file_descriptor_set_bytes[..]).unwrap();

    let files = file_descriptor_set.file;
    let ite = files
        .into_iter()
        .map(|file| (file.package, file.message_type.into_iter().filter_map(|t| t.name)));
    
    let mut decode_map = phf_codegen::Map::new();
    for (namespace, message_names) in ite {
        let namespace = match namespace {
            Some(namespace) => namespace,
            None => continue,
        };
        let rust_namespace: Vec<String> = namespace.split('.').map(escape_indent).collect();
        let rust_namespace = rust_namespace.join("::");

        for message_name in message_names {
            let domain = leading_domain_for_namespace(&namespace);
            let type_url = format!("{domain}/{namespace}.{message_name}");

            let type_path = rust_path_for(&namespace, message_name);

            let func= format!("|bytes| Ok(Box::new({type_path}::decode(bytes)?))");
            decode_map.entry(type_url, &func);
        }
    }

    let mut file = io::BufWriter::new(File::create(PathBuf::from(out_dir).join("codegen.rs")).unwrap());
    write!(
        &mut file,
        "static DECODE_MAP: phf::Map<&'static str, fn(&[u8]) -> Result<Box<dyn Message>, DecodeError>> = {}",
        decode_map
            .build()
    )
    .unwrap();
    write!(&mut file, ";\n").unwrap();
}
