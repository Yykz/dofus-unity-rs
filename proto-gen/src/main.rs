use proto_gen::{ParsedFile, ProtoPackages};

fn main() {
    let file = ParsedFile::try_from_dump(include_str!("../dump.cs")).unwrap();
    let packages = ProtoPackages::from_parsed(file, "Com.Ankama.Dofus.Server");
    packages.write_to_dir("./protos/");
}
