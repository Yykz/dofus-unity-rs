use std::{fmt::Debug, hash::Hash};

use parser_items::{FileContent, File};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct CSharpParser;

mod protobuf;
pub use protobuf::ProtoEntity;

pub mod parser_items;
pub mod generator;
pub use generator::Generator;