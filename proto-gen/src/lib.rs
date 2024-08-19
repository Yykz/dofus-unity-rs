use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub(crate) struct CSharpParser;

mod protobuf;
pub(crate) use protobuf::ProtoEntity;

pub(crate) mod parser_items;
mod generator;
pub use generator::Generator;