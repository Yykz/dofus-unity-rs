use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub(crate) struct CSharpParser;

mod protobuf;
pub(crate) use protobuf::ProtoEntity;

mod generator;
pub(crate) mod parser_items;
pub use generator::Generator;
