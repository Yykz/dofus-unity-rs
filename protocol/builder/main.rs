use std::{
    env,
    fs::File,
    io::{BufWriter, Read},
    path::PathBuf,
};

mod crawler;
use crawler::Crawler;
mod module_node;
use module_node::ModuleNode;
use prost::Message;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let file_descriptor_set_path = PathBuf::from(&out_dir).join("file_descriptor_set.bin");

    prost_gen(file_descriptor_set_path.clone());

    let mut file = File::open(file_descriptor_set_path).unwrap();
    let mut file_descriptor_set_bytes = Vec::new();
    file.read_to_end(&mut file_descriptor_set_bytes).unwrap();
    let file_descriptor_set =
        prost_types::FileDescriptorSet::decode(&file_descriptor_set_bytes[..]).unwrap();

    let mut include_file =
        BufWriter::new(File::create(PathBuf::from(&out_dir).join("_include.rs")).unwrap());
    ModuleNode::from(file_descriptor_set).generate_include(&mut include_file)
}

fn prost_gen(file_descriptor_set_path: PathBuf) {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let Crawler { dirs, files } = Crawler::start("../protos/");
    let mut prost_build = prost_build::Config::new();
    let protos_out = PathBuf::from(&out_dir).join("protos/");
    std::fs::create_dir_all(&protos_out).unwrap();
    prost_build
        .out_dir(protos_out)
        .file_descriptor_set_path(file_descriptor_set_path)
        .enable_type_names()
        .type_name_domain(["."], "type.ankama.com")
        .extern_path(".google.protobuf.Any", "::prost_types::Any")
        .compile_protos(&files, &dirs)
        .unwrap();
}
