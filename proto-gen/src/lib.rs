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

// TODO use is_none_or from std when stabilized
trait IsNoneOr<T> {
    #[allow(clippy::wrong_self_convention)]
    fn is_none_or_(self, f: impl FnOnce(T) -> bool) -> bool;
}

impl<T> IsNoneOr<T> for Option<T> {
    fn is_none_or_(self, f: impl FnOnce(T) -> bool) -> bool {
        match self {
            None => true,
            Some(x) => f(x),
        }
    }
}

pub use parser::file::ParsedFile;
pub use protobuf::package::ProtoPackages;
