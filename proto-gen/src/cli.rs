use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
pub struct Args {
    /// Path to dump.cs file.
    pub dump_file_path: String,

    /// Regex to match namespace(s) to filter.
    pub namespace_filter: Regex,

    /// Output directory for .proto files.
    #[arg(short, long, default_value_t = String::from("./protos/"))]
    pub output: String,
}
