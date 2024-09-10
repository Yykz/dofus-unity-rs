mod cli;

use std::fs;

use clap::Parser;
use proto_gen::{ParsedFile, ProtoPackages};

use cli::Args;

fn main() {
    let args = Args::parse();

    let Args {
        dump_file_path,
        output,
        namespace_filter,
    } = args;
    let dump_content = fs::read_to_string(dump_file_path).unwrap();
    let file = ParsedFile::try_from_dump(&dump_content).unwrap();
    let packages = ProtoPackages::from_parsed(file, namespace_filter);
    packages.write_to_dir(output);
}
