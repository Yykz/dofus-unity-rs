pub(crate) mod parser {
    pub mod common;
    pub mod data_structs;
    pub mod file;
    pub use data_structs::DataStruct;
    pub mod error;
    pub use error::Error;

    use pest_derive::Parser;

    pub type Pair<'a> = pest::iterators::Pair<'a, Rule>;

    #[derive(Parser)]
    #[grammar = "grammar.pest"]
    pub struct CSharpParser;
}

pub(crate) mod protobuf {
    macro_rules! impl_display_via_display_with_tabs {
        ($t:ty) => {
            impl ::std::fmt::Display for $t {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    <$t as DisplayWithTabs>::display_with_tabs(self, f, 0)
                }
            }
        };
    }

    pub mod builders;
    pub mod r#enum;
    pub mod message;
    pub mod package;

    pub trait DisplayWithTabs {
        fn display_with_tabs(
            &self,
            f: &mut std::fmt::Formatter<'_>,
            tabs_count: usize,
        ) -> std::fmt::Result;
    }
}

pub use parser::file::ParsedFile;
pub use protobuf::package::ProtoPackages;
